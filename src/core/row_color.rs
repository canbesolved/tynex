// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row background per result type in the output bar (ADR-0091).

use crate::core::view::{Rgb, Row, RowTarget};

/// A color with opacity, from `#RRGGBB` (opaque) or `#RRGGBBAA`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tint {
    pub color: Rgb,
    pub alpha: u8,
}

impl Tint {
    /// Parses `#RRGGBB` or `#RRGGBBAA`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if !text.is_ascii() {
            return None;
        }

        let (rgb, alpha) = match text.len() {
            7 => (text, "ff"),
            9 => text.split_at(7),
            _ => return None,
        };

        if !alpha.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        Some(Self {
            color: Rgb::parse(rgb)?,
            alpha: u8::from_str_radix(alpha, 16).ok()?,
        })
    }
}

/// `output.colors`: the row background of each result type. `None`
/// draws no background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RowColors {
    pub window: Option<Tint>, // Open windows
    pub tab: Option<Tint>,    // Browser tabs
    pub app: Option<Tint>,    // Apps that are not running
}

impl RowColors {
    /// Returns the background of `row`. The new-window row of a running
    /// app, the only launch row with its own title color, has none.
    #[must_use]
    pub fn of(&self, row: &Row) -> Option<Tint> {
        match row.target {
            RowTarget::Window(_) => self.window,
            RowTarget::Tab(_) => self.tab,
            RowTarget::Launch(_) if row.title_color.is_none() => self.app,
            RowTarget::Launch(_) => None,
        }
    }
}

#[cfg(test)]
#[path = "row_color_tests.rs"]
mod tests;
