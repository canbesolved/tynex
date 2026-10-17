// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Text, theme and layout together: sizes and draws the popup.

use crate::config::AppearanceConfig;
use crate::core::view::View;
use crate::core::window::WindowInfo;
use crate::ui::layouts::vertical_list::VerticalList;
use crate::ui::layouts::{Layout, Metrics, Paint, Size};
use crate::ui::render::canvas::Canvas;
use crate::ui::render::icons::Icons;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

/// Draws views with the configured appearance. Built once at start, so
/// fonts and glyph caches stay warm between shows.
pub struct Painter {
    text: Text,
    icons: Icons,
    theme: Theme,
    layout: VerticalList,
    appearance: AppearanceConfig,
}

impl Painter {
    #[must_use]
    pub fn new(text: Text, icons: Icons, appearance: AppearanceConfig) -> Self {
        Self {
            text,
            icons,
            theme: Theme::default(),
            layout: VerticalList,
            appearance,
        }
    }

    /// Loads the icons of new windows at output `scale`, so files are
    /// never read at show time.
    pub fn prepare(&mut self, windows: &[WindowInfo], scale: f32) {
        let size = self.layout.icon_size(&self.metrics(scale), &self.theme);

        for icon in windows.iter().filter_map(|w| w.icon.as_deref()) {
            self.icons.prepare(icon, size);
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
            icons: &mut self.icons,
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
            show_app: self.appearance.show_app,
        }
    }
}
