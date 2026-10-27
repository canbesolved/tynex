// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! What the presenter must draw. Layouts read it, never the switcher.

use std::ops::Range;

use crate::core::window::WindowId;

/// One list entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: WindowId,
    pub app_id: String,   // Matched by search, not drawn
    pub app_name: String, // Drawn with `show_app`, matched by search
    pub icon: Option<String>,
    pub title: String, // Sanitized, app name stripped
    pub highlights: Vec<Range<usize>>, // Byte ranges in `title`
}

/// Read-only snapshot of the popup, borrowed from the switcher state.
#[derive(Debug, Clone, Copy)]
pub struct View<'a> {
    pub query: &'a str,
    pub selected: usize, // Index into the shown rows
    rows: &'a [Row],
    shown: &'a [usize],
}

impl<'a> View<'a> {
    pub(crate) fn new(
        query: &'a str,
        selected: usize,
        rows: &'a [Row],
        shown: &'a [usize],
    ) -> Self {
        Self {
            query,
            selected,
            rows,
            shown,
        }
    }

    /// Returns the rows to draw, in display order.
    pub fn rows(&self) -> impl Iterator<Item = &'a Row> + 'a {
        let rows = self.rows;

        self.shown.iter().map(move |&i| &rows[i])
    }

    /// Returns the number of rows to draw.
    #[must_use]
    pub fn len(&self) -> usize {
        self.shown.len()
    }

    /// Returns true when there is nothing to draw.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shown.is_empty()
    }
}
