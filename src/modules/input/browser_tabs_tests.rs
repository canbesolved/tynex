// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! A window row and its active tab row are not both listed.

use super::*;
use crate::config::LauncherConfig;
use crate::core::browser_tabs::{SourceId, TabRef};
use crate::core::state::State;
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn row(target: RowTarget, title: &str) -> Row {
    Row {
        target,
        app_id: "brave-browser".into(),
        app_name: "Brave".into(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn window(id: &str, title: &str) -> Row {
    row(RowTarget::Window(WindowId(id.into())), title)
}

fn tab_row(id: i32) -> Row {
    let target = RowTarget::Tab(TabRef {
        source: SourceId(1),
        id,
    });

    row(target, "tab")
}

/// Brave with windows `w1` and `w2`; tab 1 is active, tab 2 is not.
fn brave(title: &str) -> TabList {
    let tab = |id: i32, active: bool| TabInfo {
        id,
        window: 1,
        active,
        last: 1,
        title: title.into(),
        url: "https://notes.example".into(),
        icon: String::new(),
    };

    TabList {
        source: SourceId(1),
        pid: 4,
        browser: "Brave".into(),
        icon: "brave-browser".into(),
        tabs: vec![tab(1, true), tab(2, false)],
        windows: vec![WindowId("w1".into()), WindowId("w2".into())],
    }
}

fn targets(rows: &[Row]) -> Vec<&RowTarget> {
    rows.iter().map(|r| &r.target).collect()
}

#[test]
fn the_active_tab_row_replaces_its_window_row() {
    let lists = [brave("Notes")];

    for title in ["Notes - Brave", "Notes — Brave", "Notes"] {
        let out = join(vec![window("w1", title)], vec![tab_row(1)], &lists);

        assert_eq!(targets(&out), targets(&[tab_row(1)]), "{title}");
    }
}

#[test]
fn other_windows_stay() {
    let lists = [brave("Notes")];
    let kept = [
        window("w1", "Notes - Brave"), // Its tab row did not match
        window("w9", "Notes - Brave"), // Another process
        window("w2", "Notes2 - Brave"), // Only a prefix
        row(RowTarget::Launch("brave-browser".into()), "Brave"),
    ];

    assert_eq!(join(kept[..1].to_vec(), Vec::new(), &lists), kept[..1]);
    assert_eq!(join(kept[..1].to_vec(), vec![tab_row(2)], &lists).len(), 2);

    let tab = tab_row(1);
    let out = join(kept.to_vec(), vec![tab.clone()], &lists);

    assert_eq!(
        out,
        [kept[1].clone(), kept[2].clone(), tab, kept[3].clone()]
    );
}

#[test]
fn an_empty_tab_title_never_hides_a_window() {
    let lists = [brave("")];
    let out = join(vec![window("w1", " - Brave")], vec![tab_row(1)], &lists);

    assert_eq!(out.len(), 2);
}

#[test]
fn a_search_lists_the_tab_once() {
    let input = Input::new(LauncherConfig::default(), Vec::new());
    let mut state = State::new(vec![
        window("w1", "Notes - Brave"),
        window("w5", "Notes - Kate"),
    ]);

    state.tab_lists = vec![brave("Notes")];
    state.query = "notes".into();
    input.refresh(&mut state);

    let titles: Vec<&str> =
        state.output.iter().map(|r| r.title.as_str()).collect();

    assert_eq!(titles.len(), 3, "{titles:?}");
    assert!(!titles.contains(&"Notes - Brave"), "{titles:?}");
    assert!(titles.contains(&"Notes - Kate"), "{titles:?}");
}
