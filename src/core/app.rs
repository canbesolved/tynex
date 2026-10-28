// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Installed app as seen by the core. Exec stays on the platform.

/// One launchable app. Pure data for matching and drawing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInfo {
    pub id: String,           // Desktop file id
    pub name: String,         // Localized Name
    pub icon: Option<String>, // Theme name or path
}
