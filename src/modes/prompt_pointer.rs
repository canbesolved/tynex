// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer on the title list: hover selects, a click activates.

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
    if !list_on(mode, s) || index >= s.rows.len() || index == s.selected {
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
    if !list_on(mode, s) || index >= s.rows.len() {
        return Action::None;
    }
    s.selected = index;
    match &s.rows[index].target {
        RowTarget::Window(id) => Action::Activate(id.clone()),
        RowTarget::Launch(id) => Action::Launch(id.clone()),
        RowTarget::Tab(_) => Action::None,
    }
}

/// Returns true while the title list is on screen.
fn list_on(mode: &PromptMode, s: &State) -> bool {
    mode.taskbar.titlebar() && s.query.is_empty()
}
