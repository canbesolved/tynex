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
    HintConfig, MainConfig, OutputConfig, Position, TaskbarConfig,
    Theme as Scheme,
};
use crate::core::event::Click;
use crate::core::view::View;
use crate::core::window::WindowInfo;
use crate::ui::draw::{self, Grid, Look, Paint, Size};
use crate::ui::fit;
pub(crate) use crate::ui::frames::{draw_popup, measured};
use crate::ui::render::canvas::Canvas;
use crate::ui::render::icons::Icons;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::scroll::Viewport;
use crate::ui::slide::Slide;
use crate::ui::taskbar::{icons as strip, menu, titles};

/// Draws views with the configured look. Built once at start, so fonts
/// and glyph caches stay warm between shows.
pub struct Painter {
    text: Text,
    icons: Icons,
    theme: Theme,
    look: Look,
    position: Position,
    slide: Slide,
    drawn: f32,          // Reveal of the last drawn frame
    screen: Option<u32>, // Shortest output height, physical pixels
    anchor: Option<u32>, // Logical height locked while shown
    list: Viewport,      // First row of the list on screen
    menu: menu::Place,   // First row of the open dropdown
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
        Self {
            text,
            icons,
            theme: match main.theme {
                Scheme::Dark => Theme::dark(),
                Scheme::Light => Theme::light(),
            },
            look: Look::new(main, output, hint, taskbar),
            position: main.position,
            slide: Slide::default(),
            drawn: 0.0,
            screen: None,
            anchor: None,
            list: Viewport::default(),
            menu: menu::Place::default(),
        }
    }

    /// Sets the screen height and the locked open height.
    ///
    /// `height` is physical pixels. `anchor` is the logical height from
    /// when the popup opened, or none before the first show.
    pub fn set_screen(&mut self, height: Option<u32>, anchor: Option<u32>) {
        self.screen = height;
        self.anchor = anchor;
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
        self.menu.reset();
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
        let metrics = self.look.metrics(scale);
        let size = draw::icon_size(&metrics, &self.theme);
        let strip = (self.look.icon_size * scale).round().max(1.0) as u32;

        for icon in icons {
            self.icons.prepare(icon, size);
            if self.look.taskbar.iconbar() {
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
        let panel =
            menu::open(&mut self.menu, view, &metrics, &grid, &self.text);

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
        let mut metrics = self.fitted(view, scale);
        metrics.first_row = Some(self.list.first());
        let grid = Grid::new(&metrics, &self.theme);
        let origin = self.menu.first();
        let dropdown = menu::panel(view, &metrics, &grid, &self.text, origin);
        let hit = strip::hit(
            view,
            &metrics,
            &grid,
            x,
            y,
            self.slide.reveal(Instant::now()),
            dropdown.as_ref(),
        );
        if hit == Click::Elsewhere {
            titles::hit(view, &metrics, &grid, x, y)
        } else {
            hit
        }
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

    /// Returns the metrics for `view`, shrunk to fit the screen.
    fn fitted(&self, view: &View<'_>, scale: f32) -> draw::Metrics {
        fit::fit(
            view,
            self.look.metrics(scale),
            &self.theme,
            self.screen,
            self.position,
            self.anchor,
        )
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
