// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Taskbar mode.

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
fn taskbar_defaults_to_icons() {
    let config = parse("", &no_font).expect("valid config");

    assert_eq!(config.taskbar.mode, TaskbarMode::Icons);
}

#[test]
fn taskbar_mode_values() {
    let mode = |text| parse(text, &no_font).map(|c| c.taskbar.mode);

    assert_eq!(mode("[taskbar]\nmode = \"off\"\n"), Ok(TaskbarMode::Off));
    assert_eq!(
        mode("[taskbar]\nmode = \"icons\"\n"),
        Ok(TaskbarMode::Icons)
    );
    assert_eq!(
        mode("[taskbar]\nmode = \"titles\"\n"),
        Ok(TaskbarMode::Titles)
    );
    assert_eq!(mode("[taskbar]\nmode = \"both\"\n"), Ok(TaskbarMode::Both));
    assert!(
        errors("[taskbar]\nmode = \"none\"\n")[0].contains("unknown variant")
    );
}

#[test]
fn removed_enabled_keys_are_unknown() {
    let title = &errors("[title_bar]\nenabled = true\n")[0];
    let icon = &errors("[icon_bar]\nenabled = false\n")[0];

    assert!(title.contains("unknown key `enabled`"), "{title}");
    assert!(title.contains("release notes"), "{title}");
    assert!(icon.contains("unknown key `enabled`"), "{icon}");
    assert!(icon.contains("release notes"), "{icon}");
}
