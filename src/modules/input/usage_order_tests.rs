// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Inside the exact and the fuzzy band, a higher usage score comes
//! first, whatever the kind of the row.

use crate::config::{LauncherConfig, LauncherMode};
use crate::core::app::AppInfo;
use crate::core::browser_tabs::{SourceId, TabInfo, TabList};
use crate::core::state::State;
use crate::core::usage::Usage;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn window(app_id: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(WindowId(title.into())),
        app_id: app_id.into(),
        app_name: app_id.into(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn notes_tab() -> TabList {
    TabList {
        source: SourceId(1),
        pid: 7,
        browser: "Brave".into(),
        icon: String::new(),
        tabs: vec![TabInfo {
            id: 1,
            window: 1,
            active: false,
            last: 1,
            title: "Notes".into(),
            url: "https://notes.example".into(),
            icon: String::new(),
        }],
        windows: Vec::new(),
    }
}

fn titles(usage: &str, query: &str) -> Vec<String> {
    let launcher = LauncherConfig {
        mode: LauncherMode::All,
        ..LauncherConfig::default()
    };
    let apps = vec![AppInfo {
        id: "notes-app".into(),
        name: "Notes App".into(),
        icon: None,
    }];
    let input = Input::new(launcher, apps);
    let mut state = State::new(vec![
        window("kate", "notes.txt"),
        window("okular", "Release notes"),
    ]);

    state.tab_lists = vec![notes_tab()];
    state.usage = Usage::parse(usage);
    state.query = query.into();
    input.refresh(&mut state);
    state.output.into_iter().map(|row| row.title).collect()
}

#[test]
fn unused_rows_keep_todays_order() {
    assert_eq!(
        titles("", "notes"),
        [
            "notes.txt",
            "Release notes",
            "Notes — notes.example",
            "Notes App"
        ]
    );
}

#[test]
fn the_most_used_rows_come_first_in_each_band() {
    let usage = "5\ttab\tBrave\thttps://notes.example\n\
                 3\tapp\tokular\n\
                 1\tapp\tnotes-app\n";

    assert_eq!(
        titles(usage, "notes"),
        [
            "Notes — notes.example",
            "Release notes",
            "Notes App",
            "notes.txt"
        ]
    );
    // `nts` is fuzzy for every row: the band is sorted too.
    assert_eq!(
        titles(usage, "nts")[..2],
        ["Notes — notes.example", "Release notes"]
    );
}
