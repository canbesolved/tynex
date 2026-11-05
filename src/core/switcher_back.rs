// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Back to the previous window or tab: a second hotkey press with
//! `last`, and `tynex qs` with the popup open or hidden (ADR-0096,
//! ADR-0098).

use super::Switcher;
use crate::core::event::Action;
use crate::core::places::Place;
use crate::core::traits::Mode;
use crate::core::usage::Item;

impl<M: Mode> Switcher<M> {
    /// `tynex qs`: back to the previous place, popup or not.
    pub(super) fn quick_switch(&mut self) -> Action {
        if self.state.is_some() {
            self.back()
        } else {
            self.back_hidden()
        }
    }

    /// Switches back from the open popup, or closes it when there is
    /// no previous place.
    pub(super) fn back(&mut self) -> Action {
        let action = self
            .state
            .as_ref()
            .and_then(|s| self.places.back(&self.windows, &s.tab_lists))
            .or_else(|| self.previous_window())
            .unwrap_or(Action::Close);

        if let Some(state) = self.state.take() {
            self.take_back(state, &action);
        }
        action
    }

    /// Switches back with no popup. No tab lists are asked for, so the
    /// previous tab is trusted to be open; its browser says when it is
    /// not, and then nothing is raised.
    fn back_hidden(&mut self) -> Action {
        let place = self.places.list().get(1).cloned();

        if let Some(place) = &place {
            self.mode.on_quick_switch(place);
        }
        let action = match place {
            Some(Place::Tab { tab, pid, .. }) => {
                self.places.opened(tab.clone(), pid);
                Some(Action::ActivateTab(tab))
            }
            Some(Place::Window(id))
                if self.windows.iter().any(|w| w.id == id) =>
            {
                Some(Action::Activate(id))
            }
            _ => self.previous_window(),
        };

        if let Some(Action::Activate(id)) = &action
            && let Some(window) = self.windows.iter().find(|w| w.id == *id)
        {
            self.usage.record(Item::App(window.app_id.clone()));
        }
        action.unwrap_or(Action::None)
    }

    /// Activates the window used before the current one.
    fn previous_window(&self) -> Option<Action> {
        let window = self.windows.get(1)?;

        Some(Action::Activate(window.id.clone()))
    }
}

#[cfg(test)]
#[path = "switcher_back_tests.rs"]
mod tests;
