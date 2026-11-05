// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Inputs and outputs of the core.

use crate::core::browser_tabs::{TabRef, TabsUpdate};
use crate::core::hotkey::Binding;
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
    BackTab, // Shift+Tab
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
    /// A key with Ctrl, Alt or Meta, a key with no other variant (`F2`),
    /// or a modifier tapped alone. Only hotkey bindings use it.
    Combo(Binding),
}

/// Inputs to the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Windows(Vec<WindowInfo>), // Full list, most recently used first
    Show,
    Key(Key),
    Click(Click),
    Hover(Click), // Pointer move; same places as a click
    Hide,
    QuickSwitch, // Back to the previous place, open or hidden
    Quit,
    Tabs(TabsUpdate), // From the browser tabs adapter
}

/// Outputs of the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    None,
    Redraw,
    Activate(WindowId),
    Launch(String),      // App id
    ActivateTab(TabRef), // Hide, then ask the tab source
    Close,
}

/// Where a click landed in the popup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Click {
    Icon(usize), // Index into the running apps
    Menu(usize), // Index into the open dropdown
    Row(usize),  // Index into the open windows
    Elsewhere,   // Inside the popup, not on an icon or row
}
