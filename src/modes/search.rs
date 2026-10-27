// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Search mode: typing filters windows with fuzzy matching.

use crate::core::event::{Action, Key};
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::search::fuzzy::rank;

/// Typed characters build a query; the best match is selected.
#[derive(Debug, Default)]
pub struct SearchMode;

impl Mode for SearchMode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        match key {
            Key::Char { text, .. } => {
                s.query.push(text);
                self.refresh(s);
                Action::Redraw
            }
            Key::Backspace => {
                if s.query.pop().is_some() {
                    self.refresh(s);
                }
                Action::Redraw
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
            Key::Esc if s.query.is_empty() => Action::Close,
            Key::Esc => {
                s.query.clear();
                self.refresh(s);
                Action::Redraw
            }
            Key::Tab => Action::None,
        }
    }

    fn refresh(&mut self, s: &mut State) {
        let apps: Vec<String> = s
            .rows
            .iter()
            .map(|r| format!("{} {}", r.app_name, r.app_id))
            .collect();
        let items: Vec<(&str, &str)> = s
            .rows
            .iter()
            .zip(&apps)
            .map(|(r, app)| (r.title.as_str(), app.as_str()))
            .collect();
        let hits = rank(&s.query, &items);

        for row in &mut s.rows {
            row.highlights.clear();
        }
        s.shown = hits.iter().map(|h| h.index).collect();
        for hit in hits {
            s.rows[hit.index].highlights = hit.highlights;
        }
        s.selected = 0;
    }
}

#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;
