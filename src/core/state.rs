// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! State of an open popup, shared by the switcher and the modes.

use crate::core::view::{Row, RowTarget, View};
use crate::core::window::WindowId;

/// State of an open popup. Modes read and change it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub query: String,
    pub rows: Vec<Row>,   // Open windows, for the title bar
    pub output: Vec<Row>, // Result list, for the output bar
    pub selected: usize,  // Index into `output`
}

impl State {
    /// Creates a state showing all `rows` in the output list.
    #[must_use]
    pub fn new(rows: Vec<Row>) -> Self {
        let output = rows.clone();

        Self {
            query: String::new(),
            rows,
            output,
            selected: 0,
        }
    }

    /// Returns a read-only view for the presenter.
    #[must_use]
    pub fn view(&self) -> View<'_> {
        View::new(&self.query, self.selected, &self.rows, &self.output)
    }

    /// Returns the target of the selected output row, if any.
    #[must_use]
    pub fn selected_target(&self) -> Option<&RowTarget> {
        self.output.get(self.selected).map(|r| &r.target)
    }

    /// Returns the window id of the selected row, if it is a window.
    #[must_use]
    pub fn selected_id(&self) -> Option<&WindowId> {
        match self.selected_target()? {
            RowTarget::Window(id) => Some(id),
            RowTarget::Launch(_) => None,
        }
    }

    /// Moves the selection down, wrapping to the first row.
    pub fn select_next(&mut self) {
        if !self.output.is_empty() {
            self.selected = (self.selected + 1) % self.output.len();
        }
    }

    /// Moves the selection up, wrapping to the last row.
    pub fn select_prev(&mut self) {
        if !self.output.is_empty() {
            let len = self.output.len();

            self.selected = (self.selected + len - 1) % len;
        }
    }

    /// Selects the output row of window `id`, else clamps the
    /// selection.
    pub fn select_id(&mut self, id: Option<&WindowId>) {
        let found = id.and_then(|id| {
            self.output.iter().position(
                |r| matches!(&r.target, RowTarget::Window(w) if w == id),
            )
        });

        self.selected = match found {
            Some(pos) => pos,
            None => self.selected.min(self.output.len().saturating_sub(1)),
        };
    }
}
