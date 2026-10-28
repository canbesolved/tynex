// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Inputs and outputs of the core.

use crate::core::window::{WindowId, WindowInfo};

/// Portable key press. Each platform maps native key events to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// A printable character.
    Char {
        /// Character the active layout produces, after compose.
        text: char,
        /// Same physical key in the first Latin layout, if any.
        latin: Option<char>,
    },
    Enter,
    Esc,
    Tab,
    Up,
    Down,
    Backspace,
}

/// Inputs to the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Windows(Vec<WindowInfo>), // Full list, most recently used first
    Show,
    Key(Key),
    Hide,
    Quit,
}

/// Outputs of the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    None,
    Redraw,
    Activate(WindowId),
    Launch(String), // App id
    Close,
}
