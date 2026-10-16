// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Built-in sample windows with made-up titles, covering edge cases.

use crate::core::window::{WindowId, WindowInfo};

/// Sample windows, most recently used first: `(app_id, title, icon)`.
const SAMPLES: [(&str, &str, Option<&str>); 10] = [
    (
        "org.kde.konsole",
        "~/projects/demo : bash",
        Some("utilities-terminal"),
    ),
    ("firefox", "Café — Firefox", Some("firefox")),
    (
        "org.kde.dolphin",
        "Документы — Dolphin",
        Some("system-file-manager"),
    ),
    ("com.tencent.WeChat", "微信", Some("wechat")),
    ("firefox", "Release notes — Firefox", Some("firefox")),
    (
        "org.kde.kate",
        "A very long title that does not fit in one row of the popup \
         and ends with a combining accent: cafe\u{301}",
        Some("kate"),
    ),
    (
        "org.kde.okular",
        "manual.pdf\t\u{202E}fdp.exe — Okular",
        Some("okular"),
    ),
    ("xterm", "xterm", None),
    (
        "org.kde.gwenview",
        "photo_001.jpg — Gwenview",
        Some("gwenview"),
    ),
    (
        "org.kde.systemsettings",
        "Shortcuts — System Settings",
        None,
    ),
];

/// Returns the sample windows with ids `mock-01` to `mock-10`.
pub(super) fn windows() -> Vec<WindowInfo> {
    SAMPLES
        .iter()
        .enumerate()
        .map(|(i, &(app_id, title, icon))| WindowInfo {
            id: WindowId(format!("mock-{:02}", i + 1)),
            app_id: app_id.into(),
            title: title.into(),
            icon: icon.map(Into::into),
        })
        .collect()
}
