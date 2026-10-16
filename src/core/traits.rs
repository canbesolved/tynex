// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Seams between the core and pluggable modules.

use crate::core::event::{Action, Key};
use crate::core::state::State;

/// Turns key presses into switcher actions.
pub trait Mode {
    /// Handles one key press while the popup is open.
    fn on_key(&mut self, key: Key, s: &mut State) -> Action;

    /// Recomputes `shown` and highlights after the rows or mode changed.
    fn refresh(&mut self, _s: &mut State) {}
}
