// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Drawing the dropdown panel: frame, selection and titles.

#![allow(clippy::cast_precision_loss)]

use super::Panel;
use crate::ui::draw::{self, Grid, Paint};
use crate::ui::input_bar;
use crate::ui::render::canvas::Area;

/// Draws `panel` slid in by `reveal` (0 hidden, 1 open).
pub(crate) fn draw(
    paint: &mut Paint<'_, '_>,
    grid: &Grid,
    panel: &Panel,
    reveal: f32,
    scale: f32,
) {
    let shown = panel.h * reveal.clamp(0.0, 1.0);

    if shown < 1.0 {
        return;
    }

    let radius = paint.theme.radius * scale;
    let area = Area::new(panel.x, panel.y, panel.w, shown);

    paint
        .canvas
        .fill_rounded_rect(area, radius, paint.theme.border);
    if shown > 2.0 {
        paint.canvas.fill_rounded_rect(
            Area::new(panel.x + 1.0, panel.y + 1.0, panel.w - 2.0, shown - 2.0),
            (radius - 1.0).max(0.0),
            paint.theme.background,
        );
    }

    let row_radius = radius / 2.0;

    for (index, title) in panel.titles.iter().enumerate() {
        let y = panel.y + index as f32 * panel.row_h;

        // A subpixel shortfall used to hide the last title.
        if y + panel.row_h > panel.y + shown + 1.0 {
            break;
        }
        let row = Area::new(panel.x, y, panel.w, panel.row_h);

        if index == panel.selected {
            paint.canvas.fill_rounded_rect(
                row,
                row_radius,
                paint.theme.selected,
            );
        }
        let area = Area::new(
            row.x + grid.gap,
            row.y,
            (row.w - 2.0 * grid.gap).max(0.0),
            row.h,
        );

        let color = paint.theme.text;

        match panel.beam.filter(|(at, _)| *at == index) {
            Some((_, cursor)) => {
                input_bar::draw_field(
                    paint,
                    grid,
                    (title, cursor),
                    area,
                    color,
                );
            }
            None => {
                draw::draw_text(paint, grid, (title, &[]), area, color);
            }
        }
    }
}
