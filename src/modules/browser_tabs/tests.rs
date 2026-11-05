// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row titles, search order and the window pick.

use super::{Session, browser_window, clean, fallback, row_title, rows};
use crate::core::browser_tabs::{
    SourceId, TabInfo, TabList, TabRef, TabsUpdate,
};
use crate::core::event::Action;
use crate::core::state::State;
use crate::core::window::{WindowId, WindowInfo};

fn tab(
    id: i32,
    window: i32,
    active: bool,
    last: u64,
    title: &str,
    url: &str,
) -> TabInfo {
    TabInfo {
        id,
        window,
        active,
        last,
        title: title.to_owned(),
        url: url.to_owned(),
        icon: String::new(),
    }
}

fn window(id: &str, title: &str, pid: Option<u32>) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.to_owned()),
        app_id: "brave".to_owned(),
        app_name: "Brave".to_owned(),
        title: title.to_owned(),
        icon: Some("brave".to_owned()),
        pid,
    }
}

#[test]
fn titles_drop_www_and_fall_back_to_the_url() {
    let page = tab(1, 1, true, 1, "Notes", "https://www.example.com/a");

    assert_eq!(row_title(&page), "Notes — example.com");
    let bare = tab(1, 1, true, 1, "https://example.com", "https://example.com");

    assert_eq!(row_title(&bare), "https://example.com");
}

#[test]
fn leading_matches_come_first() {
    let list = TabList {
        source: SourceId(1),
        pid: 4,
        windows: Vec::new(),
        browser: "Brave".to_owned(),
        icon: "brave".to_owned(),
        tabs: vec![
            tab(1, 1, true, 1, "Mail", "https://mail.example"),
            tab(2, 1, false, 9, "Maps", "https://maps.example"),
        ],
    };
    let found = rows("ma", &[list]);

    assert_eq!(found[0].title, "Maps — maps.example");
}

fn list(tabs: Vec<TabInfo>) -> TabsUpdate {
    TabsUpdate::List {
        source: SourceId(1),
        pid: 4,
        browser: "Brave".to_owned(),
        tabs,
    }
}

#[test]
fn a_hidden_popup_keeps_no_list() {
    let mut session = Session::default();
    let mut state = None;
    let update = list(vec![tab(1, 1, true, 1, "A", "https://a.example")]);

    assert_eq!(session.apply(update, &mut state, &[]), Action::None);
    assert!(state.is_none());
}

#[test]
fn lists_are_replaced_and_dropped_by_source() {
    let mut session = Session::default();
    let mut state = Some(State::new(Vec::new()));

    for title in ["A", "B"] {
        let update = list(vec![tab(1, 1, true, 1, title, "https://a.example")]);

        assert_eq!(session.apply(update, &mut state, &[]), Action::Redraw);
    }
    let lists = &state.as_ref().unwrap().tab_lists;

    assert_eq!((lists.len(), lists[0].tabs[0].title.as_str()), (1, "B"));
    assert_eq!(lists[0].icon, "brave-browser");
    let gone = TabsUpdate::Gone(SourceId(1));

    assert_eq!(session.apply(gone.clone(), &mut state, &[]), Action::Redraw);
    assert_eq!(session.apply(gone, &mut state, &[]), Action::None);
}

#[test]
fn a_list_keeps_the_windows_of_its_browser() {
    let mut state = Some(State::new(Vec::new()));
    let windows = [
        window("w1", "A - Brave", Some(4)),
        window("w2", "B - Kate", Some(5)),
        window("w3", "C - Brave", None),
    ];
    let update = list(vec![tab(1, 1, true, 1, "A", "https://a.example")]);

    Session::default().apply(update, &mut state, &windows);
    assert_eq!(
        state.unwrap().tab_lists[0].windows,
        [WindowId("w1".to_owned())]
    );
}

#[test]
fn clean_caps_text_and_keeps_the_newest() {
    let long = "x".repeat(300);
    let tabs = clean(vec![
        tab(1, 1, false, 1, &long, "https://a.example"),
        tab(2, 1, false, 5, "New\u{200b} tab", "https://b.example"),
    ]);

    assert_eq!(tabs[0].title, "New tab");
    assert_eq!(tabs[1].title.chars().count(), 256);
}

#[test]
fn fallback_prefers_a_window_icon() {
    let windows = [window("w", "Inbox", Some(4))];

    assert_eq!(fallback(&windows, 4, "Brave"), "brave");
    assert_eq!(fallback(&windows, 0, "Brave"), "brave-browser");
    assert_eq!(fallback(&[], 4, "Google Chrome"), "google-chrome");
    assert_eq!(fallback(&[], 4, "Opera Next"), "opera-next");
}

#[test]
fn activation_uses_the_noted_list() {
    let mut session = Session::default();
    let mut state = Some(State::new(Vec::new()));
    let windows = vec![
        window("one", "Inbox — Brave", Some(4)),
        window("two", "Other", Some(4)),
    ];
    let update = list(vec![
        tab(1, 7, true, 1, "Inbox", "https://mail.example"),
        tab(2, 7, false, 2, "Other", "https://other.example"),
    ]);
    let reply = |ok| TabsUpdate::Activated {
        source: SourceId(1),
        ok,
        window: 7,
    };

    session.apply(update, &mut state, &windows);
    let tab = TabRef {
        source: SourceId(1),
        id: 2,
    };

    session.note(&tab, state.as_ref().unwrap());
    state = None;
    assert_eq!(
        session.apply(reply(true), &mut state, &windows),
        Action::Activate(WindowId("one".to_owned()))
    );
    assert_eq!(
        session.apply(reply(true), &mut state, &windows),
        Action::None
    );
    session.note(&tab, &{
        let mut open = State::new(Vec::new());

        open.tab_lists = vec![TabList {
            source: SourceId(1),
            pid: 4,
            windows: Vec::new(),
            browser: String::new(),
            icon: String::new(),
            tabs: Vec::new(),
        }];
        open
    });
    assert_eq!(
        session.apply(reply(false), &mut state, &windows),
        Action::None
    );
}

#[test]
fn pick_covers_each_branch() {
    let tabs = vec![
        tab(1, 3, true, 1, "Inbox", "https://mail.example"),
        tab(2, 3, false, 2, "Maps", "https://maps.example"),
    ];
    let one = [window("only", "Inbox — Brave", Some(4))];

    assert!(browser_window(&one, 9, &tabs, 2, 3).is_none());
    assert_eq!(
        browser_window(&one, 4, &tabs, 2, 3),
        Some(WindowId("only".to_owned()))
    );
    let many = [
        window("old", "Inbox", Some(4)),
        window("new", "Maps <2>", Some(4)),
    ];

    assert_eq!(
        browser_window(&many, 4, &tabs, 2, 3),
        Some(WindowId("old".to_owned()))
    );
    assert_eq!(
        browser_window(&many, 4, &tabs, 1, 3),
        Some(WindowId("old".to_owned()))
    );
    let none = [
        window("x", "Nothing", Some(4)),
        window("y", "Else", Some(4)),
    ];

    assert!(browser_window(&none, 4, &tabs, 2, 3).is_none());
}

#[path = "usage_tests.rs"]
mod usage;
