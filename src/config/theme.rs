// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Built-in color theme, `main.theme`.

use serde::Deserialize;

/// Built-in popup colors, chosen with `main.theme`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Dark, // Near-black popup
    Light, // Near-white popup
}

#[cfg(test)]
#[path = "theme_tests.rs"]
mod theme_tests;
