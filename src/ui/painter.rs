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

use std::time::Instant;

use crate::config::{
    HintConfig, IconBarConfig, MainConfig, OutputBarConfig, TitleBarConfig,
};
use crate::core::event::Click;
use crate::core::view::View;
use crate::core::window::WindowInfo;
use crate::ui::draw::{self, Grid, Metrics, Paint, Size};
pub(crate) use crate::ui::frames::{draw_popup, measured};
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::icons::Icons;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::{icon_bar, icon_menu, slide::Slide};

/// Draws views with the configured look. Built once at start, so fonts
/// and glyph caches stay warm between shows.
pub struct Painter {
    text: Text,
    icons: Icons,
    theme: Theme,
    font_size: u16,
    width: u16,
    max_rows: u16,
    title_bar: bool,
    show_icons: bool,
    hint_color: Rgba,
    icon_bar: bool,
    icon_size: f32,
    border: f32,
    slide: Slide,
    drawn: f32, // Reveal of the last drawn frame
}

impl Painter {
    #[must_use]
    pub fn new(
        text: Text,
        icons: Icons,
        main: &MainConfig,
        output: &OutputBarConfig,
        title: &TitleBarConfig,
        hint: &HintConfig,
        icon: &IconBarConfig,
    ) -> Self {
        let color = hint.font_color;

        Self {
            text,
            icons,
            theme: Theme::default(),
            font_size: main.font_size,
            width: main.width,
            max_rows: output.max_rows,
            title_bar: title.enabled,
            show_icons: title.show_icons,
            hint_color: Rgba(color.r, color.g, color.b, 0xff),
            icon_bar: icon.enabled,
            icon_size: f32::from(icon.icon_size),
            border: f32::from(icon.border),
            slide: Slide::default(),
            drawn: 0.0,
        }
    }

    /// Loads the icons of new windows at output `scale`, so files are
    /// never read at show time.
    pub fn prepare(&mut self, windows: &[WindowInfo], scale: f32) {
        self.prepare_icons(
            windows.iter().filter_map(|w| w.icon.as_deref()),
            scale,
        );
    }

    /// Loads named icons at output `scale`.
    pub fn prepare_icons<'a>(
        &mut self,
        icons: impl Iterator<Item = &'a str>,
        scale: f32,
    ) {
        let size = draw::icon_size(&self.metrics(scale), &self.theme);
        let strip = (self.icon_size * scale).round().max(1.0) as u32;

        for icon in icons {
            self.icons.prepare(icon, size);
            if self.icon_bar {
                self.icons.prepare(icon, strip);
            }
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
        let now = Instant::now();
        let metrics = self.metrics(scale);
        let grid = Grid::new(&metrics, &self.theme);
        let open = icon_bar::shows(view, &metrics) && view.dropdown.is_some();
        let panel = icon_menu::panel(view, &metrics, &grid, &self.text);

        if icon_bar::shows(view, &metrics) {
            self.slide.sync(open, panel, now);
        } else {
            self.slide.cancel();
        }
        self.slide.finish(now);

        let reveal = self.slide.reveal(now);

        self.drawn = reveal;

        let mut paint = Paint {
            canvas,
            text: &mut self.text,
            icons: &mut self.icons,
            theme: &self.theme,
        };

        draw_popup(view, &metrics, &mut paint);
        if reveal > 0.0
            && let Some(panel) = self.slide.panel()
        {
            icon_menu::draw(&mut paint, &grid, panel, reveal, scale);
        }
    }

    /// Returns what a click at physical `(x, y)` hits.
    #[must_use]
    pub fn hit(&self, view: &View<'_>, scale: f32, x: f32, y: f32) -> Click {
        let metrics = self.metrics(scale);
        let grid = Grid::new(&metrics, &self.theme);

        icon_bar::hit(
            view,
            &metrics,
            &grid,
            &self.text,
            x,
            y,
            self.slide.reveal(Instant::now()),
        )
    }

    /// Returns true when the dropdown slide needs another frame.
    #[must_use]
    pub fn needs_frame(&self, now: Instant) -> bool {
        self.slide.active()
            && (self.slide.reveal(now) - self.drawn).abs() > 0.000_1
    }

    /// Returns when the next slide frame is due.
    #[must_use]
    pub fn next_frame(&self, now: Instant) -> Option<Instant> {
        self.slide.next(now)
    }

    fn metrics(&self, scale: f32) -> Metrics {
        Metrics {
            font_size: f32::from(self.font_size),
            width: f32::from(self.width),
            max_rows: usize::from(self.max_rows),
            scale,
            title_bar: self.title_bar,
            show_icons: self.show_icons,
            hint_color: self.hint_color,
            icon_bar: self.icon_bar,
            icon_size: self.icon_size,
            border: self.border,
        }
    }
}

#[cfg(test)]
#[path = "painter_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "hint_draw_tests.rs"]
mod hint_draw_tests;

#[cfg(test)]
#[path = "caret_tests.rs"]
mod caret_tests;
