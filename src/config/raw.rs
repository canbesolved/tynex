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
    Off,
    All,
    #[default]
    GuiOnly,
}

fn default_new_window() -> bool {
    true
}

fn default_new_window_text() -> Spanned<String> {
    unspanned(super::NEW_WINDOW_TEXT.into())
}

fn default_new_window_color() -> Spanned<String> {
    unspanned("#8da787".into())
}

fn default_hint_enabled() -> bool {
    true
}

fn default_hint_color() -> Spanned<String> {
    unspanned("#8da787".into())
}

fn default_hint_text() -> Spanned<String> {
    unspanned(super::HINT_TEXT.into())
}

fn default_hint_update() -> Spanned<u32> {
    unspanned(0)
}

fn default_icon_enabled() -> bool {
    true
}

fn default_icon_size() -> Spanned<u16> {
    unspanned(24)
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawConfig {
    pub main: RawMain,
    pub hotkeys: RawHotkeys,
    pub input_bar: RawInputBar,
    pub output_bar: RawOutputBar,
    pub title_bar: RawTitleBar,
    pub icon_bar: RawIconBar,
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
    pub hint: RawHint,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawHint {
    #[serde(default = "default_hint_enabled")]
    pub enabled: bool,
    #[serde(default = "default_hint_color")]
    pub font_color: Spanned<String>,
    #[serde(default = "default_hint_text")]
    pub text: Spanned<String>,
    #[serde(default = "default_hint_update")]
    pub update: Spanned<u32>,
}

impl Default for RawHint {
    fn default() -> Self {
        Self {
            enabled: default_hint_enabled(),
            font_color: default_hint_color(),
            text: default_hint_text(),
            update: default_hint_update(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawLauncher {
    pub mode: RawLauncherMode,
    #[serde(default = "default_new_window")]
    pub new_window: bool,
    #[serde(default = "default_new_window_text")]
    pub new_window_text: Spanned<String>,
    #[serde(default = "default_new_window_color")]
    pub new_window_text_color: Spanned<String>,
}

impl Default for RawLauncher {
    fn default() -> Self {
        Self {
            mode: RawLauncherMode::GuiOnly,
            new_window: default_new_window(),
            new_window_text: default_new_window_text(),
            new_window_text_color: default_new_window_color(),
        }
    }
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawIconBar {
    #[serde(default = "default_icon_enabled")]
    pub enabled: bool,
    #[serde(default = "default_icon_size")]
    pub icon_size: Spanned<u16>,
}

impl Default for RawIconBar {
    fn default() -> Self {
        Self {
            enabled: default_icon_enabled(),
            icon_size: default_icon_size(),
        }
    }
}
