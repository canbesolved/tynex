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

use crate::config::{
    HintConfig, MainConfig, OutputConfig, TaskbarConfig, TaskbarMode,
};
use crate::core::view::Row;
use crate::ui::render::canvas::{Area, Canvas, Rgba};
use crate::ui::render::icons::Icons;
use crate::ui::render::text::{Style, Text};
use crate::ui::render::theme::Theme;

/// Sizes and options from the config and the output, in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Metrics {
    pub font_size: f32,           // Title size
    pub width: f32,               // Popup width
    pub max_rows: usize,          // Dropdown and its spare room
    pub list_rows: usize,         // Title list and output bar; they scroll
    pub first_row: Option<usize>, // First list row; None follows selection
    pub scale: f32,               // Output scale factor, 1.0 or more
    pub taskbar: TaskbarMode,     // Strip, titles, both or off
    pub show_icons: bool,         // Title-list icon slot
    pub hint_color: Rgba,         // Hint text while the query is empty
    pub icon_size: f32,           // Logical pixels
    pub border: f32,              // Selection outline, logical pixels
}

/// Surface size in pixels.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Config values turned into `Metrics` at an output scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    pub font_size: u16,
    pub width: u16,
    pub max_rows: u16,
    pub taskbar: TaskbarMode,
    pub show_icons: bool,
    pub hint_color: Rgba,
    pub icon_size: f32, // Logical pixels
    pub border: f32,    // Selection outline, logical pixels
}

impl Look {
    pub(crate) fn new(
        main: &MainConfig,
        output: &OutputConfig,
        hint: &HintConfig,
        taskbar: &TaskbarConfig,
    ) -> Self {
        let color = hint.font_color;

        Self {
            font_size: main.font_size,
            width: main.width,
            max_rows: output.max_rows,
            taskbar: taskbar.mode,
            show_icons: taskbar.titlebar.show_icons,
            hint_color: Rgba(color.r, color.g, color.b, 0xff),
            icon_size: f32::from(taskbar.iconbar.size),
            border: f32::from(taskbar.iconbar.border),
        }
    }

    /// Returns metrics at output `scale`.
    pub(crate) fn metrics(self, scale: f32) -> Metrics {
        Metrics {
            font_size: f32::from(self.font_size),
            width: f32::from(self.width),
            max_rows: usize::from(self.max_rows),
            list_rows: usize::from(self.max_rows),
            first_row: None,
            scale,
            taskbar: self.taskbar,
            show_icons: self.show_icons,
            hint_color: self.hint_color,
            icon_size: self.icon_size,
            border: self.border,
        }
    }
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
///
/// Returns the advance of the drawn text.
pub(crate) fn draw_text(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    (text, highlights): (&str, &[Range<usize>]),
    area: Area,
    color: Rgba,
) -> f32 {
    let style = Style {
        size: grid.font,
        color,
        highlight: paint.theme.highlight,
        highlights,
    };
    let base = baseline(paint, grid.font, area);

    paint
        .text
        .draw(paint.canvas, text, (area.x, base), area.w, &style)
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

    let color = row
        .title_color
        .map_or(paint.theme.text, |c| Rgba(c.r, c.g, c.b, 0xff));

    draw_text(paint, grid, (&row.title, highlights), title, color);
}

/// Returns the baseline that centers text of `size` in `area`.
pub(crate) fn baseline(paint: &Paint<'_, '_>, size: f32, area: Area) -> f32 {
    let (ascent, descent) = paint.text.metrics(size);
    let center = area.y + area.h / 2.0;

    (center + f32::midpoint(ascent, descent)).round()
}
