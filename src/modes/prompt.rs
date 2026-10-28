// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Prompt mode: typing filters windows with fuzzy matching.

use crate::core::event::{Action, Key};
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::core::view::RowTarget;
use crate::modules::input::Input;

/// Typed characters build a query; the best match is selected.
#[derive(Debug)]
pub struct PromptMode {
    input: Input,
}

impl PromptMode {
    /// Creates a prompt mode with the input module.
    #[must_use]
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Mode for PromptMode {
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
            Key::Enter => match s.selected_target() {
                Some(RowTarget::Window(id)) => Action::Activate(id.clone()),
                Some(RowTarget::Launch(id)) => Action::Launch(id.clone()),
                None => Action::None,
            },
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
        self.input.refresh(s);
    }
}

#[cfg(test)]
#[path = "prompt_tests.rs"]
mod tests;
