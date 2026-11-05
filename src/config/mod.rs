// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `tynex.toml`: parsing, defaults and validation.

mod browser_tabs;
mod error;
mod file;
mod hotkeys;
mod main;
mod output;
mod raw;
mod taskbar;
mod theme;
mod validate;

use std::path::Path;

pub use crate::config::browser_tabs::{Browser, BrowserTabsConfig, Browsers};
pub use crate::config::error::ConfigError;
pub use crate::config::file::{Loaded, load_or_create};
pub use crate::config::hotkeys::HotkeysConfig;
pub use crate::config::main::{Desktop, Font, MainConfig, Position};
pub use crate::config::output::OutputConfig;
pub use crate::config::taskbar::{
    IconBarConfig, TaskbarConfig, TaskbarMode, TitleBarConfig,
};
pub use crate::config::theme::Theme;
use crate::core::switcher::HotkeyAction;
use crate::core::view::Rgb;

/// The default config file, written on first start.
pub const DEFAULT_TOML: &str = include_str!("default.toml");

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

/// Default color of that title, `#797979`: a gray that reads the same
/// on the dark and light themes.
pub(crate) const NEW_WINDOW_COLOR: Rgb = Rgb::new(0x79, 0x79, 0x79);

/// Default hint command: the script the daemon writes on start. Its
/// stdout is the date, time and battery level.
pub(crate) const HINT_TEXT: &str = "~/.local/share/tynex/hints/default.sh";

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
    pub font_color: Option<Rgb>, // None follows the theme
    pub text: String,
    pub update: u32, // Seconds after exit; 0 runs once per open
}

impl Default for HintConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            font_color: None,
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

/// The whole config file, validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub main: MainConfig,
    pub hotkeys: HotkeysConfig,
    pub input: InputConfig,
    pub output: OutputConfig,
    pub taskbar: TaskbarConfig,
    pub browser_tabs: BrowserTabsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            main: MainConfig::default(),
            hotkeys: HotkeysConfig {
                action: HotkeyAction::Last,
                rename: hotkeys::default_rename(),
                pinned: hotkeys::default_pinned(),
                select: hotkeys::default_select(),
                space: hotkeys::default_space(),
            },
            input: InputConfig::default(),
            output: OutputConfig::default(),
            taskbar: TaskbarConfig::default(),
            browser_tabs: BrowserTabsConfig::default(),
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

        vec![ConfigError::at(text, span, error::message(e.message()))]
    })?;

    validate::validate(text, raw, font_exists)
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

#[cfg(test)]
#[path = "browser_tabs_tests.rs"]
mod browser_tabs_tests;

#[cfg(test)]
#[path = "hotkeys_tests.rs"]
mod hotkeys_tests;

#[cfg(test)]
#[path = "input_keys_tests.rs"]
mod input_keys_tests;

#[cfg(test)]
#[path = "history_tests.rs"]
mod history_tests;
