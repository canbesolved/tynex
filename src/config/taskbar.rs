// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Taskbar mode, and the title list and icon strip settings.

use std::ops::Range;

use toml::Spanned;

use crate::core::order::SortBy;

/// What the taskbar shows while the query is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskbarMode {
    /// The input bar only, until a query is typed.
    Off,
    /// One icon per running app.
    #[default]
    Iconbar,
    /// The open windows.
    Titlebar,
    /// The icon strip, then the title list.
    Both,
}

impl TaskbarMode {
    /// Returns true when this mode includes the icon strip.
    #[must_use]
    pub const fn iconbar(self) -> bool {
        matches!(self, Self::Iconbar | Self::Both)
    }

    /// Returns true when this mode includes the title list.
    #[must_use]
    pub const fn titlebar(self) -> bool {
        matches!(self, Self::Titlebar | Self::Both)
    }
}

/// The `taskbar.titlebar` keys: row order and title text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleBarConfig {
    pub sort_by: SortBy,      // Row order
    pub show_icons: bool,     // Icon before each title
    pub strip_app_name: bool, // Drop the app name from titles
}

impl Default for TitleBarConfig {
    fn default() -> Self {
        Self {
            sort_by: SortBy::Recently,
            show_icons: true,
            strip_app_name: false,
        }
    }
}

/// The `taskbar.iconbar` keys: icon size, the selection border and
/// the pinned apps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconBarConfig {
    pub size: u16,           // Logical pixels, scaled with the display
    pub border: u16,         // Selection outline, logical pixels
    pub pinned: Vec<String>, // Desktop file ids, in strip order
}

impl Default for IconBarConfig {
    fn default() -> Self {
        Self {
            size: 24,
            border: 4,
            pinned: Vec::new(),
        }
    }
}

/// The taskbar: which parts show, plus their settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskbarConfig {
    pub mode: TaskbarMode,
    pub titlebar: TitleBarConfig, // `taskbar.titlebar` keys
    pub iconbar: IconBarConfig,   // `taskbar.iconbar` keys
}

impl Default for TaskbarConfig {
    fn default() -> Self {
        Self {
            mode: TaskbarMode::Iconbar,
            titlebar: TitleBarConfig::default(),
            iconbar: IconBarConfig::default(),
        }
    }
}

/// Checks `taskbar.iconbar.pinned`: desktop file ids, none empty or
/// twice.
pub(super) fn check_pinned(
    value: Option<&Spanned<Vec<String>>>,
    err: &mut impl FnMut(Range<usize>, String),
) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };
    let ids = value.get_ref();

    for (i, id) in ids.iter().enumerate() {
        if id.trim().is_empty() {
            err(
                value.span(),
                "`taskbar.iconbar.pinned` has an empty app id".into(),
            );
        } else if ids[..i].contains(id) {
            err(
                value.span(),
                format!("`taskbar.iconbar.pinned` lists {id:?} twice"),
            );
        }
    }
    ids.clone()
}
