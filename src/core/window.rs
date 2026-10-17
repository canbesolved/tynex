// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Window identity and metadata, as reported by an adapter.

/// Opaque id given by the adapter (`KWin` uses a UUID string).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowId(pub String);

/// One switchable window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowInfo {
    pub id: WindowId,
    pub app_id: String,       // Desktop file name or WM class
    pub app_name: String,     // Shown name, from a desktop file or WM class
    pub title: String,        // Sanitized, see `text::sanitize_title`
    pub icon: Option<String>, // Icon name or path
}
