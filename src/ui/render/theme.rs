// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Colors and sizes. Drawing code reads them from one struct, so other
//! sources (config, desktop colors) can fill it later.

use crate::ui::render::canvas::Rgba;

/// Visual theme of the popup.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub background: Rgba,
    pub border: Rgba,
    pub selected: Rgba, // Selected row background
    pub text: Rgba,
    pub app_text: Rgba,  // App name column
    pub highlight: Rgba, // Search matches
    pub badge: Rgba,     // Key badge background
    pub badge_text: Rgba,
    pub radius: f32,
    pub padding: f32,
}

impl Default for Theme {
    /// One dark theme with fixed colors.
    fn default() -> Self {
        Self {
            background: Rgba(0x0d, 0x11, 0x17, 0xf2),
            border: Rgba(0x30, 0x36, 0x3d, 0xff),
            selected: Rgba(0x1f, 0x6f, 0xeb, 0x66),
            text: Rgba(0xe6, 0xed, 0xf3, 0xff),
            app_text: Rgba(0x8b, 0x94, 0x9e, 0xff),
            highlight: Rgba(0x58, 0xa6, 0xff, 0xff),
            badge: Rgba(0x30, 0x36, 0x3d, 0xff),
            badge_text: Rgba(0xe6, 0xed, 0xf3, 0xff),
            radius: 12.0,
            padding: 8.0,
        }
    }
}
