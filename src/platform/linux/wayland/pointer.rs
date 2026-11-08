// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer motion, the left button and vertical scrolling, in surface
//! coordinates. Hover waits for the first move in the open popup
//! (ADR-0101).

use wayland_client::protocol::wl_pointer::{
    self, Axis, ButtonState, WlPointer,
};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, WEnum};

use crate::platform::linux::wayland::state::{PresenterEvent, Wl};

/// Linux `BTN_LEFT`.
const BTN_LEFT: u32 = 0x110;

/// A motion shorter than this, in surface coordinates, is not a move.
const STILL: f64 = 0.5;

/// A pointer event that may count as a move.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Enter,
    Motion(f64, f64), // Surface coordinates
    Leave,
}

/// Returns whether the pointer has moved in the open popup after
/// `step`, from `moved` before it and the last position `at`. An
/// `enter` on a new popup is not a move: the cursor may rest there.
fn moved_after(moved: bool, at: (f64, f64), step: Step) -> bool {
    match step {
        Step::Enter => moved,
        Step::Motion(x, y) => {
            moved || (x - at.0).abs() >= STILL || (y - at.1).abs() >= STILL
        }
        Step::Leave => true, // It went out, so the next enter is a move
    }
}

impl Dispatch<WlPointer, ()> for Wl {
    fn event(
        s: &mut Self,
        pointer: &WlPointer,
        e: wl_pointer::Event,
        (): &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_pointer::Event::Enter { serial, .. } = e {
            s.cursor.show(pointer, serial, qh);
        }
        match e {
            wl_pointer::Event::Enter {
                surface_x,
                surface_y,
                ..
            } => s.point(Step::Enter, (surface_x, surface_y)),
            wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => s.point(
                Step::Motion(surface_x, surface_y),
                (surface_x, surface_y),
            ),
            wl_pointer::Event::Leave { .. } => {
                s.pointer_moved =
                    moved_after(s.pointer_moved, s.pointer_at, Step::Leave);
            }
            wl_pointer::Event::Button { button, state, .. }
                if button == BTN_LEFT
                    && state == WEnum::Value(ButtonState::Pressed)
                    && s.popup.is_some() =>
            {
                s.events.push(PresenterEvent::Click {
                    x: s.pointer_at.0,
                    y: s.pointer_at.1,
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
    /// Moves the pointer to `at` and reports hover once it has moved in
    /// the open popup.
    fn point(&mut self, step: Step, at: (f64, f64)) {
        self.pointer_moved =
            moved_after(self.pointer_moved, self.pointer_at, step);
        self.pointer_at = at;
        if self.popup.is_some() && self.pointer_moved {
            self.events.push(PresenterEvent::Hover { x: at.0, y: at.1 });
        }
    }

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

#[cfg(test)]
#[path = "pointer_tests.rs"]
mod tests;
