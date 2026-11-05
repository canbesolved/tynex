// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The `[main]` table: process and frame settings.

use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::config::raw::RawMain;
use crate::config::theme::Theme;
use crate::config::validate::check_range;

/// Default `main.history`: windows and tabs kept for the title list.
pub(crate) const HISTORY: u16 = 10;

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
    pub switch_layout: bool, // Read keys with the US layout in the popup
    pub history: u16,        // Windows and tabs kept for the title list
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            desktop: Desktop::Auto,
            font: Font::Auto,
            font_size: 14,
            width: 480,
            position: Position::Center,
            theme: Theme::Dark,
            switch_layout: false,
            history: HISTORY,
        }
    }
}

/// Checks `[main]`. A bad value is an error; the returned value is
/// then unused. `font_exists` tells whether a font path exists.
pub(super) fn check(
    main: &RawMain,
    font_exists: &dyn Fn(&Path) -> bool,
    err: &mut impl FnMut(Range<usize>, String),
) -> MainConfig {
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

    check_range("main.font_size", &main.font_size, 8..=72, err);
    check_range("main.width", &main.width, 200..=2000, err);
    check_range("main.history", &main.history, 2..=50, err);
    MainConfig {
        desktop: main.desktop,
        font,
        font_size: *main.font_size.get_ref(),
        width: *main.width.get_ref(),
        position: main.position,
        theme: main.theme,
        switch_layout: main.switch_layout,
        history: *main.history.get_ref(),
    }
}
