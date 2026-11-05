// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Usage scores across popups: lent to the state on show, taken back
//! on close with the opened window, app or tab counted.

use std::collections::HashSet;

use super::{HotkeyAction, Switcher};
use crate::core::event::Action;
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::core::usage::{self, Usage};
use crate::core::view::RowTarget;

impl<M: Mode> Switcher<M> {
    /// Sets the usage scores and the installed app ids, `None` when
    /// they were not scanned: then only apps with a window are known.
    #[must_use]
    pub fn with_usage(
        mut self,
        usage: Usage,
        apps: Option<HashSet<String>>,
    ) -> Self {
        self.usage = usage;
        self.apps = apps;
        self
    }

    /// Lends the scores to a new popup, without apps that are gone: no
    /// window and no desktop file.
    pub(super) fn lend_usage(&mut self, state: &mut State) {
        let (windows, apps) = (&self.windows, &self.apps);

        self.usage.retain_apps(|id| {
            windows.iter().any(|w| w.app_id == id)
                || apps.as_ref().is_none_or(|apps| apps.contains(id))
        });
        state.usage = std::mem::take(&mut self.usage);
    }

    /// Returns the usage file text once after a change, while hidden.
    pub fn usage_to_save(&mut self) -> Option<String> {
        (self.state.is_none() && self.usage.take_changed())
            .then(|| self.usage.to_text())
    }

    /// Keeps `state` unless the popup is closing.
    pub(super) fn keep(&mut self, action: Action, state: State) -> Action {
        if matches!(
            action,
            Action::Activate(_)
                | Action::Launch(_)
                | Action::Close
                | Action::ActivateTab(_)
        ) {
            self.take_back(state, &action);
        } else {
            self.state = Some(state);
        }
        action
    }

    pub(super) fn close(&mut self) -> Action {
        match self.state.take() {
            Some(state) => {
                self.take_back(state, &Action::Close);
                Action::Close
            }
            None => Action::None,
        }
    }

    /// Applies `on_hotkey` to a `Show` while open.
    pub(super) fn pressed_again(&mut self) -> Action {
        let previous = match self.on_hotkey {
            HotkeyAction::Hide => None,
            HotkeyAction::Last => self
                .state
                .as_ref()
                .and_then(|s| self.places.back(&self.windows, &s.tab_lists))
                .or_else(|| {
                    let window = self.windows.get(1)?;

                    Some(Action::Activate(window.id.clone()))
                }),
        };
        let action = previous.unwrap_or(Action::Close);

        if let Some(state) = self.state.take() {
            self.take_back(state, &action);
        }
        action
    }

    /// Takes the scores back from a closing popup and counts the use
    /// in `action`.
    fn take_back(&mut self, mut state: State, action: &Action) {
        self.mode.on_close(action, &state);
        if let Action::ActivateTab(tab) = action
            && let Some(list) =
                state.tab_lists.iter().find(|l| l.source == tab.source)
        {
            self.places.opened(tab.clone(), list.pid);
        }

        let target = match action {
            Action::Activate(id) => Some(RowTarget::Window(id.clone())),
            Action::Launch(id) => Some(RowTarget::Launch(id.clone())),
            Action::ActivateTab(tab) => Some(RowTarget::Tab(tab.clone())),
            _ => None,
        };

        if let Some(item) = target.and_then(|target| {
            usage::item(&target, &state.rows, &state.tab_lists)
        }) {
            state.usage.record(item);
        }
        self.usage = std::mem::take(&mut state.usage);
    }
}

#[cfg(test)]
#[path = "switcher_usage_tests.rs"]
mod tests;
