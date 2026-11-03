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
/// shows `hint` in `hint_color`, centered in the popup.
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

    draw_text(
        paint,
        grid,
        (">", &[]),
        Area::new(bar.x, bar.y, mark_w, bar.h),
        paint.theme.text,
    );
    if view.query.is_empty() {
        draw_hint(paint, grid, view.hint, bar, inner, hint_color);
    } else if room > 0.0 {
        draw_text(
            paint,
            grid,
            (view.query, &[]),
            Area::new(query_x, bar.y, room, bar.h),
            paint.theme.text,
        );
    }

    let at = view.cursor.min(view.query.len());
    let typed = paint.text.width(&view.query[..at], grid.font);

    beam(paint, grid, bar, (mark_w + gap + typed).min(bar.w));
}

/// Draws an edited `text` in `area` with a beam at byte `cursor`.
///
/// When the text before the cursor is wider than `area`, the text
/// scrolls left so the cursor stays at the right edge. A cut at the
/// right shows an ellipsis.
pub(crate) fn draw_field(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    (text, cursor): (&str, usize),
    area: Area,
    color: Rgba,
) {
    let cursor = cursor.min(text.len());
    let room = (area.w - BEAM_WIDTH).max(0.0);
    let start = scroll_start(text, cursor, room, |part| {
        paint.text.width(part, grid.font)
    });
    let typed = paint.text.width(&text[start..cursor], grid.font);

    draw_text(paint, grid, (&text[start..], &[]), area, color);
    beam(paint, grid, area, typed);
}

/// Returns the first byte to draw so that `text[start..cursor]` fits
/// `room`: 0 when it all fits, else the earliest start that does.
pub(crate) fn scroll_start(
    text: &str,
    cursor: usize,
    room: f32,
    width: impl Fn(&str) -> f32,
) -> usize {
    let starts: Vec<usize> = text[..cursor]
        .char_indices()
        .map(|(i, _)| i)
        .chain([cursor])
        .collect();
    let first_fit =
        starts.partition_point(|&start| width(&text[start..cursor]) > room);

    starts[first_fit.min(starts.len() - 1)]
}

/// Draws `hint` centered on the popup width, in `color`.
fn draw_hint(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    hint: &str,
    bar: Area,
    inner: f32,
    color: Rgba,
) {
    if hint.is_empty() {
        return;
    }

    let width = paint.text.width(hint, grid.font);
    let popup_w = inner + 2.0 * grid.pad;
    let left = grid.pad;
    let right = popup_w - grid.pad;
    let x = ((popup_w - width) / 2.0).clamp(left, right);
    let max = right - x;

    if max > 0.0 {
        draw_text(
            paint,
            grid,
            (hint, &[]),
            Area::new(x, bar.y, max, bar.h),
            color,
        );
    }
}

/// Draws a 1 px beam, 10% shorter than the text, at `advance` from
/// the left of `bar`. It is clipped at the right edge of `bar`.
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

#[cfg(test)]
#[path = "input_bar_tests.rs"]
mod tests;
