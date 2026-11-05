// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The window raised after `tynex qs` opens a tab with no popup.

use super::Session;
use crate::core::browser_tabs::{SourceId, TabRef, TabsUpdate};
use crate::core::event::Action;
use crate::core::window::{WindowId, WindowInfo};

fn window(id: &str, pid: u32) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.to_owned()),
        app_id: "brave".to_owned(),
        app_name: "Brave".to_owned(),
        title: id.to_owned(),
        icon: None,
        pid: Some(pid),
    }
}

fn reply(ok: bool) -> TabsUpdate {
    TabsUpdate::Activated {
        source: SourceId(1),
        ok,
        window: 7,
    }
}

const TAB: TabRef = TabRef {
    source: SourceId(1),
    id: 3,
};

#[test]
fn the_known_window_is_raised() {
    let mut session = Session::default();
    let windows = vec![window("one", 4), window("two", 4)];
    let two = WindowId("two".to_owned());

    session.note_hidden(&TAB, 4, Some(&two));
    assert_eq!(
        session.apply(reply(true), &mut None, &windows),
        Action::Activate(two)
    );
}

#[test]
fn without_a_known_window_the_only_one_is_raised() {
    let mut session = Session::default();
    let windows = vec![window("other", 9), window("one", 4)];
    let gone = WindowId("gone".to_owned());

    session.note_hidden(&TAB, 4, Some(&gone));
    assert_eq!(
        session.apply(reply(true), &mut None, &windows),
        Action::Activate(WindowId("one".to_owned()))
    );
}

#[test]
fn a_closed_tab_raises_nothing() {
    let mut session = Session::default();
    let windows = vec![window("one", 4)];

    session.note_hidden(&TAB, 4, None);
    assert_eq!(
        session.apply(reply(false), &mut None, &windows),
        Action::None
    );
}
