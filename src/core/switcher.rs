// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Switcher state machine: open, refresh, route keys, close.

use crate::core::event::{Action, Event, Key, ModeKind};
use crate::core::keys::Pins;
use crate::core::order::{SortBy, order};
use crate::core::state::State;
use crate::core::text::sanitize_title;
use crate::core::title::TitleFormat;
use crate::core::traits::Mode;
use crate::core::view::{Row, View};
use crate::core::window::WindowInfo;

/// What a hotkey press does while the popup is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    Hide, // Hide the popup
    Last, // Switch to the previous (last used) window
}

/// The switcher, generic over its two modes for static dispatch.
pub struct Switcher<H, S> {
    on_hotkey: HotkeyAction,  // For `Show` while open
    format: TitleFormat,      // Row text of each window
    sort: SortBy,             // Row order
    pins: Pins,               // Letter of each window until it closes
    windows: Vec<WindowInfo>, // Cached list, MRU first, titles formatted
    state: Option<State>,     // Some while the popup is open
    hotkey: H,
    search: S,
}

impl<H: Mode, S: Mode> Switcher<H, S> {
    /// Creates a hidden switcher with the key string, hotkey action,
    /// title format and row order from the config.
    pub fn new(
        keys: &str,
        on_hotkey: HotkeyAction,
        format: TitleFormat,
        sort: SortBy,
        hotkey: H,
        search: S,
    ) -> Self {
        Self {
            on_hotkey,
            format,
            sort,
            pins: Pins::new(keys),
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

    /// Returns the cached windows, most recently used first, with their
    /// row text as title.
    #[must_use]
    pub fn windows(&self) -> &[WindowInfo] {
        &self.windows
    }

    fn update(&mut self, list: Vec<WindowInfo>) -> Action {
        self.windows = list
            .into_iter()
            .map(|w| WindowInfo {
                title: self.format.apply(&w),
                app_name: sanitize_title(&w.app_name),
                ..w
            })
            .collect();
        self.pins.update(&self.windows, self.state.is_some());

        let Some(old) = self.state.take() else {
            return Action::None;
        };
        let selected = old.selected_id().cloned();
        let mut state = State::new(old.mode, self.rows());

        state.query = old.query;
        self.refresh(&mut state);
        state.select_id(selected.as_ref());
        self.state = Some(state);
        Action::Redraw
    }

    fn show(&mut self, mode: ModeKind) -> Action {
        if self.state.is_some() {
            return self.pressed_again();
        }

        let mut state = State::new(mode, self.rows());

        self.refresh(&mut state);
        // The previous window is selected in any order, like Alt+Tab.
        state.select_id(self.windows.get(1).map(|w| &w.id));
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

        if matches!(action, Action::Activate(_) | Action::Close) {
            self.closed();
        } else {
            self.state = Some(state);
        }
        action
    }

    fn close(&mut self) -> Action {
        match self.state.take() {
            Some(_) => {
                self.closed();
                Action::Close
            }
            None => Action::None,
        }
    }

    /// Applies `on_hotkey` to a `Show` while open, whatever the mode.
    fn pressed_again(&mut self) -> Action {
        let previous = match self.on_hotkey {
            HotkeyAction::Hide => None,
            HotkeyAction::Last => self.windows.get(1).map(|w| w.id.clone()),
        };

        self.state = None;
        self.closed();
        previous.map_or(Action::Close, Action::Activate)
    }

    /// Hands out the letters held while the popup was open.
    fn closed(&mut self) {
        self.pins.update(&self.windows, false);
    }

    /// Builds the rows in `sort` order, each with its pinned letter.
    fn rows(&self) -> Vec<Row> {
        order(&self.windows, &self.pins, self.sort)
            .into_iter()
            .map(|w| row(w, self.pins.get(&w.id)))
            .collect()
    }

    fn refresh(&mut self, state: &mut State) {
        match state.mode {
            ModeKind::Hotkey => self.hotkey.refresh(state),
            ModeKind::Search => self.search.refresh(state),
        }
    }
}

/// Builds a row for `window` with letter `key`.
fn row(window: &WindowInfo, key: Option<char>) -> Row {
    Row {
        id: window.id.clone(),
        key,
        app_id: window.app_id.clone(),
        app_name: window.app_name.clone(),
        icon: window.icon.clone(),
        title: window.title.clone(),
        highlights: Vec::new(),
    }
}

#[cfg(test)]
#[path = "switcher_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "hotkey_action_tests.rs"]
mod hotkey_action_tests;

#[cfg(test)]
#[path = "letter_order_tests.rs"]
mod letter_order_tests;
