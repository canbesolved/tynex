// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for `tynex qs`, with the popup open or hidden.

use super::super::tests::{build, list};
use super::*;
use crate::core::browser_tabs::{SourceId, TabInfo, TabList, TabRef};
use crate::core::event::Event;
use crate::core::switcher::HotkeyAction;
use crate::core::window::{WindowId, WindowInfo};

fn id(name: &str) -> WindowId {
    WindowId(name.into())
}

#[test]
fn hidden_it_switches_to_the_previous_window_and_counts_it() {
    let mut sw = build(HotkeyAction::Hide);

    sw.handle(Event::Windows(list(&["active", "prev", "old"])));
    assert_eq!(sw.handle(Event::QuickSwitch), Action::Activate(id("prev")));
    assert!(sw.view().is_none());
    assert!(sw.usage_to_save().is_some_and(|t| t.contains("prev")));
}

#[test]
fn hidden_with_one_window_it_does_nothing() {
    let mut sw = build(HotkeyAction::Last);

    sw.handle(Event::Windows(list(&["only"])));
    assert_eq!(sw.handle(Event::QuickSwitch), Action::None);
}

#[test]
fn open_it_switches_back_whatever_the_hotkey_action() {
    let mut sw = build(HotkeyAction::Hide);

    sw.handle(Event::Windows(list(&["active", "prev"])));
    sw.handle(Event::Show);
    assert_eq!(sw.handle(Event::QuickSwitch), Action::Activate(id("prev")));
    assert!(sw.view().is_none());

    sw.handle(Event::Windows(list(&["only"])));
    sw.handle(Event::Show);
    assert_eq!(sw.handle(Event::QuickSwitch), Action::Close);
}

#[test]
fn hidden_it_goes_back_to_a_tab_and_then_to_the_window() {
    let app = |name: &str, pid: u32| WindowInfo {
        id: id(name),
        app_id: name.into(),
        app_name: name.into(),
        title: name.into(),
        icon: None,
        pid: Some(pid),
    };
    let github = TabRef {
        source: SourceId(1),
        id: 3,
    };
    let mut sw = build(HotkeyAction::Hide);

    sw.handle(Event::Windows(vec![app("kate", 1)]));
    sw.handle(Event::Show);
    let mut state = sw.state.take().unwrap();

    state.tab_lists = vec![TabList {
        source: SourceId(1),
        pid: 7,
        browser: "Brave".into(),
        icon: String::new(),
        tabs: vec![TabInfo {
            id: 3,
            window: 1,
            active: true,
            last: 1,
            title: "GitHub".into(),
            url: "https://github.com".into(),
            icon: String::new(),
        }],
        windows: Vec::new(),
    }];
    sw.keep(Action::ActivateTab(github.clone()), state);
    sw.handle(Event::Windows(vec![app("brave", 7), app("kate", 1)]));
    sw.handle(Event::Windows(vec![
        app("cursor", 2),
        app("brave", 7),
        app("kate", 1),
    ]));
    assert_eq!(sw.handle(Event::QuickSwitch), Action::ActivateTab(github));

    sw.handle(Event::Windows(vec![
        app("brave", 7),
        app("cursor", 2),
        app("kate", 1),
    ]));
    assert_eq!(
        sw.handle(Event::QuickSwitch),
        Action::Activate(id("cursor"))
    );
}
