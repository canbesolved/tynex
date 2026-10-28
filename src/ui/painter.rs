// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Stacks the enabled UI modules and draws them on one surface. The
//! input bar is always included.

// Popup math converts between float positions and integer pixels on
// purpose; values stay within the surface size.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use crate::config::AppearanceConfig;
use crate::core::view::View;
use crate::core::window::WindowInfo;
use crate::ui::draw::{self, Grid, Metrics, Paint, Size};
use crate::ui::render::canvas::{Area, Canvas, Rgba};
use crate::ui::render::icons::Icons;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::{input_bar, output_bar, title_bar};

/// Draws views with the configured appearance. Built once at start, so
/// fonts and glyph caches stay warm between shows.
pub struct Painter {
    text: Text,
    icons: Icons,
    theme: Theme,
    appearance: AppearanceConfig,
}

impl Painter {
    #[must_use]
    pub fn new(text: Text, icons: Icons, appearance: AppearanceConfig) -> Self {
        Self {
            text,
            icons,
            theme: Theme::default(),
            appearance,
        }
    }

    /// Loads the icons of new windows at output `scale`, so files are
    /// never read at show time.
    pub fn prepare(&mut self, windows: &[WindowInfo], scale: f32) {
        let size = draw::icon_size(&self.metrics(scale), &self.theme);

        for icon in windows.iter().filter_map(|w| w.icon.as_deref()) {
            self.icons.prepare(icon, size);
        }
    }

    /// Returns the popup size at output `scale`.
    #[must_use]
    pub fn size(&self, view: &View<'_>, scale: f32) -> Size {
        measured(view, &self.metrics(scale), &self.theme)
    }

    /// Draws `view` on `canvas` at output `scale`.
    pub fn draw(
        &mut self,
        canvas: &mut Canvas<'_>,
        view: &View<'_>,
        scale: f32,
    ) {
        let metrics = self.metrics(scale);
        let mut paint = Paint {
            canvas,
            text: &mut self.text,
            icons: &mut self.icons,
            theme: &self.theme,
        };

        draw_popup(view, &metrics, &mut paint);
    }

    fn metrics(&self, scale: f32) -> Metrics {
        Metrics {
            font_size: f32::from(self.appearance.font_size),
            width: f32::from(self.appearance.width),
            max_rows: usize::from(self.appearance.max_rows),
            scale,
            title_bar: self.appearance.title_bar,
            show_app: self.appearance.show_app,
        }
    }
}

/// Returns the surface size for `view`.
pub(crate) fn measured(
    view: &View<'_>,
    metrics: &Metrics,
    theme: &Theme,
) -> Size {
    let grid = Grid::new(metrics, theme);
    let height = 2.0 * grid.pad
        + input_bar::height(&grid)
        + output_bar::height(view, metrics, &grid)
        + title_bar::height(view, metrics, &grid);

    Size {
        width: (metrics.width * metrics.scale).round() as u32,
        height: height.round() as u32,
    }
}

/// Draws the frame, the input bar, the output bar and the title bar.
pub(crate) fn draw_popup(
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

    input_bar::draw(view, &grid, paint, top, inner);
    top += input_bar::height(&grid);
    output_bar::draw(view, metrics, &grid, paint, top, inner);
    top += output_bar::height(view, metrics, &grid);
    title_bar::draw(view, metrics, &grid, paint, top, inner);
}

#[cfg(test)]
#[path = "painter_tests.rs"]
mod tests;
