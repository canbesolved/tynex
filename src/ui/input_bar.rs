// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The input bar at the top of the popup.

use crate::core::view::View;
use crate::ui::draw::{Grid, Paint, baseline, draw_text};
use crate::ui::render::canvas::Area;

/// Returns the bar height.
pub(crate) fn height(grid: &Grid) -> f32 {
    grid.row_h
}

/// Draws `> query` at `top`, with an underline after the text.
pub(crate) fn draw(
    view: &View<'_>,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    top: f32,
    inner: f32,
) {
    let bar =
        Area::new(grid.pad + grid.gap, top, inner - 2.0 * grid.gap, grid.row_h);
    let query = format!("> {}", view.query);
    let advance = draw_text(paint, grid, (&query, &[]), bar, paint.theme.text);

    underline(paint, grid, bar, advance);
}

/// Draws a one-pixel underline, one space wide, after the text.
fn underline(paint: &mut Paint<'_, '_>, grid: &Grid, bar: Area, advance: f32) {
    let x = (bar.x + advance).round();
    let right = bar.x + bar.w;
    let width = (x + paint.text.width(" ", grid.font)).min(right) - x;

    if width < 1.0 {
        return;
    }

    let y = baseline(paint, grid.font, bar);

    paint
        .canvas
        .fill_rect(Area::new(x, y, width, 1.0), paint.theme.text);
}
