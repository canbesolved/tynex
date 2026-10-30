// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `tynex.toml`: parsing, defaults and validation.

mod error;
mod file;
mod raw;
mod taskbar;
mod theme;
mod validate;

use std::path::{Path, PathBuf};

use serde::Deserialize;

pub use crate::config::error::ConfigError;
pub use crate::config::file::{Loaded, load_or_create};
pub use crate::config::taskbar::{
    IconBarConfig, TaskbarConfig, TaskbarMode, TitleBarConfig,
};
pub use crate::config::theme::Theme;
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

/// Vertical position of the popup on the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    #[default]
    Center, // Centered when it opens; grows downward
    Top, // 24 logical pixels from the top
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
    pub position: Position,
    pub theme: Theme,
}

/// The `[hotkeys]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeysConfig {
    pub action: HotkeyAction,
}

/// How installed apps appear in the output bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LauncherMode {
    Off, // No launch rows
    All, // GUI and terminal apps
    #[default]
    GuiOnly, // Skip Terminal=true
}

/// Default title of the extra row under a running app.
pub(crate) const NEW_WINDOW_TEXT: &str = " +  New window";

/// Default color of that title, `#8da787`.
pub(crate) const NEW_WINDOW_COLOR: Rgb = Rgb::new(0x8d, 0xa7, 0x87);

/// Default hint command. Its stdout is the date and time.
pub(crate) const HINT_TEXT: &str = "date +\"%A %b %-d %T %Y\"";

/// Default hint color, `#8da787`.
pub(crate) const HINT_COLOR: Rgb = Rgb::new(0x8d, 0xa7, 0x87);

/// Installed apps in the output bar, and the extra new-window row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherConfig {
    pub mode: LauncherMode,
    pub new_window: NewWindowConfig,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            mode: LauncherMode::GuiOnly,
            new_window: NewWindowConfig::default(),
        }
    }
}

/// The extra row under a running app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewWindowConfig {
    pub enabled: bool,
    pub text_color: Rgb,
    pub text: String,
}

impl Default for NewWindowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            text_color: NEW_WINDOW_COLOR,
            text: NEW_WINDOW_TEXT.into(),
        }
    }
}

/// Hint text in the empty input bar. `text` is a shell command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HintConfig {
    pub enabled: bool,
    pub font_color: Rgb,
    pub text: String,
    pub update: u32, // Seconds after exit; 0 runs once per open
}

impl Default for HintConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            font_color: HINT_COLOR,
            text: HINT_TEXT.into(),
            update: 0,
        }
    }
}

/// The `[input]` table.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InputConfig {
    pub launcher: LauncherConfig,
    pub hint: HintConfig,
}

/// The `[output]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputConfig {
    pub max_rows: u16, // Cap for the output bar and the title list
}

/// The whole config file, validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub main: MainConfig,
    pub hotkeys: HotkeysConfig,
    pub input: InputConfig,
    pub output: OutputConfig,
    pub taskbar: TaskbarConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main: MainConfig {
                desktop: Desktop::Auto,
                font: Font::Auto,
                font_size: 14,
                width: 480,
                position: Position::Center,
                theme: Theme::Dark,
            },
            hotkeys: HotkeysConfig {
                action: HotkeyAction::Last,
            },
            input: InputConfig::default(),
            output: OutputConfig { max_rows: 19 },
            taskbar: TaskbarConfig::default(),
        }
    }
}

/// Parses and validates config `text`. Syntax and type errors stop at
/// the first error; value errors are all reported. `font_exists` tells
/// whether a font path exists.
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

#[cfg(test)]
#[path = "hint_tests.rs"]
mod hint_tests;

#[cfg(test)]
#[path = "icon_tests.rs"]
mod icon_tests;

#[cfg(test)]
#[path = "taskbar_tests.rs"]
mod taskbar_tests;

#[cfg(test)]
#[path = "sections_tests.rs"]
mod sections_tests;

#[cfg(test)]
#[path = "position_tests.rs"]
mod position_tests;
