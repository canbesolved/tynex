// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hotkey mode: the letter next to a window switches to it.

use crate::core::event::{Action, Key, ModeKind};
use crate::core::state::State;
use crate::core::traits::Mode;

/// Default mode: assigned letters activate their window.
#[derive(Debug, Default)]
pub struct HotkeyMode;

impl Mode for HotkeyMode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        match key {
            Key::Char { text: '/', .. } => {
                s.mode = ModeKind::Search;
                Action::Redraw
            }
            Key::Char { text, latin } => {
                // The produced character first, then the Latin key.
                let id = [Some(text), latin]
                    .into_iter()
                    .flatten()
                    .find_map(|c| s.id_for_key(c.to_ascii_lowercase()));

                id.map_or(Action::None, |id| Action::Activate(id.clone()))
            }
            Key::Up => {
                s.select_prev();
                Action::Redraw
            }
            Key::Down => {
                s.select_next();
                Action::Redraw
            }
            Key::Enter => s
                .selected_id()
                .map_or(Action::None, |id| Action::Activate(id.clone())),
            Key::Esc => Action::Close,
            Key::Tab | Key::Backspace => Action::None,
        }
    }

    /// Shows every row without highlights; the query stays for search.
    fn refresh(&mut self, s: &mut State) {
        for row in &mut s.rows {
            row.highlights.clear();
        }
        s.shown = (0..s.rows.len()).collect();
        s.selected = s.selected.min(s.shown.len().saturating_sub(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::view::Row;
    use crate::core::window::WindowId;

    fn state() -> State {
        let rows = [("w1", 'a'), ("w2", 's'), ("w3", 'd')]
            .into_iter()
            .map(|(id, key)| Row {
                id: WindowId(id.into()),
                key: Some(key),
                app_id: String::new(),
                icon: None,
                title: id.into(),
                highlights: Vec::new(),
            })
            .collect();

        State::new(ModeKind::Hotkey, rows)
    }

    fn press(s: &mut State, text: char, latin: Option<char>) -> Action {
        HotkeyMode.on_key(Key::Char { text, latin }, s)
    }

    fn activate(id: &str) -> Action {
        Action::Activate(WindowId(id.into()))
    }

    #[test]
    fn letter_activates_its_window() {
        assert_eq!(press(&mut state(), 's', None), activate("w2"));
        assert_eq!(press(&mut state(), 'S', None), activate("w2"));
        assert_eq!(press(&mut state(), 'x', None), Action::None);
    }

    #[test]
    fn latin_key_works_with_other_layouts() {
        // Russian layout: the `a` key types `ф`.
        assert_eq!(press(&mut state(), 'ф', Some('a')), activate("w1"));
        assert_eq!(press(&mut state(), 'ф', None), Action::None);
    }

    #[test]
    fn arrows_and_enter_use_the_selection() {
        let mut s = state();

        assert_eq!(HotkeyMode.on_key(Key::Up, &mut s), Action::Redraw);
        assert_eq!(HotkeyMode.on_key(Key::Enter, &mut s), activate("w3"));
        HotkeyMode.on_key(Key::Down, &mut s);
        assert_eq!(HotkeyMode.on_key(Key::Enter, &mut s), activate("w1"));
    }

    #[test]
    fn slash_switches_to_search_and_esc_closes() {
        let mut s = state();

        assert_eq!(press(&mut s, '/', None), Action::Redraw);
        assert_eq!(s.mode, ModeKind::Search);
        assert_eq!(HotkeyMode.on_key(Key::Esc, &mut state()), Action::Close);
    }

    #[test]
    fn enter_on_empty_list_does_nothing() {
        let mut s = State::new(ModeKind::Hotkey, Vec::new());

        assert_eq!(HotkeyMode.on_key(Key::Enter, &mut s), Action::None);
    }
}
