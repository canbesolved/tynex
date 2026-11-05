// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `hotkeys.select` and `hotkeys.space`.

use super::parse;
use crate::core::hotkey::{Binding, Bindings};

fn no_font(_: &std::path::Path) -> bool {
    false
}

fn keys(texts: &[&str]) -> Bindings {
    Bindings(texts.iter().map(|t| Binding::parse(t).unwrap()).collect())
}

fn errors(text: &str) -> Vec<String> {
    parse(text, &no_font)
        .unwrap_err()
        .iter()
        .map(ToString::to_string)
        .collect()
}

#[test]
fn enter_and_space_select_and_a_left_shift_tap_types_a_space() {
    let config = parse("", &no_font).unwrap();

    assert_eq!(config.hotkeys.select, keys(&["Enter", "Space"]));
    assert_eq!(config.hotkeys.space, keys(&["ShiftLeft"]));
}

#[test]
fn space_can_type_spaces() {
    let text = "[hotkeys]\nselect = \"Enter\"\nspace = \"Space\"\n";
    let config = parse(text, &no_font).unwrap();

    assert_eq!(config.hotkeys.select, keys(&["Enter"]));
    assert_eq!(config.hotkeys.space, keys(&["Space"]));

    let off = parse("[hotkeys]\nspace = \"\"\n", &no_font).unwrap();

    assert_eq!(off.hotkeys.space, Bindings::default());
}

#[test]
fn a_key_that_selects_and_types_a_space_is_an_error() {
    let all = errors("[hotkeys]\nspace = \"Space\"\n");

    assert_eq!(all.len(), 1, "{all:?}");
    assert!(
        all[0].contains("`hotkeys.select` and `hotkeys.space`"),
        "{all:?}"
    );
    assert!(errors("[hotkeys]\nselect = \"Hyper\"\n")[0].contains("select"));
}
