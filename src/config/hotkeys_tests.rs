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

    let select = keys(&["Enter", "Space"]);
    let space = keys(&["Space"]);

    conflicts(
        &[
            ("select".to_owned(), &select, 0..0),
            ("space".to_owned(), &space, 0..0),
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

fn pinned(text: &str) -> Result<Vec<Bindings>, Vec<String>> {
    parse(&format!("[hotkeys]\npinned = {text}\n"), &no_font)
        .map(|c| c.hotkeys.pinned)
        .map_err(|errors| errors.iter().map(ToString::to_string).collect())
}

#[test]
fn pinned_keys_default_to_alt_and_a_digit() {
    let config = parse("", &no_font).unwrap();
    let alt: Vec<String> = (1..=9).map(|n| format!("Alt+{n}")).collect();
    let alt: Vec<&str> = alt.iter().map(String::as_str).collect();
    let expected: Vec<Bindings> = alt.iter().map(|k| keys(&[k])).collect();

    assert_eq!(config.hotkeys.pinned, expected);
}

#[test]
fn pinned_keys_go_by_position_and_take_lists() {
    assert_eq!(
        pinned("[\"F1\", [\"F2\", \"Ctrl+2\"], \"\"]"),
        Ok(vec![
            keys(&["F1"]),
            keys(&["F2", "Ctrl+2"]),
            Bindings::default()
        ])
    );
    assert_eq!(pinned("[]"), Ok(Vec::new()));
}

#[test]
fn bad_pinned_keys_are_errors() {
    assert!(pinned("\"Alt+1\"").is_err());
    assert!(pinned("[\"Hyper+1\"]").is_err());

    let errors = pinned("[\"F1\", \"F1\"]").unwrap_err();

    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].contains("`hotkeys.pinned[1]` and `hotkeys.pinned[2]`"),
        "{}",
        errors[0]
    );

    let errors =
        parse("[hotkeys]\nrename = \"F1\"\npinned = [\"F1\"]\n", &no_font)
            .unwrap_err();

    assert_eq!(errors.len(), 1);
}
