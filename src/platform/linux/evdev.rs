// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Linux evdev key numbers to `core::us_layout::PhysicalKey`. Wayland
//! and X11 both report evdev numbers (X11 adds 8).

use crate::core::us_layout::PhysicalKey;

/// Letter rows from `KEY_Q` (16), `KEY_A` (30) and `KEY_Z` (44).
const LETTER_ROWS: [(u32, &[u8]); 3] =
    [(16, b"qwertyuiop"), (30, b"asdfghjkl"), (44, b"zxcvbnm")];

/// Returns the character key at evdev number `code`, if it is one.
#[must_use]
pub fn physical(code: u32) -> Option<PhysicalKey> {
    let letter = LETTER_ROWS.iter().find_map(|&(first, row)| {
        let at = usize::try_from(code.checked_sub(first)?).ok()?;

        row.get(at).map(|&c| PhysicalKey::Key(char::from(c)))
    });

    if letter.is_some() {
        return letter;
    }
    Some(match code {
        2..=10 => PhysicalKey::Digit(u8::try_from(code - 1).ok()?), // 1-9
        11 => PhysicalKey::Digit(0),
        12 => PhysicalKey::Minus,
        13 => PhysicalKey::Equal,
        26 => PhysicalKey::BracketLeft,
        27 => PhysicalKey::BracketRight,
        39 => PhysicalKey::Semicolon,
        40 => PhysicalKey::Quote,
        41 => PhysicalKey::Backquote,
        43 => PhysicalKey::Backslash,
        51 => PhysicalKey::Comma,
        52 => PhysicalKey::Period,
        53 => PhysicalKey::Slash,
        _ => return None,
    })
}

#[cfg(test)]
#[path = "evdev_tests.rs"]
mod tests;
