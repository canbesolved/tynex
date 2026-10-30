// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Taskbar mode.

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
fn taskbar_defaults_to_iconbar() {
    let config = parse("", &no_font).expect("valid config");

    assert_eq!(config.taskbar.mode, TaskbarMode::Iconbar);
}

#[test]
fn taskbar_mode_values() {
    let mode = |text| parse(text, &no_font).map(|c| c.taskbar.mode);

    assert_eq!(mode("[taskbar]\nmode = \"off\"\n"), Ok(TaskbarMode::Off));
    assert_eq!(
        mode("[taskbar]\nmode = \"iconbar\"\n"),
        Ok(TaskbarMode::Iconbar)
    );
    assert_eq!(
        mode("[taskbar]\nmode = \"titlebar\"\n"),
        Ok(TaskbarMode::Titlebar)
    );
    assert_eq!(mode("[taskbar]\nmode = \"both\"\n"), Ok(TaskbarMode::Both));
    assert!(
        errors("[taskbar]\nmode = \"none\"\n")[0].contains("unknown variant")
    );
}

#[test]
fn sort_by_values() {
    let sort = |text| parse(text, &no_font).map(|c| c.taskbar.titlebar.sort_by);

    assert_eq!(sort(""), Ok(SortBy::Recently));
    assert_eq!(
        sort("[taskbar]\ntitlebar.sort_by = \"app\"\n"),
        Ok(SortBy::App)
    );
    assert!(
        errors("[taskbar]\ntitlebar.sort_by = \"name\"\n")[0]
            .contains("unknown variant")
    );
    assert!(
        errors("[taskbar]\ntitlebar.sort_by = \"key\"\n")[0]
            .contains("unknown variant")
    );
}

#[test]
fn removed_enabled_keys_are_unknown() {
    let title = &errors("[taskbar]\ntitlebar.enabled = true\n")[0];
    let icon = &errors("[taskbar]\niconbar.enabled = false\n")[0];

    assert!(title.contains("unknown key `enabled`"), "{title}");
    assert!(title.contains("release notes"), "{title}");
    assert!(icon.contains("unknown key `enabled`"), "{icon}");
    assert!(icon.contains("release notes"), "{icon}");
}
