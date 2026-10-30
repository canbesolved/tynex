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
    HintConfig, MainConfig, OutputConfig, TaskbarConfig, TaskbarMode,
};
use crate::core::event::Click;
use crate::core::view::View;
use crate::core::window::WindowInfo;
use crate::ui::draw::{self, Grid, Metrics, Paint, Size};
use crate::ui::fit;
pub(crate) use crate::ui::frames::{draw_popup, measured};
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::icons::Icons;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::scroll::Viewport;
use crate::ui::slide::Slide;
use crate::ui::taskbar::icons as strip;
use crate::ui::taskbar::menu;

/// Draws views with the configured look. Built once at start, so fonts
/// and glyph caches stay warm between shows.
pub struct Painter {
    text: Text,
    icons: Icons,
    theme: Theme,
    font_size: u16,
    width: u16,
    max_rows: u16,
    taskbar: TaskbarMode,
    show_icons: bool,
    hint_color: Rgba,
    icon_size: f32,
    border: f32,
    slide: Slide,
    drawn: f32,          // Reveal of the last drawn frame
    screen: Option<u32>, // Shortest output height, physical pixels
    list: Viewport,      // First row of the list on screen
}

impl Painter {
    #[must_use]
    pub fn new(
        text: Text,
        icons: Icons,
        main: &MainConfig,
        output: &OutputConfig,
        hint: &HintConfig,
        taskbar: &TaskbarConfig,
    ) -> Self {
        let color = hint.font_color;

        Self {
            text,
            icons,
            theme: Theme::default(),
            font_size: main.font_size,
            width: main.width,
            max_rows: output.max_rows,
            taskbar: taskbar.mode,
            show_icons: taskbar.titlebar.show_icons,
            hint_color: Rgba(color.r, color.g, color.b, 0xff),
            icon_size: f32::from(taskbar.iconbar.size),
            border: f32::from(taskbar.iconbar.border),
            slide: Slide::default(),
            drawn: 0.0,
            screen: None,
            list: Viewport::default(),
        }
    }

    /// Sets the screen height in physical pixels; the popup fits in it.
    pub fn set_screen(&mut self, height: Option<u32>) {
        self.screen = height;
    }

    /// Scrolls the list by wheel `notches` or `dy` logical pixels.
    pub fn scroll(
        &mut self,
        view: &View<'_>,
        scale: f32,
        notches: i32,
        dy: f64,
    ) -> bool {
        let metrics = self.fitted(view, scale);
        let row_h = Grid::new(&metrics, &self.theme).row_h;
        let rows = dy as f32 * scale / row_h.max(1.0);

        self.list.scroll(view, &metrics, notches, rows)
    }

    /// Forgets the list position; the next show starts at the top.
    pub fn hidden(&mut self) {
        self.list.reset();
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
            if self.taskbar.iconbar() {
                self.icons.prepare(icon, strip);
            }
        }
    }

    /// Returns the popup size at output `scale`.
    #[must_use]
    pub fn size(&self, view: &View<'_>, scale: f32) -> Size {
        measured(view, &self.fitted(view, scale), &self.theme)
    }

    /// Draws `view` on `canvas` at output `scale`.
    pub fn draw(
        &mut self,
        canvas: &mut Canvas<'_>,
        view: &View<'_>,
        scale: f32,
    ) {
        let now = Instant::now();
        let metrics = self.list.place(view, self.fitted(view, scale));
        let grid = Grid::new(&metrics, &self.theme);
        let open = strip::shows(view, &metrics) && view.dropdown.is_some();
        let panel = menu::panel(view, &metrics, &grid, &self.text);

        if strip::shows(view, &metrics) {
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
            menu::draw(&mut paint, &grid, panel, reveal, scale);
        }
    }

    /// Returns what a click at physical `(x, y)` hits.
    #[must_use]
    pub fn hit(&self, view: &View<'_>, scale: f32, x: f32, y: f32) -> Click {
        let metrics = self.fitted(view, scale);
        let grid = Grid::new(&metrics, &self.theme);

        strip::hit(
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
    ///
    /// A slide that has reached its end still wakes once, so the frame
    /// drawn just before the deadline can settle.
    #[must_use]
    pub fn next_frame(&self, now: Instant) -> Option<Instant> {
        if self.needs_frame(now) {
            return Some(self.slide.next(now).unwrap_or(now));
        }
        self.slide.next(now)
    }

    fn metrics(&self, scale: f32) -> Metrics {
        Metrics {
            font_size: f32::from(self.font_size),
            width: f32::from(self.width),
            max_rows: usize::from(self.max_rows),
            list_rows: usize::from(self.max_rows),
            first_row: None,
            scale,
            taskbar: self.taskbar,
            show_icons: self.show_icons,
            hint_color: self.hint_color,
            icon_size: self.icon_size,
            border: self.border,
        }
    }

    /// Returns the metrics for `view`, shrunk to fit the screen.
    fn fitted(&self, view: &View<'_>, scale: f32) -> Metrics {
        fit::fit(view, self.metrics(scale), &self.theme, self.screen)
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
