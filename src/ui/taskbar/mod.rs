// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Taskbar: the icon strip and the title list, as one module.
//!
//! `TaskbarMode` picks the strip, the list, both, or neither. Typing
//! hides whichever is showing. The dropdown of one app's windows is
//! `menu`.

pub(crate) mod icons;
pub(crate) mod menu;
pub(crate) mod titles;

#[cfg(test)]
#[path = "icons_badge_tests.rs"]
mod icons_badge_tests;
