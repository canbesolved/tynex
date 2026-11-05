// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `select` confirms; `space` types a space in the query.

use crate::config::{LauncherConfig, TaskbarMode};
use crate::core::event::{Action, Key};
use crate::core::hotkey::{Binding, Bindings};
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::modes::prompt::PromptMode;
use crate::modules::input::Input;

fn keys(texts: &[&str]) -> Bindings {
    Bindings(texts.iter().map(|t| Binding::parse(t).unwrap()).collect())
}

fn key(text: &str) -> Key {
    let binding = Binding::parse(text).unwrap();

    match text {
        "Enter" => Key::Enter,
        "Space" => Key::Char {
            text: ' ',
            latin: None,
        },
        _ => Key::Combo(binding),
    }
}

fn typed(c: char) -> Key {
    Key::Char {
        text: c,
        latin: None,
    }
}

fn mode(select: &[&str], space: &[&str]) -> PromptMode {
    let input = Input::new(LauncherConfig::default(), Vec::new());

    PromptMode::new(input, TaskbarMode::Off, 19)
        .with_input_keys(keys(select), keys(space))
}

fn state() -> State {
    State::new(vec![Row {
        target: RowTarget::Window(WindowId("w1".into())),
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: "Release notes".into(),
        title_color: None,
        highlights: Vec::new(),
    }])
}

#[test]
fn a_left_shift_tap_types_a_space_and_space_selects() {
    let mut mode = mode(&["Enter", "Space"], &["ShiftLeft"]);
    let mut s = state();

    mode.on_key(typed('r'), &mut s);
    assert_eq!(mode.on_key(key("ShiftLeft"), &mut s), Action::Redraw);
    mode.on_key(typed('n'), &mut s);
    assert_eq!(s.query, "r n");
    assert_eq!(s.output.len(), 1);
    assert_eq!(
        mode.on_key(key("Space"), &mut s),
        Action::Activate(WindowId("w1".into()))
    );
}

#[test]
fn with_select_enter_space_types_a_space() {
    let mut mode = mode(&["Enter"], &["Space"]);
    let mut s = state();

    mode.on_key(typed('r'), &mut s);
    mode.on_key(key("Space"), &mut s);
    assert_eq!(s.query, "r ");
    assert_eq!(mode.on_key(key("ShiftLeft"), &mut s), Action::None);
    assert_eq!(
        mode.on_key(key("Ctrl+Enter"), &mut s),
        Action::Activate(WindowId("w1".into()))
    );
}

#[test]
fn a_combination_can_select() {
    let mut mode = mode(&["Ctrl+J"], &[]);
    let mut s = state();

    assert_eq!(mode.on_key(key("Enter"), &mut s), Action::None);
    assert_eq!(
        mode.on_key(key("Ctrl+J"), &mut s),
        Action::Activate(WindowId("w1".into()))
    );
}
