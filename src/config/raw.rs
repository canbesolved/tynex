// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! File layout as parsed by `serde`, with spans for value checks.

use serde::Deserialize;
use toml::Spanned;

use crate::config::Desktop;

/// Values not from the file get an empty span.
fn unspanned<T>(value: T) -> Spanned<T> {
    Spanned::new(0..0, value)
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RawHotkeyAction {
    Hide,
    Last,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RawSortBy {
    App,
    Recently,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub(super) enum RawLauncherMode {
    #[default]
    Off,
    All,
    GuiOnly,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawConfig {
    pub main: RawMain,
    pub hotkeys: RawHotkeys,
    pub input_bar: RawInputBar,
    pub output_bar: RawOutputBar,
    pub title_bar: RawTitleBar,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawMain {
    pub desktop: Desktop,
    pub font: Spanned<String>,
    pub font_size: Spanned<u16>,
    pub width: Spanned<u16>,
}

impl Default for RawMain {
    fn default() -> Self {
        Self {
            desktop: Desktop::Auto,
            font: unspanned("auto".into()),
            font_size: unspanned(14),
            width: unspanned(480),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawHotkeys {
    pub action: RawHotkeyAction,
}

impl Default for RawHotkeys {
    fn default() -> Self {
        Self {
            action: RawHotkeyAction::Last,
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawInputBar {
    pub launcher: RawLauncher,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawLauncher {
    pub mode: RawLauncherMode,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawOutputBar {
    pub max_rows: Spanned<u16>,
}

impl Default for RawOutputBar {
    fn default() -> Self {
        Self {
            max_rows: unspanned(19),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawTitleBar {
    pub enabled: bool,
    pub sort_by: RawSortBy,
    pub show_icons: bool,
    pub strip_app_name: bool,
}

impl Default for RawTitleBar {
    fn default() -> Self {
        Self {
            enabled: false,
            sort_by: RawSortBy::Recently,
            show_icons: true,
            strip_app_name: false,
        }
    }
}
