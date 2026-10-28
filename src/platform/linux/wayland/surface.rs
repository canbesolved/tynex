// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Layer-shell surface setup, commit and teardown.

use layer_shell::zwlr_layer_shell_v1::Layer;
use layer_shell::zwlr_layer_surface_v1::KeyboardInteractivity;
use wayland_client::QueueHandle;
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_shm::Format;
use wayland_protocols_wlr::layer_shell::v1::client as layer_shell;

use crate::platform::linux::wayland::state::{Popup, Wl};
use crate::ui::Size;

/// Creates the popup: overlay layer, no anchors (centered), exclusive
/// keyboard. It is mapped after the first configure and buffer commit.
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
    Popup {
        surface,
        layer,
        viewport,
        fractional,
        configured: false,
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
