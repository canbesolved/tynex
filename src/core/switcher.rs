// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Switcher state machine: open, refresh, route keys, close.

use crate::core::event::{Action, Event, Key};
use crate::core::order::{SortBy, order};
use crate::core::state::State;
use crate::core::text::sanitize_title;
use crate::core::title::row_title;
use crate::core::traits::Mode;
use crate::core::view::{Row, RowTarget, View};
use crate::core::window::WindowInfo;

/// What a hotkey press does while the popup is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    Hide, // Hide the popup
    Last, // Switch to the previous (last used) window
}

/// The switcher, generic over its mode for static dispatch.
pub struct Switcher<M> {
    on_hotkey: HotkeyAction,  // For `Show` while open
    strip_app_name: bool,     // Drop the app name from titles
    sort: SortBy,             // Row order
    windows: Vec<WindowInfo>, // Cached list, MRU first, row titles
    state: Option<State>,     // Some while the popup is open
    mode: M,
}

impl<M: Mode> Switcher<M> {
    /// Creates a hidden switcher with the hotkey action, title rule and
    /// row order from the config.
    pub fn new(
        on_hotkey: HotkeyAction,
        strip_app_name: bool,
        sort: SortBy,
        mode: M,
    ) -> Self {
        Self {
            on_hotkey,
            strip_app_name,
            sort,
            windows: Vec::new(),
            state: None,
            mode,
        }
    }

    /// Handles one event and returns what the platform must do.
    pub fn handle(&mut self, event: Event) -> Action {
        match event {
            Event::Windows(list) => self.update(list),
            Event::Show => self.show(),
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
    /// row title as title.
    #[must_use]
    pub fn windows(&self) -> &[WindowInfo] {
        &self.windows
    }

    fn update(&mut self, list: Vec<WindowInfo>) -> Action {
        self.windows = list
            .into_iter()
            .map(|w| WindowInfo {
                title: row_title(&w, self.strip_app_name),
                app_name: sanitize_title(&w.app_name),
                ..w
            })
            .collect();

        let Some(old) = self.state.take() else {
            return Action::None;
        };
        let selected = old.selected_id().cloned();
        let mut state = State::new(self.rows());

        state.query = old.query;
        self.mode.refresh(&mut state);
        state.select_id(selected.as_ref());
        self.state = Some(state);
        Action::Redraw
    }

    fn show(&mut self) -> Action {
        if self.state.is_some() {
            return self.pressed_again();
        }

        let mut state = State::new(self.rows());

        self.mode.refresh(&mut state);
        // The previous window is selected in any order, like Alt+Tab.
        state.select_id(self.windows.get(1).map(|w| &w.id));
        self.state = Some(state);
        Action::Redraw
    }

    fn key(&mut self, key: Key) -> Action {
        let Some(mut state) = self.state.take() else {
            return Action::None;
        };
        let action = self.mode.on_key(key, &mut state);

        if !matches!(
            action,
            Action::Activate(_) | Action::Launch(_) | Action::Close
        ) {
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

    /// Applies `on_hotkey` to a `Show` while open.
    fn pressed_again(&mut self) -> Action {
        let previous = match self.on_hotkey {
            HotkeyAction::Hide => None,
            HotkeyAction::Last => self.windows.get(1).map(|w| w.id.clone()),
        };

        self.state = None;
        previous.map_or(Action::Close, Action::Activate)
    }

    /// Builds the rows in `sort` order.
    fn rows(&self) -> Vec<Row> {
        order(&self.windows, self.sort)
            .into_iter()
            .map(row)
            .collect()
    }
}

/// Builds a row for `window`.
fn row(window: &WindowInfo) -> Row {
    Row {
        target: RowTarget::Window(window.id.clone()),
        app_id: window.app_id.clone(),
        app_name: window.app_name.clone(),
        icon: window.icon.clone(),
        title: window.title.clone(),
        title_color: None,
        highlights: Vec::new(),
    }
}

#[cfg(test)]
#[path = "switcher_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "hotkey_action_tests.rs"]
mod hotkey_action_tests;
