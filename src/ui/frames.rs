// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Frames of the popup: one rounded block, or two with a transparent
//! band between them for the dropdown.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use crate::core::view::View;
use crate::ui::draw::{Grid, Metrics, Paint, Size};
use crate::ui::render::canvas::{Area, Rgba};
use crate::ui::{icon_bar, input_bar, output_bar, title_bar};

/// Heights of the top frame, the spare room and the lower frame.
struct Bands {
    upper: f32,
    spare: f32,
    lower: f32,
}

/// Returns the surface size for `view`.
pub(crate) fn measured(
    view: &View<'_>,
    metrics: &Metrics,
    theme: &crate::ui::render::theme::Theme,
) -> Size {
    let grid = Grid::new(metrics, theme);
    let bands = bands(view, metrics, &grid);

    Size {
        width: icon_bar::popup_width(metrics) as u32,
        height: (bands.upper + bands.spare + bands.lower).round() as u32,
    }
}

/// Draws the frame, the input bar, the icon strip, the output bar and
/// the title bar. The dropdown is drawn by the painter.
pub(crate) fn draw_popup(
    view: &View<'_>,
    metrics: &Metrics,
    paint: &mut Paint<'_, '_>,
) {
    let grid = Grid::new(metrics, paint.theme);
    let width = paint.canvas.width() as f32;
    let bands = bands(view, metrics, &grid);
    let radius = paint.theme.radius * metrics.scale;
    let inner = width - 2.0 * grid.pad;

    paint.canvas.clear(Rgba(0, 0, 0, 0));
    frame(paint, 0.0, bands.upper, width, radius);
    if bands.lower > 0.0 {
        frame(paint, bands.upper + bands.spare, bands.lower, width, radius);
    }

    let mut top = grid.pad;

    input_bar::draw(view, &grid, paint, top, inner, metrics.hint_color);
    top += input_bar::height(&grid);
    icon_bar::draw(view, metrics, &grid, paint, top);
    if bands.spare > 0.0 {
        if bands.lower > 0.0 {
            title_bar::draw(
                view,
                metrics,
                &grid,
                paint,
                bands.upper + bands.spare + grid.pad,
                inner,
                false,
            );
        }
    } else {
        top += icon_bar::strip_height(view, metrics, &grid);
        output_bar::draw(view, metrics, &grid, paint, top, inner);
        top += output_bar::height(view, metrics, &grid);
        title_bar::draw(view, metrics, &grid, paint, top, inner, true);
    }
}

/// Splits the popup when the spare room would cover the title list.
fn bands(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> Bands {
    let input = input_bar::height(grid);
    let strip = icon_bar::strip_height(view, metrics, grid);
    let spare = icon_bar::spare_height(view, metrics, grid);

    if spare > 0.0 {
        let lower = if title_bar::shows(view, metrics) {
            2.0 * grid.pad + title_bar::body(view, metrics, grid)
        } else {
            0.0
        };

        Bands {
            upper: 2.0 * grid.pad + input + strip,
            spare,
            lower,
        }
    } else {
        Bands {
            upper: 2.0 * grid.pad
                + input
                + strip
                + output_bar::height(view, metrics, grid)
                + title_bar::height(view, metrics, grid),
            spare: 0.0,
            lower: 0.0,
        }
    }
}

/// Draws one rounded block of height `h` at `y`.
fn frame(paint: &mut Paint<'_, '_>, y: f32, h: f32, width: f32, radius: f32) {
    if h < 1.0 {
        return;
    }
    paint.canvas.fill_rounded_rect(
        Area::new(0.0, y, width, h),
        radius,
        paint.theme.border,
    );
    paint.canvas.fill_rounded_rect(
        Area::new(1.0, y + 1.0, width - 2.0, (h - 2.0).max(0.0)),
        (radius - 1.0).max(0.0),
        paint.theme.background,
    );
}
