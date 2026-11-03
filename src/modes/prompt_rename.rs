// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Renaming the selected window: the rename key edits its title in
//! place, `Enter` keeps the new title, `Esc` cancels.

use super::PromptMode;
use crate::core::event::{Action, Key};
use crate::core::hotkey;
use crate::core::state::State;
use crate::core::view::RowTarget;
use crate::core::window::WindowId;

/// Handles `key` while a title is edited, or the rename key. `None`
/// passes the key on.
pub(super) fn key(
    mode: &PromptMode,
    key: Key,
    s: &mut State,
) -> Option<Action> {
    if s.rename.is_some() {
        return Some(edit(key, s));
    }
    if !mode.rename.matches(key) {
        return None;
    }
    // The rename key never reaches the query, even with nothing to
    // rename.
    match target(mode, s) {
        Some(id) if s.start_rename(&id) => Some(Action::Redraw),
        _ => Some(Action::None),
    }
}

/// Edits the title: characters, deletes and cursor moves. Other keys
/// do nothing until `Enter` or `Esc`.
fn edit(key: Key, s: &mut State) -> Action {
    match hotkey::plain(key) {
        Some(Key::Enter) => s.keep_rename(),
        Some(Key::Esc) => s.cancel_rename(),
        Some(key) => {
            let edited = s.rename.as_mut().and_then(|r| r.edit(key));

            if edited.is_none() {
                return Action::None;
            }
        }
        None => return Action::None,
    }
    Action::Redraw
}

/// Returns the window whose title is selected on screen: in an open
/// icon dropdown, the title list, or the output bar.
fn target(mode: &PromptMode, s: &State) -> Option<WindowId> {
    let strip = mode.taskbar.iconbar() && s.query.is_empty();

    if strip && let Some(index) = s.dropdown {
        return window(&s.menu().get(index)?.target);
    }
    if s.query.is_empty() && !mode.taskbar.titlebar() {
        return None; // Neither list shows a title
    }
    s.selected_id().cloned()
}

/// Returns the window id of `target`, if it is a window.
fn window(target: &RowTarget) -> Option<WindowId> {
    match target {
        RowTarget::Window(id) => Some(id.clone()),
        RowTarget::Launch(_) | RowTarget::Tab(_) => None,
    }
}
