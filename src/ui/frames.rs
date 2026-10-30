// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Frames of the popup. The input bar and the icon strip are separate
//! rounded blocks, with a transparent gap between them.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use crate::core::view::View;
use crate::ui::draw::{Grid, Metrics, Paint, Size};
use crate::ui::render::canvas::{Area, Rgba};
use crate::ui::taskbar::{icons, titles};
use crate::ui::{input_bar, output_bar};

/// Gap between the input frame and the icon frame, logical pixels.
const FRAME_GAP: f32 = 3.0;

/// Gap under the icon frame, logical pixels.
///
/// The dropdown starts this far below the icons. The title frame
/// starts this far below the spare room.
const BAR_GAP: f32 = 3.0;

/// Heights of the frames and the transparent bands between them.
struct Bands {
    input: f32,
    gap: f32,
    icons: f32,
    spare: f32,
    split: f32,
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
        width: icons::popup_width(metrics) as u32,
        height: (bands.input
            + bands.gap
            + bands.icons
            + bands.spare
            + bands.split
            + bands.lower)
            .round() as u32,
    }
}

/// Top of the icon row inside its frame.
pub(crate) fn icon_top(grid: &Grid, metrics: &Metrics) -> f32 {
    input_height(grid) + gap(metrics) + grid.pad
}

/// Bottom edge of the icon frame.
pub(crate) fn icon_bottom(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
) -> f32 {
    icon_top(grid, metrics)
        + icons::strip_height(view, metrics, grid)
        + grid.pad
}

/// Top of the dropdown, one gap below the icon frame.
pub(crate) fn menu_top(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    let drop = if icons::spare_height(view, metrics, grid) > 0.0 {
        bar_gap(metrics)
    } else {
        0.0
    };

    icon_bottom(view, metrics, grid) + drop
}

/// Draws the frames, the input bar, the icon strip, the output bar and
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
    let icons_y = bands.input + bands.gap;

    paint.canvas.clear(Rgba(0, 0, 0, 0));
    frame(paint, 0.0, bands.input, width, radius);
    if bands.icons > 0.0 {
        frame(paint, icons_y, bands.icons, width, radius);
    }
    if bands.lower > 0.0 {
        frame(
            paint,
            icons_y + bands.icons + bands.spare + bands.split,
            bands.lower,
            width,
            radius,
        );
    }

    input_bar::draw(view, &grid, paint, grid.pad, inner, metrics.hint_color);
    if bands.icons > 0.0 {
        draw_icons(view, metrics, &grid, paint, inner, &bands, icons_y);
    } else {
        let below = grid.pad + input_bar::height(&grid);

        draw_results(view, metrics, &grid, paint, inner, below);
    }
}

/// Draws the strip, and the title list in its own frame.
fn draw_icons(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    inner: f32,
    bands: &Bands,
    icons_y: f32,
) {
    icons::draw(view, metrics, grid, paint, icon_top(grid, metrics));
    if bands.lower > 0.0 {
        titles::draw(
            view,
            metrics,
            grid,
            paint,
            icons_y + bands.icons + bands.spare + bands.split + grid.pad,
            inner,
            false,
        );
    }
}

/// Draws the output bar and the title bar under `top` plus the strip.
fn draw_results(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    paint: &mut Paint<'_, '_>,
    inner: f32,
    top: f32,
) {
    let mut row = top + icons::strip_height(view, metrics, grid);

    output_bar::draw(view, metrics, grid, paint, row, inner);
    row += output_bar::height(view, metrics, grid);
    titles::draw(view, metrics, grid, paint, row, inner, true);
}

/// Splits the popup. The icon frame is absent while the strip is hidden.
fn bands(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> Bands {
    let strip = icons::strip_height(view, metrics, grid);
    let menu = icons::spare_height(view, metrics, grid);
    let menu_gap = if menu > 0.0 { bar_gap(metrics) } else { 0.0 };

    if strip > 0.0 {
        let lower = if titles::shows(view, metrics) {
            2.0 * grid.pad + titles::body(view, metrics, grid)
        } else {
            0.0
        };

        Bands {
            input: input_height(grid),
            gap: gap(metrics),
            icons: 2.0 * grid.pad + strip,
            spare: menu_gap + menu,
            split: if lower > 0.0 { bar_gap(metrics) } else { 0.0 },
            lower,
        }
    } else {
        let below = output_bar::height(view, metrics, grid)
            + titles::height(view, metrics, grid);

        Bands {
            input: input_height(grid) + below,
            gap: 0.0,
            icons: 0.0,
            spare: 0.0,
            split: 0.0,
            lower: 0.0,
        }
    }
}

/// Height of the input frame: padding, the row, padding.
fn input_height(grid: &Grid) -> f32 {
    2.0 * grid.pad + input_bar::height(grid)
}

/// Transparent gap under the input frame, in physical pixels.
fn gap(metrics: &Metrics) -> f32 {
    FRAME_GAP * metrics.scale
}

/// Transparent gap under the icon frame, in physical pixels.
fn bar_gap(metrics: &Metrics) -> f32 {
    BAR_GAP * metrics.scale
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
