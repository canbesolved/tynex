// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Colors and sizes. `main.theme` fills this struct. Drawing code
//! reads it from one place, so other sources can fill it later.

use crate::ui::render::canvas::Rgba;

/// Visual theme of the popup.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub background: Rgba,
    pub border: Rgba,
    pub selected: Rgba, // Selected row background
    pub text: Rgba,
    pub highlight: Rgba, // Search matches
    pub radius: f32,
    pub padding: f32,
}

impl Theme {
    /// Near-black popup. This is the default.
    #[must_use]
    pub fn dark() -> Self {
        Self {
            background: Rgba(0x0d, 0x11, 0x17, 0xf2),
            border: Rgba(0x30, 0x36, 0x3d, 0xff),
            selected: Rgba(0x1f, 0x6f, 0xeb, 0x66),
            text: Rgba(0xe6, 0xed, 0xf3, 0xff),
            highlight: Rgba(0x58, 0xa6, 0xff, 0xff),
            radius: 12.0,
            padding: 8.0,
        }
    }

    /// Near-white popup. Same radius, padding and selection as dark.
    #[must_use]
    pub fn light() -> Self {
        Self {
            background: Rgba(0xff, 0xff, 0xff, 0xf2),
            border: Rgba(0xd0, 0xd7, 0xde, 0xff),
            text: Rgba(0x1f, 0x23, 0x28, 0xff),
            highlight: Rgba(0x09, 0x69, 0xda, 0xff),
            ..Self::dark()
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_changes_colors_only() {
        let dark = Theme::dark();
        let light = Theme::light();

        assert_eq!(Theme::default(), dark);
        assert_eq!(light.background, Rgba(0xff, 0xff, 0xff, 0xf2));
        assert_eq!(light.border, Rgba(0xd0, 0xd7, 0xde, 0xff));
        assert_eq!(light.text, Rgba(0x1f, 0x23, 0x28, 0xff));
        assert_eq!(light.highlight, Rgba(0x09, 0x69, 0xda, 0xff));
        assert_ne!(light.background, dark.background);
        assert_ne!(light.border, dark.border);
        assert_ne!(light.text, dark.text);
        assert_ne!(light.highlight, dark.highlight);
        assert_eq!(light.selected, dark.selected);
        assert_eq!(light.radius, dark.radius);
        assert_eq!(light.padding, dark.padding);
    }
}
