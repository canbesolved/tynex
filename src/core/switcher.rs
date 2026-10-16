// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Switcher state machine: open, refresh, route keys, close.

use std::collections::HashMap;

use crate::core::event::{Action, Event, Key, ModeKind};
use crate::core::keys;
use crate::core::state::State;
use crate::core::text::sanitize_title;
use crate::core::traits::Mode;
use crate::core::view::{Row, View};
use crate::core::window::{WindowId, WindowInfo};

/// The switcher, generic over its two modes for static dispatch.
pub struct Switcher<H, S> {
    keys: String,
    next_key: usize, // Next unused letter in `keys` while open
    windows: Vec<WindowInfo>, // Cached list, most recently used first
    state: Option<State>, // Some while the popup is open
    hotkey: H,
    search: S,
}

impl<H: Mode, S: Mode> Switcher<H, S> {
    /// Creates a hidden switcher with the key string from the config.
    pub fn new(keys: impl Into<String>, hotkey: H, search: S) -> Self {
        let keys = keys.into();

        Self {
            keys,
            next_key: 0,
            windows: Vec::new(),
            state: None,
            hotkey,
            search,
        }
    }

    /// Handles one event and returns what the platform must do.
    pub fn handle(&mut self, event: Event) -> Action {
        match event {
            Event::Windows(list) => self.update(list),
            Event::Show(mode) => self.show(mode),
            Event::Key(key) => self.key(key),
            Event::Hide | Event::Quit => self.close(),
        }
    }

    /// Returns what to draw, or `None` while hidden.
    #[must_use]
    pub fn view(&self) -> Option<View<'_>> {
        self.state.as_ref().map(State::view)
    }

    fn update(&mut self, list: Vec<WindowInfo>) -> Action {
        self.windows = list
            .into_iter()
            .map(|w| WindowInfo {
                title: sanitize_title(&w.title),
                ..w
            })
            .collect();

        let Some(old) = self.state.take() else {
            return Action::None;
        };

        // Keys stay with their windows while open. New windows get the
        // letters after the last one used.
        let kept: HashMap<&WindowId, Option<char>> =
            old.rows.iter().map(|r| (&r.id, r.key)).collect();
        let mut letters = self.keys.chars().skip(self.next_key);
        let mut rows = Vec::with_capacity(self.windows.len());

        for w in active_last(&self.windows) {
            let key = kept.get(&w.id).copied().unwrap_or_else(|| {
                let next = letters.next();

                self.next_key += usize::from(next.is_some());
                next
            });

            rows.push(row(w, key));
        }

        let selected = old.selected_id().cloned();
        let mut state = State::new(old.mode, rows);

        state.query = old.query;
        self.refresh(&mut state);
        state.select_id(selected.as_ref());
        self.state = Some(state);
        Action::Redraw
    }

    fn show(&mut self, mode: ModeKind) -> Action {
        let mut state = if let Some(mut open) = self.state.take() {
            open.mode = mode;
            open
        } else {
            let letters = keys::assign(self.windows.len(), &self.keys);

            self.next_key = self.windows.len().min(self.keys.chars().count());

            let rows = active_last(&self.windows)
                .zip(letters)
                .map(|(w, key)| row(w, key))
                .collect();

            State::new(mode, rows)
        };

        self.refresh(&mut state);
        self.state = Some(state);
        Action::Redraw
    }

    fn key(&mut self, key: Key) -> Action {
        let Some(mut state) = self.state.take() else {
            return Action::None;
        };

        let action = if key == Key::Tab {
            state.mode = match state.mode {
                ModeKind::Hotkey => ModeKind::Search,
                ModeKind::Search => ModeKind::Hotkey,
            };
            self.refresh(&mut state);
            Action::Redraw
        } else {
            let before = state.mode;
            let action = match state.mode {
                ModeKind::Hotkey => self.hotkey.on_key(key, &mut state),
                ModeKind::Search => self.search.on_key(key, &mut state),
            };

            // A mode may switch to the other one, for example with `/`.
            if state.mode != before {
                self.refresh(&mut state);
            }
            action
        };

        if !matches!(action, Action::Activate(_) | Action::Close) {
            self.state = Some(state);
        }
        action
    }

    fn close(&mut self) -> Action {
        match self.state.take() {
            Some(_) => Action::Close,
            None => Action::None,
        }
    }

    fn refresh(&mut self, state: &mut State) {
        match state.mode {
            ModeKind::Hotkey => self.hotkey.refresh(state),
            ModeKind::Search => self.search.refresh(state),
        }
    }
}

/// Yields windows with the active one (first in the list) moved last.
fn active_last(windows: &[WindowInfo]) -> impl Iterator<Item = &WindowInfo> {
    windows.iter().skip(1).chain(windows.first())
}

/// Builds a row for `window` with letter `key`.
fn row(window: &WindowInfo, key: Option<char>) -> Row {
    Row {
        id: window.id.clone(),
        key,
        icon: window.icon.clone(),
        title: window.title.clone(),
        highlights: Vec::new(),
    }
}

#[cfg(test)]
#[path = "switcher_tests.rs"]
mod tests;
