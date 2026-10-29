// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! State of an open popup, shared by the switcher and the modes.

use crate::core::view::{Row, RowTarget, View};
use crate::core::window::WindowId;

/// State of an open popup. Modes read and change it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub query: String,
    pub cursor: usize,    // Byte index in `query`
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
            cursor: 0,
            rows,
            output,
            selected: 0,
        }
    }

    /// Inserts `c` at the cursor and moves the cursor past it.
    pub fn insert(&mut self, c: char) {
        self.query.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    /// Deletes the character before the cursor.
    ///
    /// Returns false when the cursor is already at the start.
    pub fn backspace(&mut self) -> bool {
        let Some(start) = prev_char(&self.query, self.cursor) else {
            return false;
        };

        self.query.drain(start..self.cursor);
        self.cursor = start;
        true
    }

    /// Deletes the character at the cursor.
    ///
    /// Returns false when the cursor is already at the end.
    pub fn delete(&mut self) -> bool {
        let Some(end) = next_char(&self.query, self.cursor) else {
            return false;
        };

        self.query.drain(self.cursor..end);
        true
    }

    /// Moves the cursor one character left.
    pub fn move_left(&mut self) {
        if let Some(i) = prev_char(&self.query, self.cursor) {
            self.cursor = i;
        }
    }

    /// Moves the cursor one character right.
    pub fn move_right(&mut self) {
        if let Some(i) = next_char(&self.query, self.cursor) {
            self.cursor = i;
        }
    }

    /// Moves the cursor to the start of the previous word.
    pub fn move_word_left(&mut self) {
        let head = self.query[..self.cursor].trim_end();

        self.cursor = match head.rfind(char::is_whitespace) {
            Some(i) => i + head[i..].chars().next().map_or(0, char::len_utf8),
            None => 0,
        };
    }

    /// Moves the cursor to the start of the next word.
    pub fn move_word_right(&mut self) {
        let rest = &self.query[self.cursor..];
        let word = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let spaces = rest[word..].len() - rest[word..].trim_start().len();

        self.cursor += word + spaces;
    }

    /// Returns a read-only view for the presenter.
    #[must_use]
    pub fn view(&self) -> View<'_> {
        View::new(
            &self.query,
            self.cursor,
            self.selected,
            &self.rows,
            &self.output,
        )
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

/// Returns the byte index of the character before `index`.
fn prev_char(text: &str, index: usize) -> Option<usize> {
    text[..index].char_indices().next_back().map(|(i, _)| i)
}

/// Returns the byte index after the character at `index`.
fn next_char(text: &str, index: usize) -> Option<usize> {
    text[index..].chars().next().map(|c| index + c.len_utf8())
}
