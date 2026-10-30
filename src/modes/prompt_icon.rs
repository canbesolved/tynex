// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Keys and clicks for the icon strip. The query is empty.

use crate::core::event::{Action, Click, Key};
use crate::core::state::State;
use crate::core::view::RowTarget;

/// Handles a key while the strip is showing.
///
/// `None` means the key belongs to the query or the title list. With
/// the title list off, `Tab` moves to the next icon and `Shift+Tab` to
/// the previous one. With it on,
/// `Enter` and `Space` confirm that list unless a dropdown is open.
pub(super) fn key(
    key: Key,
    s: &mut State,
    cap: usize,
    titles: bool,
) -> Option<Action> {
    match key {
        Key::Left => {
            s.icon_prev();
            s.close_dropdown();
            Some(Action::Redraw)
        }
        Key::Right => {
            s.icon_next();
            s.close_dropdown();
            Some(Action::Redraw)
        }
        Key::Char { text: ' ', .. } | Key::Enter
            if !titles || s.dropdown.is_some() =>
        {
            Some(confirm(s, cap))
        }
        Key::Up | Key::BackTab if s.dropdown.is_some() => {
            Some(step(s, cap, false))
        }
        Key::Down | Key::Tab if s.dropdown.is_some() => {
            Some(step(s, cap, true))
        }
        Key::Tab if !titles => {
            s.icon_next();
            Some(Action::Redraw)
        }
        Key::BackTab if !titles => {
            s.icon_prev();
            Some(Action::Redraw)
        }
        Key::Esc if s.dropdown.is_some() => {
            s.close_dropdown();
            Some(Action::Redraw)
        }
        _ => None,
    }
}

/// Handles a click while the strip is showing.
pub(super) fn click(click: Click, s: &mut State, cap: usize) -> Action {
    match click {
        Click::Icon(index) => {
            if index >= s.icons.len() {
                return Action::None;
            }
            let switched = s.icon != index;

            let app = s.icons[index].app_id.clone();

            s.select_icon(Some(&app));
            if switched {
                s.close_dropdown();
            }
            confirm(s, cap)
        }
        Click::Menu(index) if index < s.menu_len(cap) => {
            s.dropdown = Some(index);
            activate_menu(s, cap)
        }
        Click::Elsewhere if s.dropdown.is_some() => {
            s.close_dropdown();
            Action::Redraw
        }
        Click::Menu(_) | Click::Elsewhere => Action::None,
    }
}

/// Selects the icon under the pointer.
///
/// Leaving the icons keeps the current selection. Moving to another
/// icon closes an open dropdown.
pub(super) fn hover(click: Click, s: &mut State) -> Action {
    let Click::Icon(index) = click else {
        return Action::None;
    };
    if index >= s.icons.len() || index == s.icon {
        return Action::None;
    }
    let app = s.icons[index].app_id.clone();

    s.select_icon(Some(&app));
    s.close_dropdown();
    Action::Redraw
}

/// Moves the dropdown selection one step.
fn step(s: &mut State, cap: usize, forward: bool) -> Action {
    s.step_dropdown(cap, forward);
    Action::Redraw
}

/// Does what a click on the selected icon does.
fn confirm(s: &mut State, cap: usize) -> Action {
    if s.dropdown.is_some() {
        return activate_menu(s, cap);
    }
    let Some(icon) = s.icons.get(s.icon) else {
        return Action::None;
    };

    match icon.windows.len() {
        0 => Action::Launch(icon.app_id.clone()),
        1 => Action::Activate(icon.windows[0].clone()),
        _ => {
            s.open_dropdown();
            Action::Redraw
        }
    }
}

/// Activates the highlighted dropdown title.
fn activate_menu(s: &State, cap: usize) -> Action {
    let index = s.dropdown.unwrap_or(0);
    let menu = s.menu();

    if index >= cap {
        return Action::None;
    }
    match menu.get(index).map(|row| &row.target) {
        Some(RowTarget::Window(id)) => Action::Activate(id.clone()),
        Some(RowTarget::Launch(id)) => Action::Launch(id.clone()),
        None => Action::None,
    }
}
