// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The history of places, windows and browser tabs, newest first. The
//! title list follows it; the switch back uses its first two entries
//! (ADR-0096, ADR-0097). Pure logic.

use crate::core::browser_tabs::{SourceId, TabInfo, TabList, TabRef};
use crate::core::event::Action;
use crate::core::title::shows_tab;
use crate::core::window::{WindowId, WindowInfo};

/// Entries kept when no limit is set: the current and the previous.
const MIN: usize = 2;

/// Where the user was: a window, or a tab of a browser process. A
/// browser window counts as its active tab once its list is known.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Place {
    Window(WindowId),
    Tab {
        tab: TabRef,
        pid: u32,
        window: Option<WindowId>, // The window that shows it, once known
    },
}

impl Place {
    /// True for a tab that window `id` shows.
    fn shown_by(&self, id: &WindowId) -> bool {
        matches!(self, Self::Tab { window: Some(w), .. } if w == id)
    }

    /// True for a tab of browser `source`.
    fn of(&self, source: SourceId) -> bool {
        matches!(self, Self::Tab { tab, .. } if tab.source == source)
    }

    /// True when both are the same window or the same tab.
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Window(a), Self::Window(b)) => a == b,
            (Self::Tab { tab: a, .. }, Self::Tab { tab: b, .. }) => a == b,
            _ => false,
        }
    }
}

/// Places, newest first, at most `limit` of them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Places {
    list: Vec<Place>,
    limit: usize,
}

impl Default for Places {
    fn default() -> Self {
        Self::new(MIN)
    }
}

impl Places {
    /// Keeps at most `limit` places, never fewer than two.
    #[must_use]
    pub fn new(limit: usize) -> Self {
        Self {
            list: Vec::new(),
            limit: limit.max(MIN),
        }
    }

    /// Returns the places, newest first.
    #[must_use]
    pub fn list(&self) -> &[Place] {
        &self.list
    }

    /// Notes the window in front. A window of the newest tab's browser
    /// is that tab, so raising it changes nothing; a window known by a
    /// tab moves that tab to the front.
    pub fn focused(&mut self, front: &WindowInfo) {
        if let Some(Place::Tab { pid, window, .. }) = self.list.first_mut()
            && front.pid == Some(*pid)
            && window.as_ref().is_none_or(|w| *w == front.id)
        {
            *window = Some(front.id.clone());
            return;
        }
        let known =
            self.list.iter().position(|place| place.shown_by(&front.id));

        match known {
            Some(at) => {
                let place = self.list.remove(at);

                self.list.insert(0, place);
            }
            None => self.push(Place::Window(front.id.clone())),
        }
    }

    /// Notes a tab opened from the popup.
    pub fn opened(&mut self, tab: TabRef, pid: u32) {
        let window = self.list.iter().find_map(|place| match place {
            Place::Tab { tab: t, window, .. } if *t == tab => window.clone(),
            _ => None,
        });

        self.push(Place::Tab { tab, pid, window });
    }

    /// Notes a browser's tab list. Closed tabs leave; a window of the
    /// browser that shows an active tab becomes that tab; while a
    /// window of the browser is in front, its active tab is the newest
    /// place, so a tab switched in the browser counts too.
    pub fn listed(
        &mut self,
        source: SourceId,
        pid: u32,
        tabs: &[TabInfo],
        windows: &[WindowInfo],
    ) {
        self.list.retain(|place| match place {
            Place::Tab { tab, .. } if tab.source == source => {
                tabs.iter().any(|t| t.id == tab.id)
            }
            _ => true,
        });
        if pid == 0 {
            return;
        }
        let active = |window: &WindowInfo| {
            tabs.iter()
                .filter(|t| t.active)
                .find(|t| shows_tab(&window.title, &t.title))
        };
        let tab_of = |window: &WindowInfo, tab: &TabInfo| Place::Tab {
            tab: TabRef { source, id: tab.id },
            pid,
            window: Some(window.id.clone()),
        };

        for place in &mut self.list {
            let Place::Window(id) = place else { continue };
            let found = windows
                .iter()
                .find(|w| w.id == *id && w.pid == Some(pid))
                .and_then(|w| active(w).map(|tab| tab_of(w, tab)));

            if let Some(tab) = found {
                *place = tab;
            }
        }
        self.dedup();

        let Some(front) = windows.first().filter(|w| w.pid == Some(pid)) else {
            return;
        };
        let newest = tabs.iter().filter(|t| t.active).max_by_key(|t| t.last);

        if let Some(tab) = active(front).or(newest) {
            self.push(tab_of(front, tab));
        }
    }

    /// Drops the tabs of a browser that went away.
    pub fn gone(&mut self, source: SourceId) {
        self.list.retain(|place| !place.of(source));
    }

    /// Drops windows that closed.
    pub fn retain(&mut self, windows: &[WindowInfo]) {
        self.list.retain(|place| match place {
            Place::Window(id) => windows.iter().any(|w| w.id == *id),
            Place::Tab { .. } => true,
        });
    }

    /// What switches back to the previous place. A tab whose list is
    /// not here yet raises its window, else a window of its browser.
    /// `None` falls back to the previous window.
    #[must_use]
    pub fn back(
        &self,
        windows: &[WindowInfo],
        lists: &[TabList],
    ) -> Option<Action> {
        match self.list.get(1)? {
            Place::Window(id) => windows
                .iter()
                .any(|w| w.id == *id)
                .then(|| Action::Activate(id.clone())),
            Place::Tab { tab, pid, window } => {
                let listed = lists.iter().any(|list| {
                    list.source == tab.source
                        && list.tabs.iter().any(|t| t.id == tab.id)
                });

                if listed {
                    return Some(Action::ActivateTab(tab.clone()));
                }
                windows
                    .iter()
                    .find(|w| Some(&w.id) == window.as_ref())
                    .or_else(|| windows.iter().find(|w| w.pid == Some(*pid)))
                    .map(|w| Action::Activate(w.id.clone()))
            }
        }
    }

    /// Moves `place` to the front and keeps at most `limit` places.
    fn push(&mut self, place: Place) {
        self.list.retain(|have| !have.same(&place));
        self.list.insert(0, place);
        self.list.truncate(self.limit);
    }

    /// Keeps the newest of two entries for one tab.
    fn dedup(&mut self) {
        let mut kept: Vec<Place> = Vec::with_capacity(self.list.len());

        for place in self.list.drain(..) {
            if !kept.iter().any(|have| have.same(&place)) {
                kept.push(place);
            }
        }
        self.list = kept;
    }
}

#[cfg(test)]
#[path = "places_tests.rs"]
mod tests;
