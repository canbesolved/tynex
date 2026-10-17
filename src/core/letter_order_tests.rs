// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for letters and row order in the switcher.

use super::tests::{Filter, Letters, build, list, shown};
use super::*;

#[test]
fn key_order_keeps_rows_and_selects_the_previous_window() {
    let mut sw = Switcher::new(
        "asd",
        HotkeyAction::Hide,
        true,
        SortBy::Key,
        Letters,
        Filter,
    );
    let rows: [(String, _); 3] = [
        ("prev".into(), Some('a')),
        ("older".into(), Some('s')),
        ("active".into(), Some('d')),
    ];

    sw.handle(Event::Windows(list(&["active", "prev", "older"])));
    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(shown(&sw), rows);
    assert_eq!(sw.view().unwrap().selected, 0);

    // After switching to "older", rows stay; "active" is now selected.
    sw.handle(Event::Hide);
    sw.handle(Event::Windows(list(&["older", "active", "prev"])));
    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(shown(&sw), rows);
    assert_eq!(sw.view().unwrap().selected, 2);
}

#[test]
fn letters_freed_while_open_are_reused_after_close() {
    let mut sw = build("asdfh", HotkeyAction::Hide, Letters);

    sw.handle(Event::Windows(list(&["x", "a", "b"])));
    sw.handle(Event::Show(ModeKind::Hotkey)); // a, b, x get a, s, d

    // "b" closes, "n1" opens: it gets "f", not the freed "s".
    sw.handle(Event::Windows(list(&["n1", "a", "x"])));
    assert_eq!(
        shown(&sw),
        [
            ("a".into(), Some('a')),
            ("x".into(), Some('d')),
            ("n1".into(), Some('f')),
        ]
    );

    // Two more windows: the most recent gets "h", then none are left.
    sw.handle(Event::Windows(list(&["n3", "n2", "n1", "a", "x"])));
    assert_eq!(
        shown(&sw),
        [
            ("n2".into(), None),
            ("n1".into(), Some('f')),
            ("a".into(), Some('a')),
            ("x".into(), Some('d')),
            ("n3".into(), Some('h')),
        ]
    );

    // After closing, the held "s" goes to "n2".
    sw.handle(Event::Hide);
    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(shown(&sw)[0], ("n2".into(), Some('s')));
}
