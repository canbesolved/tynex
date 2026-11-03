// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Evdev numbers to key positions.

use super::*;

#[test]
fn rows_and_symbols_map_by_position() {
    assert_eq!(physical(16), Some(PhysicalKey::Key('q')));
    assert_eq!(physical(25), Some(PhysicalKey::Key('p')));
    assert_eq!(physical(38), Some(PhysicalKey::Key('l')));
    assert_eq!(physical(50), Some(PhysicalKey::Key('m')));
    assert_eq!(physical(2), Some(PhysicalKey::Digit(1)));
    assert_eq!(physical(11), Some(PhysicalKey::Digit(0)));
    assert_eq!(physical(52), Some(PhysicalKey::Period));
    assert_eq!(physical(28), None); // Enter
    assert_eq!(physical(57), None); // Space
}
