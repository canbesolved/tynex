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

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawConfig {
    pub desktop: Desktop,
    pub hotkey_action: RawHotkeyAction,
    pub sort_by: RawSortBy,
    pub appearance: RawAppearance,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            desktop: Desktop::Auto,
            hotkey_action: RawHotkeyAction::Last,
            sort_by: RawSortBy::Recently,
            appearance: RawAppearance::default(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawAppearance {
    pub font: Spanned<String>,
    pub font_size: Spanned<u16>,
    pub width: Spanned<u16>,
    pub max_rows: Spanned<u16>,
    pub title_bar: bool,
    pub show_app: bool,
    pub strip_app_name: bool,
}

impl Default for RawAppearance {
    fn default() -> Self {
        Self {
            font: unspanned("auto".into()),
            font_size: unspanned(14),
            width: unspanned(480),
            max_rows: unspanned(19),
            title_bar: false,
            show_app: false,
            strip_app_name: false,
        }
    }
}
