// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Value checks on a parsed file. Every invalid key is reported.

use std::ops::RangeInclusive;
use std::path::Path;

use toml::Spanned;

use crate::config::error::ConfigError;
use crate::config::output::check_output;
use crate::config::raw::{
    RawConfig, RawHint, RawIconBar, RawLauncher, RawLauncherMode, RawSortBy,
    RawTaskbarMode,
};
use crate::config::{
    Config, HintConfig, IconBarConfig, InputConfig, LauncherConfig,
    LauncherMode, NewWindowConfig, TaskbarConfig, TaskbarMode, TitleBarConfig,
};
use crate::core::order::SortBy;
use crate::core::view::Rgb;

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
    let main = crate::config::main::check(&raw.main, font_exists, &mut err);
    check_icon(&raw.taskbar.iconbar, &mut err);
    let pinned = crate::config::taskbar::check_pinned(
        raw.taskbar.iconbar.pinned.as_ref(),
        &mut err,
    );
    let browser_tabs =
        crate::config::browser_tabs::check(raw.browser_tabs, &mut err);
    let new_window = check_new_window(&raw.input.launcher, &mut err);
    let hint = check_hint(&raw.input.hint, &mut err);
    let hotkeys = crate::config::hotkeys::check(&raw.hotkeys, &mut err);
    let output = check_output(&raw.output, &mut err);

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(Config {
        main,
        hotkeys,
        input: InputConfig {
            launcher: LauncherConfig {
                mode: match raw.input.launcher.mode {
                    RawLauncherMode::Off => LauncherMode::Off,
                    RawLauncherMode::All => LauncherMode::All,
                    RawLauncherMode::GuiOnly => LauncherMode::GuiOnly,
                },
                new_window,
            },
            hint,
        },
        output,
        taskbar: TaskbarConfig {
            mode: taskbar_mode(&raw.taskbar.mode),
            titlebar: TitleBarConfig {
                sort_by: match raw.taskbar.titlebar.sort_by {
                    RawSortBy::App => SortBy::App,
                    RawSortBy::Recently => SortBy::Recently,
                },
                show_icons: raw.taskbar.titlebar.show_icons,
                strip_app_name: raw.taskbar.titlebar.strip_app_name,
                select_on_hover: raw.taskbar.titlebar.select_on_hover,
            },
            iconbar: IconBarConfig {
                size: raw.taskbar.iconbar.size.into_inner(),
                border: raw.taskbar.iconbar.border.into_inner(),
                pinned,
                select_on_hover: raw.taskbar.iconbar.select_on_hover,
            },
        },
        browser_tabs,
    })
}

/// Maps the file's mode onto `TaskbarMode`.
fn taskbar_mode(mode: &RawTaskbarMode) -> TaskbarMode {
    match mode {
        RawTaskbarMode::Off => TaskbarMode::Off,
        RawTaskbarMode::Iconbar => TaskbarMode::Iconbar,
        RawTaskbarMode::Titlebar => TaskbarMode::Titlebar,
        RawTaskbarMode::Both => TaskbarMode::Both,
    }
}

/// Checks the new-window row. A bad color or an empty title is an
/// error; the returned value is then unused.
fn check_new_window(
    raw: &RawLauncher,
    err: &mut impl FnMut(std::ops::Range<usize>, String),
) -> NewWindowConfig {
    let color = raw.new_window_text_color.get_ref();
    let text_color = Rgb::parse(color).unwrap_or_else(|| {
        err(
            raw.new_window_text_color.span(),
            format!(
                "`input.launcher.new_window_text_color` = {color:?} \
                 is not a color, expected \"#RRGGBB\""
            ),
        );
        NewWindowConfig::default().text_color
    });
    let text = raw.new_window_text.get_ref();

    if text.is_empty() {
        err(
            raw.new_window_text.span(),
            "`input.launcher.new_window_text` is empty, expected a title"
                .into(),
        );
    }

    NewWindowConfig {
        enabled: raw.new_window,
        text_color,
        text: text.clone(),
    }
}

/// Checks the hint. A bad color or an out-of-range wait is an error;
/// the returned value is then unused. An empty command is allowed.
fn check_hint(
    raw: &RawHint,
    err: &mut impl FnMut(std::ops::Range<usize>, String),
) -> HintConfig {
    let color = raw.font_color.get_ref();
    let font_color = Rgb::parse(color);

    if font_color.is_none() && !color.is_empty() {
        err(
            raw.font_color.span(),
            format!(
                "`input.hint.font_color` = {color:?} is not a color, \
                 expected \"\" or \"#RRGGBB\""
            ),
        );
    }
    let update = *raw.update.get_ref();

    if update > 86_400 {
        err(
            raw.update.span(),
            format!(
                "`input.hint.update` = {update} is out of range, \
                 expected 0 to 86400"
            ),
        );
    }

    HintConfig {
        enabled: raw.enabled,
        font_color,
        text: raw.text.get_ref().clone(),
        update,
    }
}

/// Checks the icon size and the selection border.
fn check_icon(
    raw: &RawIconBar,
    err: &mut impl FnMut(std::ops::Range<usize>, String),
) {
    check_range("taskbar.iconbar.size", &raw.size, 8..=128, err);
    check_range("taskbar.iconbar.border", &raw.border, 1..=32, err);
}

/// Reports `<name>` when its value is outside `range`.
pub(super) fn check_range(
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
