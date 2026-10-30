// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon strip of the taskbar. One icon per running app, hidden while typing.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use super::menu;
use crate::core::event::Click;
use crate::core::view::View;
use crate::ui::draw::{Grid, Metrics, Paint};
use crate::ui::frames;
use crate::ui::render::canvas::Area;
use crate::ui::render::text::{Style, Text};

/// Gap between icons, as a fraction of the icon side.
const GAP_RATIO: f32 = 0.35;

/// Window count font size, as a fraction of the icon side.
const BADGE_RATIO: f32 = 0.4;

/// Placed icons for one frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Layout {
    pub size: f32,
    pub gap: f32,
    pub x: f32,      // Left edge of the first icon
    pub border: f32, // Selection outline
}

/// Returns true while the strip is on screen.
pub(crate) fn shows(view: &View<'_>, metrics: &Metrics) -> bool {
    metrics.taskbar.iconbar() && view.query.is_empty()
}

/// Returns the surface width in physical pixels.
pub(crate) fn popup_width(metrics: &Metrics) -> f32 {
    (metrics.width * metrics.scale).round()
}

/// Places the icons, shrinking them until the row fits.
pub(crate) fn layout(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
) -> Layout {
    let base = metrics.icon_size * metrics.scale;
    let count_icons = view.icons().len();
    let inner = (popup_width(metrics) - 2.0 * grid.pad).max(0.0);
    let mut size = base;
    let mut gap = base * GAP_RATIO;

    if count_icons > 0 {
        let n = count_icons as f32;
        let row = n * size + (n - 1.0).max(0.0) * gap;

        if row > inner && row > 0.0 {
            let factor = inner / row;

            size *= factor;
            gap *= factor;
        }
    }

    let n = count_icons as f32;
    let row = n * size + (n - 1.0).max(0.0) * gap;

    Layout {
        size,
        gap,
        x: grid.pad + (inner - row).max(0.0) / 2.0,
        border: metrics.border * metrics.scale,
    }
}

/// Returns the strip height: one icon side, or zero when hidden.
pub(crate) fn strip_height(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
) -> f32 {
    if !shows(view, metrics) {
        return 0.0;
    }
    layout(view, metrics, grid).size
}

/// Returns how many dropdown rows the spare room holds.
pub(crate) fn spare_rows(view: &View<'_>, metrics: &Metrics) -> usize {
    if !shows(view, metrics) {
        return 0;
    }
    let tallest = view
        .icons()
        .iter()
        .map(|icon| icon.windows.len())
        .max()
        .unwrap_or(0);

    if tallest <= 1 {
        0
    } else {
        tallest.min(metrics.max_rows.max(1))
    }
}

/// Returns the transparent band under the strip.
pub(crate) fn spare_height(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
) -> f32 {
    spare_rows(view, metrics) as f32 * grid.row_h
}

/// Draws the icons and their count badges at `top`.
pub(crate) fn draw(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    top: f32,
) {
    if !shows(view, metrics) {
        return;
    }
    let placed = layout(view, metrics, grid);

    for (index, icon) in view.icons().iter().enumerate() {
        let x = placed.x + index as f32 * (placed.size + placed.gap);

        if index == view.icon {
            selection(paint, x, top, &placed);
        }
        if let Some(name) = &icon.icon {
            let side = placed.size.round().max(1.0) as u32;

            paint.icons.draw(
                paint.canvas,
                name,
                side,
                (x.round() as i32, top.round() as i32),
            );
        }
        if icon.windows.len() > 1 {
            badge(paint, &placed, x, top, icon.windows.len());
        }
    }
}

/// Draws the selection border outside the icon.
fn selection(paint: &mut Paint<'_, '_>, x: f32, y: f32, placed: &Layout) {
    let (size, out) = (placed.size, placed.border);
    paint.canvas.stroke_rounded_rect(
        Area::new(x - out / 2.0, y - out / 2.0, size + out, size + out),
        (size / 4.0).max(1.0) + out / 2.0,
        out,
        paint.theme.selected,
    );
}

/// Returns what a click at `(x, y)` hits. `reveal` is 0 to 1.
pub(crate) fn hit(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    text: &Text,
    x: f32,
    y: f32,
    reveal: f32,
) -> Click {
    if !shows(view, metrics) {
        return Click::Elsewhere;
    }
    let placed = layout(view, metrics, grid);
    let top = frames::icon_top(grid, metrics);
    let icons = view.icons().len();

    for index in 0..icons {
        let left = placed.x + index as f32 * (placed.size + placed.gap);
        let width = if index + 1 == icons {
            placed.size
        } else {
            placed.size + placed.gap
        };

        if x >= left && x < left + width && y >= top && y < top + placed.size {
            return Click::Icon(index);
        }
    }

    if let Some(panel) = menu::panel(view, metrics, grid, text) {
        let shown = panel.h * reveal.clamp(0.0, 1.0);

        if x >= panel.x
            && x < panel.x + panel.w
            && y >= panel.y
            && y < panel.y + shown
        {
            let row = ((y - panel.y) / panel.row_h) as usize;

            if row < panel.titles.len() {
                return Click::Menu(row);
            }
        }
    }
    Click::Elsewhere
}

/// Draws the window count in a pill on the bottom center of the icon.
fn badge(
    paint: &mut Paint<'_, '_>,
    placed: &Layout,
    x: f32,
    top: f32,
    n: usize,
) {
    let label = n.to_string();
    let size = (placed.size * BADGE_RATIO).max(1.0);
    let width = paint.text.width(&label, size);
    let h = (size * 1.25).min(placed.size);
    let w = (width + h * 0.6).clamp(h, placed.size);
    let pill =
        Area::new(x + (placed.size - w) / 2.0, top + placed.size - h, w, h);
    let style = Style {
        size,
        color: paint.theme.text,
        highlight: paint.theme.highlight,
        highlights: &[],
    };
    let (ascent, descent) = paint.text.metrics(size);
    let base = (pill.y + h / 2.0 + f32::midpoint(ascent, descent)).round();
    let left = x + (placed.size - width) / 2.0;

    paint
        .canvas
        .fill_rounded_rect(pill, h / 2.0, paint.theme.border);
    paint
        .text
        .draw(paint.canvas, &label, (left, base), width.max(1.0), &style);
}

#[cfg(test)]
#[path = "icons_tests.rs"]
mod tests;
