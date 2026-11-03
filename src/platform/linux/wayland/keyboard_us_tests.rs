// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `main.switch_layout`: character keys read with the US layout.

use super::*;
use crate::core::hotkey::Mods;

const KEY_A: u32 = 30; // evdev codes
const KEY_Y: u32 = 21;
const KEY_GRAVE: u32 = 41;
const KEY_ENTER: u32 = 28;
const KEY_CAPSLOCK: u32 = 58;
const KEY_DOT: u32 = 52;
const SHIFT: u32 = 1; // Modifier masks of the system keymap
const LOCK: u32 = 2;
const CTRL: u32 = 4;

/// A keyboard with system `layouts` and `options`, reading US keys.
fn keyboard(layouts: &str, options: &str) -> Keyboard {
    let mut kb = Keyboard::new();
    let keymap = xkb::Keymap::new_from_names(
        &kb.context,
        "",
        "",
        layouts,
        "",
        Some(options.into()),
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .expect("XKB data for the test layouts");
    let state = xkb::State::new(&keymap);

    kb.state = Some((keymap, state));
    kb.compose = None;
    kb.set_us_layout(true);
    kb
}

fn char_key(text: char, latin: Option<char>) -> Key {
    Key::Char { text, latin }
}

#[test]
fn greek_layout_types_latin_letters() {
    let mut kb = keyboard("gr", "");
    let now = Instant::now();

    assert_eq!(kb.press(KEY_A, now), Some(char_key('a', Some('a'))));
    kb.modifiers(SHIFT, 0, 0, 0);
    assert_eq!(kb.press(KEY_A, now), Some(char_key('A', Some('a'))));
    kb.modifiers(0, 0, LOCK, 0); // Caps Lock
    assert_eq!(kb.press(KEY_A, now), Some(char_key('A', Some('a'))));
    kb.modifiers(0, 0, 0, 0);
    assert_eq!(kb.press(KEY_ENTER, now), Some(Key::Enter));
}

#[test]
fn second_layout_active_still_types_us_keys() {
    let mut kb = keyboard("us,gr", "");

    kb.modifiers(0, 0, 0, 1); // Greek
    assert_eq!(
        kb.press(KEY_A, Instant::now()),
        Some(char_key('a', Some('a')))
    );
}

#[test]
fn german_keys_and_dead_keys_follow_us_positions() {
    let mut kb = keyboard("de", "");
    let now = Instant::now();

    assert_eq!(kb.press(KEY_Y, now), Some(char_key('y', Some('y'))));
    // Dead circumflex in de, a backtick in us.
    assert_eq!(kb.press(KEY_GRAVE, now), Some(char_key('`', None)));
}

#[test]
fn remapped_special_keys_keep_the_system_meaning() {
    let mut kb = keyboard("gr", "caps:escape");

    assert_eq!(kb.press(KEY_CAPSLOCK, Instant::now()), Some(Key::Esc));
}

#[test]
fn combos_use_the_us_key() {
    let mut kb = keyboard("ru", "");
    let dot = Binding {
        mods: Mods::CTRL,
        code: Code::Char('.'),
    };

    kb.modifiers(CTRL, 0, 0, 0); // The key types `ю` in ru
    assert_eq!(kb.press(KEY_DOT, Instant::now()), Some(Key::Combo(dot)));
}
