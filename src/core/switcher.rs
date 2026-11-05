// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Switcher state machine: open, refresh, route keys, close.

use std::collections::HashSet;

use crate::core::app::AppInfo;
use crate::core::event::{Action, Click, Event, Key};
use crate::core::order::{SortBy, order};
use crate::core::places::Places;
use crate::core::rename::Renames;
use crate::core::state::State;
use crate::core::text::sanitize_title;
use crate::core::title::row_title;
use crate::core::traits::Mode;
use crate::core::usage::Usage;
use crate::core::view::{Row, View};
use crate::core::window::WindowInfo;

/// What a hotkey press does while the popup is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    Hide, // Hide the popup
    Last, // Switch to the previous (last used) window
}

/// The switcher, generic over its mode for static dispatch.
pub struct Switcher<M> {
    on_hotkey: HotkeyAction,       // For `Show` while open
    strip_app_name: bool,          // Drop the app name from titles
    sort: SortBy,                  // Row order
    hint: String,                  // Latest hint, including the loading dots
    windows: Vec<WindowInfo>,      // Cached list, MRU first, row titles
    renames: Renames,              // Titles set from the popup
    pinned: Vec<AppInfo>,          // Apps first in the strip, in order
    usage: Usage,                  // Lent to `state` while open
    places: Places,                // Windows and tabs, newest first
    apps: Option<HashSet<String>>, // Installed app ids, if scanned
    state: Option<State>,          // Some while the popup is open
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
            hint: String::new(),
            windows: Vec::new(),
            renames: Renames::default(),
            pinned: Vec::new(),
            usage: Usage::default(),
            places: Places::default(),
            apps: None,
            state: None,
            mode,
        }
    }

    /// Sets the apps that stay first in the icon strip, running or not.
    #[must_use]
    pub fn with_pinned(mut self, pinned: Vec<AppInfo>) -> Self {
        self.pinned = pinned;
        self
    }

    /// Sets the hint drawn while the query is empty.
    pub fn set_hint(&mut self, text: String) {
        if let Some(state) = &mut self.state {
            state.hint.clone_from(&text);
        }
        self.hint = text;
    }

    /// Handles one event and returns what the platform must do.
    pub fn handle(&mut self, event: Event) -> Action {
        match event {
            Event::Windows(list) => self.update(list),
            Event::Show => self.show(),
            Event::Key(key) => self.key(key),
            Event::Click(click) => self.click(click),
            Event::Hover(click) => self.hover(click),
            Event::Hide | Event::Quit => self.close(),
            Event::Tabs(update) => self.tabs(update),
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
        self.renames.apply(&mut self.windows);
        self.places.retain(&self.windows);
        if let Some(front) = self.windows.first() {
            self.places.focused(front);
        }

        let Some(old) = self.state.take() else {
            return Action::None;
        };

        self.state = Some(self.rebuild(old));
        Action::Redraw
    }

    /// Builds a new state from the cached windows, keeping the query,
    /// the selection, the strip and a title being edited from `old`.
    fn rebuild(&mut self, old: State) -> State {
        let selected = old.selected_target().cloned();
        let keep = old.icons.get(old.icon).filter(|_| !old.fresh).cloned();
        let mut state = State::new(self.rows());

        state.query = old.query;
        state.cursor = old.cursor;
        state.hint = old.hint;
        state.tab_lists = old.tab_lists;
        state.usage = old.usage;
        state.fresh = old.fresh;
        self.lend_history(&mut state);
        state.fill_icons(
            (&self.pinned, &self.windows),
            self.opening_place().as_ref(),
            keep.as_ref(),
            old.dropdown,
        );
        self.mode.refresh(&mut state);
        if state.fresh && state.query.is_empty() {
            self.preselect(&mut state);
        } else {
            state.select_target(selected.as_ref());
        }
        state.rename = old
            .rename
            .filter(|r| self.windows.iter().any(|w| w.id == r.id));
        state
    }

    fn show(&mut self) -> Action {
        if self.state.is_some() {
            return self.pressed_again();
        }

        let mut state = State::new(self.rows());

        self.lend_usage(&mut state);
        self.lend_history(&mut state);
        state.hint.clone_from(&self.hint);
        state.fresh = true;
        let previous = self.opening_place();

        state.fill_icons(
            (&self.pinned, &self.windows),
            previous.as_ref(),
            None,
            None,
        );
        self.mode.refresh(&mut state);
        self.preselect(&mut state);
        self.state = Some(state);
        Action::Redraw
    }

    fn key(&mut self, key: Key) -> Action {
        let Some(mut state) = self.state.take() else {
            return Action::None;
        };

        state.fresh = false;
        let action = self.mode.on_key(key, &mut state);

        if let Some((id, title)) = state.renamed.take() {
            self.renames.set(&mut self.windows, &id, &title);
            state = self.rebuild(state);
        }
        self.keep(action, state)
    }

    /// Routes one click to the mode. Same lifetime rules as `key`.
    fn click(&mut self, click: Click) -> Action {
        let Some(mut state) = self.state.take() else {
            return Action::None;
        };

        state.fresh = false;
        let action = self.mode.on_click(click, &mut state);

        self.keep(action, state)
    }

    /// Routes one pointer move to the mode. Same lifetime rules as `key`.
    fn hover(&mut self, click: Click) -> Action {
        let Some(mut state) = self.state.take() else {
            return Action::None;
        };

        state.fresh = false;
        let action = self.mode.on_hover(click, &mut state);

        self.keep(action, state)
    }

    /// Builds the rows in `sort` order.
    fn rows(&self) -> Vec<Row> {
        order(&self.windows, self.sort)
            .into_iter()
            .map(Row::window)
            .collect()
    }
}

#[path = "switcher_history.rs"]
mod history;

#[path = "switcher_usage.rs"]
mod usage;

#[cfg(test)]
#[path = "switcher_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "hotkey_action_tests.rs"]
mod hotkey_action_tests;
