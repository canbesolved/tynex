// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for config parsing, validation and the file layer.

use std::fs;

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
fn default_file_matches_defaults() {
    assert_eq!(parse(DEFAULT_TOML, &no_font), Ok(Config::default()));
}

#[test]
fn missing_keys_use_defaults() {
    assert_eq!(parse("", &no_font), Ok(Config::default()));

    let config =
        parse("[main]\nwidth = 600\n", &no_font).expect("valid config");

    assert_eq!(config.main.width, 600);
    assert_eq!(config.main.font_size, 14);
}

#[test]
fn hotkey_action_values() {
    let action = |text| parse(text, &no_font).map(|c| c.hotkeys.action);

    assert_eq!(
        action("[hotkeys]\naction = \"last\"\n"),
        Ok(HotkeyAction::Last)
    );
    assert!(
        errors("[hotkeys]\naction = \"toggle\"\n")[0]
            .contains("unknown variant")
    );
}

#[test]
fn title_bar_icons_and_strip_values() {
    let text = "[taskbar]\ntitlebar.show_icons = false\n\
                titlebar.strip_app_name = true\n";
    let config = parse(text, &no_font).expect("valid config");

    assert!(!config.taskbar.titlebar.show_icons);
    assert!(config.taskbar.titlebar.strip_app_name);
}

#[test]
fn show_icons_defaults_to_true() {
    let config = parse("[taskbar]\n", &no_font).expect("valid config");

    assert!(config.taskbar.titlebar.show_icons);
}

#[test]
fn removed_show_app_icon_is_an_unknown_key() {
    let msg = &errors("[taskbar]\ntitlebar.show_app_icon = true\n")[0];

    assert!(msg.contains("unknown key `show_app_icon`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn empty_input_bar_uses_launcher_gui_only() {
    let config = parse("[input]\n", &no_font).expect("valid config");

    assert_eq!(config.input.launcher.mode, LauncherMode::GuiOnly);
}

#[test]
fn launcher_mode_values() {
    let mode = |text| parse(text, &no_font).map(|c| c.input.launcher.mode);

    assert_eq!(mode(""), Ok(LauncherMode::GuiOnly));
    assert_eq!(
        mode("[input]\nlauncher.mode = \"all\"\n"),
        Ok(LauncherMode::All)
    );
    assert_eq!(
        mode("[input]\nlauncher.mode = \"gui_only\"\n"),
        Ok(LauncherMode::GuiOnly)
    );
    assert!(
        errors("[input]\nlauncher.mode = \"yes\"\n")[0]
            .contains("unknown variant")
    );
}

#[test]
fn removed_appearance_is_an_unknown_key() {
    let msg = &errors("[appearance]\nfont = \"auto\"\n")[0];

    assert!(msg.contains("unknown key `appearance`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn old_top_level_keys_are_unknown() {
    for key in ["desktop", "hotkey_action", "sort_by"] {
        let msg = &errors(&format!("{key} = \"auto\"\n"))[0];

        assert!(
            msg.contains(&format!("unknown key `{key}`")),
            "{key}: {msg}"
        );
        assert!(msg.contains("release notes"), "{key}: {msg}");
    }
}

#[test]
fn old_appearance_keys_are_unknown_in_main() {
    let msg = &errors("[main]\nmax_rows = 10\n")[0];

    assert!(msg.contains("unknown key `max_rows`"), "{msg}");
}

#[test]
fn old_title_bar_key_is_unknown_in_appearance_path() {
    let msg = &errors("[taskbar]\ntitlebar.title_bar = true\n")[0];

    assert!(msg.contains("unknown key `title_bar`"), "{msg}");
}

#[test]
fn removed_show_app_is_an_unknown_key() {
    let msg = &errors("[taskbar]\ntitlebar.show_app = true\n")[0];

    assert!(msg.contains("unknown key `show_app`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn removed_title_format_is_an_unknown_key() {
    let msg = &errors("[taskbar]\ntitlebar.title_format = \"{title}\"\n")[0];

    assert!(msg.contains("unknown key `title_format`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn removed_mode_and_keys_are_unknown_keys() {
    let mode = &errors("mode = \"hotkey\"\n")[0];
    let keys = &errors("keys = \"asdf\"\n")[0];

    assert!(mode.contains("unknown key `mode`"), "{mode}");
    assert!(keys.contains("unknown key `keys`"), "{keys}");
}

#[test]
fn unknown_key_is_reported_with_position_and_hint() {
    let msg = &errors("[main]\ndesktop = \"kde\"\nadaptr = \"kde\"\n")[0];

    assert!(msg.contains("unknown key `adaptr`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn removed_layout_is_an_unknown_key() {
    let msg = &errors("layout = \"vertical_list\"\n")[0];

    assert!(msg.contains("unknown key `layout`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn syntax_and_type_errors_stop_at_first() {
    assert_eq!(errors("[taskbar]\ntitlebar.sort_by = \n").len(), 1);
    assert!(
        errors("[main]\ndesktop = \"gnome\"\n")[0].contains("unknown variant")
    );
    assert!(errors("[main]\nwidth = \"wide\"\n")[0].contains(':'));
}

#[test]
fn value_errors_are_all_reported() {
    let text = "[main]\nfont = \"/nope.ttf\"\n\
                font_size = 120\nwidth = 100\n\
                [output]\nmax_rows = 0\n";
    let all = errors(text);

    assert_eq!(all.len(), 4, "{all:?}");
    assert!(all.iter().any(|m| m.contains("`main.font`")), "{all:?}");
    assert!(
        all.iter().any(|m| m.contains(
            "`main.font_size` = 120 is out of range, expected 8 to 72"
        )),
        "{all:?}"
    );
    assert!(
        all.iter().any(|m| m.contains("`output.max_rows`")),
        "{all:?}"
    );
}

#[test]
fn existing_font_path_is_accepted() {
    let config = parse("[main]\nfont = \"/f.ttf\"\n", &|_| true);

    assert_eq!(config.map(|c| c.main.font), Ok(Font::Path("/f.ttf".into())));
}

#[test]
fn missing_file_is_created_then_read() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("target/tynex_config_test_{}", std::process::id()));
    let path = dir.join("tynex").join("tynex.toml");

    assert_eq!(
        load_or_create(&path),
        Ok(Loaded::Created(Config::default()))
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), DEFAULT_TOML);
    assert_eq!(
        load_or_create(&path),
        Ok(Loaded::Existing(Config::default()))
    );
    fs::remove_dir_all(dir).unwrap();
}
