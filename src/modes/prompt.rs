// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Prompt mode: typing filters windows with fuzzy matching.
//!
//! `Space` confirms the selected row, the same as `Enter`. `Tab`
//! walks the list on screen, or the icons when the title list is off.
//! `Shift+Tab` walks it back.

use crate::config::TaskbarMode;
use crate::core::browser_tabs::TabsUpdate;
use crate::core::event::{Action, Click, Key};
use crate::core::hotkey::{self, Bindings};
use crate::core::state::State;
use crate::core::text_edit;
use crate::core::traits::Mode;
use crate::core::view::RowTarget;
use crate::core::window::WindowInfo;
use crate::modules::browser_tabs::Session;
use crate::modules::input::Input;

#[path = "prompt_pointer.rs"]
mod pointer;
#[path = "prompt_icon.rs"]
mod prompt_icon;
#[path = "prompt_rename.rs"]
mod rename;
#[path = "prompt_tabs.rs"]
mod tabs;

/// Typed characters build a query; the best match is selected.
#[derive(Debug)]
pub struct PromptMode {
    input: Input,
    taskbar: TaskbarMode,
    max_rows: usize, // Cap for the dropdown
    tabs: Option<Session>,
    rename: Bindings, // Keys that rename the selected window
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
            tabs: None,
            rename: Bindings::default(),
        }
    }

    /// Sets the keys that rename the selected window. None by default.
    #[must_use]
    pub fn with_rename(mut self, keys: Bindings) -> Self {
        self.rename = keys;
        self
    }

    /// Turns tab search on. Off by default, so the popup is unchanged.
    #[must_use]
    pub fn with_tabs(mut self, enabled: bool) -> Self {
        if enabled {
            self.tabs = Some(Session::default());
        }
        self
    }

    /// Returns true while the icon strip is on screen.
    fn strip(&self, s: &State) -> bool {
        self.taskbar.iconbar() && s.query.is_empty()
    }
}

impl Mode for PromptMode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        if let Some(action) = rename::key(self, key, s) {
            return action;
        }
        let Some(key) = hotkey::plain(key) else {
            return Action::None;
        };
        if self.strip(s)
            && let Some(action) =
                prompt_icon::key(key, s, self.max_rows, self.taskbar.titlebar())
        {
            return action;
        }
        if matches!(key, Key::Char { text: ' ', .. } | Key::Enter) {
            return confirm(self, s);
        }
        if let Some(changed) =
            text_edit::apply(&mut s.query, &mut s.cursor, key)
        {
            if changed {
                s.close_dropdown();
                self.refresh(s);
            }
            return Action::Redraw;
        }
        match key {
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
            _ => Action::None,
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
        if s.rename.is_some() {
            return Action::None;
        }
        if let Click::Row(index) = click {
            return pointer::click_row(self, s, index);
        }
        if self.strip(s) {
            prompt_icon::click(click, s, self.max_rows)
        } else {
            Action::None
        }
    }

    fn on_tabs(
        &mut self,
        update: TabsUpdate,
        state: &mut Option<State>,
        windows: &[WindowInfo],
    ) -> Action {
        tabs::on_tabs(self, update, state, windows)
    }

    fn on_hover(&mut self, click: Click, s: &mut State) -> Action {
        if s.rename.is_some() {
            return Action::None;
        }
        if let Click::Row(index) = click {
            return pointer::hover_row(self, s, index);
        }
        if self.strip(s) {
            prompt_icon::hover(click, s, self.max_rows)
        } else {
            Action::None
        }
    }
}

/// Activates, launches or switches to the selected row.
fn confirm(mode: &mut PromptMode, s: &State) -> Action {
    let action = match s.selected_target() {
        Some(RowTarget::Window(id)) => Action::Activate(id.clone()),
        Some(RowTarget::Launch(id)) => Action::Launch(id.clone()),
        Some(RowTarget::Tab(tab)) => Action::ActivateTab(tab.clone()),
        None => Action::None,
    };

    tabs::note(mode, &action, s);
    action
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

#[cfg(test)]
#[path = "prompt_pointer_tests.rs"]
mod pointer_tests;

#[cfg(test)]
#[path = "prompt_rename_tests.rs"]
mod rename_tests;
