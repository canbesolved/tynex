// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `[hotkeys]` key bindings.

use super::hotkeys::{conflicts, default_rename};
use super::parse;
use crate::core::hotkey::{Binding, Bindings};

fn no_font(_: &std::path::Path) -> bool {
    false
}

fn rename(text: &str) -> Result<Bindings, Vec<String>> {
    parse(&format!("[hotkeys]\nrename = {text}\n"), &no_font)
        .map(|c| c.hotkeys.rename)
        .map_err(|errors| errors.iter().map(ToString::to_string).collect())
}

fn keys(texts: &[&str]) -> Bindings {
    Bindings(texts.iter().map(|t| Binding::parse(t).unwrap()).collect())
}

#[test]
fn rename_defaults_to_a_dot() {
    let config = parse("", &no_font).unwrap();

    assert_eq!(config.hotkeys.rename, keys(&["."]));
    assert_eq!(config.hotkeys.rename, default_rename());
}

#[test]
fn rename_takes_one_key_or_a_list() {
    assert_eq!(rename("\"F2\""), Ok(keys(&["F2"])));
    assert_eq!(
        rename("[\"F2\", \"Ctrl + R\"]"),
        Ok(keys(&["F2", "Ctrl+R"]))
    );
}

#[test]
fn an_empty_string_or_list_turns_rename_off() {
    assert_eq!(rename("\"\""), Ok(Bindings::default()));
    assert_eq!(rename("[]"), Ok(Bindings::default()));
}

#[test]
fn a_bad_binding_is_reported_with_its_line() {
    let errors = rename("\"Hyper+R\"").unwrap_err();

    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0],
        "2:10: `hotkeys.rename` = \"Hyper+R\" is not a key: `Hyper` is not \
         a modifier"
    );
    assert!(rename("42").is_err());
    assert!(rename("[\"F2\", 1]").is_err());
}

#[test]
fn a_key_used_by_two_actions_is_an_error() {
    let mut errors = Vec::new();
    let mut err = |_, message: String| errors.push(message);

    conflicts(
        &[
            ("select", &keys(&["Enter", "Space"]), 0..0),
            ("space", &keys(&["Space"]), 0..0),
        ],
        &mut err,
    );
    assert_eq!(
        errors,
        [
            "`hotkeys.select` and `hotkeys.space` use the same key, expected \
          different keys"
        ]
    );
}
