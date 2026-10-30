// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Section names from 0.2.11.

use super::*;
use crate::core::order::SortBy;

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
fn old_sections_are_unknown_keys() {
    for section in ["input_bar", "output_bar", "title_bar", "icon_bar"] {
        let msg = &errors(&format!("[{section}]\n"))[0];

        assert!(
            msg.contains(&format!("unknown key `{section}`")),
            "{section}: {msg}"
        );
        assert!(msg.contains("release notes"), "{section}: {msg}");
    }
}

#[test]
fn old_mode_values_are_rejected() {
    for value in ["icons", "titles"] {
        let msg = &errors(&format!("[taskbar]\nmode = \"{value}\"\n"))[0];

        assert!(msg.contains("unknown variant"), "{value}: {msg}");
        assert!(msg.contains(value), "{value}: {msg}");
    }
}

#[test]
fn new_layout_parses() {
    let text = "\
[input]\n\
launcher.mode = \"off\"\n\
[output]\n\
max_rows = 7\n\
[taskbar]\n\
mode = \"titlebar\"\n\
titlebar.sort_by = \"app\"\n\
titlebar.show_icons = false\n\
titlebar.strip_app_name = true\n\
iconbar.size = 32\n\
iconbar.border = 2\n";
    let config = parse(text, &no_font).expect("valid config");

    assert_eq!(config.input.launcher.mode, LauncherMode::Off);
    assert_eq!(config.output.max_rows, 7);
    assert_eq!(config.taskbar.mode, TaskbarMode::Titlebar);
    assert_eq!(config.taskbar.titlebar.sort_by, SortBy::App);
    assert!(!config.taskbar.titlebar.show_icons);
    assert!(config.taskbar.titlebar.strip_app_name);
    assert_eq!(config.taskbar.iconbar.size, 32);
    assert_eq!(config.taskbar.iconbar.border, 2);
}
