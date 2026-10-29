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
    Left,
    Right,
    Home,
    End,
    Backspace,
    Delete,
    WordLeft,  // Ctrl+Left
    WordRight, // Ctrl+Right
}

/// Inputs to the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Windows(Vec<WindowInfo>), // Full list, most recently used first
    Show,
    Key(Key),
    Click(Click),
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

/// Where a click landed in the popup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Click {
    Icon(usize), // Index into the running apps
    Menu(usize), // Index into the open dropdown
    Elsewhere,   // Inside the popup, not on an icon or title
}
