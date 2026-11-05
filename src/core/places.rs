// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The current and the previous place, a window or a browser tab, for
//! the switch back to the last one (ADR-0096). Pure logic.

use crate::core::browser_tabs::{SourceId, TabInfo, TabList, TabRef};
use crate::core::event::Action;
use crate::core::window::{WindowId, WindowInfo};

/// Where the user was: a window, or a tab of a browser process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Place {
    Window(WindowId),
    Tab { tab: TabRef, pid: u32 },
}

/// The last two places, newest first.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Places {
    current: Option<Place>,
    previous: Option<Place>,
}

impl Places {
    /// Notes the window in front. A window of the current tab's browser
    /// is that tab, so raising it changes nothing.
    pub fn focused(&mut self, front: &WindowInfo) {
        if let Some(Place::Tab { pid, .. }) = &self.current
            && front.pid == Some(*pid)
        {
            return;
        }
        self.push(Place::Window(front.id.clone()));
    }

    /// Notes a tab opened from the popup.
    pub fn opened(&mut self, tab: TabRef, pid: u32) {
        self.push(Place::Tab { tab, pid });
    }

    /// Notes a browser's tab list. While its window is in `front`, its
    /// newest active tab is the current place, so a tab switched in the
    /// browser counts too.
    pub fn listed(
        &mut self,
        source: SourceId,
        pid: u32,
        tabs: &[TabInfo],
        front: Option<&WindowInfo>,
    ) {
        let Some(front) = front.filter(|w| pid != 0 && w.pid == Some(pid))
        else {
            return;
        };
        let Some(active) =
            tabs.iter().filter(|t| t.active).max_by_key(|t| t.last)
        else {
            return;
        };
        let place = Place::Tab {
            tab: TabRef {
                source,
                id: active.id,
            },
            pid,
        };

        match &self.current {
            // The same window, now known by its tab.
            Some(Place::Window(id)) if *id == front.id => {
                self.current = Some(place);
            }
            _ => self.push(place),
        }
    }

    /// What switches back to the previous place. A tab whose list is
    /// not here yet raises its browser's window instead. `None` falls
    /// back to the previous window.
    #[must_use]
    pub fn back(
        &self,
        windows: &[WindowInfo],
        lists: &[TabList],
    ) -> Option<Action> {
        match self.previous.as_ref()? {
            Place::Window(id) => windows
                .iter()
                .any(|w| w.id == *id)
                .then(|| Action::Activate(id.clone())),
            Place::Tab { tab, pid } => {
                let listed = lists.iter().any(|list| {
                    list.source == tab.source
                        && list.tabs.iter().any(|t| t.id == tab.id)
                });

                if listed {
                    Some(Action::ActivateTab(tab.clone()))
                } else {
                    windows
                        .iter()
                        .find(|w| w.pid == Some(*pid))
                        .map(|w| Action::Activate(w.id.clone()))
                }
            }
        }
    }

    fn push(&mut self, place: Place) {
        if self.current.as_ref() != Some(&place) {
            self.previous = self.current.replace(place);
        }
    }
}

#[cfg(test)]
#[path = "places_tests.rs"]
mod tests;
