// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon selection and the dropdown, on an open popup.

use crate::core::icons;
use crate::core::state::State;
use crate::core::view::Row;

impl State {
    /// Selects the icon of `app_id`, or the first icon.
    pub fn select_icon(&mut self, app_id: Option<&str>) {
        let found = app_id.and_then(|id| {
            self.icons.iter().position(|icon| icon.app_id == id)
        });
        let last = self.icons.len().saturating_sub(1);

        self.icon = found.unwrap_or(0).min(last);
    }

    /// Moves to the next icon, wrapping.
    pub fn icon_next(&mut self) {
        if !self.icons.is_empty() {
            self.icon = (self.icon + 1) % self.icons.len();
        }
    }

    /// Moves to the previous icon, wrapping.
    pub fn icon_prev(&mut self) {
        if !self.icons.is_empty() {
            let len = self.icons.len();

            self.icon = (self.icon + len - 1) % len;
        }
    }

    /// Opens the dropdown on its first title.
    pub fn open_dropdown(&mut self) {
        self.dropdown = Some(0);
    }

    /// Closes the dropdown.
    pub fn close_dropdown(&mut self) {
        self.dropdown = None;
    }

    /// Returns how many dropdown titles fit in `cap`.
    #[must_use]
    pub fn menu_len(&self, cap: usize) -> usize {
        self.icons
            .get(self.icon)
            .map_or(0, |icon| icon.windows.len().min(cap.max(1)))
    }

    /// Moves the dropdown selection by one title, wrapping.
    pub fn step_dropdown(&mut self, cap: usize, forward: bool) {
        let Some(index) = self.dropdown else {
            return;
        };
        let len = self.menu_len(cap);

        if len == 0 {
            return;
        }
        self.dropdown = Some(if forward {
            (index + 1) % len
        } else {
            (index + len - 1) % len
        });
    }

    /// Drops a dropdown that no longer has several titles.
    ///
    /// `cap` is the most titles the menu shows.
    pub fn settle_dropdown(&mut self, cap: usize) {
        let len = self.menu_len(cap);

        match self.dropdown {
            Some(index) if len > 1 => {
                if index >= len {
                    self.dropdown = Some(len - 1);
                }
            }
            Some(_) => self.dropdown = None,
            None => {}
        }
    }

    /// Returns the windows of the selected icon, alphabetical.
    #[must_use]
    pub fn menu(&self) -> Vec<&Row> {
        self.icons
            .get(self.icon)
            .map(|icon| icons::ordered(icon, &self.rows))
            .unwrap_or_default()
    }
}
