// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The input bar at the top of the popup.

use crate::core::view::View;
use crate::ui::draw::{Grid, Paint, draw_text};
use crate::ui::render::canvas::Area;

/// Returns the bar height.
pub(crate) fn height(grid: &Grid) -> f32 {
    grid.row_h
}

/// Draws `> query` at `top`.
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

    draw_text(paint, grid, (&query, &[]), bar, paint.theme.text);
}
