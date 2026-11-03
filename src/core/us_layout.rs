// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! US layout for `main.switch_layout` (ADR-0090): the character each
//! physical key types, whatever the system layout is. Platforms map
//! their native key numbers to `PhysicalKey`.

/// A character key by position, named after W3C `KeyboardEvent.code`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalKey {
    Key(char), // `KeyA` to `KeyZ`, as 'a' to 'z'
    Digit(u8), // `Digit0` to `Digit9`
    Backquote,
    Minus,
    Equal,
    BracketLeft,
    BracketRight,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
}

/// Shifted digits, `Digit0` first.
const SHIFTED_DIGITS: &[u8; 10] = b")!@#$%^&*(";

/// Returns the character `key` types on the US layout. Caps Lock acts
/// on letters only. `None` for a letter or digit out of range.
#[must_use]
pub fn us_char(key: PhysicalKey, shift: bool, caps: bool) -> Option<char> {
    let (plain, shifted) = match key {
        PhysicalKey::Key(c) if c.is_ascii_lowercase() => {
            let upper = c.to_ascii_uppercase();

            return Some(if shift == caps { c } else { upper });
        }
        PhysicalKey::Key(_) => return None,
        PhysicalKey::Digit(d) => {
            let shifted = *SHIFTED_DIGITS.get(usize::from(d))?;

            (char::from(b'0' + d), char::from(shifted))
        }
        PhysicalKey::Backquote => ('`', '~'),
        PhysicalKey::Minus => ('-', '_'),
        PhysicalKey::Equal => ('=', '+'),
        PhysicalKey::BracketLeft => ('[', '{'),
        PhysicalKey::BracketRight => (']', '}'),
        PhysicalKey::Backslash => ('\\', '|'),
        PhysicalKey::Semicolon => (';', ':'),
        PhysicalKey::Quote => ('\'', '"'),
        PhysicalKey::Comma => (',', '<'),
        PhysicalKey::Period => ('.', '>'),
        PhysicalKey::Slash => ('/', '?'),
    };

    Some(if shift { shifted } else { plain })
}

#[cfg(test)]
#[path = "us_layout_tests.rs"]
mod tests;
