// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `main.theme`: dark by default, or light.

use super::super::parse;
use super::Theme;

#[test]
fn theme_defaults_to_dark() {
    let config = parse("", &|_| false).expect("empty config");

    assert_eq!(config.main.theme, Theme::Dark);
}

#[test]
fn theme_light() {
    let config =
        parse("[main]\ntheme = \"light\"\n", &|_| false).expect("valid config");

    assert_eq!(config.main.theme, Theme::Light);
}

#[test]
fn theme_rejects_other_values() {
    let msg = parse("[main]\ntheme = \"blue\"\n", &|_| false)
        .expect_err("blue is not a theme");

    assert!(msg[0].to_string().contains("unknown variant"), "{msg:?}");
}
