// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Fits the popup in the screen height.

#![allow(clippy::cast_precision_loss)]

use crate::config::Position;
use crate::core::view::View;
use crate::ui::draw::Metrics;
use crate::ui::frames::measured;
use crate::ui::place;
use crate::ui::render::theme::Theme;

/// Drops list rows, then dropdown rows, until the popup fits `screen`
/// physical pixels. The lists scroll, so they go first. Without a
/// screen height the metrics stay as they are. `anchor` is the logical
/// height locked when a centered popup opened.
pub(crate) fn fit(
    view: &View<'_>,
    mut metrics: Metrics,
    theme: &Theme,
    screen: Option<u32>,
    position: Position,
    anchor: Option<u32>,
) -> Metrics {
    let Some(screen) = screen else {
        return metrics;
    };
    let limit = place::limit(position, screen, metrics.scale, anchor);
    let tall = |m: &Metrics| measured(view, m, theme).height as f32 > limit;

    while metrics.list_rows > 1 && tall(&metrics) {
        metrics.list_rows -= 1;
    }
    while metrics.max_rows > 1 && tall(&metrics) {
        metrics.max_rows -= 1;
    }
    metrics
}

#[cfg(test)]
#[path = "fit_tests.rs"]
mod tests;
