// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Browser tab events for prompt mode.

use super::PromptMode;
use crate::core::browser_tabs::TabsUpdate;
use crate::core::event::Action;
use crate::core::places::Place;
use crate::core::state::State;
use crate::core::window::WindowInfo;

/// Applies `update` when tab search is on.
pub(super) fn on_tabs(
    mode: &mut PromptMode,
    update: TabsUpdate,
    state: &mut Option<State>,
    windows: &[WindowInfo],
) -> Action {
    let Some(session) = mode.tabs.as_mut() else {
        return Action::None;
    };
    let rows_changed =
        matches!(update, TabsUpdate::List { .. } | TabsUpdate::Gone(_));
    let raise = matches!(update, TabsUpdate::Activated { ok: true, .. });
    let action = session.apply(update, state, windows);

    if raise {
        match &action {
            Action::Activate(id) => {
                log::debug!("browser tabs: raising window {}", id.0);
            }
            _ => log::debug!("browser tabs: no window matched, not raising"),
        }
    }

    match state {
        Some(state) if rows_changed && action == Action::Redraw => {
            let target = state.selected_target().cloned();

            mode.input.refresh(state);
            state.select_target(target.as_ref());
            Action::Redraw
        }
        _ => action,
    }
}

/// Remembers the tab of `action` before the popup state is dropped.
pub(super) fn note(mode: &mut PromptMode, action: &Action, state: &State) {
    if let (Action::ActivateTab(tab), Some(session)) = (action, &mut mode.tabs)
    {
        session.note(tab, state);
    }
}

/// Remembers a tab that `tynex qs` opens while hidden.
pub(super) fn note_hidden(mode: &mut PromptMode, place: &Place) {
    if let (Place::Tab { tab, pid, window }, Some(session)) =
        (place, &mut mode.tabs)
    {
        session.note_hidden(tab, *pid, window.as_ref());
    }
}
