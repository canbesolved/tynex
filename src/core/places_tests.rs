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
    places.listed(SourceId(1), BRAVE, &lists[0].tabs, Some(&cursor));
    assert_eq!(
        places.back(&[cursor, browser, kate], &lists),
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
    places.listed(SourceId(1), BRAVE, &[tab(1, true, 5)], Some(&browser));
    places.opened(tab_ref(2), BRAVE);

    let lists = [brave(vec![tab(1, false, 5), tab(2, true, 9)])];

    assert_eq!(
        places.back(&windows, &lists),
        Some(Action::ActivateTab(tab_ref(1)))
    );

    // Back to GitHub by a click in the browser: LinkedIn is previous.
    let clicked = [tab(1, true, 12), tab(2, false, 9)];

    places.listed(SourceId(1), BRAVE, &clicked, Some(&browser));
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
