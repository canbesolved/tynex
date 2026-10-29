// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Seat and keyboard events, translated by `Keyboard`.

use std::fs::File;
use std::time::Instant;

use wayland_client::protocol::wl_keyboard::{
    self, KeyState, KeymapFormat, WlKeyboard,
};
use wayland_client::protocol::wl_seat::{self, Capability, WlSeat};
use wayland_client::{Connection, Dispatch, QueueHandle, WEnum};

use crate::platform::linux::wayland::state::{PresenterEvent, Wl};

impl Dispatch<WlSeat, ()> for Wl {
    fn event(
        s: &mut Self,
        seat: &WlSeat,
        e: wl_seat::Event,
        (): &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(caps),
        } = e
        else {
            return;
        };
        let has_keyboard = caps.contains(Capability::Keyboard);
        let has_pointer = caps.contains(Capability::Pointer);

        if has_keyboard && s.keyboard_proxy.is_none() {
            s.keyboard_proxy = Some(seat.get_keyboard(qh, ()));
        } else if !has_keyboard && let Some(keyboard) = s.keyboard_proxy.take()
        {
            keyboard.release();
        }
        if has_pointer && s.pointer_proxy.is_none() {
            s.pointer_proxy = Some(seat.get_pointer(qh, ()));
        } else if !has_pointer && let Some(pointer) = s.pointer_proxy.take() {
            pointer.release();
        }
    }
}

impl Dispatch<WlKeyboard, ()> for Wl {
    fn event(
        s: &mut Self,
        _: &WlKeyboard,
        e: wl_keyboard::Event,
        (): &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match e {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                if format == WEnum::Value(KeymapFormat::XkbV1) {
                    s.keyboard.load_keymap(&File::from(fd), size as usize);
                }
            }
            wl_keyboard::Event::Key { key, state, .. } => match state {
                WEnum::Value(KeyState::Pressed) => {
                    if let Some(k) = s.keyboard.press(key, Instant::now()) {
                        s.events.push(PresenterEvent::Key(k));
                    }
                }
                _ => s.keyboard.release(key),
            },
            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => s.keyboard.modifiers(
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            ),
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                s.keyboard.repeat_info(rate, delay);
            }
            wl_keyboard::Event::Leave { .. } => {
                s.keyboard.stop();
                if s.popup.is_some() {
                    s.events.push(PresenterEvent::FocusLost);
                }
            }
            _ => {}
        }
    }
}
