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
use crate::ui::scroll;

/// Returns true when the input has produced something to show.
pub(crate) fn shows(view: &View<'_>) -> bool {
    !view.query.is_empty()
}

/// Returns the height of the result rows, or zero when hidden.
pub(crate) fn body(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if !shows(view) {
        return 0.0;
    }
    let rows = view.len().clamp(1, metrics.list_rows.max(1)) as f32;

    rows * grid.row_h
}

/// Draws the current results, starting at `top`.
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

    let len = view.len();
    let max = metrics.list_rows.max(1);
    let first = scroll::first_row(metrics, view.selected, len);
    let width = inner - scroll::reserve(metrics, len);
    let radius = paint.theme.radius * metrics.scale;
    let shown = len.clamp(1, max) as f32 * grid.row_h;
    let rows = Area::new(grid.pad, top, inner, shown);

    for (index, row) in view.rows().enumerate().skip(first).take(max) {
        let area = Area::new(grid.pad, top, width, grid.row_h);

        if index == view.selected {
            paint.canvas.fill_rounded_rect(
                area,
                radius / 2.0,
                paint.theme.selected,
            );
        }
        draw::draw_row(paint, grid, row, area, &row.highlights, true);
        top += grid.row_h;
    }
    scroll::bar(paint, metrics, rows, first, len);
}
