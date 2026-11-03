// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pinned app keys: focus a running app or launch it.

use super::*;
use crate::config::{LauncherConfig, LauncherMode, TaskbarMode};
use crate::core::app::AppInfo;
use crate::core::hotkey::Binding;
use crate::core::view::{Row, RowTarget};
use crate::core::window::{WindowId, WindowInfo};
use crate::modules::input::Input;

fn key(text: &str) -> Key {
    Key::Combo(Binding::parse(text).unwrap())
}

fn window(id: &str, app: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.into()),
        app_id: app.into(),
        app_name: app.into(),
        title: id.into(),
        icon: None,
        pid: None,
    }
}

fn app(id: &str) -> AppInfo {
    AppInfo {
        id: id.into(),
        name: id.into(),
        icon: None,
    }
}

/// Kate and GIMP are pinned; Kate and Firefox run.
fn session() -> (PromptMode, State) {
    let windows = [window("w1", "firefox"), window("w2", "kate")];
    let rows = windows.iter().map(Row::window).collect();
    let launcher = LauncherConfig {
        mode: LauncherMode::Off,
        ..LauncherConfig::default()
    };
    let mut mode = PromptMode::new(
        Input::new(launcher, Vec::new()),
        TaskbarMode::Iconbar,
        19,
    )
    .with_pinned(
        ["F1", "F2", "F3"]
            .iter()
            .map(|k| Bindings(vec![Binding::parse(k).unwrap()]))
            .collect(),
    );
    let mut s = State::new(rows);

    s.fill_icons((&[app("kate"), app("gimp")], &windows), None, None);
    mode.refresh(&mut s);
    (mode, s)
}

#[test]
fn a_pinned_key_focuses_a_running_app() {
    let (mut mode, mut s) = session();

    assert_eq!(s.pinned, 2);
    assert_eq!(
        mode.on_key(key("F1"), &mut s),
        Action::Activate(WindowId("w2".into()))
    );
}

#[test]
fn a_pinned_key_launches_an_app_that_is_not_running() {
    let (mut mode, mut s) = session();

    assert_eq!(
        mode.on_key(key("F2"), &mut s),
        Action::Launch("gimp".into())
    );
}

#[test]
fn a_key_past_the_pinned_apps_does_nothing() {
    let (mut mode, mut s) = session();

    assert_eq!(mode.on_key(key("F3"), &mut s), Action::None);
    assert_eq!(s.query, "");
}

#[test]
fn pinned_keys_work_while_typing_but_not_while_renaming() {
    let (mut mode, mut s) = session();

    mode.on_key(
        Key::Char {
            text: 'f',
            latin: None,
        },
        &mut s,
    );
    assert_eq!(
        mode.on_key(key("F2"), &mut s),
        Action::Launch("gimp".into())
    );

    s.start_rename(&WindowId("w1".into()));
    assert_eq!(mode.on_key(key("F2"), &mut s), Action::None);
    assert!(matches!(
        s.output.first().map(|r| &r.target),
        Some(RowTarget::Window(_))
    ));
}
