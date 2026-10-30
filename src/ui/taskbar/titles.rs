// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Title list of the taskbar: the open windows.
//!
//! Search matches are not drawn here. They belong to the output bar.

#![allow(clippy::cast_precision_loss)]

use crate::core::view::View;
use crate::ui::draw::{self, Grid, Metrics, Paint};
use crate::ui::render::canvas::Area;
use crate::ui::scroll;

/// Returns true when the open-window list is on screen.
///
/// It shows only while enabled and the query is empty. Typing hides
/// it; clearing the query shows it again.
pub(crate) fn shows(view: &View<'_>, metrics: &Metrics) -> bool {
    metrics.taskbar.titlebar() && view.query.is_empty()
}

/// Returns the height of the window rows, or zero when hidden.
pub(crate) fn body(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if !shows(view, metrics) {
        return 0.0;
    }
    let rows = view.window_count().clamp(1, metrics.list_rows.max(1)) as f32;

    rows * grid.row_h
}

/// Draws the open windows, starting at `top`.
pub(crate) fn draw(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    mut top: f32,
    inner: f32,
) {
    if !shows(view, metrics) {
        return;
    }

    // Drawn only while the query is empty, so the selection is here.
    let len = view.window_count();
    let max = metrics.list_rows.max(1);
    let first = scroll::first_row(metrics, view.selected, len);
    let width = inner - scroll::reserve(metrics, len);
    let radius = paint.theme.radius * metrics.scale;
    let rows = Area::new(grid.pad, top, inner, body(view, metrics, grid));

    for (index, row) in view.windows().enumerate().skip(first).take(max) {
        let area = Area::new(grid.pad, top, width, grid.row_h);

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
    scroll::bar(paint, metrics, rows, first, len);
}
