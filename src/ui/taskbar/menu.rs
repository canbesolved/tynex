// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Dropdown of one app's window titles, under its icon. With the title
//! list on, it opens over that list.

#![allow(clippy::cast_precision_loss)]

use super::icons as strip;
use crate::core::icons;
use crate::core::view::View;
use crate::ui::draw::{self, Grid, Metrics, Paint};
use crate::ui::frames;
use crate::ui::render::canvas::Area;
use crate::ui::render::text::Text;
use crate::ui::scroll;

/// Longest title, in logical pixels, before the ellipsis.
const TITLE_CAP: f32 = 450.0;

/// First dropdown row on screen. A new selection moves it only far
/// enough to keep that title visible.
#[derive(Debug, Default)]
pub(crate) struct Place {
    first: usize,
    index: Option<usize>,
}

impl Place {
    /// Forgets the window, for example when the dropdown closes.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns the first row of the last alignment.
    #[must_use]
    pub fn first(&self) -> usize {
        self.first
    }

    /// Moves the window only when `index` would leave the shown rows.
    pub fn align(&mut self, index: usize, shown: usize, len: usize) -> usize {
        let shown = shown.max(1);

        if self.index != Some(index) {
            self.first = scroll::follow(self.first, index, shown);
            self.index = Some(index);
        }
        self.first = self.first.min(len.saturating_sub(shown));
        self.first
    }
}

/// Updates `place` from the open dropdown, or forgets it when closed.
pub(crate) fn follow(place: &mut Place, view: &View<'_>, metrics: &Metrics) {
    if !strip::shows(view, metrics) {
        place.reset();
        return;
    }
    let Some(index) = view.dropdown else {
        place.reset();
        return;
    };
    let Some(icon) = view.icons().get(view.icon) else {
        place.reset();
        return;
    };
    let len = icons::ordered(icon, view.window_rows()).len();

    if len == 0 {
        place.reset();
        return;
    }
    let index = index.min(len - 1);
    let shown = rows(view, metrics).min(len);

    place.align(index, shown, len);
}

/// A dropdown captured so it can slide closed after the state drops it.
#[derive(Debug, Clone)]
pub(crate) struct Panel {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub row_h: f32,
    pub titles: Vec<String>, // The rows shown
    pub selected: usize,     // Among the rows shown
    pub first: usize,        // Dropdown index of the first row shown
}

/// Aligns the dropdown window and returns the panel to draw.
pub(crate) fn open(
    place: &mut Place,
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    text: &Text,
) -> Option<Panel> {
    follow(place, view, metrics);
    panel(view, metrics, grid, text, place.first())
}

/// Returns the open dropdown, or `None` when it is closed.
pub(crate) fn panel(
    view: &View<'_>,
    metrics: &Metrics,
    grid: &Grid,
    text: &Text,
    origin: usize,
) -> Option<Panel> {
    if !strip::shows(view, metrics) {
        return None;
    }
    let index = view.dropdown?;
    let icon = view.icons().get(view.icon)?;
    let all: Vec<String> = icons::ordered(icon, view.window_rows())
        .into_iter()
        .map(|row| row.title.clone())
        .collect();

    if all.is_empty() {
        return None;
    }

    let index = index.min(all.len() - 1);
    let rows = rows(view, metrics).min(all.len());
    let first = origin.min(all.len().saturating_sub(rows));
    let titles = all[first..first + rows].to_vec();
    let font = grid.font;
    let longest = all
        .iter()
        .map(|title| text.width(title, font))
        .fold(0.0, f32::max);
    let text_w = longest.min(TITLE_CAP * metrics.scale);
    let placed = strip::layout(view, metrics, grid);
    let popup = strip::popup_width(metrics);
    let limit = (popup - 2.0 * grid.pad).max(placed.size);
    let w = (text_w + 2.0 * grid.gap).clamp(placed.size, limit);
    let icon_x = placed.x + view.icon as f32 * (placed.size + placed.gap);
    let x = icon_x.clamp(grid.pad, (popup - grid.pad - w).max(grid.pad));
    let y = frames::menu_top(view, metrics, grid);

    Some(Panel {
        x,
        y,
        w,
        h: titles.len() as f32 * grid.row_h,
        row_h: grid.row_h,
        selected: index.saturating_sub(first),
        titles,
        first,
    })
}

/// Returns how many titles the dropdown shows. Over the title list it
/// stops at that list's bottom edge, so the popup does not grow.
fn rows(view: &View<'_>, metrics: &Metrics) -> usize {
    let cap = metrics.max_rows.max(1);

    if frames::overlays(view, metrics) {
        cap.min(view.window_count().clamp(1, metrics.list_rows.max(1)))
    } else {
        cap
    }
}

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
        draw::draw_text(
            paint,
            grid,
            (title, &[]),
            Area::new(
                row.x + grid.gap,
                row.y,
                (row.w - 2.0 * grid.gap).max(0.0),
                row.h,
            ),
            paint.theme.text,
        );
    }
}

#[cfg(test)]
#[path = "menu_tests.rs"]
mod tests;
