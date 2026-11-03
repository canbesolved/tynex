// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! State of an open popup, shared by the switcher and the modes.

use crate::core::browser_tabs::TabList;
use crate::core::rename::Rename;
use crate::core::text_edit;
use crate::core::view::{AppIcon, Edit, Row, RowTarget, Strip, View};
use crate::core::window::WindowId;

/// State of an open popup. Modes read and change it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub query: String,
    pub hint: String,     // Input-bar hint, hidden while typing
    pub cursor: usize,    // Byte index in `query`
    pub rows: Vec<Row>,   // Open windows, for the title bar
    pub output: Vec<Row>, // Result list, for the output bar
    pub selected: usize,  // Index into `output`
    pub icons: Vec<AppIcon>, // Running apps, most recent first
    pub icon: usize,      // Index into `icons`
    pub dropdown: Option<usize>, // Selected title while open
    pub tab_lists: Vec<TabList>, // Empty unless the popup is open
    pub rename: Option<Rename>, // Title being edited
    pub renamed: Option<(WindowId, String)>, // Kept title, for the switcher
}

impl State {
    /// Creates a state showing all `rows` in the output list.
    #[must_use]
    pub fn new(rows: Vec<Row>) -> Self {
        let output = rows.clone();

        Self {
            query: String::new(),
            hint: String::new(),
            cursor: 0,
            rows,
            output,
            selected: 0,
            icons: Vec::new(),
            icon: 0,
            dropdown: None,
            tab_lists: Vec::new(),
            rename: None,
            renamed: None,
        }
    }

    /// Inserts `c` at the cursor and moves the cursor past it.
    pub fn insert(&mut self, c: char) {
        text_edit::insert(&mut self.query, &mut self.cursor, c);
    }

    /// Deletes the character before the cursor.
    ///
    /// Returns false when the cursor is already at the start.
    pub fn backspace(&mut self) -> bool {
        text_edit::backspace(&mut self.query, &mut self.cursor)
    }

    /// Deletes the character at the cursor.
    ///
    /// Returns false when the cursor is already at the end.
    pub fn delete(&mut self) -> bool {
        text_edit::delete(&mut self.query, self.cursor)
    }

    /// Moves the cursor one character left.
    pub fn move_left(&mut self) {
        self.cursor = text_edit::left(&self.query, self.cursor);
    }

    /// Moves the cursor one character right.
    pub fn move_right(&mut self) {
        self.cursor = text_edit::right(&self.query, self.cursor);
    }

    /// Moves the cursor to the start of the previous word.
    pub fn move_word_left(&mut self) {
        self.cursor = text_edit::word_left(&self.query, self.cursor);
    }

    /// Moves the cursor to the start of the next word.
    pub fn move_word_right(&mut self) {
        self.cursor = text_edit::word_right(&self.query, self.cursor);
    }

    /// Returns a read-only view for the presenter.
    #[must_use]
    pub fn view(&self) -> View<'_> {
        View::new(
            &self.query,
            &self.hint,
            self.cursor,
            self.selected,
            &self.rows,
            &self.output,
            &Strip {
                icon: self.icon,
                dropdown: self.dropdown,
                icons: &self.icons,
            },
        )
        .with_edit(self.rename.as_ref().map(|r| Edit {
            id: &r.id,
            text: &r.text,
            cursor: r.cursor,
        }))
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
            RowTarget::Launch(_) | RowTarget::Tab(_) => None,
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

    /// Selects the output row with `target`, else clamps.
    pub fn select_target(&mut self, target: Option<&RowTarget>) {
        let found = target.and_then(|target| {
            self.output.iter().position(|row| &row.target == target)
        });

        self.selected = match found {
            Some(pos) => pos,
            None => self.selected.min(self.output.len().saturating_sub(1)),
        };
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
