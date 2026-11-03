// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Dropdown of one app's window titles, under its icon. With the title
//! list on, it opens over that list.

#![allow(clippy::cast_precision_loss)]

pub(crate) use self::panel_draw::draw;
use super::icons as strip;
use crate::core::icons;
use crate::core::view::View;
use crate::ui::draw::{Grid, Metrics};
use crate::ui::frames;
use crate::ui::render::text::Text;
use crate::ui::scroll;

#[path = "menu_draw.rs"]
mod panel_draw;

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
    pub titles: Vec<String>,          // The rows shown
    pub selected: usize,              // Among the rows shown
    pub beam: Option<(usize, usize)>, // Row shown and cursor of a rename
    pub first: usize,                 // Dropdown index of the first row shown
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
    let ordered = icons::ordered(icon, view.window_rows());
    let edits: Vec<_> =
        ordered.iter().map(|row| view.edit(&row.target)).collect();
    let all: Vec<String> = ordered
        .iter()
        .zip(&edits)
        .map(|(row, edit)| edit.map_or(&row.title[..], |(text, _)| text).into())
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
        beam: edits.iter().enumerate().find_map(|(at, edit)| {
            let (_, cursor) = (*edit)?;

            (first..first + rows)
                .contains(&at)
                .then(|| (at - first, cursor))
        }),
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

#[cfg(test)]
#[path = "menu_tests.rs"]
mod tests;
