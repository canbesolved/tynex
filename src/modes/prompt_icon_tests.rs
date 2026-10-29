// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon strip keys: move, open the dropdown, confirm, then type.

use super::*;
use crate::config::{LauncherConfig, LauncherMode};
use crate::core::event::Click;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn mode() -> PromptMode {
    PromptMode::new(
        Input::new(
            LauncherConfig {
                mode: LauncherMode::Off,
                ..LauncherConfig::default()
            },
            Vec::new(),
        ),
        false,
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
    (mode(), state)
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
