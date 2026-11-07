// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `main.history`, `main.switch_counter` and `browser_tabs.wait`.

use super::parse;

fn no_font(_: &std::path::Path) -> bool {
    false
}

fn errors(text: &str) -> Vec<String> {
    parse(text, &no_font)
        .unwrap_err()
        .iter()
        .map(ToString::to_string)
        .collect()
}

#[test]
fn ten_places_and_150_ms_by_default() {
    let config = parse("", &no_font).unwrap();

    assert_eq!(config.main.history, 10);
    assert_eq!(config.browser_tabs.wait, 150);
}

#[test]
fn values_in_range_are_kept_and_others_are_errors() {
    let text = "[main]\nhistory = 50\n[browser_tabs]\nwait = 0\n";
    let config = parse(text, &no_font).unwrap();

    assert_eq!((config.main.history, config.browser_tabs.wait), (50, 0));
    assert!(errors("[main]\nhistory = 1\n")[0].contains("`main.history`"));
    assert!(
        errors("[browser_tabs]\nwait = 1001\n")[0]
            .contains("`browser_tabs.wait` = 1001 is out of range")
    );
}

#[test]
fn the_switch_counter_is_off_by_default() {
    let on = parse("[main]\nswitch_counter = true\n", &no_font).unwrap();

    assert!(!parse("", &no_font).unwrap().main.switch_counter);
    assert!(on.main.switch_counter);
    assert!(errors("[main]\nswitch_counter = \"yes\"\n")[0].contains("bool"));
}
