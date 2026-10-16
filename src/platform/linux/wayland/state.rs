// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Wayland dispatch state: globals, the popup surface and its events.

use fractional::wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1;
use fractional::wp_fractional_scale_v1::{self, WpFractionalScaleV1};
use layer_shell::zwlr_layer_shell_v1::ZwlrLayerShellV1;
use layer_shell::zwlr_layer_surface_v1::{self, ZwlrLayerSurfaceV1};
use viewporter::wp_viewport::WpViewport;
use viewporter::wp_viewporter::WpViewporter;
use wayland_client::globals::GlobalListContents;
use wayland_client::protocol::wl_buffer::{self, WlBuffer};
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_keyboard::WlKeyboard;
use wayland_client::protocol::wl_registry::WlRegistry;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_shm::{self, Format, WlShm};
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use wayland_client::protocol::wl_surface::{self, WlSurface};
use wayland_client::{Connection, Dispatch, QueueHandle, WEnum, delegate_noop};
use wayland_protocols::wp::fractional_scale::v1::client as fractional;
use wayland_protocols::wp::viewporter::client as viewporter;
use wayland_protocols_wlr::layer_shell::v1::client as layer_shell;

use crate::core::event::Key;
use crate::platform::linux::wayland::buffer::Buffers;
use crate::platform::linux::wayland::keyboard::Keyboard;

/// Events for the caller, collected while dispatching.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PresenterEvent {
    Key(Key),   // A key press, or a repeat of a held key
    Configured, // The popup can be drawn
    Scale(f32), // The output scale changed; redraw at the new size
    Closed,     // The compositor closed the popup
}

/// The open popup surface.
pub(super) struct Popup {
    pub surface: WlSurface,
    pub layer: ZwlrLayerSurfaceV1,
    pub viewport: Option<WpViewport>,
    pub fractional: Option<WpFractionalScaleV1>,
    pub configured: bool,
}

/// Dispatch state shared with `wayland-client`.
pub(super) struct Wl {
    pub compositor: WlCompositor,
    pub shm: WlShm,
    pub layer_shell: ZwlrLayerShellV1,
    pub viewporter: Option<WpViewporter>,
    pub fractional: Option<WpFractionalScaleManagerV1>,
    pub _seat: Option<WlSeat>, // Kept bound for keyboard events
    pub keyboard_proxy: Option<WlKeyboard>,
    pub keyboard: Keyboard,
    pub formats: Vec<Format>,
    pub scale: f32,
    pub popup: Option<Popup>,
    pub buffers: Option<Buffers>,
    pub events: Vec<PresenterEvent>,
}

impl Dispatch<WlRegistry, GlobalListContents> for Wl {
    fn event(
        _: &mut Self,
        _: &WlRegistry,
        _: <WlRegistry as wayland_client::Proxy>::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // Globals are bound once at start; later changes are ignored.
    }
}

impl Dispatch<WlShm, ()> for Wl {
    fn event(
        s: &mut Self,
        _: &WlShm,
        e: wl_shm::Event,
        (): &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_shm::Event::Format {
            format: WEnum::Value(f),
        } = e
        {
            s.formats.push(f);
        }
    }
}

impl Dispatch<WlBuffer, usize> for Wl {
    fn event(
        s: &mut Self,
        _: &WlBuffer,
        e: wl_buffer::Event,
        &i: &usize,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let (wl_buffer::Event::Release, Some(b)) = (e, &mut s.buffers) {
            b.busy[i] = false;
        }
    }
}

impl Dispatch<WlSurface, ()> for Wl {
    fn event(
        s: &mut Self,
        _: &WlSurface,
        e: wl_surface::Event,
        (): &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // Integer scale, used only without the fractional scale protocol.
        if let wl_surface::Event::PreferredBufferScale { factor } = e
            && s.fractional.is_none()
        {
            s.set_scale(factor as f32);
        }
    }
}

impl Dispatch<WpFractionalScaleV1, ()> for Wl {
    fn event(
        s: &mut Self,
        _: &WpFractionalScaleV1,
        e: wp_fractional_scale_v1::Event,
        (): &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wp_fractional_scale_v1::Event::PreferredScale { scale } = e {
            s.set_scale(scale as f32 / 120.0);
        }
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for Wl {
    fn event(
        s: &mut Self,
        layer: &ZwlrLayerSurfaceV1,
        e: zwlr_layer_surface_v1::Event,
        (): &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match e {
            zwlr_layer_surface_v1::Event::Configure { serial, .. } => {
                layer.ack_configure(serial);
                if let Some(popup) = &mut s.popup {
                    popup.configured = true;
                }
                s.events.push(PresenterEvent::Configured);
            }
            zwlr_layer_surface_v1::Event::Closed => {
                s.events.push(PresenterEvent::Closed);
            }
            _ => {}
        }
    }
}

impl Wl {
    fn set_scale(&mut self, scale: f32) {
        if scale > 0.0 && (scale - self.scale).abs() > f32::EPSILON {
            self.scale = scale;
            self.events.push(PresenterEvent::Scale(scale));
        }
    }
}

delegate_noop!(Wl: ignore WlCompositor);
delegate_noop!(Wl: ignore WlShmPool);
delegate_noop!(Wl: ignore ZwlrLayerShellV1);
delegate_noop!(Wl: ignore WpViewporter);
delegate_noop!(Wl: ignore WpViewport);
delegate_noop!(Wl: ignore WpFractionalScaleManagerV1);
