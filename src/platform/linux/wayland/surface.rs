// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Layer-shell surface setup, commit and teardown.

use layer_shell::zwlr_layer_shell_v1::Layer;
use layer_shell::zwlr_layer_surface_v1::{Anchor, KeyboardInteractivity};
use wayland_client::QueueHandle;
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_shm::Format;
use wayland_protocols_wlr::layer_shell::v1::client as layer_shell;

use crate::config::Position;
use crate::platform::linux::wayland::buffer::Buffers;
use crate::platform::linux::wayland::state::{Popup, Wl};
use crate::ui::Size;
use crate::ui::place;
use crate::ui::render::canvas::Canvas;

/// Creates the popup: overlay layer, anchored to the top and centered
/// horizontally, exclusive keyboard. It is mapped after the first
/// configure and buffer commit.
pub(super) fn create(wl: &Wl, qh: &QueueHandle<Wl>) -> Popup {
    let surface = wl.compositor.create_surface(qh, ());
    let viewport = wl
        .viewporter
        .as_ref()
        .map(|v| v.get_viewport(&surface, qh, ()));
    let fractional = wl
        .fractional
        .as_ref()
        .map(|f| f.get_fractional_scale(&surface, qh, ()));
    let layer = wl.layer_shell.get_layer_surface(
        &surface,
        None,
        Layer::Overlay,
        "tynex".into(),
        qh,
        (),
    );

    layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    layer.set_anchor(Anchor::Top | Anchor::Left | Anchor::Right);
    layer.set_exclusive_zone(-1);
    Popup {
        surface,
        layer,
        viewport,
        fractional,
        configured: false,
        sent_margin: None,
        sent_size: None,
    }
}

/// Sends the top margin and the size. Unchanged values are not sent,
/// so a configure does not cause another one. `anchor` is the logical
/// height locked at open, not the current height.
pub(super) fn place(
    popup: &mut Popup,
    size: Size,
    scale: f32,
    position: Position,
    screen: Option<u32>,
    anchor: Option<u32>,
) {
    let logical_size = logical(size, scale);
    let top = place::margin(position, screen, scale, anchor);

    if popup.sent_margin != Some(top) {
        popup.layer.set_margin(top, 0, 0, 0);
        popup.sent_margin = Some(top);
    }
    if popup.sent_size != Some(logical_size) {
        popup.layer.set_size(logical_size.0, logical_size.1);
        popup.sent_size = Some(logical_size);
    }
}

/// Attaches `buffer` of `size` physical pixels and commits.
pub(super) fn commit(popup: &Popup, buffer: &WlBuffer, size: Size, scale: f32) {
    let s = &popup.surface;

    s.attach(Some(buffer), 0, 0);
    s.damage_buffer(0, 0, size.width as i32, size.height as i32);
    match &popup.viewport {
        Some(viewport) => {
            let (w, h) = logical(size, scale);

            viewport.set_destination(w as i32, h as i32);
        }
        None => s.set_buffer_scale(scale.round().max(1.0) as i32),
    }
    s.commit();
}

/// Destroys the popup's objects in reverse order of creation.
pub(super) fn destroy(popup: Popup) {
    popup.layer.destroy();
    if let Some(f) = popup.fractional {
        f.destroy();
    }
    if let Some(v) = popup.viewport {
        v.destroy();
    }
    popup.surface.destroy();
}

/// Returns the logical (surface) size for `size` physical pixels.
pub(super) fn logical(size: Size, scale: f32) -> (u32, u32) {
    let s = scale.max(1.0);

    (
        (size.width as f32 / s).ceil() as u32,
        (size.height as f32 / s).ceil() as u32,
    )
}

/// Picks `Abgr8888` (same byte order as the renderer) when offered.
pub(super) fn format(formats: &[Format]) -> Format {
    if formats.contains(&Format::Abgr8888) {
        Format::Abgr8888
    } else {
        Format::Argb8888
    }
}

/// Draws a frame with `paint` and commits it.
///
/// Returns false before configure or while both buffers are busy.
pub(super) fn draw(
    wl: &mut Wl,
    qh: &QueueHandle<Wl>,
    size: Size,
    paint: impl FnOnce(&mut Canvas<'_>),
) -> bool {
    let Some(popup) = wl.popup.as_ref().filter(|p| p.configured) else {
        return false;
    };
    let (width, height) = (size.width.max(1), size.height.max(1));
    let format = format(&wl.formats);

    if wl
        .buffers
        .as_ref()
        .is_none_or(|b| (b.width, b.height) != (width, height))
    {
        wl.buffers = Buffers::new(&wl.shm, qh, (width, height), format).ok();
    }

    let Some(buffers) = &mut wl.buffers else {
        return false;
    };
    let Some((i, pixels)) = buffers.free() else {
        wl.frame_pending = true;
        return false;
    };
    let Some(mut canvas) = Canvas::new(pixels, width, height) else {
        return false;
    };

    paint(&mut canvas);
    if format == Format::Argb8888 {
        // Memory order B, G, R, A: swap red and blue.
        pixels
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .for_each(|px| px.swap(0, 2));
    }
    commit(popup, buffers.buffer(i), size, wl.scale);
    buffers.busy[i] = true;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_size_rounds_up() {
        let size = Size {
            width: 601,
            height: 300,
        };

        assert_eq!(logical(size, 1.0), (601, 300));
        assert_eq!(logical(size, 1.5), (401, 200));
        assert_eq!(logical(size, 2.0), (301, 150));
    }

    #[test]
    fn prefers_renderer_byte_order() {
        assert_eq!(
            format(&[Format::Argb8888, Format::Abgr8888]),
            Format::Abgr8888
        );
        assert_eq!(format(&[Format::Argb8888]), Format::Argb8888);
    }
}
