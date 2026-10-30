// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Frames of the popup. Each visible bar is its own rounded block,
//! with a transparent gap between blocks.

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

/// Gap between the input frame and the next frame, logical pixels.
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

/// Returns true when an open dropdown is drawn over the title list.
pub(crate) fn overlays(view: &View<'_>, metrics: &Metrics) -> bool {
    titles::shows(view, metrics)
}

/// Transparent room under the icon frame for the dropdown. The title
/// list holds the dropdown instead when it is shown.
fn spare(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if overlays(view, metrics) {
        0.0
    } else {
        icons::spare_height(view, metrics, grid)
    }
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
    let lower_y = icons_y + bands.icons + bands.spare + bands.split;

    paint.canvas.clear(Rgba(0, 0, 0, 0));
    frame(paint, 0.0, bands.input, width, radius);
    if bands.icons > 0.0 {
        frame(paint, icons_y, bands.icons, width, radius);
    }
    if bands.lower > 0.0 {
        frame(paint, lower_y, bands.lower, width, radius);
    }

    input_bar::draw(view, &grid, paint, grid.pad, inner, metrics.hint_color);
    if bands.icons > 0.0 {
        icons::draw(view, metrics, &grid, paint, icon_top(&grid, metrics));
    }
    if bands.lower > 0.0 {
        let top = lower_y + grid.pad;

        output_bar::draw(view, metrics, &grid, paint, top, inner);
        titles::draw(view, metrics, &grid, paint, top, inner);
    }
}

/// Splits the popup into the input block and whatever sits under it.
fn bands(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> Bands {
    let strip = icons::strip_height(view, metrics, grid);
    let menu = spare(view, metrics, grid);
    let menu_gap = if menu > 0.0 { bar_gap(metrics) } else { 0.0 };
    let rows = lower_body(view, metrics, grid);
    let lower = if rows > 0.0 {
        2.0 * grid.pad + rows
    } else {
        0.0
    };
    let icons_h = if strip > 0.0 {
        2.0 * grid.pad + strip
    } else {
        0.0
    };

    Bands {
        input: input_height(grid),
        gap: if icons_h > 0.0 || lower > 0.0 {
            gap(metrics)
        } else {
            0.0
        },
        icons: icons_h,
        spare: menu_gap + menu,
        split: if icons_h > 0.0 && lower > 0.0 {
            bar_gap(metrics)
        } else {
            0.0
        },
        lower,
    }
}

/// Height of the title rows or the output rows, whichever is shown.
fn lower_body(view: &View<'_>, metrics: &Metrics, grid: &Grid) -> f32 {
    if titles::shows(view, metrics) {
        titles::body(view, metrics, grid)
    } else {
        output_bar::body(view, metrics, grid)
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
