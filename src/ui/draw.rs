// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Shared sizes and text drawing for the popup modules.

// Row math converts between float positions and integer pixels on
// purpose; values stay within the surface size.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use std::ops::Range;

use crate::core::view::Row;
use crate::ui::render::canvas::{Area, Canvas, Rgba};
use crate::ui::render::icons::Icons;
use crate::ui::render::text::{Style, Text};
use crate::ui::render::theme::Theme;

/// Sizes and options from the config and the output, in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Metrics {
    pub font_size: f32, // Title size
    pub width: f32,     // Popup width
    pub max_rows: usize,
    pub scale: f32,       // Output scale factor, 1.0 or more
    pub title_bar: bool,  // Open window list
    pub show_icons: bool, // Title-bar icon slot
}

/// Surface size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Everything a module draws with.
pub(crate) struct Paint<'a, 'c> {
    pub canvas: &'a mut Canvas<'c>,
    pub text: &'a mut Text,
    pub icons: &'a mut Icons,
    pub theme: &'a Theme,
}

/// Geometry derived from the metrics, in physical pixels.
pub(crate) struct Grid {
    pub pad: f32,   // Outer padding
    pub row_h: f32, // Row height
    pub icon: f32,  // Icon slot side
    pub gap: f32,   // Space between parts of a row
    pub font: f32,  // Text size
}

impl Grid {
    pub(crate) fn new(metrics: &Metrics, theme: &Theme) -> Self {
        let font = metrics.font_size * metrics.scale;

        Self {
            pad: theme.padding * metrics.scale,
            row_h: (font * 2.0).round(),
            icon: (font * 1.5).round(),
            gap: (font * 0.6).round(),
            font,
        }
    }
}

/// Draws `text` in `color`, vertically centered in `area`, with the
/// `highlights` byte ranges in the highlight color.
pub(crate) fn draw_text(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    (text, highlights): (&str, &[Range<usize>]),
    area: Area,
    color: Rgba,
) {
    let style = Style {
        size: grid.font,
        color,
        highlight: paint.theme.highlight,
        highlights,
    };
    let base = baseline(paint, grid.font, area);

    paint
        .text
        .draw(paint.canvas, text, (area.x, base), area.w, &style);
}

/// Returns the icon side in pixels.
pub(crate) fn icon_size(metrics: &Metrics, theme: &Theme) -> u32 {
    Grid::new(metrics, theme).icon as u32
}

/// Draws one window row: optional icon, then the title.
///
/// `show_icon` reserves the icon slot. A row without an icon keeps
/// the empty slot so titles stay aligned. `highlights` are search
/// matches; the title bar passes an empty slice.
pub(crate) fn draw_row(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    row: &Row,
    area: Area,
    highlights: &[Range<usize>],
    show_icon: bool,
) {
    let mut left = area.x + grid.gap;

    if show_icon {
        if let Some(icon) = &row.icon {
            let top = area.y + (area.h - grid.icon) / 2.0;

            paint.icons.draw(
                paint.canvas,
                icon,
                grid.icon as u32,
                (left as i32, top.round() as i32),
            );
        }
        left += grid.icon + grid.gap;
    }

    let title =
        Area::new(left, area.y, area.x + area.w - grid.gap - left, area.h);

    draw_text(
        paint,
        grid,
        (&row.title, highlights),
        title,
        paint.theme.text,
    );
}

/// Returns the baseline that centers text of `size` in `area`.
fn baseline(paint: &Paint<'_, '_>, size: f32, area: Area) -> f32 {
    let (ascent, descent) = paint.text.metrics(size);
    let center = area.y + area.h / 2.0;

    (center + f32::midpoint(ascent, descent)).round()
}
