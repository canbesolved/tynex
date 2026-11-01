// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Favicon store methods, kept out of `painter.rs` for the line limit.

use super::Painter;
use super::draw;
use crate::core::browser_tabs::SourceId;
use crate::core::view::View;
use crate::ui::fit;

impl Painter {
    /// Keeps a decoded favicon. False when it was rejected.
    pub fn store_tab_icon(
        &mut self,
        source: SourceId,
        key: &str,
        png: &[u8],
    ) -> bool {
        self.tabs.insert(source, key, png)
    }

    /// Drops the favicons of a browser that disconnected.
    pub fn drop_tab_icons(&mut self, source: SourceId) {
        self.tabs.drop_source(source);
    }

    pub(super) fn fitted(&self, view: &View<'_>, scale: f32) -> draw::Metrics {
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
