// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon bar config.

use super::*;

fn no_font(_: &Path) -> bool {
    false
}

fn errors(text: &str) -> Vec<String> {
    parse(text, &no_font)
        .expect_err("config must be invalid")
        .iter()
        .map(ToString::to_string)
        .collect()
}

#[test]
fn icon_bar_defaults() {
    let config = parse("", &no_font).expect("valid config");

    assert!(config.icon_bar.enabled);
    assert_eq!(config.icon_bar.icon_size, 24);
}

#[test]
fn icon_size_range() {
    let small = &errors("[icon_bar]\nicon_size = 7\n")[0];
    let large = &errors("[icon_bar]\nicon_size = 129\n")[0];

    assert!(small.contains("out of range"), "{small}");
    assert!(large.contains("out of range"), "{large}");

    let config =
        parse("[icon_bar]\nenabled = false\nicon_size = 32\n", &no_font)
            .expect("valid config");

    assert!(!config.icon_bar.enabled);
    assert_eq!(config.icon_bar.icon_size, 32);
}
