// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Layouts: how a `View` is arranged on the canvas.

// Layout math converts between float positions and integer pixels on
// purpose; values stay within the surface size.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

pub mod vertical_list;

use crate::core::view::View;
use crate::ui::render::canvas::Canvas;
use crate::ui::render::icons::Icons;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

/// Sizes from the config and the output, in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metrics {
    pub font_size: f32, // Title size
    pub width: f32,     // Popup width
    pub max_rows: usize,
    pub scale: f32, // Output scale factor, 1.0 or more
}

/// Surface size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Everything a layout draws with.
pub struct Paint<'a, 'c> {
    pub canvas: &'a mut Canvas<'c>,
    pub text: &'a mut Text,
    pub icons: &'a mut Icons,
    pub theme: &'a Theme,
}

/// Arranges entries on a canvas. Reads only the `View`.
pub trait Layout {
    /// Returns the surface size for `view`; pure.
    fn size(&self, view: &View<'_>, m: &Metrics, theme: &Theme) -> Size;

    /// Returns the icon side in pixels; pure.
    fn icon_size(&self, m: &Metrics, theme: &Theme) -> u32;

    /// Draws the whole popup.
    fn draw(&self, view: &View<'_>, m: &Metrics, p: &mut Paint<'_, '_>);
}
