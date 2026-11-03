// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Renaming: the edit state, live titles and the per-window store.

use super::*;
use crate::core::view::Row;

fn id(text: &str) -> WindowId {
    WindowId(text.into())
}

fn row(window: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(id(window)),
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn window(window: &str, title: &str) -> WindowInfo {
    WindowInfo {
        id: id(window),
        app_id: String::new(),
        app_name: String::new(),
        title: title.into(),
        icon: None,
        pid: None,
    }
}

fn typed(text: char) -> Key {
    Key::Char { text, latin: None }
}

#[test]
fn editing_starts_at_the_end_and_leaves_rows_alone() {
    let mut s = State::new(vec![row("w1", "Notes"), row("w2", "Mail")]);

    assert!(s.start_rename(&id("w1")));
    assert_eq!(s.rename.as_ref().map(|r| r.cursor), Some(5));

    let rename = s.rename.as_mut().unwrap();

    assert_eq!(rename.edit(typed('!')), Some(true));
    assert_eq!(rename.edit(Key::Home), Some(false));
    assert_eq!(rename.edit(Key::Tab), None);
    assert_eq!(rename.text, "Notes!");
    assert_eq!(s.rows[0].title, "Notes");
    assert_eq!(
        s.view().edit(&RowTarget::Window(id("w1"))),
        Some(("Notes!", 0))
    );
    assert_eq!(s.view().edit(&RowTarget::Window(id("w2"))), None);
    assert!(!s.start_rename(&id("w9")));
}

#[test]
fn cancel_drops_and_keep_hands_over() {
    let mut s = State::new(vec![row("w1", "Notes")]);

    s.start_rename(&id("w1"));
    s.rename.as_mut().unwrap().edit(Key::Backspace);
    s.cancel_rename();
    assert_eq!(s.rename, None);
    assert_eq!(s.renamed, None);

    s.start_rename(&id("w1"));
    s.rename.as_mut().unwrap().edit(typed('.'));
    s.keep_rename();
    assert_eq!(s.rename, None);
    assert_eq!(s.renamed, Some((id("w1"), "Notes.".into())));
}

#[test]
fn a_rename_lasts_until_the_window_closes() {
    let mut renames = Renames::default();
    let mut windows = vec![window("w1", "Inbox - Mail"), window("w2", "Kate")];

    renames.set(&mut windows, &id("w1"), "  Work mail ");
    assert_eq!(windows[0].title, "Work mail");

    let mut fresh =
        vec![window("w1", "Inbox (2) - Mail"), window("w2", "Kate")];

    renames.apply(&mut fresh);
    assert_eq!(fresh[0].title, "Work mail");

    let mut closed = vec![window("w2", "Kate")];
    let mut reopened = vec![window("w1", "Inbox - Mail")];

    renames.apply(&mut closed);
    renames.apply(&mut reopened);
    assert_eq!(reopened[0].title, "Inbox - Mail");
}

#[test]
fn a_blank_or_own_title_removes_the_rename() {
    let mut renames = Renames::default();
    let mut windows = vec![window("w1", "Kate")];

    renames.set(&mut windows, &id("w1"), "Editor");
    renames.set(&mut windows, &id("w1"), "   ");
    assert_eq!(windows[0].title, "Kate");

    renames.set(&mut windows, &id("w1"), "Editor");
    renames.set(&mut windows, &id("w1"), "Kate");
    assert_eq!(windows[0].title, "Kate");

    let mut fresh = vec![window("w1", "Kate - file.rs")];

    renames.apply(&mut fresh);
    assert_eq!(fresh[0].title, "Kate - file.rs");
}
