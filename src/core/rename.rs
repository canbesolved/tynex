// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Renaming a window from the popup. The new title is tynex's own: it
//! replaces the window title in rows and search until the window
//! closes. The window itself is not changed.

use std::collections::HashMap;

use crate::core::event::Key;
use crate::core::state::State;
use crate::core::text::sanitize_title;
use crate::core::text_edit;
use crate::core::view::RowTarget;
use crate::core::window::{WindowId, WindowInfo};

/// A title being edited in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename {
    pub id: WindowId,
    pub text: String,     // Edited title
    pub cursor: usize,    // Byte index in `text`
    pub original: String, // Title when editing started
}

impl Rename {
    /// Starts editing `title` of window `id`, with the cursor at the end.
    #[must_use]
    pub fn new(id: WindowId, title: &str) -> Self {
        Self {
            id,
            text: title.to_owned(),
            cursor: title.len(),
            original: title.to_owned(),
        }
    }

    /// Applies an editing key. Returns `None` when `key` does not edit;
    /// otherwise whether the text changed.
    pub fn edit(&mut self, key: Key) -> Option<bool> {
        text_edit::apply(&mut self.text, &mut self.cursor, key)
    }
}

impl State {
    /// Starts editing the title of window `id`, if a row shows it. The
    /// rows keep their titles; the view shows the edited text instead.
    pub fn start_rename(&mut self, id: &WindowId) -> bool {
        let Some(row) = self.rows.iter().find(|row| is(&row.target, id)) else {
            return false;
        };

        self.rename = Some(Rename::new(id.clone(), &row.title));
        true
    }

    /// Stops editing and keeps the title from before.
    pub fn cancel_rename(&mut self) {
        self.rename = None;
    }

    /// Stops editing and hands the new title to the switcher.
    pub fn keep_rename(&mut self) {
        if let Some(rename) = self.rename.take() {
            self.renamed = Some((rename.id, rename.text));
        }
    }
}

/// A title set from the popup, and the window's own title under it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Renamed {
    title: String,
    real: String,
}

/// Renamed windows. An entry lasts until its window closes.
#[derive(Debug, Default)]
pub struct Renames(HashMap<WindowId, Renamed>);

impl Renames {
    /// Applies the renames to a fresh window list and forgets the
    /// windows that closed.
    pub fn apply(&mut self, windows: &mut [WindowInfo]) {
        self.0.retain(|id, _| windows.iter().any(|w| &w.id == id));
        for window in windows {
            if let Some(renamed) = self.0.get_mut(&window.id) {
                renamed.real =
                    std::mem::replace(&mut window.title, renamed.title.clone());
            }
        }
    }

    /// Renames window `id` to `title`. A blank title, or the window's
    /// own title, removes the rename.
    pub fn set(
        &mut self,
        windows: &mut [WindowInfo],
        id: &WindowId,
        title: &str,
    ) {
        let Some(window) = windows.iter_mut().find(|w| &w.id == id) else {
            return;
        };
        let real = self
            .0
            .remove(id)
            .map_or_else(|| window.title.clone(), |renamed| renamed.real);
        let title = sanitize_title(title.trim());

        if title.is_empty() || title == real {
            window.title = real;
            return;
        }
        window.title.clone_from(&title);
        self.0.insert(id.clone(), Renamed { title, real });
    }
}

/// Returns true when `target` is window `id`.
fn is(target: &RowTarget, id: &WindowId) -> bool {
    matches!(target, RowTarget::Window(w) if w == id)
}

#[cfg(test)]
#[path = "rename_tests.rs"]
mod tests;
