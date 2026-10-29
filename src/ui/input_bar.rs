// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The input bar at the top of the popup.

use crate::core::view::View;
use crate::ui::draw::{Grid, Paint, baseline, draw_text};
use crate::ui::render::canvas::{Area, Rgba};

/// Beam thickness in pixels.
const BEAM_WIDTH: f32 = 1.0;

/// Beam height relative to the ascender-to-descender span.
const BEAM_HEIGHT: f32 = 0.9;

/// Returns the bar height.
pub(crate) fn height(grid: &Grid) -> f32 {
    grid.row_h
}

/// Draws `>` and the query at `top`, with a beam at the cursor.
///
/// The gap after `>` is the row gap plus one space. An empty query
/// shows `hint` in `hint_color` instead.
pub(crate) fn draw(
    view: &View<'_>,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    top: f32,
    inner: f32,
    hint_color: Rgba,
) {
    let bar =
        Area::new(grid.pad + grid.gap, top, inner - 2.0 * grid.gap, grid.row_h);
    let mark_w = paint.text.width(">", grid.font);
    let gap = grid.gap + paint.text.width(" ", grid.font);
    let query_x = bar.x + mark_w + gap;
    let room = bar.x + bar.w - query_x;
    let (text, color) = if view.query.is_empty() {
        (view.hint, hint_color)
    } else {
        (view.query, paint.theme.text)
    };

    draw_text(
        paint,
        grid,
        (">", &[]),
        Area::new(bar.x, bar.y, mark_w, bar.h),
        paint.theme.text,
    );
    if room > 0.0 && !text.is_empty() {
        draw_text(
            paint,
            grid,
            (text, &[]),
            Area::new(query_x, bar.y, room, bar.h),
            color,
        );
    }

    let at = view.cursor.min(view.query.len());
    let typed = paint.text.width(&view.query[..at], grid.font);

    beam(paint, grid, bar, (mark_w + gap + typed).min(bar.w));
}

/// Draws a 1 px beam, 10% shorter than the text, at `advance`.
fn beam(paint: &mut Paint<'_, '_>, grid: &Grid, bar: Area, advance: f32) {
    let x = (bar.x + advance).round();
    let right = bar.x + bar.w;
    let width = (x + BEAM_WIDTH).min(right) - x;

    if width < 1.0 {
        return;
    }

    let (ascent, descent) = paint.text.metrics(grid.font);
    let base = baseline(paint, grid.font, bar);
    let top = base - ascent;
    let bottom = base - descent; // Descent is negative
    let inset = (bottom - top) * (1.0 - BEAM_HEIGHT) / 2.0;
    let y = (top + inset).round();
    let height = ((bottom - inset).round() - y).max(BEAM_WIDTH);

    paint
        .canvas
        .fill_rect(Area::new(x, y, width, height), paint.theme.text);
}
