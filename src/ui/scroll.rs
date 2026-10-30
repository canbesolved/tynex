// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Scrolling lists: the first row shown, the wheel and the scrollbar.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use crate::core::view::View;
use crate::ui::draw::{Metrics, Paint};
use crate::ui::output_bar;
use crate::ui::render::canvas::{Area, Rgba};
use crate::ui::taskbar::titles;

/// Rows one wheel notch moves.
const NOTCH_ROWS: i64 = 3;

/// Scrollbar width, and the gap before it, in logical pixels.
const BAR: f32 = 3.0;

/// First row shown in the list on screen. The wheel moves it; a new
/// selection moves it only as far as needed to show that row.
#[derive(Debug, Default)]
pub(crate) struct Viewport {
    first: usize,
    seen: Option<(usize, String)>, // Selection and query of last frame
    rest: f32,                     // Wheel rows not applied yet
}

impl Viewport {
    /// Forgets the position, for example when the popup closes.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns `metrics` with the first row set, following a selection
    /// that changed since the last frame.
    pub fn place(&mut self, view: &View<'_>, mut metrics: Metrics) -> Metrics {
        let Some((len, rows)) = list(view, &metrics) else {
            return metrics;
        };
        let seen = (view.selected, view.query.to_owned());

        if self.seen.as_ref() != Some(&seen) {
            self.first = follow(self.first, view.selected, rows);
            self.seen = Some(seen);
        }
        self.first = self.first.min(len.saturating_sub(rows));
        metrics.first_row = Some(self.first);
        metrics
    }

    /// Scrolls by wheel `notches` and `rows` of smooth scrolling.
    /// Returns true when the first row changed.
    pub fn scroll(
        &mut self,
        view: &View<'_>,
        metrics: &Metrics,
        notches: i32,
        rows: f32,
    ) -> bool {
        let Some((len, shown)) = list(view, metrics) else {
            return false;
        };

        self.rest += rows;

        let whole = self.rest.trunc();

        self.rest -= whole;

        let step = i64::from(notches) * NOTCH_ROWS + whole as i64;
        let last = len.saturating_sub(shown) as i64;
        let first = (self.first as i64 + step).clamp(0, last) as usize;
        let moved = first != self.first;

        self.first = first;
        moved
    }
}

/// Returns the length and the shown rows of the list on screen.
fn list(view: &View<'_>, metrics: &Metrics) -> Option<(usize, usize)> {
    let rows = metrics.list_rows.max(1);

    if titles::shows(view, metrics) {
        Some((view.window_count(), rows))
    } else if output_bar::shows(view) {
        Some((view.len(), rows))
    } else {
        None
    }
}

/// Moves `first` the least so row `selected` is among `rows` shown.
fn follow(first: usize, selected: usize, rows: usize) -> usize {
    if selected < first {
        selected
    } else if selected >= first + rows {
        selected + 1 - rows
    } else {
        first
    }
}

/// Returns the first row to draw. Without a set first row the list
/// follows the selection.
pub(crate) fn first_row(
    metrics: &Metrics,
    selected: usize,
    len: usize,
) -> usize {
    let rows = metrics.list_rows.max(1);
    let first = metrics
        .first_row
        .unwrap_or_else(|| (selected + 1).saturating_sub(rows));

    first.min(len.saturating_sub(rows))
}

/// Returns the width rows leave free for the scrollbar.
pub(crate) fn reserve(metrics: &Metrics, len: usize) -> f32 {
    if len > metrics.list_rows.max(1) {
        2.0 * BAR * metrics.scale
    } else {
        0.0
    }
}

/// Draws the scrollbar thumb at the right edge of `area`, the rows.
pub(crate) fn bar(
    paint: &mut Paint<'_, '_>,
    metrics: &Metrics,
    area: Area,
    first: usize,
    len: usize,
) {
    let rows = metrics.list_rows.max(1);

    if len <= rows {
        return;
    }
    let width = BAR * metrics.scale;
    let thumb = (area.h * rows as f32 / len as f32).clamp(2.0 * width, area.h);
    let top = area.y + (area.h - thumb) * first as f32 / (len - rows) as f32;
    let text = paint.theme.text;

    paint.canvas.fill_rounded_rect(
        Area::new(area.x + area.w - width, top, width, thumb),
        width / 2.0,
        Rgba(text.0, text.1, text.2, 0x66),
    );
}

#[cfg(test)]
#[path = "scroll_tests.rs"]
mod tests;
