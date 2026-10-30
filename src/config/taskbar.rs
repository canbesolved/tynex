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
    Icons,
    /// The open windows.
    Titles,
    /// The icon strip, then the title list.
    Both,
}

impl TaskbarMode {
    /// Returns true when this mode includes the icon strip.
    #[must_use]
    pub const fn icons(self) -> bool {
        matches!(self, Self::Icons | Self::Both)
    }

    /// Returns true when this mode includes the title list.
    #[must_use]
    pub const fn titles(self) -> bool {
        matches!(self, Self::Titles | Self::Both)
    }
}

/// The `[title_bar]` table: row order and title text.
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

/// The `[icon_bar]` table: icon size and the selection border.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconBarConfig {
    pub icon_size: u16, // Logical pixels, scaled with the display
    pub border: u16,    // Selection outline, logical pixels
}

impl Default for IconBarConfig {
    fn default() -> Self {
        Self {
            icon_size: 24,
            border: 4,
        }
    }
}

/// The taskbar: which parts show, plus their settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskbarConfig {
    pub mode: TaskbarMode,
    pub titles: TitleBarConfig, // `[title_bar]` keys
    pub icons: IconBarConfig,   // `[icon_bar]` keys
}

impl Default for TaskbarConfig {
    fn default() -> Self {
        Self {
            mode: TaskbarMode::Icons,
            titles: TitleBarConfig::default(),
            icons: IconBarConfig::default(),
        }
    }
}
