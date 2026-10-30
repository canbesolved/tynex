// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Value checks on a parsed file. Every invalid key is reported.

use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use toml::Spanned;

use crate::config::error::ConfigError;
use crate::config::raw::{
    RawConfig, RawHint, RawHotkeyAction, RawIconBar, RawLauncher,
    RawLauncherMode, RawSortBy, RawTaskbarMode,
};
use crate::config::{
    Config, Font, HintConfig, HotkeysConfig, IconBarConfig, InputBarConfig,
    LauncherConfig, LauncherMode, MainConfig, NewWindowConfig, OutputBarConfig,
    TaskbarConfig, TaskbarMode, TitleBarConfig,
};
use crate::core::order::SortBy;
use crate::core::switcher::HotkeyAction;
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
    check_icon(&raw.icon_bar, &mut err);
    let new_window = check_new_window(&raw.input_bar.launcher, &mut err);
    let hint = check_hint(&raw.input_bar.hint, &mut err);

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
        input_bar: InputBarConfig {
            launcher: LauncherConfig {
                mode: match raw.input_bar.launcher.mode {
                    RawLauncherMode::Off => LauncherMode::Off,
                    RawLauncherMode::All => LauncherMode::All,
                    RawLauncherMode::GuiOnly => LauncherMode::GuiOnly,
                },
                new_window,
            },
            hint,
        },
        output_bar: OutputBarConfig {
            max_rows: raw.output_bar.max_rows.into_inner(),
        },
        taskbar: TaskbarConfig {
            mode: taskbar_mode(&raw.taskbar.mode),
            titles: TitleBarConfig {
                sort_by: match raw.title_bar.sort_by {
                    RawSortBy::App => SortBy::App,
                    RawSortBy::Recently => SortBy::Recently,
                },
                show_icons: raw.title_bar.show_icons,
                strip_app_name: raw.title_bar.strip_app_name,
            },
            icons: IconBarConfig {
                icon_size: raw.icon_bar.icon_size.into_inner(),
                border: raw.icon_bar.border.into_inner(),
            },
        },
    })
}

/// Maps the file's mode onto `TaskbarMode`.
fn taskbar_mode(mode: &RawTaskbarMode) -> TaskbarMode {
    match mode {
        RawTaskbarMode::Off => TaskbarMode::Off,
        RawTaskbarMode::Icons => TaskbarMode::Icons,
        RawTaskbarMode::Titles => TaskbarMode::Titles,
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
                "`input_bar.launcher.new_window_text_color` = {color:?} \
                 is not a color, expected \"#RRGGBB\""
            ),
        );
        NewWindowConfig::default().text_color
    });
    let text = raw.new_window_text.get_ref();

    if text.is_empty() {
        err(
            raw.new_window_text.span(),
            "`input_bar.launcher.new_window_text` is empty, expected \
             a title"
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
    let font_color = Rgb::parse(color).unwrap_or_else(|| {
        err(
            raw.font_color.span(),
            format!(
                "`input_bar.hint.font_color` = {color:?} is not a color, \
                 expected \"#RRGGBB\""
            ),
        );
        HintConfig::default().font_color
    });
    let update = *raw.update.get_ref();

    if update > 86_400 {
        err(
            raw.update.span(),
            format!(
                "`input_bar.hint.update` = {update} is out of range, \
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
    check_range("icon_bar.icon_size", &raw.icon_size, 8..=128, err);
    check_range("icon_bar.border", &raw.border, 1..=32, err);
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
