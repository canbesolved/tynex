// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Back to the previous window or tab.

use super::*;

const BRAVE: u32 = 7;

fn window(id: &str, pid: u32) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.into()),
        app_id: id.into(),
        app_name: id.into(),
        title: id.into(),
        icon: None,
        pid: Some(pid),
    }
}

fn tab(id: i32, active: bool, last: u64) -> TabInfo {
    TabInfo {
        id,
        window: 1,
        active,
        last,
        title: String::new(),
        url: format!("https://{id}.example"),
        icon: String::new(),
    }
}

fn tab_ref(id: i32) -> TabRef {
    TabRef {
        source: SourceId(1),
        id,
    }
}

fn brave(tabs: Vec<TabInfo>) -> TabList {
    TabList {
        source: SourceId(1),
        pid: BRAVE,
        browser: "Brave".into(),
        icon: String::new(),
        tabs,
        windows: Vec::new(),
    }
}

#[test]
fn a_tab_opened_before_another_app_is_the_way_back() {
    let (kate, browser, cursor) = (
        window("kate", 1),
        window("brave", BRAVE),
        window("cursor", 2),
    );
    let lists = [brave(vec![tab(1, true, 5)])];
    let mut places = Places::default();

    places.focused(&kate);
    places.opened(tab_ref(1), BRAVE); // GitHub, from the popup
    places.focused(&browser); // Raising its window changes nothing
    assert_eq!(
        places.back(&[browser.clone(), kate.clone()], &lists),
        Some(Action::Activate(kate.id.clone()))
    );

    places.focused(&cursor);
    let windows = [cursor, browser, kate];

    places.listed(SourceId(1), BRAVE, &lists[0].tabs, &windows);
    assert_eq!(
        places.back(&windows, &lists),
        Some(Action::ActivateTab(tab_ref(1)))
    );
}

#[test]
fn the_previous_tab_of_the_same_browser_is_the_way_back() {
    let browser = window("brave", BRAVE);
    let windows = [browser.clone()];
    let mut places = Places::default();

    // GitHub, then LinkedIn from the popup.
    places.focused(&browser);
    places.listed(SourceId(1), BRAVE, &[tab(1, true, 5)], &windows);
    places.opened(tab_ref(2), BRAVE);

    let lists = [brave(vec![tab(1, false, 5), tab(2, true, 9)])];

    assert_eq!(
        places.back(&windows, &lists),
        Some(Action::ActivateTab(tab_ref(1)))
    );

    // Back to GitHub by a click in the browser: LinkedIn is previous.
    let clicked = [tab(1, true, 12), tab(2, false, 9)];

    places.listed(SourceId(1), BRAVE, &clicked, &windows);
    assert_eq!(
        places.back(&windows, &lists),
        Some(Action::ActivateTab(tab_ref(2)))
    );
}

#[test]
fn a_tab_without_its_list_raises_its_browser() {
    let (browser, cursor) = (window("brave", BRAVE), window("cursor", 2));
    let mut places = Places::default();

    places.opened(tab_ref(1), BRAVE);
    places.focused(&cursor);
    assert_eq!(
        places.back(&[cursor.clone(), browser.clone()], &[]),
        Some(Action::Activate(browser.id.clone()))
    );
    assert_eq!(places.back(&[cursor], &[]), None); // The browser closed
}

fn titled(id: &str, pid: u32, title: &str) -> WindowInfo {
    WindowInfo {
        title: title.into(),
        ..window(id, pid)
    }
}

fn named(id: i32, active: bool, title: &str) -> TabInfo {
    TabInfo {
        title: title.into(),
        ..tab(id, active, 1)
    }
}

#[test]
fn a_browser_window_becomes_its_active_tab() {
    let kate = window("kate", 1);
    let browser = titled("brave", BRAVE, "Mail - Brave");
    let tabs = [named(1, true, "Mail"), named(2, false, "Docs")];
    let mut places = Places::new(10);

    places.focused(&browser);
    places.focused(&kate);
    places.listed(SourceId(1), BRAVE, &tabs, &[kate.clone(), browser.clone()]);
    assert_eq!(
        places.list(),
        [
            Place::Window(kate.id.clone()),
            Place::Tab {
                tab: tab_ref(1),
                pid: BRAVE,
                window: Some(browser.id.clone()),
            },
        ]
    );

    // Focusing the browser again moves its tab to the front.
    places.focused(&browser);
    assert!(matches!(places.list()[0], Place::Tab { .. }));
    assert_eq!(places.list().len(), 2);
}

#[test]
fn the_history_is_capped_and_drops_what_closed() {
    let mut places = Places::new(3);
    let windows: Vec<WindowInfo> = ["a", "b", "c", "d"]
        .iter()
        .map(|id| window(id, 1))
        .collect();

    for w in &windows {
        places.focused(w);
    }
    places.opened(tab_ref(1), BRAVE);
    assert_eq!(places.list().len(), 3);
    assert!(matches!(places.list()[0], Place::Tab { .. }));
    assert_eq!(places.list()[1], Place::Window(windows[3].id.clone()));

    places.retain(&windows[..3]); // `d` closed
    places.gone(SourceId(1)); // The browser went away
    assert_eq!(places.list(), [Place::Window(windows[2].id.clone())]);

    places.opened(tab_ref(5), BRAVE);
    places.listed(SourceId(1), BRAVE, &[tab(6, true, 1)], &[]);
    assert_eq!(places.list().len(), 1); // Tab 5 closed
}
