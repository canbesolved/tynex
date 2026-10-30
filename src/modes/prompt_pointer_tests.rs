// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer selection in the title list and the dropdown.

use super::*;
use crate::config::{LauncherConfig, LauncherMode, TaskbarMode};
use crate::core::event::Click;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn mode(taskbar: TaskbarMode) -> PromptMode {
    PromptMode::new(
        Input::new(
            LauncherConfig {
                mode: LauncherMode::Off,
                ..LauncherConfig::default()
            },
            Vec::new(),
        ),
        taskbar,
        19,
    )
}

fn row(id: &str, app: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(WindowId(id.into())),
        app_id: app.into(),
        app_name: app.into(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn session(taskbar: TaskbarMode) -> (PromptMode, State) {
    let mut state = State::new(vec![
        row("w1", "firefox", "Notes"),
        row("w2", "firefox", "Mail"),
        row("w3", "kate", "File"),
    ]);

    state.icons = vec![
        AppIcon {
            app_id: "firefox".into(),
            icon: None,
            windows: vec![WindowId("w1".into()), WindowId("w2".into())],
        },
        AppIcon {
            app_id: "kate".into(),
            icon: None,
            windows: vec![WindowId("w3".into())],
        },
    ];
    state.select_icon(Some("firefox"));
    (mode(taskbar), state)
}

#[test]
fn hovering_a_title_selects_it_and_a_click_activates() {
    let (mut mode, mut state) = session(TaskbarMode::Titlebar);

    assert_eq!(mode.on_hover(Click::Row(2), &mut state), Action::Redraw);
    assert_eq!(state.selected, 2);
    assert_eq!(mode.on_hover(Click::Row(2), &mut state), Action::None);
    assert_eq!(mode.on_hover(Click::Elsewhere, &mut state), Action::None);
    assert_eq!(state.selected, 2);
    assert_eq!(
        mode.on_click(Click::Row(1), &mut state),
        Action::Activate(WindowId("w2".into()))
    );
}

#[test]
fn a_title_click_works_with_the_icon_strip() {
    let (mut mode, mut state) = session(TaskbarMode::Both);

    assert_eq!(mode.on_hover(Click::Row(1), &mut state), Action::Redraw);
    assert_eq!(
        mode.on_click(Click::Row(1), &mut state),
        Action::Activate(WindowId("w2".into()))
    );
}

#[test]
fn typing_ignores_a_title_row() {
    let (mut mode, mut state) = session(TaskbarMode::Titlebar);

    state.insert('a');
    assert_eq!(mode.on_hover(Click::Row(1), &mut state), Action::None);
    assert_eq!(mode.on_click(Click::Row(1), &mut state), Action::None);
    assert_eq!(state.selected, 0);
}

#[test]
fn hovering_a_dropdown_title_moves_the_selection() {
    let (mut mode, mut state) = session(TaskbarMode::Iconbar);

    state.open_dropdown();
    assert_eq!(mode.on_hover(Click::Menu(1), &mut state), Action::Redraw);
    assert_eq!(state.dropdown, Some(1));
    assert_eq!(state.icon, 0);
    assert_eq!(mode.on_hover(Click::Menu(1), &mut state), Action::None);
}

#[test]
fn a_second_click_on_the_icon_closes_the_dropdown() {
    let (mut mode, mut state) = session(TaskbarMode::Iconbar);

    assert_eq!(mode.on_click(Click::Icon(0), &mut state), Action::Redraw);
    assert_eq!(state.dropdown, Some(0));
    assert_eq!(mode.on_click(Click::Icon(0), &mut state), Action::Redraw);
    assert_eq!(state.dropdown, None);
    assert_eq!(state.icon, 0);
}

#[test]
fn a_click_on_another_icon_still_switches() {
    let (mut mode, mut state) = session(TaskbarMode::Iconbar);

    mode.on_click(Click::Icon(0), &mut state);
    assert_eq!(
        mode.on_click(Click::Icon(1), &mut state),
        Action::Activate(WindowId("w3".into()))
    );
}
