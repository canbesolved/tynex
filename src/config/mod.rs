// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `tynex.toml`: parsing, defaults and validation.

mod error;
mod file;
mod raw;
mod validate;

use std::path::{Path, PathBuf};

use serde::Deserialize;

pub use crate::config::error::ConfigError;
pub use crate::config::file::{Loaded, load_or_create};
use crate::core::order::SortBy;
use crate::core::switcher::HotkeyAction;
use crate::core::view::Rgb;

/// The default config file, written on first start.
pub const DEFAULT_TOML: &str = include_str!("default.toml");

/// Window source adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Desktop {
    Auto,
    Kde,
    Mock,
}

/// Font for titles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Font {
    Auto, // Resolved with fc-match
    Path(PathBuf),
}

/// The `[main]` table: process and frame settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainConfig {
    pub desktop: Desktop,
    pub font: Font,
    pub font_size: u16,
    pub width: u16,
}

/// The `[hotkeys]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeysConfig {
    pub action: HotkeyAction,
}

/// How installed apps appear in the output bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LauncherMode {
    #[default]
    Off, // No launch rows
    All,     // GUI and terminal apps
    GuiOnly, // Skip Terminal=true
}

/// Installed apps in the output bar, and the extra new-window row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherConfig {
    pub mode: LauncherMode,
    pub new_window: NewWindowConfig,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            mode: LauncherMode::Off,
            new_window: NewWindowConfig::default(),
        }
    }
}

/// The extra row under a running app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewWindowConfig {
    pub enabled: bool,
    pub font_color: Rgb,
    pub text: String,
}

impl Default for NewWindowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            font_color: Rgb::new(0xA8, 0xB3, 0xBD),
            text: "  +  New window".into(),
        }
    }
}

/// The `[input_bar]` table.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InputBarConfig {
    pub launcher: LauncherConfig,
}

/// The `[output_bar]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputBarConfig {
    pub max_rows: u16, // Cap for the output bar and the title bar
}

/// The `[title_bar]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleBarConfig {
    pub enabled: bool,        // Window list before typing
    pub sort_by: SortBy,      // Row order
    pub show_icons: bool,     // Icon before each title
    pub strip_app_name: bool, // Drop the app name from titles
}

/// The whole config file, validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub main: MainConfig,
    pub hotkeys: HotkeysConfig,
    pub input_bar: InputBarConfig,
    pub output_bar: OutputBarConfig,
    pub title_bar: TitleBarConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main: MainConfig {
                desktop: Desktop::Auto,
                font: Font::Auto,
                font_size: 14,
                width: 480,
            },
            hotkeys: HotkeysConfig {
                action: HotkeyAction::Last,
            },
            input_bar: InputBarConfig {
                launcher: LauncherConfig::default(),
            },
            output_bar: OutputBarConfig { max_rows: 19 },
            title_bar: TitleBarConfig {
                enabled: false,
                sort_by: SortBy::Recently,
                show_icons: true,
                strip_app_name: false,
            },
        }
    }
}

/// Parses and validates config `text`.
///
/// Syntax and type errors stop at the first error; value errors are all
/// reported. `font_exists` tells whether a font path exists.
///
/// # Errors
///
/// Returns every problem found, with line and column when known.
pub fn parse(
    text: &str,
    font_exists: &dyn Fn(&Path) -> bool,
) -> Result<Config, Vec<ConfigError>> {
    let raw = toml::from_str(text).map_err(|e| {
        let span = e.span().unwrap_or(0..0);

        vec![ConfigError::at(text, span, message(e.message()))]
    })?;

    validate::validate(text, raw, font_exists)
}

/// Rewords `serde` messages for users and hints at renamed keys.
fn message(raw: &str) -> String {
    match raw.strip_prefix("unknown field") {
        Some(rest) => format!(
            "unknown key{rest}; a key renamed or removed in a release is \
             listed in the release notes"
        ),
        None => raw.to_owned(),
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "new_window_tests.rs"]
mod new_window_tests;
