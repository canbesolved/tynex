// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer on the list on screen: the title list while the query is
//! empty, the output rows while one is typed. Hover selects, a click
//! activates. `State::output` holds the rows of that list, the history
//! first in the title list (ADR-0097). Hover can be turned off for
//! each list and for the icon strip with its dropdown; a click still
//! works (ADR-0101, ADR-0102).

use super::PromptMode;
use crate::core::event::Action;
use crate::core::state::State;
use crate::core::view::RowTarget;

/// What hover selects, from the three `select_on_hover` keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hover {
    pub titles: bool, // Title-list rows
    pub icons: bool,  // Icons and their dropdown titles
    pub output: bool, // Output rows
}

impl Default for Hover {
    fn default() -> Self {
        Self {
            titles: true,
            icons: true,
            output: true,
        }
    }
}

impl PromptMode {
    /// Sets what hover selects. Everything by default.
    #[must_use]
    pub fn with_hover(mut self, hover: Hover) -> Self {
        self.hover = hover;
        self
    }
}

/// Selects the row under the pointer in the list on screen.
pub(super) fn hover_row(
    mode: &PromptMode,
    s: &mut State,
    index: usize,
) -> Action {
    let on = if s.query.is_empty() {
        mode.hover.titles
    } else {
        mode.hover.output
    };

    if !on
        || !list_on(mode, s)
        || index >= s.output.len()
        || index == s.selected
    {
        return Action::None;
    }
    s.selected = index;
    Action::Redraw
}

/// Activates the row under the pointer in the list on screen.
pub(super) fn click_row(
    mode: &PromptMode,
    s: &mut State,
    index: usize,
) -> Action {
    let Some(row) = s.output.get(index).filter(|_| list_on(mode, s)) else {
        return Action::None;
    };
    let action = match &row.target {
        RowTarget::Window(id) => Action::Activate(id.clone()),
        RowTarget::Launch(id) => Action::Launch(id.clone()),
        RowTarget::Tab(tab) => Action::ActivateTab(tab.clone()),
    };

    s.selected = index;
    action
}

/// Returns true while a list is on screen: the title list, or the
/// output rows once a query is typed.
fn list_on(mode: &PromptMode, s: &State) -> bool {
    mode.taskbar.titlebar() || !s.query.is_empty()
}
