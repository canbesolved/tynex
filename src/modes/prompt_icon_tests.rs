// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon strip keys: move, open the dropdown, confirm, then type.

use super::*;
use crate::config::{LauncherConfig, LauncherMode};
use crate::core::event::Click;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn mode(title_bar: bool) -> PromptMode {
    PromptMode::new(
        Input::new(
            LauncherConfig {
                mode: LauncherMode::Off,
                ..LauncherConfig::default()
            },
            Vec::new(),
        ),
        title_bar,
        true,
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

fn opened() -> (PromptMode, State) {
    session(false)
}

fn session(title_bar: bool) -> (PromptMode, State) {
    let rows = vec![
        row("w1", "firefox", "Notes"),
        row("w2", "firefox", "Mail"),
        row("w3", "kate", "File"),
    ];
    let mut state = State::new(rows);

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
    (mode(title_bar), state)
}

#[test]
fn arrows_move_along_the_strip_until_typing() {
    let (mut mode, mut state) = opened();

    assert_eq!(mode.on_key(Key::Right, &mut state), Action::Redraw);
    assert_eq!(state.icon, 1);
    assert_eq!(mode.on_key(Key::Left, &mut state), Action::Redraw);
    assert_eq!(state.icon, 0);

    mode.on_key(
        Key::Char {
            text: 'a',
            latin: None,
        },
        &mut state,
    );
    assert_eq!(state.query, "a");
    assert_eq!(mode.on_key(Key::Left, &mut state), Action::Redraw);
    assert_eq!(state.cursor, 0);
    assert_eq!(state.icon, 0);
}

#[test]
fn enter_opens_then_activates_and_esc_closes() {
    let (mut mode, mut state) = opened();

    assert_eq!(mode.on_key(Key::Enter, &mut state), Action::Redraw);
    assert_eq!(state.dropdown, Some(0));
    assert_eq!(
        mode.on_key(Key::Enter, &mut state),
        Action::Activate(WindowId("w2".into()))
    );

    let (mut mode, mut state) = opened();

    mode.on_key(Key::Enter, &mut state);
    assert_eq!(mode.on_key(Key::Esc, &mut state), Action::Redraw);
    assert_eq!(state.dropdown, None);
}

#[test]
fn the_title_bar_wins_confirm_over_the_icon() {
    let (mut mode, mut state) = session(true);

    assert_eq!(
        mode.on_key(Key::Enter, &mut state),
        Action::Activate(WindowId("w1".into()))
    );
    assert_eq!(state.dropdown, None);

    let (mut mode, mut state) = session(true);

    mode.on_key(Key::Down, &mut state);
    assert_eq!(
        mode.on_key(
            Key::Char {
                text: ' ',
                latin: None,
            },
            &mut state,
        ),
        Action::Activate(WindowId("w2".into()))
    );
    assert_eq!(state.dropdown, None);
}

#[test]
fn an_open_dropdown_still_takes_enter_with_the_title_bar() {
    let (mut mode, mut state) = session(true);

    assert_eq!(mode.on_click(Click::Icon(0), &mut state), Action::Redraw);
    assert_eq!(state.dropdown, Some(0));
    assert_eq!(
        mode.on_key(Key::Enter, &mut state),
        Action::Activate(WindowId("w2".into()))
    );
}

#[test]
fn one_window_activates_and_a_click_matches_enter() {
    let (mut mode, mut state) = opened();

    state.select_icon(Some("kate"));
    assert_eq!(
        mode.on_key(Key::Enter, &mut state),
        Action::Activate(WindowId("w3".into()))
    );

    let (mut mode, mut state) = opened();

    assert_eq!(
        mode.on_click(Click::Icon(1), &mut state),
        Action::Activate(WindowId("w3".into()))
    );
    let (mut mode, mut state) = opened();

    mode.on_key(Key::Enter, &mut state);
    assert_eq!(
        mode.on_click(Click::Menu(1), &mut state),
        Action::Activate(WindowId("w1".into()))
    );
}

#[test]
fn tab_moves_the_next_icon_without_a_title_bar() {
    let (mut mode, mut state) = opened();

    assert_eq!(mode.on_key(Key::Tab, &mut state), Action::Redraw);
    assert_eq!(state.icon, 1);
    assert_eq!(state.selected, 0);
    mode.on_key(Key::Tab, &mut state);
    assert_eq!(state.icon, 0);
}

#[test]
fn tab_steps_an_open_dropdown_before_the_icons() {
    let (mut mode, mut state) = opened();

    mode.on_key(Key::Enter, &mut state);
    assert_eq!(mode.on_key(Key::Tab, &mut state), Action::Redraw);
    assert_eq!(state.dropdown, Some(1));
    assert_eq!(state.icon, 0);
}

#[test]
fn tab_keeps_the_title_list_when_that_bar_is_on() {
    let (mut mode, mut state) = session(true);

    assert_eq!(mode.on_key(Key::Tab, &mut state), Action::Redraw);
    assert_eq!(state.icon, 0);
    assert_eq!(state.selected, 1);
}
