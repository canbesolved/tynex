// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Dropdown slide. The poll loop wakes only while this is running.

use std::time::{Duration, Instant};

use crate::ui::taskbar::menu::Panel;

/// How long the dropdown takes to slide.
const SLIDE: Duration = Duration::from_millis(150);

/// One frame at about 120 per second.
const FRAME: Duration = Duration::from_nanos(1_000_000_000 / 120);

/// Slide toward open or closed.
#[derive(Debug)]
pub(crate) struct Slide {
    active: bool,
    shown: bool, // Target: open when true
    origin: f32, // Reveal when this slide started
    start: Instant,
    snap: Option<Panel>,
}

impl Default for Slide {
    fn default() -> Self {
        Self {
            active: false,
            shown: false,
            origin: 0.0,
            start: Instant::now(),
            snap: None,
        }
    }
}

impl Slide {
    /// Aims the slide at `open`, keeping `panel` for the way back.
    pub(crate) fn sync(
        &mut self,
        open: bool,
        panel: Option<Panel>,
        now: Instant,
    ) {
        if let Some(panel) = panel {
            self.snap = Some(panel);
        }
        if open == self.shown {
            return;
        }
        self.origin = self.reveal(now);
        self.start = now;
        self.shown = open;
        self.active = true;
    }

    /// Stops the slide and drops the captured dropdown.
    pub(crate) fn cancel(&mut self) {
        self.active = false;
        self.shown = false;
        self.origin = 0.0;
        self.snap = None;
    }

    /// Ends the slide once `SLIDE` has passed.
    pub(crate) fn finish(&mut self, now: Instant) {
        if self.active && now.saturating_duration_since(self.start) >= SLIDE {
            self.active = false;
            self.origin = f32::from(u8::from(self.shown));
            if !self.shown {
                self.snap = None;
            }
        }
    }

    /// Returns how open the dropdown is, from 0 to 1.
    pub(crate) fn reveal(&self, now: Instant) -> f32 {
        if !self.active {
            return f32::from(u8::from(self.shown));
        }
        let t = now.saturating_duration_since(self.start).as_secs_f32()
            / SLIDE.as_secs_f32();
        let t = t.clamp(0.0, 1.0);
        let eased = t * t * (3.0 - 2.0 * t);
        let target = f32::from(u8::from(self.shown));

        self.origin + (target - self.origin) * eased
    }

    /// Returns the captured dropdown while it is still drawn.
    pub(crate) fn panel(&self) -> Option<&Panel> {
        self.snap.as_ref()
    }

    /// Returns true while frames are still due.
    pub(crate) fn active(&self) -> bool {
        self.active
    }

    /// Returns when the next frame is due.
    pub(crate) fn next(&self, now: Instant) -> Option<Instant> {
        if !self.active {
            return None;
        }
        let end = self.start + SLIDE;

        if now >= end {
            None
        } else {
            Some(end.min(now + FRAME))
        }
    }
}
