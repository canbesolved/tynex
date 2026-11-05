// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer on the title list: hover selects, a click activates. Rows
//! are the title list as drawn, the history first (ADR-0097).

use super::PromptMode;
use crate::core::event::Action;
use crate::core::state::State;
use crate::core::view::RowTarget;

/// Selects a title-list row. The output bar is left alone.
pub(super) fn hover_row(
    mode: &PromptMode,
    s: &mut State,
    index: usize,
) -> Action {
    if !list_on(mode, s) || index >= s.titles().len() || index == s.selected {
        return Action::None;
    }
    s.selected = index;
    Action::Redraw
}

/// Activates the title-list row under the pointer.
pub(super) fn click_row(
    mode: &PromptMode,
    s: &mut State,
    index: usize,
) -> Action {
    let Some(row) = s.titles().get(index).filter(|_| list_on(mode, s)) else {
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

/// Returns true while the title list is on screen.
fn list_on(mode: &PromptMode, s: &State) -> bool {
    mode.taskbar.titlebar() && s.query.is_empty()
}
