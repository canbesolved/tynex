// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer motion, the left button and vertical scrolling, in surface
//! coordinates.

use wayland_client::protocol::wl_pointer::{
    self, Axis, ButtonState, WlPointer,
};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, WEnum};

use crate::platform::linux::wayland::state::{PresenterEvent, Wl};

/// Linux `BTN_LEFT`.
const BTN_LEFT: u32 = 0x110;

impl Dispatch<WlPointer, ()> for Wl {
    fn event(
        s: &mut Self,
        pointer: &WlPointer,
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
                if s.popup.is_some() {
                    s.events.push(PresenterEvent::Hover {
                        x: surface_x,
                        y: surface_y,
                    });
                }
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
            wl_pointer::Event::Axis {
                axis: WEnum::Value(Axis::VerticalScroll),
                value,
                ..
            } => {
                s.wheel.1 += value;
                if pointer.version() < 5 {
                    s.flush_wheel(); // No frame events before version 5
                }
            }
            wl_pointer::Event::AxisDiscrete {
                axis: WEnum::Value(Axis::VerticalScroll),
                discrete,
            } => s.wheel.0 += discrete,
            wl_pointer::Event::Frame => s.flush_wheel(),
            _ => {}
        }
    }
}

impl Wl {
    /// Reports the scrolling of one pointer frame. A wheel sends notches
    /// and pixels for the same move; the notches win.
    fn flush_wheel(&mut self) {
        let (notches, dy) = std::mem::take(&mut self.wheel);

        if self.popup.is_none() || (notches == 0 && dy == 0.0) {
            return;
        }
        let dy = if notches == 0 { dy } else { 0.0 };

        self.events.push(PresenterEvent::Scroll { notches, dy });
    }
}
