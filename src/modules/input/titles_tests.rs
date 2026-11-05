// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The title list follows the history, lists tabs, and shows a browser
//! window as its active tab.

use super::titles;
use crate::core::browser_tabs::{SourceId, TabInfo, TabList, TabRef};
use crate::core::places::Place;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;

fn window(id: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(WindowId(id.into())),
        app_id: id.into(),
        app_name: id.into(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn tab(id: i32, active: bool, title: &str) -> TabInfo {
    TabInfo {
        id,
        window: 1,
        active,
        last: 1,
        title: title.into(),
        url: format!("https://{}.example", title.to_lowercase()),
        icon: String::new(),
    }
}

/// Brave with window `brave`: Mail is active, GitHub is not.
fn brave() -> TabList {
    TabList {
        source: SourceId(1),
        pid: 7,
        browser: "Brave".into(),
        icon: String::new(),
        tabs: vec![tab(1, true, "Mail"), tab(2, false, "GitHub")],
        windows: vec![WindowId("brave".into())],
    }
}

fn tab_place(id: i32) -> Place {
    Place::Tab {
        tab: TabRef {
            source: SourceId(1),
            id,
        },
        pid: 7,
        window: Some(WindowId("brave".into())),
    }
}

fn window_place(id: &str) -> Place {
    Place::Window(WindowId(id.into()))
}

fn titles_of(rows: &[Row]) -> Vec<&str> {
    rows.iter().map(|row| row.title.as_str()).collect()
}

/// Row order from `order`: the previous window first, the active last.
fn rows() -> Vec<Row> {
    vec![
        window("kate", "notes.txt"),
        window("brave", "Mail - Brave"),
        window("cursor", "main.rs"),
    ]
}

#[test]
fn without_history_a_browser_window_is_its_active_tab() {
    let out = titles(&rows(), &[], &[brave()]);

    assert_eq!(
        titles_of(&out),
        ["notes.txt", "Mail — mail.example", "main.rs"]
    );
    assert!(matches!(out[1].target, RowTarget::Tab(_)));
}

#[test]
fn the_history_comes_first_with_the_current_place_last() {
    // Newest first: Cursor now, GitHub before, then Kate.
    let history = [window_place("cursor"), tab_place(2), window_place("kate")];
    let out = titles(&rows(), &history, &[brave()]);

    assert_eq!(
        titles_of(&out),
        [
            "GitHub — github.example",
            "notes.txt",
            "Mail — mail.example",
            "main.rs",
        ]
    );
}

#[test]
fn a_tab_is_listed_once_and_unknown_tabs_are_skipped() {
    let history = [window_place("kate"), tab_place(1), tab_place(9)];
    let out = titles(&rows(), &history, &[brave()]);

    // Mail stands for the Brave window, so the window is not listed.
    assert_eq!(
        titles_of(&out),
        ["Mail — mail.example", "notes.txt", "main.rs"]
    );

    // Before the list came, the browser window is itself.
    assert_eq!(titles_of(&titles(&rows(), &history, &[])), {
        ["notes.txt", "Mail - Brave", "main.rs"]
    });
}
