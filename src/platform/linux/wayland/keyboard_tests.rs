// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Keyboard tests with real keymaps from the system's XKB data.

use super::*;

const KEY_A: u32 = 30; // evdev codes
const KEY_DOWN: u32 = 108;
const KEY_ENTER: u32 = 28;

/// A keyboard with `layouts` (for example "us,ru") and no compose.
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
    .expect("XKB data for us and ru");
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
    assert_eq!(
        to_key(keysyms::KEY_ISO_Left_Tab, None, None),
        Some(Key::Tab)
    );
    assert_eq!(to_key(keysyms::KEY_KP_Down, None, None), Some(Key::Down));
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
fn latin_key_with_russian_layout_active() {
    let mut kb = keyboard("us,ru");
    let now = Instant::now();

    kb.modifiers(0, 0, 0, 1); // Second layout: Russian
    assert_eq!(
        kb.press(KEY_A, now),
        Some(Key::Char {
            text: 'ф',
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
