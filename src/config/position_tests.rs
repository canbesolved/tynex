// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `main.position`: centered by default, or at the top.

use super::*;

#[test]
fn position_defaults_to_center() {
    let config = parse("", &|_| false).expect("empty config");

    assert_eq!(config.main.position, Position::Center);
}

#[test]
fn position_top() {
    let config = parse("[main]\nposition = \"top\"\n", &|_| false)
        .expect("valid config");

    assert_eq!(config.main.position, Position::Top);
}

#[test]
fn position_rejects_other_values() {
    let msg = parse("[main]\nposition = \"bottom\"\n", &|_| false)
        .expect_err("bottom is not a position");

    assert!(msg[0].to_string().contains("unknown variant"), "{msg:?}");
}
