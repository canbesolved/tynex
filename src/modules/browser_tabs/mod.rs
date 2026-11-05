// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Browser tabs module: tab lists, rows and the pending activation.
//! Pure logic. The adapter moves the bytes.

mod list;
mod pick;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "hidden_tests.rs"]
mod hidden_tests;

pub use list::{clean, fallback, row_title, rows, tab_row};
pub use pick::browser_window;

use crate::core::browser_tabs::{SourceId, TabInfo, TabRef, TabsUpdate};
use crate::core::event::Action;
use crate::core::state::State;
use crate::core::window::{WindowId, WindowInfo};

/// One activation waiting for the browser's reply.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pending {
    source: SourceId,
    pid: u32,
    id: i32,
    tabs: Vec<TabInfo>, // The source's list, for the window pick
    window: Option<WindowId>, // Raised first, for `tynex qs`
}

/// Tab state that outlives one popup: the pending activation.
/// Lists live on `State` and die with the popup.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Session {
    pending: Option<Pending>,
}

impl Session {
    /// Applies one update. `state` is `None` while the popup is hidden.
    /// `Redraw` means the rows changed and the caller refreshes them.
    pub fn apply(
        &mut self,
        update: TabsUpdate,
        state: &mut Option<State>,
        windows: &[WindowInfo],
    ) -> Action {
        match update {
            TabsUpdate::List {
                source,
                pid,
                browser,
                tabs,
            } => store(state, windows, source, pid, browser, tabs),
            TabsUpdate::Gone(source) => self.gone(state, source),
            TabsUpdate::Icons if state.is_some() => Action::Redraw,
            TabsUpdate::Icons => Action::None,
            TabsUpdate::Activated { source, ok, window } => self
                .activated(windows, source, ok, window)
                .map_or(Action::None, Action::Activate),
        }
    }

    /// Remembers the tab before the popup state is dropped.
    pub fn note(&mut self, tab: &TabRef, state: &State) {
        self.pending = state
            .tab_lists
            .iter()
            .find(|list| list.source == tab.source)
            .map(|list| Pending {
                source: tab.source,
                pid: list.pid,
                id: tab.id,
                tabs: list.tabs.clone(),
                window: None,
            });
    }

    /// Remembers a tab opened with no popup, so no list: the reply
    /// raises `window`, else a window of browser process `pid`.
    pub fn note_hidden(
        &mut self,
        tab: &TabRef,
        pid: u32,
        window: Option<&WindowId>,
    ) {
        self.pending = Some(Pending {
            source: tab.source,
            pid,
            id: tab.id,
            tabs: Vec::new(),
            window: window.cloned(),
        });
    }

    /// Drops one source and its pending activation.
    fn gone(&mut self, state: &mut Option<State>, source: SourceId) -> Action {
        if self.pending.as_ref().is_some_and(|p| p.source == source) {
            self.pending = None;
        }
        let Some(state) = state else {
            return Action::None;
        };
        let before = state.tab_lists.len();

        state.tab_lists.retain(|list| list.source != source);
        if state.tab_lists.len() == before {
            Action::None
        } else {
            Action::Redraw
        }
    }

    /// Picks the window to raise. A miss or a failure raises nothing.
    fn activated(
        &mut self,
        windows: &[WindowInfo],
        source: SourceId,
        ok: bool,
        window: i32,
    ) -> Option<WindowId> {
        let pending = self.pending.take_if(|p| p.source == source)?;

        if !ok {
            return None; // The tab is gone; raise nothing
        }
        if let Some(id) = pending
            .window
            .filter(|id| windows.iter().any(|w| w.id == *id))
        {
            return Some(id);
        }
        browser_window(windows, pending.pid, &pending.tabs, pending.id, window)
    }
}

/// Stores the tabs of one source while the popup is open, and drops
/// the usage scores of that browser's closed tabs.
fn store(
    state: &mut Option<State>,
    windows: &[WindowInfo],
    source: SourceId,
    pid: u32,
    browser: String,
    tabs: Vec<TabInfo>,
) -> Action {
    let Some(state) = state else {
        return Action::None;
    };
    let list = crate::core::browser_tabs::TabList {
        source,
        pid,
        icon: fallback(windows, pid, &browser),
        browser,
        tabs: clean(tabs),
        windows: windows
            .iter()
            .filter(|w| pid != 0 && w.pid == Some(pid))
            .map(|w| w.id.clone())
            .collect(),
    };

    let browser = list.browser.clone();

    match state.tab_lists.iter_mut().find(|l| l.source == source) {
        Some(slot) => *slot = list,
        None => state.tab_lists.push(list),
    }
    // Only a connected browser's list can tell that a tab was closed.
    state.usage.retain_tabs(&browser, &state.tab_lists);
    Action::Redraw
}
