// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Title bar: the open windows, under the input bar.
//!
//! Search matches are not drawn here. They belong to the output bar.

#![allow(clippy::cast_precision_loss)]

use crate::core::view::View;
use crate::ui::draw::{self, Grid, Metrics, Paint};
use crate::ui::render::canvas::Area;

/// Returns true when the open-window list is enabled.
pub(crate) fn shows(metrics: &Metrics) -> bool {
    metrics.title_bar
}

/// Returns the extra popup height for the separator and the windows.
pub(crate) fn height(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if !shows(metrics) {
        return 0.0;
    }
    let rows = view.window_count().clamp(1, metrics.max_rows.max(1)) as f32;

    1.0 + rows * grid.row_h
}

/// Draws the separator and the open windows, starting at `top`.
pub(crate) fn draw(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    mut top: f32,
    inner: f32,
) {
    if !shows(metrics) {
        return;
    }
    paint
        .canvas
        .fill_rect(Area::new(grid.pad, top, inner, 1.0), paint.theme.border);
    top += 1.0;

    // The selection lives in this list only while there is no output.
    // A query selects a match in the output bar instead.
    let max = metrics.max_rows.max(1);
    let selected = view.query.is_empty().then_some(view.selected);
    let first = selected.map_or(0, |index| (index + 1).saturating_sub(max));
    let radius = paint.theme.radius * metrics.scale;

    for (index, row) in view.windows().enumerate().skip(first).take(max) {
        let area = Area::new(grid.pad, top, inner, grid.row_h);

        if selected == Some(index) {
            paint.canvas.fill_rounded_rect(
                area,
                radius / 2.0,
                paint.theme.selected,
            );
        }
        draw::draw_row(paint, grid, row, area, &[], metrics.show_app_icon);
        top += grid.row_h;
    }
}
