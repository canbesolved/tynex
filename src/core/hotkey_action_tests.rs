// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for a hotkey press while the popup is open.

use super::tests::{Filter, Letters, build, list};
use super::*;
use crate::core::window::WindowId;

/// A switcher with `ids` open in hotkey mode.
fn open(action: HotkeyAction, ids: &[&str]) -> Switcher<Letters, Filter> {
    let mut sw = build("asd", action, Letters);

    sw.handle(Event::Windows(list(ids)));
    sw.handle(Event::Show(ModeKind::Hotkey));
    sw
}

#[test]
fn hide_hides_the_open_popup() {
    let mut sw = open(HotkeyAction::Hide, &["active", "prev"]);

    assert_eq!(sw.handle(Event::Show(ModeKind::Hotkey)), Action::Close);
    assert!(sw.view().is_none());
}

#[test]
fn last_switches_to_the_previous_window() {
    let mut sw = open(HotkeyAction::Last, &["active", "prev", "old"]);

    assert_eq!(
        sw.handle(Event::Show(ModeKind::Hotkey)),
        Action::Activate(WindowId("prev".into()))
    );
    assert!(sw.view().is_none());
}

#[test]
fn last_without_a_previous_window_hides() {
    let mut sw = open(HotkeyAction::Last, &["only"]);

    assert_eq!(sw.handle(Event::Show(ModeKind::Hotkey)), Action::Close);
}

#[test]
fn any_show_while_open_counts_as_a_second_press() {
    let mut sw = open(HotkeyAction::Hide, &["active", "prev"]);

    sw.handle(Event::Key(Key::Tab)); // Now in search mode
    assert_eq!(sw.handle(Event::Show(ModeKind::Search)), Action::Close);
}

#[test]
fn letters_held_while_open_are_released_on_a_second_press() {
    let mut sw = open(HotkeyAction::Hide, &["x", "a", "b"]); // b gets s

    sw.handle(Event::Windows(list(&["n", "x", "a"]))); // s held
    sw.handle(Event::Show(ModeKind::Hotkey));
    sw.handle(Event::Windows(list(&["m", "n", "x", "a"])));
    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(sw.view().unwrap().rows().next().unwrap().key, Some('s'));
}
