// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Exact matches come before fuzzy ones, whatever their source.

use super::exact_first;
use crate::config::LauncherConfig;
use crate::core::app::AppInfo;
use crate::core::browser_tabs::{SourceId, TabInfo, TabList, TabRef};
use crate::core::state::State;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::input::Input;

fn row(target: RowTarget, app_id: &str, app_name: &str, title: &str) -> Row {
    Row {
        target,
        app_id: app_id.into(),
        app_name: app_name.into(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn window(app_id: &str, app_name: &str, title: &str) -> Row {
    row(
        RowTarget::Window(WindowId(title.into())),
        app_id,
        app_name,
        title,
    )
}

fn launch(app_id: &str, app_name: &str, title: &str) -> Row {
    row(RowTarget::Launch(app_id.into()), app_id, app_name, title)
}

fn tab(title: &str) -> Row {
    let target = RowTarget::Tab(TabRef {
        source: SourceId(1),
        id: 1,
    });

    row(target, "", "Brave", title)
}

fn titles(rows: &[Row]) -> Vec<&str> {
    rows.iter().map(|r| r.title.as_str()).collect()
}

#[test]
fn an_exact_tab_beats_a_fuzzy_window() {
    let rows = vec![
        window("chromium-browser", "Chromium Web Browser", "Calendar"),
        tab("Overall - Miro — miro.com"),
    ];

    assert_eq!(
        titles(&exact_first("miro", rows)),
        ["Overall - Miro — miro.com", "Calendar"]
    );
}

#[test]
fn bands_keep_their_order_and_new_window_rows_stay() {
    let rows = vec![
        window("firefox", "Firefox", "Firefox docs"),
        launch("firefox", "Firefox", " +  New window"),
        window("code", "Code", "fiddle.rs"),
        tab("Firefox release notes"),
        launch("ff", "Fast Forward", "Fast Forward"),
        launch("fox", "Firefox Nightly", "Firefox Nightly"),
    ];

    assert_eq!(
        titles(&exact_first("firefox", rows)),
        [
            "Firefox docs",
            " +  New window",
            "Firefox release notes",
            "Firefox Nightly",
            "fiddle.rs",
            "Fast Forward",
        ]
    );
}

#[test]
fn the_app_name_counts_for_windows_and_apps() {
    let rows = vec![
        window("x", "Xterm", "monitor"),
        window("org.kde.konsole", "Konsole", "~ : bash"),
    ];

    assert_eq!(titles(&exact_first("kons", rows)), ["~ : bash", "monitor"]);
}

#[test]
fn refresh_puts_the_miro_tab_first() {
    let input = Input::new(
        LauncherConfig::default(),
        vec![AppInfo {
            id: "chromium-browser".into(),
            name: "Chromium Web Browser".into(),
            icon: None,
        }],
    );
    let mut state = State::new(vec![window(
        "chromium-browser",
        "Chromium Web Browser",
        "Google Calendar - Week of October 26, 2026 - Chromium",
    )]);

    state.tab_lists = vec![TabList {
        source: SourceId(1),
        pid: 7,
        windows: Vec::new(),
        browser: "Brave".into(),
        icon: "brave-browser".into(),
        tabs: vec![TabInfo {
            id: 3,
            window: 1,
            active: false,
            last: 1,
            title: "Overall - Miro".into(),
            url: "https://miro.com/app".into(),
            icon: "https://miro.com".into(),
        }],
    }];
    state.query = "miro".into();
    input.refresh(&mut state);
    assert_eq!(state.output[0].title, "Overall - Miro — miro.com");
    assert!(matches!(state.output[0].target, RowTarget::Tab(_)));
}
