// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Letter keys for windows, pinned until the window closes (ADR-0041).

use std::collections::{HashMap, HashSet};

use crate::core::window::{WindowId, WindowInfo};

/// Default key order: home row first, then `g` and the top row.
pub const DEFAULT_KEYS: &str = "asdfhjklgqwertyuiop";

/// Letters pinned to windows. Windows beyond the key string get none and
/// are reachable with arrows or search.
#[derive(Debug, Clone, Default)]
pub struct Pins {
    keys: Vec<char>, // Assignment order, unique (checked by the config)
    pinned: HashMap<WindowId, char>,
    held: Vec<char>, // Freed while the popup is open
}

impl Pins {
    /// Creates pins that hand out letters from `keys`, in order.
    #[must_use]
    pub fn new(keys: &str) -> Self {
        Self {
            keys: keys.chars().collect(),
            ..Self::default()
        }
    }

    /// Returns the letter of window `id`, if it has one.
    #[must_use]
    pub fn get(&self, id: &WindowId) -> Option<char> {
        self.pinned.get(id).copied()
    }

    /// Returns the position of window `id`'s letter in `keys`.
    #[must_use]
    pub fn rank(&self, id: &WindowId) -> Option<usize> {
        let key = self.get(id)?;

        self.keys.iter().position(|&k| k == key)
    }

    /// Follows a new window list, most recently used first.
    ///
    /// Closed windows free their letters. While `open`, freed letters are
    /// held until the popup closes, so a letter on screen never moves to
    /// another window. Windows without a letter take free letters in
    /// `keys` order: most recent first, except on the first assignment,
    /// where the active window comes last so the previous one gets the
    /// first letter.
    pub fn update(&mut self, windows: &[WindowInfo], open: bool) {
        let ids: HashSet<&WindowId> = windows.iter().map(|w| &w.id).collect();
        let held = &mut self.held;

        self.pinned.retain(|id, key| {
            let alive = ids.contains(id);

            if !alive && open {
                held.push(*key);
            }
            alive
        });
        if !open {
            held.clear();
        }

        let used: HashSet<char> =
            self.pinned.values().chain(held.iter()).copied().collect();
        let mut free = self.keys.iter().filter(|k| !used.contains(k));
        let mut order: Vec<&WindowInfo> = windows.iter().collect();

        if self.pinned.is_empty() && !order.is_empty() {
            order.rotate_left(1); // Active window last
        }
        for w in order {
            if self.pinned.contains_key(&w.id) {
                continue;
            }

            let Some(&key) = free.next() else { break };

            self.pinned.insert(w.id.clone(), key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn windows(ids: &[&str]) -> Vec<WindowInfo> {
        ids.iter()
            .map(|id| WindowInfo {
                id: WindowId((*id).into()),
                app_id: String::new(),
                app_name: String::new(),
                title: String::new(),
                icon: None,
            })
            .collect()
    }

    /// Letters of `ids`, in that order.
    fn letters(pins: &Pins, ids: &[&str]) -> Vec<Option<char>> {
        ids.iter()
            .map(|id| pins.get(&WindowId((*id).into())))
            .collect()
    }

    #[test]
    fn first_assignment_gives_the_previous_window_the_first_letter() {
        let mut pins = Pins::new(DEFAULT_KEYS);

        pins.update(&windows(&["active", "prev", "older"]), false);
        assert_eq!(
            letters(&pins, &["prev", "older", "active"]),
            [Some('a'), Some('s'), Some('d')]
        );
    }

    #[test]
    fn letters_survive_switching() {
        let mut pins = Pins::new("asd");

        pins.update(&windows(&["x", "y", "z"]), false);
        pins.update(&windows(&["z", "x", "y"]), false);
        assert_eq!(
            letters(&pins, &["y", "z", "x"]),
            [Some('a'), Some('s'), Some('d')]
        );
    }

    #[test]
    fn new_window_gets_the_first_free_letter() {
        let mut pins = Pins::new("asdf");

        pins.update(&windows(&["x", "y", "z"]), false); // y a, z s, x d
        pins.update(&windows(&["x", "z"]), false); // y frees a
        pins.update(&windows(&["new", "x", "z"]), false);
        assert_eq!(letters(&pins, &["new"]), [Some('a')]);
    }

    #[test]
    fn freed_letter_goes_to_the_most_recent_window_without_one() {
        let mut pins = Pins::new("as");

        pins.update(&windows(&["x", "y", "z", "w"]), false); // y a, z s
        pins.update(&windows(&["w", "x", "y"]), false); // z frees s
        assert_eq!(
            letters(&pins, &["w", "x", "y"]),
            [Some('s'), None, Some('a')]
        );
    }

    #[test]
    fn letters_freed_while_open_wait_for_close() {
        let mut pins = Pins::new("asd");

        pins.update(&windows(&["x", "y"]), false); // y a, x s
        pins.update(&windows(&["new", "x"]), true); // y frees a, held
        assert_eq!(letters(&pins, &["new", "x"]), [Some('d'), Some('s')]);

        pins.update(&windows(&["new2", "new", "x"]), true);
        assert_eq!(letters(&pins, &["new2"]), [None]);

        pins.update(&windows(&["new2", "new", "x"]), false); // Closed
        assert_eq!(letters(&pins, &["new2"]), [Some('a')]);
    }

    #[test]
    fn no_windows_or_no_keys() {
        let mut pins = Pins::new("");

        pins.update(&windows(&[]), false);
        pins.update(&windows(&["x"]), false);
        assert_eq!(letters(&pins, &["x"]), [None]);
    }
}
