// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `[browser_tabs]` parsing.

use super::{Browsers, parse};

fn no_font(_: &std::path::Path) -> bool {
    false
}

#[test]
fn auto_and_a_list() {
    let config = parse(
        "[browser_tabs]\nenabled = true\nbrowsers = \"auto\"\n",
        &no_font,
    )
    .unwrap();

    assert!(config.browser_tabs.enabled);
    assert_eq!(config.browser_tabs.browsers, Browsers::Auto);
    let config = parse(
        "[browser_tabs]\nbrowsers = [\"brave\", \"chrome\", \"chromium\"]\n",
        &no_font,
    )
    .unwrap();

    assert_eq!(
        config.browser_tabs.browsers,
        Browsers::List(vec![
            super::Browser::Brave,
            super::Browser::Chrome,
            super::Browser::Chromium,
        ])
    );
}

#[test]
fn one_name_is_a_list_of_one() {
    let config =
        parse("[browser_tabs]\nbrowsers = \"brave\"\n", &no_font).unwrap();

    assert_eq!(
        config.browser_tabs.browsers,
        Browsers::List(vec![super::Browser::Brave])
    );
}

#[test]
fn an_unknown_browser_is_an_error() {
    for name in ["lynx", "edge", "vivaldi"] {
        let list = format!("[browser_tabs]\nbrowsers = [\"{name}\"]\n");
        let one = format!("[browser_tabs]\nbrowsers = \"{name}\"\n");

        assert!(parse(&list, &no_font).is_err(), "{name}");
        assert!(parse(&one, &no_font).is_err(), "{name}");
    }
}
