// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! State of an open popup, shared by the switcher and the modes.

use crate::core::event::ModeKind;
use crate::core::view::{Row, View};
use crate::core::window::WindowId;

/// State of an open popup. Modes read and change it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub mode: ModeKind,
    pub query: String,
    pub rows: Vec<Row>,    // All windows, active window last
    pub shown: Vec<usize>, // Indices into `rows`, in display order
    pub selected: usize,   // Index into `shown`
}

impl State {
    /// Creates a state showing all `rows` in order.
    #[must_use]
    pub fn new(mode: ModeKind, rows: Vec<Row>) -> Self {
        let shown = (0..rows.len()).collect();

        Self {
            mode,
            query: String::new(),
            rows,
            shown,
            selected: 0,
        }
    }

    /// Returns a read-only view for the presenter.
    #[must_use]
    pub fn view(&self) -> View<'_> {
        View::new(
            self.mode,
            &self.query,
            self.selected,
            &self.rows,
            &self.shown,
        )
    }

    /// Returns the id of the selected row, if any row is shown.
    #[must_use]
    pub fn selected_id(&self) -> Option<&WindowId> {
        let &i = self.shown.get(self.selected)?;

        Some(&self.rows[i].id)
    }

    /// Returns the id of the shown row with letter `key`.
    #[must_use]
    pub fn id_for_key(&self, key: char) -> Option<&WindowId> {
        self.shown
            .iter()
            .map(|&i| &self.rows[i])
            .find(|row| row.key == Some(key))
            .map(|row| &row.id)
    }

    /// Moves the selection down, wrapping to the first row.
    pub fn select_next(&mut self) {
        if !self.shown.is_empty() {
            self.selected = (self.selected + 1) % self.shown.len();
        }
    }

    /// Moves the selection up, wrapping to the last row.
    pub fn select_prev(&mut self) {
        if !self.shown.is_empty() {
            let len = self.shown.len();

            self.selected = (self.selected + len - 1) % len;
        }
    }

    /// Selects the shown row of window `id`, else clamps the selection.
    pub fn select_id(&mut self, id: Option<&WindowId>) {
        let found = id.and_then(|id| {
            self.shown.iter().position(|&i| &self.rows[i].id == id)
        });

        self.selected = match found {
            Some(pos) => pos,
            None => self.selected.min(self.shown.len().saturating_sub(1)),
        };
    }
}
