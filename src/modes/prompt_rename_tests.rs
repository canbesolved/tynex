// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Renaming the selected window with the rename key.

use super::*;
use crate::config::{LauncherConfig, LauncherMode, TaskbarMode};
use crate::core::event::Event;
use crate::core::hotkey::Binding;
use crate::core::order::SortBy;
use crate::core::switcher::{HotkeyAction, Switcher};
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::{WindowId, WindowInfo};
use crate::modules::input::Input;

fn mode(taskbar: TaskbarMode, keys: &[&str]) -> PromptMode {
    let launcher = LauncherConfig {
        mode: LauncherMode::Off,
        ..LauncherConfig::default()
    };

    PromptMode::new(Input::new(launcher, Vec::new()), taskbar, 19).with_rename(
        Bindings(keys.iter().map(|k| Binding::parse(k).unwrap()).collect()),
    )
}

fn id(text: &str) -> WindowId {
    WindowId(text.into())
}

fn row(window: &str, app: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(id(window)),
        app_id: app.into(),
        app_name: app.into(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

/// An open popup with two Firefox windows and one Kate window.
fn session(taskbar: TaskbarMode) -> (PromptMode, State) {
    let mut mode = mode(taskbar, &["."]);
    let mut s = State::new(vec![
        row("w1", "firefox", "Notes"),
        row("w2", "firefox", "Mail"),
        row("w3", "kate", "File"),
    ]);

    s.icons = vec![
        AppIcon {
            app_id: "firefox".into(),
            icon: None,
            windows: vec![id("w1"), id("w2")],
        },
        AppIcon {
            app_id: "kate".into(),
            icon: None,
            windows: vec![id("w3")],
        },
    ];
    s.select_icon(Some("firefox"));
    mode.refresh(&mut s);
    (mode, s)
}

fn press(mode: &mut PromptMode, s: &mut State, text: &str) {
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

fn editing(s: &State) -> Option<(&str, &str)> {
    s.rename
        .as_ref()
        .map(|r| (r.id.0.as_str(), r.text.as_str()))
}

#[test]
fn the_rename_key_edits_a_search_result_and_enter_keeps_it() {
    let (mut mode, mut s) = session(TaskbarMode::Iconbar);

    press(&mut mode, &mut s, "mai");
    press(&mut mode, &mut s, ".");
    assert_eq!(s.query, "mai");
    assert_eq!(editing(&s), Some(("w2", "Mail")));

    press(&mut mode, &mut s, ". box");
    mode.on_key(Key::Left, &mut s);
    mode.on_key(Key::Backspace, &mut s);
    assert_eq!(editing(&s), Some(("w2", "Mail. bx")));
    assert_eq!(mode.on_key(Key::Enter, &mut s), Action::Redraw);
    assert_eq!(s.rename, None);
    assert_eq!(s.renamed, Some((id("w2"), "Mail. bx".into())));
}

#[test]
fn esc_cancels_without_closing_or_clearing_the_query() {
    let (mut mode, mut s) = session(TaskbarMode::Iconbar);

    press(&mut mode, &mut s, "no.x");
    assert_eq!(mode.on_key(Key::Esc, &mut s), Action::Redraw);
    assert_eq!(s.rename, None);
    assert_eq!(s.renamed, None);
    assert_eq!(s.query, "no");
}

#[test]
fn the_rename_key_never_types_even_with_nothing_to_rename() {
    let (mut mode, mut s) = session(TaskbarMode::Iconbar);

    let dot = Key::Char {
        text: '.',
        latin: None,
    };

    assert_eq!(mode.on_key(dot, &mut s), Action::None);
    assert_eq!(s.query, "");
    assert_eq!(s.rename, None);
}

#[test]
fn an_open_dropdown_renames_its_selected_title() {
    let (mut mode, mut s) = session(TaskbarMode::Iconbar);

    mode.on_key(Key::Enter, &mut s); // Two windows: opens the dropdown
    mode.on_key(Key::Down, &mut s);

    let index = s.dropdown.unwrap();
    let RowTarget::Window(expected) = &s.menu()[index].target else {
        panic!("dropdown rows are windows");
    };
    let expected = expected.clone();

    press(&mut mode, &mut s, ".");
    assert_eq!(s.rename.as_ref().map(|r| &r.id), Some(&expected));
}

#[test]
fn the_title_list_renames_its_selected_row() {
    let (mut mode, mut s) = session(TaskbarMode::Titlebar);

    mode.on_key(Key::Down, &mut s);
    press(&mut mode, &mut s, ".");
    assert_eq!(editing(&s), Some(("w2", "Mail")));
}

#[test]
fn other_keys_and_the_pointer_wait_while_editing() {
    let (mut mode, mut s) = session(TaskbarMode::Titlebar);
    let ctrl_r = Key::Combo(Binding::parse("Ctrl+R").unwrap());

    press(&mut mode, &mut s, ".");
    assert_eq!(mode.on_key(Key::Down, &mut s), Action::None);
    assert_eq!(mode.on_key(ctrl_r, &mut s), Action::None);
    assert_eq!(mode.on_click(Click::Row(2), &mut s), Action::None);
    assert_eq!(mode.on_hover(Click::Row(2), &mut s), Action::None);
    assert_eq!(s.selected, 0);
    assert_eq!(editing(&s), Some(("w1", "Notes")));
}

#[test]
fn without_rename_keys_a_dot_is_typed() {
    let (_, mut s) = session(TaskbarMode::Iconbar);
    let mut plain = mode(TaskbarMode::Iconbar, &[]);

    press(&mut plain, &mut s, "a.b");
    assert_eq!(s.query, "a.b");
}

fn window(window: &str, title: &str) -> WindowInfo {
    WindowInfo {
        id: id(window),
        app_id: "app".into(),
        app_name: "App".into(),
        title: title.into(),
        icon: None,
        pid: None,
    }
}

fn titles(sw: &Switcher<PromptMode>) -> Vec<&str> {
    sw.windows().iter().map(|w| w.title.as_str()).collect()
}

#[test]
fn a_kept_title_lasts_until_the_window_closes() {
    let mut sw = Switcher::new(
        HotkeyAction::Hide,
        false,
        SortBy::Recently,
        mode(TaskbarMode::Titlebar, &["F2"]),
    );
    let f2 = Key::Combo(Binding::parse("F2").unwrap());

    sw.handle(Event::Windows(vec![
        window("w1", "Inbox"),
        window("w2", "Kate"),
    ]));
    sw.handle(Event::Show); // Selects the previous window, w2
    sw.handle(Event::Key(f2));
    for _ in 0..4 {
        sw.handle(Event::Key(Key::Backspace));
    }
    for c in "Editor".chars() {
        sw.handle(Event::Key(Key::Char {
            text: c,
            latin: None,
        }));
    }
    assert_eq!(sw.handle(Event::Key(Key::Enter)), Action::Redraw);
    assert_eq!(titles(&sw), ["Inbox", "Editor"]);

    let view = sw.view().unwrap();

    assert!(view.windows().any(|r| r.title == "Editor"));

    sw.handle(Event::Windows(vec![
        window("w1", "Inbox"),
        window("w2", "Kate 2"),
    ]));
    assert_eq!(titles(&sw), ["Inbox", "Editor"]);
    sw.handle(Event::Windows(vec![window("w1", "Inbox")]));
    sw.handle(Event::Windows(vec![
        window("w1", "Inbox"),
        window("w2", "Kate"),
    ]));
    assert_eq!(titles(&sw), ["Inbox", "Kate"]);
}
