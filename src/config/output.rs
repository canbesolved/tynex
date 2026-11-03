// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The `[output]` table: row cap and row colors.

use std::ops::Range;

use serde::Deserialize;
use toml::Spanned;

use crate::core::row_color::{RowColors, Tint};

/// The `[output]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputConfig {
    pub max_rows: u16, // Cap for the output bar and the title list
    pub colors: RowColors, // Row background per result type
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            max_rows: 19,
            colors: RowColors::default(),
        }
    }
}

/// `output.colors` as written: `""` or a color per result type.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
pub(super) struct RawColors {
    pub window: Option<Spanned<String>>,
    pub tab: Option<Spanned<String>>,
    pub app: Option<Spanned<String>>,
}

/// Checks the row colors. A bad value is an error; the returned value
/// is then unused.
pub(super) fn check_colors(
    raw: &RawColors,
    err: &mut impl FnMut(Range<usize>, String),
) -> RowColors {
    let mut one = |key: &str, value: Option<&Spanned<String>>| {
        let value = value?;
        let text = value.get_ref();

        if text.is_empty() {
            return None;
        }
        let tint = Tint::parse(text);

        if tint.is_none() {
            err(
                value.span(),
                format!(
                    "`output.colors.{key}` = {text:?} is not a color, \
                     expected \"\", \"#RRGGBB\" or \"#RRGGBBAA\""
                ),
            );
        }
        tint
    };

    RowColors {
        window: one("window", raw.window.as_ref()),
        tab: one("tab", raw.tab.as_ref()),
        app: one("app", raw.app.as_ref()),
    }
}

#[cfg(test)]
#[path = "output_tests.rs"]
mod tests;
