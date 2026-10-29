// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Prompt mode key tests.

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

fn mode() -> PromptMode {
    prompt(false, launcher(LauncherMode::Off), Vec::new())
}

fn mode_with_apps(apps: Vec<AppInfo>) -> PromptMode {
    prompt(false, launcher(LauncherMode::All), apps)
}

fn prompt(
    title_bar: bool,
    launcher: LauncherConfig,
    apps: Vec<AppInfo>,
) -> PromptMode {
    PromptMode::new(Input::new(launcher, apps), title_bar, false, 19)
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

    mode().refresh(&mut s);
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

fn titles(s: &State) -> Vec<String> {
    s.view().rows().map(|r| r.title.clone()).collect()
}

#[test]
fn typing_filters_and_selects_the_best_match() {
    let mut mode = mode();
    let mut s = state();

    typed(&mut mode, &mut s, "cafe");
    assert_eq!(titles(&s), ["Café"]);
    assert_eq!(s.output[0].highlights.first(), Some(&(0..5)));
    assert_eq!(s.output[0].highlights.len(), 1);
    assert_eq!(
        mode.on_key(Key::Enter, &mut s),
        Action::Activate(WindowId("w2".into()))
    );
}

#[test]
fn backspace_widens_the_filter() {
    let mut mode = mode();
    let mut s = state();

    typed(&mut mode, &mut s, "kz"); // Too short for a typo
    assert_eq!(titles(&s), [] as [&str; 0]);
    mode.on_key(Key::Backspace, &mut s);
    assert_eq!(titles(&s), ["Kate"]);
}

#[test]
fn esc_clears_the_query_then_closes() {
    let mut mode = mode();
    let mut s = state();

    typed(&mut mode, &mut s, "kate");
    assert_eq!(mode.on_key(Key::Esc, &mut s), Action::Redraw);
    assert_eq!(s.query, "");
    assert_eq!(titles(&s).len(), 3);
    assert!(s.rows.iter().all(|r| r.highlights.is_empty()));
    assert_eq!(mode.on_key(Key::Esc, &mut s), Action::Close);
}

#[test]
fn enter_without_matches_does_nothing() {
    let mut mode = mode();
    let mut s = state();

    typed(&mut mode, &mut s, "zzzz");
    assert_eq!(mode.on_key(Key::Enter, &mut s), Action::None);
}

#[test]
fn app_names_match() {
    let mut mode = mode();
    let mut s = state();

    s.rows[0].app_name = "Configuración".into(); // Not in the title
    typed(&mut mode, &mut s, "configuración");
    assert_eq!(titles(&s), ["Release notes"]);
}

#[test]
fn enter_on_launch_row_returns_launch() {
    let apps = vec![AppInfo {
        id: "firefox".into(),
        name: "Firefox".into(),
        icon: None,
    }];
    let mut mode = mode_with_apps(apps);
    let mut s = State::new(Vec::new());

    typed(&mut mode, &mut s, "fire");
    assert_eq!(titles(&s), ["Firefox"]);
    assert_eq!(
        mode.on_key(Key::Enter, &mut s),
        Action::Launch("firefox".into())
    );
}

#[test]
fn cursor_edits_at_its_position() {
    let mut mode = mode();
    let mut s = state();

    typed(&mut mode, &mut s, "é");
    assert_eq!(s.cursor, "é".len());
    mode.on_key(Key::Left, &mut s);
    assert_eq!(s.cursor, 0);
    mode.on_key(Key::Right, &mut s);
    s.query = "é xx".into();
    s.cursor = s.query.len();
    mode.on_key(Key::WordLeft, &mut s);
    assert_eq!(s.cursor, "é ".len());
    mode.on_key(Key::Home, &mut s);
    mode.on_key(Key::Delete, &mut s);
    assert_eq!(s.query, " xx");
    mode.on_key(Key::End, &mut s);
    mode.on_key(Key::Left, &mut s);
    mode.on_key(Key::Backspace, &mut s);
    assert_eq!(s.query, " x");
    mode.on_key(Key::WordRight, &mut s);
    assert_eq!(s.cursor, s.query.len());
}
