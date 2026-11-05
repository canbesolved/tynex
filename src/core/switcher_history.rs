// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The history of windows and tabs on an open popup: lent for the
//! title list, kept up to date by tab lists, and the place the popup
//! opens on (ADR-0097).

use super::Switcher;
use crate::core::browser_tabs::TabsUpdate;
use crate::core::event::Action;
use crate::core::order::{SortBy, order};
use crate::core::places::{Place, Places};
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::core::view::{Row, RowTarget};

impl<M: Mode> Switcher<M> {
    /// Keeps up to `limit` windows and tabs for the title list.
    #[must_use]
    pub fn with_history(mut self, limit: usize) -> Self {
        self.places = Places::new(limit);
        self
    }

    /// Applies a tab update to the history, then to the mode. A new
    /// list or a browser that left rebuilds the open popup, so tabs
    /// take their place in the title list and the strip.
    pub(super) fn tabs(&mut self, update: TabsUpdate) -> Action {
        let rebuild = match &update {
            TabsUpdate::List {
                source, pid, tabs, ..
            } => {
                self.places.listed(*source, *pid, tabs, &self.windows);
                true
            }
            TabsUpdate::Gone(source) => {
                self.places.gone(*source);
                true
            }
            TabsUpdate::Icons | TabsUpdate::Activated { .. } => false,
        };
        let action = self.mode.on_tabs(update, &mut self.state, &self.windows);

        if rebuild
            && action == Action::Redraw
            && let Some(old) = self.state.take()
        {
            self.state = Some(self.rebuild(old));
        }
        action
    }

    /// Builds the rows in `sort` order.
    pub(super) fn rows(&self) -> Vec<Row> {
        order(&self.windows, self.sort)
            .into_iter()
            .map(Row::window)
            .collect()
    }

    /// Lends the history for the title list, unless rows sort by app.
    pub(super) fn lend_history(&self, state: &mut State) {
        state.history = (self.sort == SortBy::Recently)
            .then(|| self.places.list().to_vec());
    }

    /// The place the popup opens on: the previous one, else the
    /// previous window, else the current place.
    pub(super) fn opening_place(&self) -> Option<Place> {
        let list = self.places.list();

        list.get(1)
            .cloned()
            .or_else(|| {
                let window = self.windows.get(1)?;

                Some(Place::Window(window.id.clone()))
            })
            .or_else(|| list.first().cloned())
    }

    /// Selects the previous place in the title list, else the previous
    /// window, like Alt+Tab.
    pub(super) fn preselect(&self, state: &mut State) {
        let target = self.places.list().get(1).map(|place| match place {
            Place::Window(id) => RowTarget::Window(id.clone()),
            Place::Tab { tab, .. } => RowTarget::Tab(tab.clone()),
        });

        match target.filter(|t| state.output.iter().any(|r| r.target == *t)) {
            Some(target) => state.select_target(Some(&target)),
            None => state.select_id(self.windows.get(1).map(|w| &w.id)),
        }
    }
}
