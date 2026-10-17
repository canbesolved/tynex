// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Title format and app name stripping tests.

use super::*;
use crate::core::window::WindowId;

fn window(app_id: &str, app_name: &str, title: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId("w".into()),
        app_id: app_id.into(),
        app_name: app_name.into(),
        title: title.into(),
        icon: None,
    }
}

fn format(text: &str) -> TitleFormat {
    TitleFormat::parse(text, true).expect("valid format")
}

#[test]
fn default_is_the_stripped_title() {
    let firefox = window("firefox", "Firefox", "Café — Firefox");

    assert_eq!(TitleFormat::default(), format("{title}"));
    assert_eq!(TitleFormat::default().apply(&firefox), "Café");
}

#[test]
fn placeholders_and_text() {
    let cursor = window("cursor", "Cursor", "My homelab project - Cursor");

    assert_eq!(
        format("{app} : {title}").apply(&cursor),
        "Cursor : My homelab project"
    );
    assert_eq!(format("[{app}]").apply(&cursor), "[Cursor]");
}

#[test]
fn invalid_formats() {
    let error = |text| TitleFormat::parse(text, true).unwrap_err();

    assert_eq!(error("{name}"), "unknown placeholder `{name}`");
    assert_eq!(error("{title"), "a brace outside a placeholder");
    assert_eq!(error("} {title}"), "a brace outside a placeholder");
    assert_eq!(error("tynex"), "no placeholder");
    assert_eq!(error(""), "no placeholder");
}

#[test]
fn strip_can_be_turned_off() {
    let kate = window("org.kde.kate", "Kate", "notes.md — Kate");
    let keep = TitleFormat::parse("{title}", false).unwrap();

    assert_eq!(keep.apply(&kate), "notes.md — Kate");
}

#[test]
fn strip_matches_app_name_or_short_app_id() {
    let cases = [
        ("org.kde.konsole", "", "~ : bash — Konsole", "~ : bash"),
        (
            "code",
            "Visual Studio Code",
            "a.rs - x - Visual Studio Code",
            "a.rs - x",
        ),
        ("org.kde.dolphin", "Dolphin", "Música – DOLPHIN", "Música"),
        (
            "org.kde.systemsettings",
            "Configuración",
            "Fuentes — CONFIGURACIÓN",
            "Fuentes",
        ),
    ];

    for (app_id, app_name, title, stripped) in cases {
        let w = window(app_id, app_name, title);

        assert_eq!(format("{title}").apply(&w), stripped, "{title}");
    }
}

#[test]
fn strip_keeps_other_suffixes_and_whole_names() {
    let cases = [
        ("firefox", "Firefox", "Meeting - Zoom"),
        ("firefox", "Firefox", "Firefox"),
        ("firefox", "Firefox", "Mozilla Firefox"),
        ("xterm", "", "xterm"),
        ("", "", "a — b"),
    ];

    for (app_id, app_name, title) in cases {
        let w = window(app_id, app_name, title);

        assert_eq!(format("{title}").apply(&w), title);
    }
}

#[test]
fn text_is_sanitized() {
    let w = window("okular", "Okular", "a\t\u{202E}b — Okular");
    let no_name = window("x", "", "Title");

    assert_eq!(format("{title}").apply(&w), "a b");
    assert_eq!(format("{app} {title}").apply(&no_name), "Title");
    assert_eq!(format("\t{title}\n").apply(&no_name), "Title");
}
