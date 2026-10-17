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
        parse("mode = \"search\"\n[appearance]\nwidth = 600\n", &no_font)
            .expect("valid config");

    assert_eq!(config.mode, ModeKind::Search);
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

    assert_eq!(sort(""), Ok(SortBy::Key));
    assert_eq!(sort("sort_by = \"app\"\n"), Ok(SortBy::App));
    assert_eq!(sort("sort_by = \"recently\"\n"), Ok(SortBy::Recently));
    assert!(errors("sort_by = \"name\"\n")[0].contains("unknown variant"));
}

#[test]
fn title_format_values() {
    let text = "[appearance]\ntitle_format = \"{app} : {title}\"\n\
                strip_app_name = false\n";
    let expected = TitleFormat::parse("{app} : {title}", false).unwrap();
    let msg = &errors("[appearance]\ntitle_format = \"{name}\"\n")[0];

    assert_eq!(
        parse(text, &no_font).map(|c| c.appearance.title),
        Ok(expected)
    );
    assert!(
        msg.starts_with(
            "2:16: `appearance.title_format` = \"{name}\" is invalid: \
             unknown placeholder `{name}`"
        ),
        "{msg}"
    );
}

#[test]
fn unknown_key_is_reported_with_position_and_hint() {
    let msg = &errors("mode = \"hotkey\"\nadaptr = \"kde\"\n")[0];

    assert!(msg.starts_with("2:1: unknown key `adaptr`"), "{msg}");
    assert!(msg.contains("release notes"), "{msg}");
}

#[test]
fn syntax_and_type_errors_stop_at_first() {
    assert_eq!(errors("keys = \n").len(), 1);
    assert!(errors("desktop = \"gnome\"\n")[0].contains("unknown variant"));
    assert!(errors("[appearance]\nwidth = \"wide\"\n")[0].starts_with("2:9:"));
}

#[test]
fn value_errors_are_all_reported() {
    let text = "keys = \"aa\"\n[appearance]\nfont = \"/nope.ttf\"\n\
                font_size = 120\nwidth = 100\nmax_rows = 0\n";
    let all = errors(text);

    assert_eq!(all.len(), 5, "{all:?}");
    assert!(all[0].starts_with("1:8: `keys`"));
    assert!(all[2].starts_with(
        "4:13: `appearance.font_size` = 120 is out of range, expected 8 to 72"
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
