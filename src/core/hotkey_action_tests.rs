// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for a hotkey press while the popup is open.

use super::tests::{Filter, build, list};
use super::*;
use crate::core::window::WindowId;

/// A switcher with `ids` open.
fn open(action: HotkeyAction, ids: &[&str]) -> Switcher<Filter> {
    let mut sw = build(action);

    sw.handle(Event::Windows(list(ids)));
    sw.handle(Event::Show);
    sw
}

#[test]
fn hide_hides_the_open_popup() {
    let mut sw = open(HotkeyAction::Hide, &["active", "prev"]);

    assert_eq!(sw.handle(Event::Show), Action::Close);
    assert!(sw.view().is_none());
}

#[test]
fn last_switches_to_the_previous_window() {
    let mut sw = open(HotkeyAction::Last, &["active", "prev", "old"]);

    assert_eq!(
        sw.handle(Event::Show),
        Action::Activate(WindowId("prev".into()))
    );
    assert!(sw.view().is_none());
}

#[test]
fn last_without_a_previous_window_hides() {
    let mut sw = open(HotkeyAction::Last, &["only"]);

    assert_eq!(sw.handle(Event::Show), Action::Close);
}

#[test]
fn a_second_press_works_after_typing() {
    let mut sw = open(HotkeyAction::Hide, &["active", "prev"]);

    sw.handle(Event::Key(Key::Char {
        text: 'p',
        latin: None,
    }));
    assert_eq!(sw.handle(Event::Show), Action::Close);
}
