// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The pointer cursor over the popup, set by name with
//! `cursor-shape-v1`.

use shape::wp_cursor_shape_device_v1::{Shape, WpCursorShapeDeviceV1};
use shape::wp_cursor_shape_manager_v1::WpCursorShapeManagerV1;
use wayland_client::globals::GlobalList;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::{QueueHandle, delegate_noop};
use wayland_protocols::wp::cursor_shape::v1::client as shape;

use crate::platform::linux::wayland::state::Wl;

/// The cursor shape manager and the device of the current pointer.
#[derive(Default)]
pub(super) struct Cursor {
    manager: Option<WpCursorShapeManagerV1>,
    device: Option<(WlPointer, WpCursorShapeDeviceV1)>,
}

impl Cursor {
    /// Binds the manager. Without it the compositor keeps its cursor.
    pub fn bind(globals: &GlobalList, qh: &QueueHandle<Wl>) -> Self {
        Self {
            manager: globals.bind(qh, 1..=1, ()).ok(),
            device: None,
        }
    }

    /// Shows the default arrow; called on pointer enter, whose
    /// `serial` the compositor checks.
    pub fn show(
        &mut self,
        pointer: &WlPointer,
        serial: u32,
        qh: &QueueHandle<Wl>,
    ) {
        let Some(manager) = &self.manager else {
            return;
        };

        if self.device.as_ref().is_none_or(|(p, _)| p != pointer) {
            let device = manager.get_pointer(pointer, qh, ());

            if let Some((_, old)) =
                self.device.replace((pointer.clone(), device))
            {
                old.destroy();
            }
        }
        if let Some((_, device)) = &self.device {
            device.set_shape(serial, Shape::Default);
        }
    }
}

delegate_noop!(Wl: ignore WpCursorShapeManagerV1);
delegate_noop!(Wl: ignore WpCursorShapeDeviceV1);
