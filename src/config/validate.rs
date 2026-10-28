// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Value checks on a parsed file. Every invalid key is reported.

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use toml::Spanned;

use crate::config::error::ConfigError;
use crate::config::raw::{RawConfig, RawHotkeyAction, RawSortBy};
use crate::config::{AppearanceConfig, Config, Font};
use crate::core::order::SortBy;
use crate::core::switcher::HotkeyAction;

/// Checks `raw` from `text`; `font_exists` tells if a font path exists.
pub(super) fn validate(
    text: &str,
    raw: RawConfig,
    font_exists: &dyn Fn(&Path) -> bool,
) -> Result<Config, Vec<ConfigError>> {
    let mut errors = Vec::new();
    let mut err = |span, message| {
        errors.push(ConfigError::at(text, span, message));
    };
    let a = raw.appearance;

    let font = match a.font.get_ref().as_str() {
        "auto" => Font::Auto,
        path if font_exists(Path::new(path)) => Font::Path(PathBuf::from(path)),
        path => {
            err(
                a.font.span(),
                format!(
                    "`appearance.font` = {path:?} is not an existing file, \
                     expected a font file path or \"auto\""
                ),
            );
            Font::Auto
        }
    };

    for (name, value, range) in [
        ("font_size", &a.font_size, 8..=72),
        ("width", &a.width, 200..=2000),
        ("max_rows", &a.max_rows, 1..=50),
    ] {
        check_range(name, value, &range, &mut err);
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(Config {
        desktop: raw.desktop,
        hotkey_action: match raw.hotkey_action {
            RawHotkeyAction::Hide => HotkeyAction::Hide,
            RawHotkeyAction::Last => HotkeyAction::Last,
        },
        sort_by: match raw.sort_by {
            RawSortBy::App => SortBy::App,
            RawSortBy::Recently => SortBy::Recently,
        },
        appearance: AppearanceConfig {
            font,
            font_size: a.font_size.into_inner(),
            width: a.width.into_inner(),
            max_rows: a.max_rows.into_inner(),
            title_bar: a.title_bar,
            show_app: a.show_app,
            strip_app_name: a.strip_app_name,
        },
    })
}

/// Reports `appearance.<name>` when its value is outside `range`.
fn check_range(
    name: &str,
    value: &Spanned<u16>,
    range: &RangeInclusive<u16>,
    err: &mut impl FnMut(std::ops::Range<usize>, String),
) {
    let v = *value.get_ref();

    if !range.contains(&v) {
        err(
            value.span(),
            format!(
                "`appearance.{name}` = {v} is out of range, expected {} to {}",
                range.start(),
                range.end()
            ),
        );
    }
}
