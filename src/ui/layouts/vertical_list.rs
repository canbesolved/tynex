// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Vertical list: a search prompt, then one row per window in columns:
//! icon, app name and title.

use std::ops::Range;

use crate::core::view::{Row, View};
use crate::ui::layouts::{Layout, Metrics, Paint, Size};
use crate::ui::render::canvas::{Area, Rgba};
use crate::ui::render::text::Style;
use crate::ui::render::theme::Theme;

/// Share of the text width that the app name column takes.
const APP_SHARE: f32 = 0.3;

/// The default layout.
#[derive(Debug, Default)]
pub struct VerticalList;

/// Geometry derived from the metrics, in physical pixels.
struct Grid {
    pad: f32,   // Outer padding
    row_h: f32, // Row height
    icon: f32,  // Icon slot side
    gap: f32,   // Space between parts of a row
    font: f32,  // Text size
    app: f32,   // Share of the text width for app names, 0 when hidden
}

impl Grid {
    fn new(metrics: &Metrics, theme: &Theme) -> Self {
        let font = metrics.font_size * metrics.scale;

        Self {
            pad: theme.padding * metrics.scale,
            row_h: (font * 2.0).round(),
            icon: (font * 1.5).round(),
            gap: (font * 0.6).round(),
            font,
            app: if metrics.show_app { APP_SHARE } else { 0.0 },
        }
    }
}

/// Returns true when rows are drawn: always with `title_bar`, else only
/// once a query is typed.
fn shows_rows(view: &View<'_>, metrics: &Metrics) -> bool {
    metrics.title_bar || !view.query.is_empty()
}

impl Layout for VerticalList {
    fn size(&self, view: &View<'_>, metrics: &Metrics, theme: &Theme) -> Size {
        let grid = Grid::new(metrics, theme);
        let rows = view.len().clamp(1, metrics.max_rows.max(1)) as f32;
        let list = if shows_rows(view, metrics) {
            1.0 + rows * grid.row_h // Separator line and rows
        } else {
            0.0
        };
        let height = 2.0 * grid.pad + grid.row_h + list;

        Size {
            width: (metrics.width * metrics.scale).round() as u32,
            height: height.round() as u32,
        }
    }

    fn icon_size(&self, metrics: &Metrics, theme: &Theme) -> u32 {
        Grid::new(metrics, theme).icon as u32
    }

    fn draw(
        &self,
        view: &View<'_>,
        metrics: &Metrics,
        paint: &mut Paint<'_, '_>,
    ) {
        let grid = Grid::new(metrics, paint.theme);
        let width = paint.canvas.width() as f32;
        let height = paint.canvas.height() as f32;
        let radius = paint.theme.radius * metrics.scale;
        let inner = width - 2.0 * grid.pad;

        paint.canvas.clear(Rgba(0, 0, 0, 0));
        paint.canvas.fill_rounded_rect(
            Area::new(0.0, 0.0, width, height),
            radius,
            paint.theme.border,
        );
        paint.canvas.fill_rounded_rect(
            Area::new(1.0, 1.0, width - 2.0, height - 2.0),
            radius - 1.0,
            paint.theme.background,
        );

        let mut top = grid.pad;
        let bar = Area::new(
            grid.pad + grid.gap,
            top,
            inner - 2.0 * grid.gap,
            grid.row_h,
        );
        let query = format!("/ {}", view.query);
        let color = paint.theme.text;

        draw_text(paint, &grid, (&query, &[]), bar, color);
        if !shows_rows(view, metrics) {
            return;
        }
        top += grid.row_h;
        paint.canvas.fill_rect(
            Area::new(grid.pad, top, inner, 1.0),
            paint.theme.border,
        );
        top += 1.0;

        // Scroll so the selected row is always visible.
        let max = metrics.max_rows.max(1);
        let first = (view.selected + 1).saturating_sub(max);

        for (index, row) in view.rows().enumerate().skip(first).take(max) {
            let area = Area::new(grid.pad, top, inner, grid.row_h);

            if index == view.selected {
                paint.canvas.fill_rounded_rect(
                    area,
                    radius / 2.0,
                    paint.theme.selected,
                );
            }
            draw_row(paint, &grid, row, area);
            top += grid.row_h;
        }
    }
}

/// Draws one row: icon, app name (`show_app`), title.
fn draw_row(paint: &mut Paint<'_, '_>, grid: &Grid, row: &Row, area: Area) {
    let mut left = area.x + grid.gap;

    if let Some(icon) = &row.icon {
        let top = area.y + (area.h - grid.icon) / 2.0;

        paint.icons.draw(
            paint.canvas,
            icon,
            grid.icon as u32,
            (left as i32, top.round() as i32),
        );
    }
    left += grid.icon + grid.gap;

    if grid.app > 0.0 {
        let width = ((area.x + area.w - grid.gap - left) * grid.app).round();
        let app = Area::new(left, area.y, width, area.h);
        let color = paint.theme.app_text;

        draw_text(paint, grid, (&row.app_name, &[]), app, color);
        left += width + grid.gap;
    }

    let title =
        Area::new(left, area.y, area.x + area.w - grid.gap - left, area.h);
    let color = paint.theme.text;

    draw_text(paint, grid, (&row.title, &row.highlights), title, color);
}

/// Draws `text` in `color`, vertically centered in `area`, with the
/// `highlights` byte ranges in the highlight color.
fn draw_text(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    (text, highlights): (&str, &[Range<usize>]),
    area: Area,
    color: Rgba,
) {
    let style = Style {
        size: grid.font,
        color,
        highlight: paint.theme.highlight,
        highlights,
    };
    let base = baseline(paint, grid.font, area);

    paint
        .text
        .draw(paint.canvas, text, (area.x, base), area.w, &style);
}

/// Returns the baseline that centers text of `size` in `area`.
fn baseline(paint: &Paint<'_, '_>, size: f32, area: Area) -> f32 {
    let (ascent, descent) = paint.text.metrics(size);
    let center = area.y + area.h / 2.0;

    (center + f32::midpoint(ascent, descent)).round()
}

#[cfg(test)]
#[path = "vertical_list_tests.rs"]
mod tests;
