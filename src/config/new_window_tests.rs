// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! New-window row config.

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
fn missing_keys_keep_the_row_on() {
    let row = parse("[input]\nlauncher.mode = \"all\"\n", &no_font)
        .expect("valid config")
        .input
        .launcher
        .new_window;

    assert!(row.enabled);
    assert_eq!(row.text_color, NEW_WINDOW_COLOR);
    assert_eq!(row.text, NEW_WINDOW_TEXT);
}

#[test]
fn new_window_values() {
    let text = "\
[input]\n\
launcher.new_window = false\n\
launcher.new_window_text_color = \"#112233\"\n\
launcher.new_window_text = \" +  Open\"\n";
    let row = parse(text, &no_font)
        .expect("valid config")
        .input
        .launcher
        .new_window;

    assert!(!row.enabled);
    assert_eq!(row.text_color, Rgb::new(0x11, 0x22, 0x33));
    assert_eq!(row.text, " +  Open");
}

#[test]
fn bad_color_and_empty_text_are_reported() {
    let text = "\
[input]\n\
launcher.new_window_text_color = \"##112233\"\n\
launcher.new_window_text = \"\"\n";
    let all = errors(text);

    assert!(
        all.iter().any(|m| {
            m.contains("new_window_text_color") && m.contains("#RRGGBB")
        }),
        "{all:?}"
    );
    assert!(all.iter().any(|m| m.contains("new_window_text")), "{all:?}");
}

#[test]
fn nested_new_window_table_is_unknown() {
    let msg = &errors("[input.launcher.new_window]\nenabled = true\n")[0];

    assert!(
        msg.contains("new_window") || msg.contains("invalid type"),
        "{msg}"
    );
}
