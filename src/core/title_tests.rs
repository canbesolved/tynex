// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row title and app name stripping tests.

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

#[test]
fn stripped_by_default_rule() {
    let firefox = window("firefox", "Firefox", "Café — Firefox");

    assert_eq!(row_title(&firefox, true), "Café");
}

#[test]
fn strip_can_be_turned_off() {
    let kate = window("org.kde.kate", "Kate", "notes.md — Kate");

    assert_eq!(row_title(&kate, false), "notes.md — Kate");
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

        assert_eq!(row_title(&w, true), stripped, "{title}");
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

        assert_eq!(row_title(&w, true), title);
    }
}

#[test]
fn text_is_sanitized() {
    let w = window("okular", "Okular", "a\t\u{202E}b — Okular");

    assert_eq!(row_title(&w, true), "a b");
    assert_eq!(row_title(&w, false), "a b — Okular");
}
