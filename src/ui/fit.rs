// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Fits the popup in the screen height.

#![allow(clippy::cast_precision_loss)]

use crate::core::view::View;
use crate::ui::draw::Metrics;
use crate::ui::frames::measured;
use crate::ui::render::theme::Theme;

/// Room kept free above and below the popup, logical pixels.
const SCREEN_MARGIN: f32 = 24.0;

/// Drops list rows, then dropdown rows, until the popup fits `screen`
/// physical pixels with a margin. The lists scroll, so they go first.
/// Without a screen height the metrics stay as they are.
pub(crate) fn fit(
    view: &View<'_>,
    mut metrics: Metrics,
    theme: &Theme,
    screen: Option<u32>,
) -> Metrics {
    let Some(screen) = screen else {
        return metrics;
    };
    let limit = screen as f32 - 2.0 * SCREEN_MARGIN * metrics.scale;
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
