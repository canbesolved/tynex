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

/// The `[appearance]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppearanceConfig {
    pub font: Font,
    pub font_size: u16,
    pub width: u16,
    pub max_rows: u16,
    pub title_bar: bool,      // Window list before typing
    pub show_app: bool,       // App name column
    pub strip_app_name: bool, // Drop the app name from titles
}

/// The whole config file, validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub desktop: Desktop,
    pub hotkey_action: HotkeyAction,
    pub sort_by: SortBy,
    pub appearance: AppearanceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            desktop: Desktop::Auto,
            hotkey_action: HotkeyAction::Last,
            sort_by: SortBy::Recently,
            appearance: AppearanceConfig {
                font: Font::Auto,
                font_size: 14,
                width: 480,
                max_rows: 19,
                title_bar: false,
                show_app: false,
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
