// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for the daily `qs` and popup call counts.

use super::*;
use crate::core::window::WindowId;

const DAY: &str = "2026-11-05";

/// Pending counts: `qs` switches and popup calls on `DAY`.
fn pending(qs: usize, show: usize) -> Pending {
    let mut pending = Pending::default();

    for _ in 0..qs {
        pending.add(DAY, Counter::Qs);
    }
    for _ in 0..show {
        pending.add(DAY, Counter::Show);
    }
    pending
}

#[test]
fn every_popup_call_counts_but_only_a_qs_that_switched() {
    let switch = Action::Activate(WindowId("prev".into()));

    assert_eq!(Counter::of(&Event::Show), Some(Counter::Show));
    assert_eq!(Counter::of(&Event::QuickSwitch), Some(Counter::Qs));
    assert_eq!(Counter::of(&Event::Hide), None);
    assert!(Counter::Show.counts(&Action::Redraw));
    assert!(Counter::Qs.counts(&switch));
    assert!(!Counter::Qs.counts(&Action::None));
}

#[test]
fn the_first_write_starts_the_file() {
    let mut counts = pending(1, 2);

    assert_eq!(counts.apply(""), "2026-11-05 qs 1 show 2\n");
    assert!(counts.is_empty());
}

#[test]
fn the_same_day_adds_up() {
    let text = "2026-11-04 qs 7 show 1\n2026-11-05 qs 2 show 5\n";

    assert_eq!(
        pending(1, 3).apply(text),
        "2026-11-04 qs 7 show 1\n2026-11-05 qs 3 show 8\n"
    );
}

#[test]
fn counts_across_midnight_get_a_line_per_day() {
    let mut counts = pending(1, 0);

    counts.add("2026-11-06", Counter::Show);
    counts.add("2026-11-06", Counter::Show);
    assert_eq!(
        counts.apply("2026-11-05 qs 3 show 4\n"),
        "2026-11-05 qs 4 show 4\n2026-11-06 qs 0 show 2\n"
    );
}

#[test]
fn a_line_of_the_first_version_counts_as_qs() {
    assert_eq!(
        pending(0, 1).apply("2026-11-05 3\n"),
        "2026-11-05 qs 3 show 1\n"
    );
}

#[test]
fn a_broken_last_line_is_kept_and_a_new_one_starts() {
    assert_eq!(
        pending(1, 0).apply("2026-11-05 qs x\n\n"),
        "2026-11-05 qs x\n2026-11-05 qs 1 show 0\n"
    );
}

#[test]
fn dates_are_zero_padded() {
    assert_eq!(date(2026, 1, 9), "2026-01-09");
}
