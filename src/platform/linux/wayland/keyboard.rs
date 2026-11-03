// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Keyboard: xkb keymap, compose, Latin fallback, repeat, `core::Key`.

use std::fs::File;
use std::os::unix::fs::FileExt;
use std::time::{Duration, Instant};

use xkbcommon::xkb::{self, compose};

use crate::core::event::Key;
use crate::core::hotkey::{Binding, Code, Mods, Side};
use crate::platform::linux::wayland::keys::{
    self, combo, latin, to_key, word_move,
};

/// Keys that repeat while held: moving and deleting.
const REPEATED: &[Key] = &[
    Key::Up,
    Key::Down,
    Key::Left,
    Key::Right,
    Key::WordLeft,
    Key::WordRight,
    Key::Backspace,
    Key::Delete,
];

/// Translates raw key events into `core::Key` values.
pub(super) struct Keyboard {
    context: xkb::Context,
    state: Option<(xkb::Keymap, xkb::State)>,
    compose: Option<compose::State>,
    rate: u32,       // Repeats per second; 0 disables repeat
    delay: Duration, // Before the first repeat
    held: Option<(u32, Key, Instant)>, // Repeating key, next repeat time
    tap: Option<(u32, Side)>, // Modifier pressed with no key after it
}

impl Keyboard {
    pub fn new() -> Self {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let locale = ["LC_ALL", "LC_CTYPE", "LANG"]
            .iter()
            .find_map(|v| std::env::var_os(v).filter(|l| !l.is_empty()))
            .unwrap_or_else(|| "C".into());
        let compose = compose::Table::new_from_locale(
            &context,
            &locale,
            compose::COMPILE_NO_FLAGS,
        )
        .ok()
        .map(|table| compose::State::new(&table, compose::STATE_NO_FLAGS));

        Self {
            context,
            state: None,
            compose,
            rate: 25,
            delay: Duration::from_millis(600),
            held: None,
            tap: None,
        }
    }

    /// Loads the keymap sent by the compositor as `size` bytes of `fd`.
    pub fn load_keymap(&mut self, file: &File, size: usize) {
        let mut bytes = vec![0; size];

        if file.read_exact_at(&mut bytes, 0).is_err() {
            return;
        }
        while bytes.last() == Some(&0) {
            bytes.pop(); // Drop the trailing NUL
        }

        let Ok(text) = String::from_utf8(bytes) else {
            return;
        };
        let keymap = xkb::Keymap::new_from_string(
            &self.context,
            text,
            xkb::KEYMAP_FORMAT_TEXT_V1,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );

        self.state = keymap.map(|k| {
            let state = xkb::State::new(&k);

            (k, state)
        });
    }

    pub fn modifiers(
        &mut self,
        depressed: u32,
        latched: u32,
        locked: u32,
        group: u32,
    ) {
        if let Some((_, state)) = &mut self.state {
            state.update_mask(depressed, latched, locked, 0, 0, group);
        }
    }

    pub fn repeat_info(&mut self, rate: i32, delay: i32) {
        self.rate = u32::try_from(rate).unwrap_or(0);
        self.delay = Duration::from_millis(u64::try_from(delay).unwrap_or(0));
    }

    /// Handles a press of evdev `key`; returns the key to report.
    pub fn press(&mut self, key: u32, now: Instant) -> Option<Key> {
        let (keymap, state) = self.state.as_ref()?;
        let code = xkb::Keycode::new(key + 8);
        let sym = state.key_get_one_sym(code).raw();

        self.tap = keys::side(sym).map(|side| (key, side));
        if self.tap.is_some() {
            self.held = None;
            return None;
        }

        let text = match compose_step(self.compose.as_mut(), sym) {
            Compose::Pending => return None,
            Compose::Text(text) => Some(text),
            Compose::Plain => Some(state.key_get_utf8(code)),
        };
        let mods = keys::mods(state);
        let latin = latin(keymap, code);
        let plain = to_key(sym, text.as_deref(), latin)
            .map(|key| word_move(key, mods.has(Mods::CTRL)));
        let key_out = match plain {
            Some(Key::WordLeft | Key::WordRight) if mods == Mods::CTRL => plain,
            _ => combo(sym, mods, latin).map(Key::Combo).or(plain),
        };

        self.held = key_out
            .filter(|k| REPEATED.contains(k) && self.rate > 0)
            .map(|k| (key, k, now + self.delay));
        key_out
    }

    /// Handles a release of evdev `key`. Returns a modifier tap when
    /// `key` is a modifier released with no other key pressed since.
    pub fn release(&mut self, key: u32) -> Option<Key> {
        if self.held.is_some_and(|(held, ..)| held == key) {
            self.held = None;
        }

        let (_, side) = self.tap.take_if(|(tapped, _)| *tapped == key)?;

        Some(Key::Combo(Binding {
            mods: Mods::default(),
            code: Code::Tap(side),
        }))
    }

    /// Stops repeating, for example when focus leaves the popup.
    pub fn stop(&mut self) {
        self.held = None;
        self.tap = None;
    }

    /// Returns when the held key repeats next, if any.
    pub fn next_repeat(&self) -> Option<Instant> {
        self.held.map(|(.., at)| at)
    }

    /// Returns the held key if its repeat time has come.
    pub fn tick(&mut self, now: Instant) -> Option<Key> {
        let (code, key, at) = self.held?;

        (at <= now).then(|| {
            let step = Duration::from_secs(1) / self.rate.max(1);

            self.held = Some((code, key, now + step));
            key
        })
    }
}

/// Outcome of feeding a keysym to the compose state.
enum Compose {
    Pending,      // Inside a sequence, or the sequence was cancelled
    Text(String), // A sequence finished, for example dead key + e = é
    Plain,        // Not part of a sequence: use the key's own text
}

/// Feeds `sym` to `compose`, if compose is available.
fn compose_step(compose: Option<&mut compose::State>, sym: u32) -> Compose {
    let Some(c) = compose else {
        return Compose::Plain;
    };

    if c.feed(sym.into()) != compose::FeedResult::Accepted {
        return Compose::Plain;
    }
    match c.status() {
        compose::Status::Composing => Compose::Pending,
        compose::Status::Composed => {
            let text = c.utf8();

            c.reset();
            text.map_or(Compose::Pending, Compose::Text)
        }
        compose::Status::Cancelled => {
            c.reset();
            Compose::Pending
        }
        compose::Status::Nothing => Compose::Plain,
    }
}

#[cfg(test)]
#[path = "keyboard_tests.rs"]
mod tests;
