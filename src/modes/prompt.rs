// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Prompt mode: typing filters windows with fuzzy matching.
//!
//! `Space` confirms the selected row, the same as `Enter`. `Tab`
//! walks the list on screen, or the icons when the title list is off.
//! `Shift+Tab` walks it back.

use crate::config::TaskbarMode;
use crate::core::event::{Action, Click, Key};
use crate::core::state::State;
use crate::core::traits::Mode;
use crate::core::view::RowTarget;
use crate::modules::input::Input;

#[path = "prompt_icon.rs"]
mod prompt_icon;

/// Typed characters build a query; the best match is selected.
#[derive(Debug)]
pub struct PromptMode {
    input: Input,
    taskbar: TaskbarMode,
    max_rows: usize, // Cap for the dropdown
}

impl PromptMode {
    /// Creates a prompt mode.
    ///
    /// `taskbar` picks the strip, the title list, both or neither.
    /// `max_rows` caps the dropdown.
    #[must_use]
    pub fn new(input: Input, taskbar: TaskbarMode, max_rows: u16) -> Self {
        Self {
            input,
            taskbar,
            max_rows: usize::from(max_rows).max(1),
        }
    }

    /// Returns true while the icon strip is on screen.
    fn strip(&self, s: &State) -> bool {
        self.taskbar.iconbar() && s.query.is_empty()
    }
}

impl Mode for PromptMode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        if self.strip(s)
            && let Some(action) =
                prompt_icon::key(key, s, self.max_rows, self.taskbar.titlebar())
        {
            return action;
        }
        match key {
            Key::Char { text: ' ', .. } | Key::Enter => confirm(s),
            Key::Char { text, .. } => {
                s.close_dropdown();
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
            Key::Tab | Key::BackTab
                if !s.query.is_empty() || self.taskbar.titlebar() =>
            {
                if key == Key::Tab {
                    s.select_next();
                } else {
                    s.select_prev();
                }
                Action::Redraw
            }
            Key::Tab | Key::BackTab => Action::None,
        }
    }

    fn refresh(&mut self, s: &mut State) {
        self.input.refresh(s);
        if self.strip(s) {
            s.settle_dropdown(self.max_rows);
        } else {
            s.close_dropdown();
        }
    }

    fn on_click(&mut self, click: Click, s: &mut State) -> Action {
        if self.strip(s) {
            prompt_icon::click(click, s, self.max_rows)
        } else {
            Action::None
        }
    }

    fn on_hover(&mut self, click: Click, s: &mut State) -> Action {
        if self.strip(s) {
            prompt_icon::hover(click, s)
        } else {
            Action::None
        }
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

#[cfg(test)]
#[path = "prompt_icon_tests.rs"]
mod icon_tests;
