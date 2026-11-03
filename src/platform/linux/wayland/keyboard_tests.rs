// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Keyboard tests with real keymaps from the system's XKB data.

use xkbcommon::xkb::keysyms;

use super::*;
use crate::core::hotkey::{Named, Side};

const KEY_A: u32 = 30; // evdev codes
const KEY_DOWN: u32 = 108;
const KEY_ENTER: u32 = 28;
const KEY_TAB: u32 = 15;
const KEY_DOT: u32 = 52;
const KEY_F2: u32 = 60;
const KEY_LEFT: u32 = 105;
const KEY_LEFTSHIFT: u32 = 42;
const SHIFT: u32 = 1; // Modifier masks of the default keymap
const CTRL: u32 = 4;
const ALT: u32 = 8;

/// A keyboard with `layouts` (for example "us,gr") and no compose.
fn keyboard(layouts: &str) -> Keyboard {
    let mut kb = Keyboard::new();
    let keymap = xkb::Keymap::new_from_names(
        &kb.context,
        "",
        "",
        layouts,
        "",
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS,
    )
    .expect("XKB data for us and gr");
    let state = xkb::State::new(&keymap);

    kb.state = Some((keymap, state));
    kb.compose = None;
    kb
}

#[test]
fn special_keys_map_by_keysym() {
    assert_eq!(
        to_key(keysyms::KEY_Return, Some("\r"), None),
        Some(Key::Enter)
    );
    assert_eq!(
        to_key(keysyms::KEY_Escape, Some("\u{1b}"), None),
        Some(Key::Esc)
    );
    assert_eq!(to_key(keysyms::KEY_Tab, None, None), Some(Key::Tab));
    assert_eq!(
        to_key(keysyms::KEY_ISO_Left_Tab, None, None),
        Some(Key::BackTab)
    );
    assert_eq!(to_key(keysyms::KEY_KP_Down, None, None), Some(Key::Down));
    assert_eq!(to_key(keysyms::KEY_Left, None, None), Some(Key::Left));
    assert_eq!(
        to_key(keysyms::KEY_KP_Delete, None, None),
        Some(Key::Delete)
    );
    assert_eq!(word_move(Key::Left, true), Key::WordLeft);
    assert_eq!(word_move(Key::Right, false), Key::Right);
}

#[test]
fn text_must_be_one_printable_char() {
    let key = |t| to_key(0x61, Some(t), Some('a'));

    assert_eq!(
        key("a"),
        Some(Key::Char {
            text: 'a',
            latin: Some('a')
        })
    );
    assert_eq!(key("\u{1}"), None); // Ctrl+A
    assert_eq!(key("ab"), None);
    assert_eq!(to_key(0x61, None, None), None);
}

#[test]
fn latin_key_with_greek_layout_active() {
    let mut kb = keyboard("us,gr");
    let now = Instant::now();

    kb.modifiers(0, 0, 0, 1); // Second layout: Greek
    assert_eq!(
        kb.press(KEY_A, now),
        Some(Key::Char {
            text: 'α',
            latin: Some('a')
        })
    );
    kb.modifiers(0, 0, 0, 0);
    assert_eq!(
        kb.press(KEY_A, now),
        Some(Key::Char {
            text: 'a',
            latin: Some('a')
        })
    );
    assert_eq!(kb.press(KEY_ENTER, now), Some(Key::Enter));
}

#[test]
fn shift_tab_is_back_tab() {
    let mut kb = keyboard("us");
    let now = Instant::now();

    assert_eq!(kb.press(KEY_TAB, now), Some(Key::Tab));
    kb.modifiers(1, 0, 0, 0); // Shift
    assert_eq!(kb.press(KEY_TAB, now), Some(Key::BackTab));
}

#[test]
fn arrows_repeat_until_released() {
    let mut kb = keyboard("us");
    let now = Instant::now();

    kb.repeat_info(10, 500);
    assert_eq!(kb.press(KEY_DOWN, now), Some(Key::Down));
    assert_eq!(kb.next_repeat(), Some(now + Duration::from_millis(500)));
    assert_eq!(kb.tick(now + Duration::from_millis(499)), None);

    let later = now + Duration::from_millis(500);

    assert_eq!(kb.tick(later), Some(Key::Down));
    assert_eq!(kb.next_repeat(), Some(later + Duration::from_millis(100)));
    kb.release(KEY_DOWN);
    assert_eq!(kb.next_repeat(), None);
}

#[test]
fn letters_do_not_repeat_and_rate_zero_disables_repeat() {
    let mut kb = keyboard("us");
    let now = Instant::now();

    kb.press(KEY_A, now);
    assert_eq!(kb.next_repeat(), None);
    kb.repeat_info(0, 500);
    kb.press(KEY_DOWN, now);
    assert_eq!(kb.next_repeat(), None);
}

fn combo(mods: Mods, code: Code) -> Key {
    Key::Combo(Binding { mods, code })
}

#[test]
fn ctrl_and_alt_letters_are_combos() {
    let mut kb = keyboard("us,gr");
    let now = Instant::now();
    let ctrl = Mods::CTRL;

    kb.modifiers(CTRL, 0, 0, 0);
    assert_eq!(kb.press(KEY_A, now), Some(combo(ctrl, Code::Char('a'))));
    kb.modifiers(CTRL, 0, 0, 1); // Greek layout: still the Latin letter
    assert_eq!(kb.press(KEY_A, now), Some(combo(ctrl, Code::Char('a'))));
    kb.modifiers(ALT, 0, 0, 0);
    assert_eq!(
        kb.press(KEY_DOT, now),
        Some(combo(Mods::ALT, Code::Char('.')))
    );
}

#[test]
fn ctrl_arrows_stay_word_moves_and_f_keys_are_combos() {
    let mut kb = keyboard("us");
    let now = Instant::now();

    kb.modifiers(CTRL, 0, 0, 0);
    assert_eq!(kb.press(KEY_LEFT, now), Some(Key::WordLeft));
    kb.modifiers(0, 0, 0, 0);
    assert_eq!(
        kb.press(KEY_F2, now),
        Some(combo(Mods::default(), Code::Named(Named::F(2))))
    );
    assert_eq!(
        kb.press(KEY_DOT, now),
        Some(Key::Char {
            text: '.',
            latin: None
        })
    );
}

#[test]
fn a_modifier_alone_is_a_tap_on_release() {
    let mut kb = keyboard("us");
    let now = Instant::now();
    let tap = Some(combo(Mods::default(), Code::Tap(Side::ShiftLeft)));

    assert_eq!(kb.press(KEY_LEFTSHIFT, now), None);
    assert_eq!(kb.release(KEY_LEFTSHIFT), tap);

    kb.press(KEY_LEFTSHIFT, now);
    kb.modifiers(SHIFT, 0, 0, 0);
    kb.press(KEY_A, now); // Shift+A types A: no tap
    assert_eq!(kb.release(KEY_A), None);
    assert_eq!(kb.release(KEY_LEFTSHIFT), None);
}
