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
fn missing_table_uses_the_dim_default() {
    let config = parse("", &no_font).expect("valid config");
    let row = config.input_bar.launcher.new_window;

    assert!(row.enabled);
    assert_eq!(row.font_color, Rgb::new(0xA8, 0xB3, 0xBD));
    assert_eq!(row.text, "  +  New window");
}

#[test]
fn new_window_values() {
    let text = "\
[input_bar.launcher.new_window]\n\
enabled = false\n\
font_color = \"#112233\"\n\
text = \"  +  Open\"\n";
    let row = parse(text, &no_font)
        .expect("valid config")
        .input_bar
        .launcher
        .new_window;

    assert!(!row.enabled);
    assert_eq!(row.font_color, Rgb::new(0x11, 0x22, 0x33));
    assert_eq!(row.text, "  +  Open");
}

#[test]
fn bad_color_and_empty_text_are_reported() {
    let text = "\
[input_bar.launcher.new_window]\n\
font_color = \"##112233\"\n\
text = \"\"\n";
    let all = errors(text);

    assert!(
        all.iter()
            .any(|m| m.contains("font_color") && m.contains("#RRGGBB")),
        "{all:?}"
    );
    assert!(all.iter().any(|m| m.contains("text")), "{all:?}");
}
