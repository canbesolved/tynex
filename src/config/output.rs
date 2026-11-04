// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The `[output]` table: result cap and row colors.

use std::ops::Range;

use serde::Deserialize;
use toml::Spanned;

use crate::config::raw::RawOutput;
use crate::config::validate::check_range;
use crate::core::row_color::{RowColors, Tint};

/// The `[output]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputConfig {
    pub max_results: u16, // Cap for the output bar and the title list
    pub colors: RowColors, // Row background per result type
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            max_results: 10,
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

/// Checks `[output]`. A bad value is an error; the returned value is
/// then unused.
pub(super) fn check_output(
    raw: &RawOutput,
    err: &mut impl FnMut(Range<usize>, String),
) -> OutputConfig {
    let cap = &raw.max_results;

    check_range("output.max_results", cap, 1..=50, err);
    OutputConfig {
        max_results: *cap.get_ref(),
        colors: check_colors(&raw.colors, err),
    }
}

/// Checks the row colors. A bad value is an error; the returned value
/// is then unused.
fn check_colors(
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
