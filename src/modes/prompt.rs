// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Prompt mode: typing filters windows with fuzzy matching.
//!
//! `Space` confirms the selected row, the same as `Enter`. `Tab`
//! walks the list that is on screen.

use crate::core::event::{Action, Key};
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::core::view::RowTarget;
use crate::modules::input::Input;

/// Typed characters build a query; the best match is selected.
#[derive(Debug)]
pub struct PromptMode {
    input: Input,
    title_bar: bool, // `title_bar.enabled` from the config
}

impl PromptMode {
    /// Creates a prompt mode. `title_bar` is `title_bar.enabled`.
    #[must_use]
    pub fn new(input: Input, title_bar: bool) -> Self {
        Self { input, title_bar }
    }
}

impl Mode for PromptMode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        match key {
            Key::Char { text: ' ', .. } | Key::Enter => confirm(s),
            Key::Char { text, .. } => {
                s.insert(text);
                self.refresh(s);
                Action::Redraw
            }
            Key::Backspace => {
                if s.backspace() {
                    self.refresh(s);
                }
                Action::Redraw
            }
            Key::Delete => {
                if s.delete() {
                    self.refresh(s);
                }
                Action::Redraw
            }
            Key::Left => {
                s.move_left();
                Action::Redraw
            }
            Key::Right => {
                s.move_right();
                Action::Redraw
            }
            Key::WordLeft => {
                s.move_word_left();
                Action::Redraw
            }
            Key::WordRight => {
                s.move_word_right();
                Action::Redraw
            }
            Key::Home => {
                s.cursor = 0;
                Action::Redraw
            }
            Key::End => {
                s.cursor = s.query.len();
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
            Key::Esc if s.query.is_empty() => Action::Close,
            Key::Esc => {
                s.query.clear();
                s.cursor = 0;
                self.refresh(s);
                Action::Redraw
            }
            // The title bar or the output bar is showing.
            Key::Tab if !s.query.is_empty() || self.title_bar => {
                s.select_next();
                Action::Redraw
            }
            Key::Tab => Action::None,
        }
    }

    fn refresh(&mut self, s: &mut State) {
        self.input.refresh(s);
    }
}

/// Activates or launches the selected row.
fn confirm(s: &State) -> Action {
    match s.selected_target() {
        Some(RowTarget::Window(id)) => Action::Activate(id.clone()),
        Some(RowTarget::Launch(id)) => Action::Launch(id.clone()),
        None => Action::None,
    }
}

#[cfg(test)]
#[path = "prompt_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "prompt_key_tests.rs"]
mod key_tests;
