// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Scores stay within 0 to 100 and survive a round trip.

use super::*;
use crate::core::browser_tabs::{SourceId, TabInfo, TabRef};
use crate::core::window::WindowId;

fn app(id: &str) -> Item {
    Item::App(id.into())
}

fn tab(browser: &str, url: &str) -> Item {
    Item::Tab {
        browser: browser.into(),
        url: url.into(),
    }
}

fn list(browser: &str, source: u32, urls: &[&str]) -> TabList {
    TabList {
        source: SourceId(source),
        pid: 1,
        browser: browser.into(),
        icon: String::new(),
        tabs: urls
            .iter()
            .zip(1..)
            .map(|(url, id)| TabInfo {
                id,
                window: 1,
                active: false,
                last: 0,
                title: String::new(),
                url: (*url).into(),
                icon: String::new(),
            })
            .collect(),
        windows: Vec::new(),
    }
}

#[test]
fn a_use_adds_one_and_past_100_all_scores_halve() {
    let mut usage = Usage::default();

    for _ in 0..100 {
        usage.record(app("kate"));
    }
    usage.record(app("firefox"));
    assert_eq!(usage.score(&app("kate")), 100);
    assert!(usage.take_changed());
    assert!(!usage.take_changed());

    usage.record(app("kate"));
    assert_eq!(usage.score(&app("kate")), 50);
    assert_eq!(usage.score(&app("firefox")), 0); // Halved to nothing
    assert_eq!(usage.to_text(), "50\tapp\tkate\n");
}

#[test]
fn text_round_trips_and_bad_lines_are_skipped() {
    let mut usage = Usage::default();

    usage.record(tab("Brave", "https://notes.example/a"));
    usage.record(app("kate"));
    usage.record(app("kate"));
    usage.record(app("bad\tid"));

    let text = usage.to_text();

    assert_eq!(
        text,
        "2\tapp\tkate\n1\ttab\tBrave\thttps://notes.example/a\n"
    );
    assert_eq!(Usage::parse(&text).to_text(), text);

    let parsed = Usage::parse("x\tapp\ta\n300\tapp\tb\n0\tapp\tc\n1\tzip\td\n");

    assert_eq!(parsed.to_text(), "100\tapp\tb\n");
}

#[test]
fn gone_apps_and_closed_tabs_are_dropped() {
    let mut usage = Usage::parse(
        "3\tapp\tkate\n2\tapp\tgone\n\
         2\ttab\tBrave\thttps://a.example\n\
         2\ttab\tBrave\thttps://b.example\n\
         2\ttab\tChrome\thttps://c.example\n",
    );

    usage.retain_apps(|id| id == "kate");
    assert_eq!(usage.score(&app("gone")), 0);
    assert!(usage.take_changed());

    // Two Brave profiles: a tab open in either one stays.
    let lists = [
        list("Brave", 1, &["https://a.example"]),
        list("Brave", 2, &["https://x.example"]),
    ];

    usage.retain_tabs("Brave", &lists);
    assert_eq!(usage.score(&tab("Brave", "https://a.example")), 2);
    assert_eq!(usage.score(&tab("Brave", "https://b.example")), 0);
    assert_eq!(usage.score(&tab("Chrome", "https://c.example")), 2);
    assert_eq!(usage.score(&app("kate")), 3);
}

#[test]
fn rows_name_their_item() {
    let window = RowTarget::Window(WindowId("w1".into()));
    let rows = [Row {
        target: window.clone(),
        app_id: "kate".into(),
        app_name: "Kate".into(),
        icon: None,
        title: "Notes".into(),
        title_color: None,
        highlights: Vec::new(),
    }];
    let lists = [list("Brave", 1, &["https://a.example"])];
    let page = RowTarget::Tab(TabRef {
        source: SourceId(1),
        id: 1,
    });

    assert_eq!(item(&window, &rows, &lists), Some(app("kate")));
    assert_eq!(
        item(&RowTarget::Launch("firefox".into()), &rows, &lists),
        Some(app("firefox"))
    );
    assert_eq!(
        item(&page, &rows, &lists),
        Some(tab("Brave", "https://a.example"))
    );
    assert_eq!(
        item(&RowTarget::Window(WindowId("w9".into())), &rows, &lists),
        None
    );
}
