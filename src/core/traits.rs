// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Seams between the core and pluggable modules.

use std::fmt;

use crate::core::browser_tabs::TabsUpdate;
use crate::core::event::{Action, Click, Key};
use crate::core::places::Place;
use crate::core::queue::EventTx;
use crate::core::state::State;
use crate::core::window::{WindowId, WindowInfo};

/// Error reported by a window source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(pub String);

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SourceError {}

/// Source of windows: a desktop environment or window manager.
pub trait WindowSource {
    /// Starts sending `Event::Windows` updates to `tx`.
    ///
    /// # Errors
    ///
    /// Returns an error when the desktop cannot be reached.
    fn start(&mut self, tx: EventTx) -> Result<(), SourceError>;

    /// Activates window `id`.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown window or a failed request.
    fn activate(&mut self, id: &WindowId) -> Result<(), SourceError>;

    /// Undoes what `start` set up in the desktop. Called once on exit.
    fn stop(&mut self) {}
}

/// Turns key presses into switcher actions.
pub trait Mode {
    /// Handles one key press while the popup is open.
    fn on_key(&mut self, key: Key, s: &mut State) -> Action;

    /// Recomputes `output` and highlights after the rows or mode
    /// changed.
    fn refresh(&mut self, _s: &mut State) {}

    /// Handles one click while the popup is open.
    fn on_click(&mut self, _click: Click, _s: &mut State) -> Action {
        Action::None
    }

    /// Handles one pointer move while the popup is open.
    fn on_hover(&mut self, _click: Click, _s: &mut State) -> Action {
        Action::None
    }

    /// Sees the action that closes the popup, before `s` is dropped.
    fn on_close(&mut self, _action: &Action, _s: &State) {}

    /// Sees the place `tynex qs` switches to while hidden.
    fn on_quick_switch(&mut self, _place: &Place) {}

    /// Applies a browser tab change. `state` is `None` while hidden.
    /// The default ignores it.
    fn on_tabs(
        &mut self,
        _update: TabsUpdate,
        _state: &mut Option<State>,
        _windows: &[WindowInfo],
    ) -> Action {
        Action::None
    }
}
