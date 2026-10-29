// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `Space` confirms the selected row. `Tab` walks the visible list.

use super::*;
use crate::config::{LauncherConfig, LauncherMode};
use crate::core::app::AppInfo;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn launcher(mode: LauncherMode) -> LauncherConfig {
    LauncherConfig {
        mode,
        ..LauncherConfig::default()
    }
}

fn prompt(
    title_bar: bool,
    launcher: LauncherConfig,
    apps: Vec<AppInfo>,
) -> PromptMode {
    PromptMode::new(Input::new(launcher, apps), title_bar)
}

fn space() -> Key {
    Key::Char {
        text: ' ',
        latin: None,
    }
}

fn state() -> State {
    let rows = [("w1", "Release notes"), ("w2", "Café"), ("w3", "Kate")]
        .into_iter()
        .map(|(id, title)| Row {
            target: RowTarget::Window(WindowId(id.into())),
            app_id: String::new(),
            app_name: String::new(),
            icon: None,
            title: title.into(),
            title_color: None,
            highlights: Vec::new(),
        })
        .collect();
    let mut s = State::new(rows);

    prompt(false, launcher(LauncherMode::Off), Vec::new()).refresh(&mut s);
    s
}

fn typed(mode: &mut PromptMode, s: &mut State, text: &str) {
    for c in text.chars() {
        mode.on_key(
            Key::Char {
                text: c,
                latin: None,
            },
            s,
        );
    }
}

#[test]
fn space_activates_the_selected_window() {
    let mut mode = prompt(false, launcher(LauncherMode::Off), Vec::new());
    let mut s = state();

    typed(&mut mode, &mut s, "cafe");
    assert_eq!(
        mode.on_key(space(), &mut s),
        Action::Activate(WindowId("w2".into()))
    );
    assert_eq!(s.query, "cafe");
}

#[test]
fn space_on_an_empty_query_activates_the_selection() {
    let mut mode = prompt(false, launcher(LauncherMode::Off), Vec::new());
    let mut s = state();

    assert_eq!(
        mode.on_key(space(), &mut s),
        Action::Activate(WindowId("w1".into()))
    );
    assert_eq!(s.query, "");
}

#[test]
fn space_launches_the_selected_app() {
    let apps = vec![AppInfo {
        id: "firefox".into(),
        name: "Firefox".into(),
        icon: None,
    }];
    let mut mode = prompt(false, launcher(LauncherMode::All), apps);
    let mut s = State::new(Vec::new());

    typed(&mut mode, &mut s, "fire");
    assert_eq!(
        mode.on_key(space(), &mut s),
        Action::Launch("firefox".into())
    );
    assert_eq!(s.query, "fire");
}

#[test]
fn space_without_matches_leaves_the_query() {
    let mut mode = prompt(false, launcher(LauncherMode::Off), Vec::new());
    let mut s = state();

    typed(&mut mode, &mut s, "zzzz");
    assert_eq!(mode.on_key(space(), &mut s), Action::None);
    assert_eq!(s.query, "zzzz");
}

#[test]
fn tab_walks_the_title_bar_and_wraps() {
    let mut mode = prompt(true, launcher(LauncherMode::Off), Vec::new());
    let mut s = state();

    assert_eq!(mode.on_key(Key::Tab, &mut s), Action::Redraw);
    assert_eq!(s.selected, 1);
    mode.on_key(Key::Tab, &mut s);
    mode.on_key(Key::Tab, &mut s);
    assert_eq!(s.selected, 0);
}

#[test]
fn tab_walks_output_matches() {
    let apps = vec![
        AppInfo {
            id: "firefox".into(),
            name: "Firefox".into(),
            icon: None,
        },
        AppInfo {
            id: "files".into(),
            name: "Files".into(),
            icon: None,
        },
    ];

    for title_bar in [false, true] {
        let mut mode =
            prompt(title_bar, launcher(LauncherMode::All), apps.clone());
        let mut s = State::new(Vec::new());

        typed(&mut mode, &mut s, "f");
        assert_eq!(s.output.len(), 2);
        assert_eq!(mode.on_key(Key::Tab, &mut s), Action::Redraw);
        assert_eq!(s.selected, 1);
        mode.on_key(Key::Tab, &mut s);
        assert_eq!(s.selected, 0);
    }
}

#[test]
fn tab_does_nothing_without_a_visible_list() {
    let mut mode = prompt(false, launcher(LauncherMode::Off), Vec::new());
    let mut s = state();

    assert_eq!(mode.on_key(Key::Tab, &mut s), Action::None);
    assert_eq!(s.selected, 0);
}
