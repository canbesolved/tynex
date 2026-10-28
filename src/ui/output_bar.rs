// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Output bar: the result of what was typed in the input bar.
//!
//! Today that result is the windows the query matched. Later results,
//! such as launched apps or a model answer, are drawn here too.

#![allow(clippy::cast_precision_loss)]

use crate::core::view::View;
use crate::ui::draw::{self, Grid, Metrics, Paint};
use crate::ui::render::canvas::Area;

/// Returns true when the input has produced something to show.
pub(crate) fn shows(view: &View<'_>) -> bool {
    !view.query.is_empty()
}

/// Returns the extra popup height for the separator and the results.
pub(crate) fn height(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if !shows(view) {
        return 0.0;
    }
    let rows = view.len().clamp(1, metrics.max_rows.max(1)) as f32;

    1.0 + rows * grid.row_h
}

/// Draws the separator and the current results, starting at `top`.
pub(crate) fn draw(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    mut top: f32,
    inner: f32,
) {
    if !shows(view) {
        return;
    }
    paint
        .canvas
        .fill_rect(Area::new(grid.pad, top, inner, 1.0), paint.theme.border);
    top += 1.0;

    // Scroll so the selected result stays visible.
    let max = metrics.max_rows.max(1);
    let first = (view.selected + 1).saturating_sub(max);
    let radius = paint.theme.radius * metrics.scale;

    for (index, row) in view.rows().enumerate().skip(first).take(max) {
        let area = Area::new(grid.pad, top, inner, grid.row_h);

        if index == view.selected {
            paint.canvas.fill_rounded_rect(
                area,
                radius / 2.0,
                paint.theme.selected,
            );
        }
        draw::draw_row(paint, grid, row, area, &row.highlights);
        top += grid.row_h;
    }
}
