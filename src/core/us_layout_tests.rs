// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! US characters by key position, Shift and Caps Lock.

use super::*;

#[test]
fn letters_follow_shift_and_caps_lock() {
    let a = PhysicalKey::Key('a');

    assert_eq!(us_char(a, false, false), Some('a'));
    assert_eq!(us_char(a, true, false), Some('A'));
    assert_eq!(us_char(a, false, true), Some('A'));
    assert_eq!(us_char(a, true, true), Some('a'));
}

#[test]
fn caps_lock_leaves_digits_and_symbols() {
    assert_eq!(us_char(PhysicalKey::Digit(1), false, true), Some('1'));
    assert_eq!(us_char(PhysicalKey::Digit(1), true, false), Some('!'));
    assert_eq!(us_char(PhysicalKey::Digit(0), true, true), Some(')'));
    assert_eq!(us_char(PhysicalKey::Period, false, true), Some('.'));
    assert_eq!(us_char(PhysicalKey::Quote, true, false), Some('"'));
}

#[test]
fn out_of_range_keys_have_no_char() {
    assert_eq!(us_char(PhysicalKey::Key('A'), false, false), None);
    assert_eq!(us_char(PhysicalKey::Key('ä'), false, false), None);
    assert_eq!(us_char(PhysicalKey::Digit(10), false, false), None);
}
