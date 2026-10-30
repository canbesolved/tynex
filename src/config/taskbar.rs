// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Taskbar mode, and the title list and icon strip settings.

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

/// The `taskbar.iconbar` keys: icon size and the selection border.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconBarConfig {
    pub size: u16,   // Logical pixels, scaled with the display
    pub border: u16, // Selection outline, logical pixels
}

impl Default for IconBarConfig {
    fn default() -> Self {
        Self {
            size: 24,
            border: 4,
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
