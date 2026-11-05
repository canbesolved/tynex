// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Opening a window through the popup counts its app; gone apps are
//! dropped when the popup opens.

use std::collections::HashSet;

use crate::core::event::{Action, Event, Key};
use crate::core::switcher::HotkeyAction;
use crate::core::switcher::tests::{build, list};
use crate::core::usage::Usage;

#[test]
fn an_activation_is_counted_and_saved_once_while_hidden() {
    let mut sw = build(HotkeyAction::Last);

    sw.handle(Event::Windows(list(&["kate", "firefox"])));
    sw.handle(Event::Show);
    assert_eq!(sw.usage_to_save(), None); // Lent while open
    assert_eq!(
        sw.handle(Event::Key(Key::Enter)),
        Action::Activate(crate::core::window::WindowId("firefox".into()))
    );
    assert_eq!(sw.usage_to_save().as_deref(), Some("1\tapp\tfirefox\n"));
    assert_eq!(sw.usage_to_save(), None);

    sw.handle(Event::Show);
    sw.handle(Event::Show); // Pressed again: the last window
    assert_eq!(sw.usage_to_save().as_deref(), Some("2\tapp\tfirefox\n"));

    sw.handle(Event::Show);
    sw.handle(Event::Key(Key::Esc));
    assert_eq!(sw.usage_to_save(), None); // Nothing was opened
}

#[test]
fn apps_with_no_window_and_no_desktop_file_are_dropped_on_show() {
    let usage = Usage::parse("3\tapp\tkate\n2\tapp\tgimp\n1\tapp\tgone\n");
    let installed: HashSet<String> = ["gimp".to_owned()].into();
    let mut sw = build(HotkeyAction::Hide).with_usage(usage, Some(installed));

    sw.handle(Event::Windows(list(&["kate"])));
    sw.handle(Event::Show);
    sw.handle(Event::Hide);
    assert_eq!(
        sw.usage_to_save().as_deref(),
        Some("3\tapp\tkate\n2\tapp\tgimp\n")
    );

    // Not scanned: only apps with a window are known, so none is
    // dropped.
    let usage = Usage::parse("1\tapp\tgone\n");
    let mut sw = build(HotkeyAction::Hide).with_usage(usage, None);

    sw.handle(Event::Show);
    sw.handle(Event::Hide);
    assert_eq!(sw.usage_to_save(), None);
}

#[test]
fn the_hotkey_goes_back_to_a_tab_opened_before_another_app() {
    use crate::core::browser_tabs::{SourceId, TabInfo, TabList, TabRef};
    use crate::core::window::{WindowId, WindowInfo};

    let app = |id: &str, pid: u32| WindowInfo {
        id: WindowId(id.into()),
        app_id: id.into(),
        app_name: id.into(),
        title: id.into(),
        icon: None,
        pid: Some(pid),
    };
    let github = TabRef {
        source: SourceId(1),
        id: 3,
    };
    let lists = vec![TabList {
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
    let mut sw = build(HotkeyAction::Last);

    sw.handle(Event::Windows(vec![app("kate", 1)]));
    sw.handle(Event::Show);
    let mut state = sw.state.take().unwrap();

    state.tab_lists.clone_from(&lists);
    sw.keep(Action::ActivateTab(github.clone()), state);
    sw.handle(Event::Windows(vec![app("brave", 7), app("kate", 1)]));
    sw.handle(Event::Windows(vec![
        app("cursor", 2),
        app("brave", 7),
        app("kate", 1),
    ]));
    sw.handle(Event::Show);
    sw.state.as_mut().unwrap().tab_lists = lists;
    assert_eq!(sw.handle(Event::Show), Action::ActivateTab(github));
    assert_eq!(
        sw.usage_to_save().as_deref(),
        Some("2\ttab\tBrave\thttps://github.com\n")
    );
}
