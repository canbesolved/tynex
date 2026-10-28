// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Value checks on a parsed file. Every invalid key is reported.

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use toml::Spanned;

use crate::config::error::ConfigError;
use crate::config::raw::{RawConfig, RawHotkeyAction, RawSortBy};
use crate::config::{
    Config, Font, HotkeysConfig, InputBarConfig, MainConfig, OutputBarConfig,
    TitleBarConfig,
};
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
    let main = raw.main;

    let font = match main.font.get_ref().as_str() {
        "auto" => Font::Auto,
        path if font_exists(Path::new(path)) => Font::Path(PathBuf::from(path)),
        path => {
            err(
                main.font.span(),
                format!(
                    "`main.font` = {path:?} is not an existing file, \
                     expected a font file path or \"auto\""
                ),
            );
            Font::Auto
        }
    };

    check_range("main.font_size", &main.font_size, 8..=72, &mut err);
    check_range("main.width", &main.width, 200..=2000, &mut err);
    check_range(
        "output_bar.max_rows",
        &raw.output_bar.max_rows,
        1..=50,
        &mut err,
    );

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(Config {
        main: MainConfig {
            desktop: main.desktop,
            font,
            font_size: main.font_size.into_inner(),
            width: main.width.into_inner(),
        },
        hotkeys: HotkeysConfig {
            action: match raw.hotkeys.action {
                RawHotkeyAction::Hide => HotkeyAction::Hide,
                RawHotkeyAction::Last => HotkeyAction::Last,
            },
        },
        input_bar: InputBarConfig {},
        output_bar: OutputBarConfig {
            max_rows: raw.output_bar.max_rows.into_inner(),
        },
        title_bar: TitleBarConfig {
            enabled: raw.title_bar.enabled,
            sort_by: match raw.title_bar.sort_by {
                RawSortBy::App => SortBy::App,
                RawSortBy::Recently => SortBy::Recently,
            },
            show_app_icon: raw.title_bar.show_app_icon,
            strip_app_name: raw.title_bar.strip_app_name,
        },
    })
}

/// Reports `<name>` when its value is outside `range`.
fn check_range(
    name: &str,
    value: &Spanned<u16>,
    range: RangeInclusive<u16>,
    err: &mut impl FnMut(std::ops::Range<usize>, String),
) {
    let v = *value.get_ref();

    if !range.contains(&v) {
        err(
            value.span(),
            format!(
                "`{name}` = {v} is out of range, expected {} to {}",
                range.start(),
                range.end()
            ),
        );
    }
}
