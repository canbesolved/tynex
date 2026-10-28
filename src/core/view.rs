// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! What the presenter must draw. Popup modules read it, never the
//! switcher.

use std::ops::Range;

use crate::core::window::WindowId;

/// What selecting a row does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowTarget {
    Window(WindowId),
    Launch(String), // App id
}

/// One list entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub target: RowTarget,
    pub app_id: String,   // Matched by search, not drawn
    pub app_name: String, // Matched by search, not drawn
    pub icon: Option<String>,
    pub title: String, // Sanitized, app name stripped
    pub highlights: Vec<Range<usize>>, // Byte ranges in `title`
}

/// Read-only snapshot of the popup, borrowed from the switcher state.
#[derive(Debug, Clone, Copy)]
pub struct View<'a> {
    pub query: &'a str,
    pub selected: usize, // Index into the output rows
    windows: &'a [Row],
    output: &'a [Row],
}

impl<'a> View<'a> {
    pub(crate) fn new(
        query: &'a str,
        selected: usize,
        windows: &'a [Row],
        output: &'a [Row],
    ) -> Self {
        Self {
            query,
            selected,
            windows,
            output,
        }
    }

    /// Returns the open windows, in list order. The title bar draws
    /// these. Search does not filter them.
    pub fn windows(&self) -> impl Iterator<Item = &'a Row> + 'a {
        self.windows.iter()
    }

    /// Returns the number of open windows.
    #[must_use]
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Returns the output rows, in display order. The output bar draws
    /// these: matches of the current query.
    pub fn rows(&self) -> impl Iterator<Item = &'a Row> + 'a {
        self.output.iter()
    }

    /// Returns the number of output rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.output.len()
    }

    /// Returns true when there is nothing to draw.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }
}
