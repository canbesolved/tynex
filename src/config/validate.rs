// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Value checks on a parsed file. Every invalid key is reported.

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use toml::Spanned;

use crate::config::error::ConfigError;
use crate::config::raw::{RawConfig, RawHotkeyAction, RawMode, RawSortBy};
use crate::config::{AppearanceConfig, Config, Font};
use crate::core::event::ModeKind;
use crate::core::order::SortBy;
use crate::core::switcher::HotkeyAction;
use crate::core::title::TitleFormat;

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

    if !valid_keys(raw.keys.get_ref()) {
        err(
            raw.keys.span(),
            format!(
                "`keys` = {:?} is invalid, expected 1 to 26 unique \
                 lowercase ASCII letters",
                raw.keys.get_ref()
            ),
        );
    }

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

    let title = TitleFormat::parse(a.title_format.get_ref(), a.strip_app_name)
        .unwrap_or_else(|e| {
            err(
                a.title_format.span(),
                format!(
                    "`appearance.title_format` = {:?} is invalid: {e}, \
                     expected text with `{{app}}` or `{{title}}`",
                    a.title_format.get_ref()
                ),
            );
            TitleFormat::default()
        });

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
        mode: match raw.mode {
            RawMode::Hotkey => ModeKind::Hotkey,
            RawMode::Search => ModeKind::Search,
        },
        hotkey_action: match raw.hotkey_action {
            RawHotkeyAction::Hide => HotkeyAction::Hide,
            RawHotkeyAction::Last => HotkeyAction::Last,
        },
        sort_by: match raw.sort_by {
            RawSortBy::Key => SortBy::Key,
            RawSortBy::App => SortBy::App,
            RawSortBy::Recently => SortBy::Recently,
        },
        layout: raw.layout,
        keys: raw.keys.into_inner(),
        appearance: AppearanceConfig {
            font,
            font_size: a.font_size.into_inner(),
            width: a.width.into_inner(),
            max_rows: a.max_rows.into_inner(),
            title,
        },
    })
}

/// Returns true for 1 to 26 unique lowercase ASCII letters.
fn valid_keys(keys: &str) -> bool {
    let mut seen = 0_u32;

    (1..=26).contains(&keys.len())
        && keys.bytes().all(|b| {
            let bit = b.is_ascii_lowercase().then(|| 1 << (b - b'a'));

            bit.is_some_and(|bit| {
                let new = seen & bit == 0;

                seen |= bit;
                new
            })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_rules() {
        assert!(valid_keys("asdf"));
        assert!(valid_keys("abcdefghijklmnopqrstuvwxyz"));
        assert!(!valid_keys(""));
        assert!(!valid_keys("aa"));
        assert!(!valid_keys("aS"));
        assert!(!valid_keys("a1"));
        assert!(!valid_keys("año"));
    }
}
