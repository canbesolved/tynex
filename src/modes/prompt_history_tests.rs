// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The history in the popup: Kate, then GitHub in Brave, then Cursor.
//! The popup opens on the GitHub tab, in the title list and the strip.

use crate::config::{LauncherConfig, TaskbarMode};
use crate::core::browser_tabs::{SourceId, TabInfo, TabRef, TabsUpdate};
use crate::core::event::{Action, Event, Key};
use crate::core::order::SortBy;
use crate::core::switcher::{HotkeyAction, Switcher};
use crate::core::window::{WindowId, WindowInfo};
use crate::modes::prompt::PromptMode;
use crate::modules::input::Input;

fn window(id: &str, pid: u32, title: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.into()),
        app_id: id.into(),
        app_name: id.into(),
        title: title.into(),
        icon: Some(id.into()),
        pid: Some(pid),
    }
}

fn github() -> TabRef {
    TabRef {
        source: SourceId(1),
        id: 3,
    }
}

/// A switcher that saw Kate, Brave on GitHub, then Cursor, opened.
fn opened(taskbar: TaskbarMode, sort: SortBy) -> Switcher<PromptMode> {
    let input = Input::new(LauncherConfig::default(), Vec::new());
    let mode = PromptMode::new(input, taskbar, 10).with_tabs(true);
    let mut sw =
        Switcher::new(HotkeyAction::Last, false, sort, mode).with_history(10);
    let (kate, brave, cursor) = (
        window("kate", 1, "notes.txt"),
        window("brave", 7, "GitHub - Brave"),
        window("cursor", 2, "main.rs"),
    );

    sw.handle(Event::Windows(vec![kate.clone()]));
    sw.handle(Event::Windows(vec![brave.clone(), kate.clone()]));
    sw.handle(Event::Windows(vec![cursor, brave, kate]));
    sw.handle(Event::Show);
    sw.handle(Event::Tabs(TabsUpdate::List {
        source: SourceId(1),
        pid: 7,
        browser: "Brave".into(),
        tabs: vec![TabInfo {
            id: 3,
            window: 1,
            active: true,
            last: 1,
            title: "GitHub".into(),
            url: "https://github.com".into(),
            icon: "https://github.com".into(),
        }],
    }));
    sw
}

#[test]
fn the_title_list_opens_on_the_github_tab() {
    let mut sw = opened(TaskbarMode::Titlebar, SortBy::Recently);
    let view = sw.view().unwrap();
    let titles: Vec<&str> = view.titles().map(|r| r.title.as_str()).collect();

    assert_eq!(titles, ["GitHub — github.com", "notes.txt", "main.rs"]);
    assert_eq!(view.selected, 0);
    assert_eq!(
        sw.handle(Event::Key(Key::Enter)),
        Action::ActivateTab(github())
    );
}

#[test]
fn the_strip_shows_the_github_tab_after_the_pinned_apps() {
    let mut sw = opened(TaskbarMode::Iconbar, SortBy::Recently);
    let view = sw.view().unwrap();
    let icons = view.icons();

    assert_eq!(icons[0].tab.as_ref().map(|t| t.tab.clone()), Some(github()));
    assert_eq!(view.icon, 0);
    assert!(icons.iter().any(|i| i.app_id == "brave")); // Still listed
    assert_eq!(
        sw.handle(Event::Key(Key::Enter)),
        Action::ActivateTab(github())
    );
}

#[test]
fn rows_by_app_keep_their_order_but_the_hotkey_still_goes_back() {
    let mut sw = opened(TaskbarMode::Titlebar, SortBy::App);
    let titles: Vec<String> = sw
        .view()
        .unwrap()
        .titles()
        .map(|r| r.title.clone())
        .collect();

    assert_eq!(titles, ["GitHub - Brave", "main.rs", "notes.txt"]);
    assert_eq!(sw.handle(Event::Show), Action::ActivateTab(github()));
}

#[test]
fn a_pinned_app_is_selected_in_place() {
    let input = Input::new(LauncherConfig::default(), Vec::new());
    let mode = PromptMode::new(input, TaskbarMode::Iconbar, 10);
    let kate = crate::core::app::AppInfo {
        id: "kate".into(),
        name: "Kate".into(),
        icon: None,
    };
    let mut sw =
        Switcher::new(HotkeyAction::Last, false, SortBy::Recently, mode)
            .with_history(10)
            .with_pinned(vec![kate]);

    sw.handle(Event::Windows(vec![window("kate", 1, "notes.txt")]));
    sw.handle(Event::Windows(vec![
        window("cursor", 2, "main.rs"),
        window("kate", 1, "notes.txt"),
    ]));
    sw.handle(Event::Show);

    let view = sw.view().unwrap();
    let apps: Vec<&str> =
        view.icons().iter().map(|i| i.app_id.as_str()).collect();

    assert_eq!(apps, ["kate", "cursor"]);
    assert_eq!(view.icon, 0);
}

#[test]
fn a_selection_moved_before_the_list_stays() {
    let input = Input::new(LauncherConfig::default(), Vec::new());
    let mode =
        PromptMode::new(input, TaskbarMode::Titlebar, 10).with_tabs(true);
    let mut sw =
        Switcher::new(HotkeyAction::Last, false, SortBy::Recently, mode)
            .with_history(10);

    sw.handle(Event::Windows(vec![window("brave", 7, "GitHub - Brave")]));
    sw.handle(Event::Windows(vec![
        window("kate", 1, "notes.txt"),
        window("brave", 7, "GitHub - Brave"),
    ]));
    sw.handle(Event::Show);
    sw.handle(Event::Key(Key::Down)); // From Brave to Kate
    sw.handle(Event::Tabs(TabsUpdate::List {
        source: SourceId(1),
        pid: 7,
        browser: "Brave".into(),
        tabs: vec![TabInfo {
            id: 3,
            window: 1,
            active: true,
            last: 1,
            title: "GitHub".into(),
            url: "https://github.com".into(),
            icon: String::new(),
        }],
    }));

    let view = sw.view().unwrap();
    let selected = view.titles().nth(view.selected).unwrap();

    assert_eq!(selected.title, "notes.txt");
}
