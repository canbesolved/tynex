// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer motion and the left button, in surface coordinates.

use wayland_client::protocol::wl_pointer::{self, ButtonState, WlPointer};
use wayland_client::{Connection, Dispatch, QueueHandle, WEnum};

use crate::platform::linux::wayland::state::{PresenterEvent, Wl};

/// Linux `BTN_LEFT`.
const BTN_LEFT: u32 = 0x110;

impl Dispatch<WlPointer, ()> for Wl {
    fn event(
        s: &mut Self,
        _: &WlPointer,
        e: wl_pointer::Event,
        (): &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match e {
            wl_pointer::Event::Enter {
                surface_x,
                surface_y,
                ..
            }
            | wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                s.pointer_x = surface_x;
                s.pointer_y = surface_y;
            }
            wl_pointer::Event::Button { button, state, .. }
                if button == BTN_LEFT
                    && state == WEnum::Value(ButtonState::Pressed)
                    && s.popup.is_some() =>
            {
                s.events.push(PresenterEvent::Click {
                    x: s.pointer_x,
                    y: s.pointer_y,
                });
            }
            _ => {}
        }
    }
}
