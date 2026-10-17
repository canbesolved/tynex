// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Built-in sample windows with made-up titles, covering edge cases.

use crate::core::window::{WindowId, WindowInfo};

/// Sample windows, most recently used first:
/// `(app_id, app_name, title, icon)`.
const SAMPLES: [(&str, &str, &str, Option<&str>); 10] = [
    (
        "org.kde.konsole",
        "Konsole",
        "~/projects/demo : bash",
        Some("utilities-terminal"),
    ),
    ("firefox", "Firefox", "Café — Firefox", Some("firefox")),
    (
        "org.kde.dolphin",
        "Dolphin",
        "Música — Dolphin",
        Some("system-file-manager"),
    ),
    ("com.tencent.WeChat", "WeChat", "微信", Some("wechat")),
    (
        "firefox",
        "Firefox",
        "Release notes — Firefox",
        Some("firefox"),
    ),
    (
        "org.kde.kate",
        "Kate",
        "A very long title that does not fit in one row of the popup \
         and ends with a combining accent: cafe\u{301}",
        Some("kate"),
    ),
    (
        "org.kde.okular",
        "Okular",
        "manual.pdf\t\u{202E}fdp.exe — Okular",
        Some("okular"),
    ),
    ("xterm", "xterm", "xterm", None),
    (
        "org.kde.gwenview",
        "Gwenview",
        "photo_001.jpg — Gwenview",
        Some("gwenview"),
    ),
    (
        "org.kde.systemsettings",
        "System Settings",
        "Shortcuts — System Settings",
        None,
    ),
];

/// Returns the sample windows with ids `mock-01` to `mock-10`.
pub(super) fn windows() -> Vec<WindowInfo> {
    SAMPLES
        .iter()
        .enumerate()
        .map(|(i, &(app_id, app_name, title, icon))| WindowInfo {
            id: WindowId(format!("mock-{:02}", i + 1)),
            app_id: app_id.into(),
            app_name: app_name.into(),
            title: title.into(),
            icon: icon.map(Into::into),
        })
        .collect()
}
