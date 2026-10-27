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
        parse("[appearance]\nwidth = 600\n", &no_font).expect("valid config");

    assert_eq!(config.appearance.width, 600);
    assert_eq!(config.appearance.font_size, 14);
}

#[test]
fn hotkey_action_values() {
    let action = |text| parse(text, &no_font).map(|c| c.hotkey_action);

    assert_eq!(action("hotkey_action = \"last\"\n"), Ok(HotkeyAction::Last));
    assert!(
        errors("hotkey_action = \"toggle\"\n")[0].contains("unknown variant")
    );
}

#[test]
fn sort_by_values() {
    let sort = |text| parse(text, &no_font).map(|c| c.sort_by);

    assert_eq!(sort(""), Ok(SortBy::Recently));
    assert_eq!(sort("sort_by = \"app\"\n"), Ok(SortBy::App));
    assert!(errors("sort_by = \"name\"\n")[0].contains("unknown variant"));
    assert!(errors("sort_by = \"key\"\n")[0].contains("unknown variant"));
}

#[test]
fn title_bar_columns_and_strip_values() {
    let text = "[appearance]\ntitle_bar = true\nshow_app = true\n\
                strip_app_name = true\n";
    let config = parse(text, &no_font).expect("valid config");

    assert!(config.appearance.title_bar);
    assert!(config.appearance.show_app);
    assert!(config.appearance.strip_app_name);
}

#[test]
fn removed_title_format_is_an_unknown_key() {
    let msg = &errors("[appearance]\ntitle_format = \"{title}\"\n")[0];

    assert!(msg.starts_with("2:1: unknown key `title_format`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn removed_mode_and_keys_are_unknown_keys() {
    let mode = &errors("mode = \"hotkey\"\n")[0];
    let keys = &errors("keys = \"asdf\"\n")[0];

    assert!(mode.starts_with("1:1: unknown key `mode`"), "{mode}");
    assert!(keys.starts_with("1:1: unknown key `keys`"), "{keys}");
}

#[test]
fn unknown_key_is_reported_with_position_and_hint() {
    let msg = &errors("desktop = \"kde\"\nadaptr = \"kde\"\n")[0];

    assert!(msg.starts_with("2:1: unknown key `adaptr`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn syntax_and_type_errors_stop_at_first() {
    assert_eq!(errors("layout = \n").len(), 1);
    assert!(errors("desktop = \"gnome\"\n")[0].contains("unknown variant"));
    assert!(errors("[appearance]\nwidth = \"wide\"\n")[0].starts_with("2:9:"));
}

#[test]
fn value_errors_are_all_reported() {
    let text = "[appearance]\nfont = \"/nope.ttf\"\n\
                font_size = 120\nwidth = 100\nmax_rows = 0\n";
    let all = errors(text);

    assert_eq!(all.len(), 4, "{all:?}");
    assert!(all[0].starts_with("2:8: `appearance.font`"));
    assert!(all[1].starts_with(
        "3:13: `appearance.font_size` = 120 is out of range, expected 8 to 72"
    ));
}

#[test]
fn existing_font_path_is_accepted() {
    let config = parse("[appearance]\nfont = \"/f.ttf\"\n", &|_| true);

    assert_eq!(
        config.map(|c| c.appearance.font),
        Ok(Font::Path("/f.ttf".into()))
    );
}

#[test]
fn missing_file_is_created_then_read() {
    let dir = std::env::temp_dir()
        .join(format!("tynex_config_test_{}", std::process::id()));
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
