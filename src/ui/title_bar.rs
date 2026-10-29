// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Title bar: the open windows, under the input bar.
//!
//! Search matches are not drawn here. They belong to the output bar.

#![allow(clippy::cast_precision_loss)]

use crate::core::view::View;
use crate::ui::draw::{self, Grid, Metrics, Paint};
use crate::ui::render::canvas::Area;

/// Returns true when the open-window list is on screen.
///
/// It shows only while enabled and the query is empty. Typing hides
/// it; clearing the query shows it again.
pub(crate) fn shows(view: &View<'_>, metrics: &Metrics) -> bool {
    metrics.title_bar && view.query.is_empty()
}

/// Returns the height of the window rows, without the separator.
pub(crate) fn body(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if !shows(view, metrics) {
        return 0.0;
    }
    let rows = view.window_count().clamp(1, metrics.max_rows.max(1)) as f32;

    rows * grid.row_h
}

/// Returns the extra popup height for the separator and the windows.
pub(crate) fn height(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    let rows = body(view, metrics, grid);

    if rows == 0.0 { 0.0 } else { rows + 1.0 }
}

/// Draws the open windows, starting at `top`.
///
/// `separator` draws the line above the rows. A detached title bar
/// omits it.
pub(crate) fn draw(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    mut top: f32,
    inner: f32,
    separator: bool,
) {
    if !shows(view, metrics) {
        return;
    }
    if separator {
        paint.canvas.fill_rect(
            Area::new(grid.pad, top, inner, 1.0),
            paint.theme.border,
        );
        top += 1.0;
    }

    // Drawn only while the query is empty, so the selection is here.
    let max = metrics.max_rows.max(1);
    let first = (view.selected + 1).saturating_sub(max);
    let radius = paint.theme.radius * metrics.scale;

    for (index, row) in view.windows().enumerate().skip(first).take(max) {
        let area = Area::new(grid.pad, top, inner, grid.row_h);

        if index == view.selected {
            paint.canvas.fill_rounded_rect(
                area,
                radius / 2.0,
                paint.theme.selected,
            );
        }
        draw::draw_row(paint, grid, row, area, &[], metrics.show_icons);
        top += grid.row_h;
    }
}
