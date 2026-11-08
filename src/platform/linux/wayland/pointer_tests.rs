// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hover waits for the first pointer move in the open popup.

use super::*;

const AT: (f64, f64) = (100.0, 40.0);

#[test]
fn an_enter_under_a_resting_cursor_is_not_a_move() {
    assert!(!moved_after(false, AT, Step::Enter));
}

#[test]
fn a_motion_in_place_is_not_a_move() {
    assert!(!moved_after(false, AT, Step::Motion(100.0, 40.0)));
    assert!(!moved_after(false, AT, Step::Motion(100.4, 39.7)));
}

#[test]
fn a_real_motion_is_a_move_and_it_stays() {
    assert!(moved_after(false, AT, Step::Motion(101.0, 40.0)));
    assert!(moved_after(false, AT, Step::Motion(100.0, 38.0)));
    assert!(moved_after(true, AT, Step::Motion(100.0, 40.0)));
    assert!(moved_after(true, AT, Step::Enter));
}

#[test]
fn leaving_the_popup_makes_the_next_enter_a_move() {
    let moved = moved_after(false, AT, Step::Leave);

    assert!(moved);
    assert!(moved_after(moved, AT, Step::Enter));
}
