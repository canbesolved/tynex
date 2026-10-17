// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Text, theme and layout together: sizes and draws the popup.

use crate::config::AppearanceConfig;
use crate::core::view::View;
use crate::core::window::WindowInfo;
use crate::ui::layouts::vertical_list::VerticalList;
use crate::ui::layouts::{Layout, Metrics, Paint, Size};
use crate::ui::render::canvas::Canvas;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

/// Draws views with the configured appearance. Built once at start, so
/// fonts and glyph caches stay warm between shows.
pub struct Painter {
    text: Text,
    theme: Theme,
    layout: VerticalList,
    appearance: AppearanceConfig,
}

impl Painter {
    #[must_use]
    pub fn new(text: Text, appearance: AppearanceConfig) -> Self {
        Self {
            text,
            theme: Theme::default(),
            layout: VerticalList,
            appearance,
        }
    }

    /// Resolves fonts for new row texts, so lookups never run at show
    /// time.
    pub fn prepare(&mut self, windows: &[WindowInfo]) {
        for window in windows {
            self.text.prepare(&window.title);
        }
    }

    /// Returns the popup size at output `scale`.
    #[must_use]
    pub fn size(&self, view: &View<'_>, scale: f32) -> Size {
        self.layout.size(view, &self.metrics(scale), &self.theme)
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
            theme: &self.theme,
        };

        self.layout.draw(view, &metrics, &mut paint);
    }

    fn metrics(&self, scale: f32) -> Metrics {
        Metrics {
            font_size: f32::from(self.appearance.font_size),
            width: f32::from(self.appearance.width),
            max_rows: usize::from(self.appearance.max_rows),
            scale,
        }
    }
}
