// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hint config.

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
fn missing_keys_keep_the_hint_on() {
    let hint = parse("[input_bar]\n", &no_font)
        .expect("valid config")
        .input_bar
        .hint;

    assert!(hint.enabled);
    assert_eq!(hint.font_color, HINT_COLOR);
    assert_eq!(hint.text, HINT_TEXT);
    assert_eq!(hint.update, 0);
}

#[test]
fn hint_values() {
    let text = "\
[input_bar]\n\
hint.enabled = false\n\
hint.font_color = \"#112233\"\n\
hint.text = \"printf hi\"\n\
hint.update = 15\n";
    let hint = parse(text, &no_font).expect("valid config").input_bar.hint;

    assert!(!hint.enabled);
    assert_eq!(hint.font_color, Rgb::new(0x11, 0x22, 0x33));
    assert_eq!(hint.text, "printf hi");
    assert_eq!(hint.update, 15);
}

#[test]
fn bad_hint_color_and_update_fail() {
    let color = &errors("[input_bar]\nhint.font_color = \"blue\"\n")[0];
    let update = &errors("[input_bar]\nhint.update = 86401\n")[0];

    assert!(color.contains("hint.font_color"), "{color}");
    assert!(color.contains("#RRGGBB"), "{color}");
    assert!(update.contains("hint.update"), "{update}");
    assert!(update.contains("0 to 86400"), "{update}");
}
