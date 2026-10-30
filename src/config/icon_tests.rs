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

    assert_eq!(config.taskbar.iconbar.size, 24);
    assert_eq!(config.taskbar.iconbar.border, 4);
}

#[test]
fn icon_size_range() {
    let small = &errors("[taskbar]\niconbar.size = 7\n")[0];
    let large = &errors("[taskbar]\niconbar.size = 129\n")[0];

    assert!(small.contains("out of range"), "{small}");
    assert!(large.contains("out of range"), "{large}");

    let config = parse("[taskbar]\niconbar.size = 32\n", &no_font)
        .expect("valid config");

    assert_eq!(config.taskbar.iconbar.size, 32);
}

#[test]
fn border_range() {
    let small = &errors("[taskbar]\niconbar.border = 0\n")[0];
    let large = &errors("[taskbar]\niconbar.border = 33\n")[0];

    assert!(small.contains("out of range"), "{small}");
    assert!(large.contains("out of range"), "{large}");
    assert_eq!(
        parse("[taskbar]\niconbar.border = 8\n", &no_font)
            .expect("valid config")
            .taskbar
            .iconbar
            .border,
        8
    );
}
