// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Key bindings of prompt mode, from `[hotkeys]`.

use super::PromptMode;
use crate::config::HotkeysConfig;
use crate::core::hotkey::{Binding, Bindings, Code, Mods, Named};

impl PromptMode {
    /// Sets every key binding from `[hotkeys]`.
    #[must_use]
    pub fn with_hotkeys(self, keys: &HotkeysConfig) -> Self {
        self.with_rename(keys.rename.clone())
            .with_pinned(keys.pinned.clone())
            .with_input_keys(keys.select.clone(), keys.space.clone())
    }

    /// Sets the keys of the pinned apps, by position. None by default.
    #[must_use]
    pub fn with_pinned(mut self, keys: Vec<Bindings>) -> Self {
        self.pinned = keys;
        self
    }

    /// Sets the keys that rename the selected window. None by default.
    #[must_use]
    pub fn with_rename(mut self, keys: Bindings) -> Self {
        self.rename = keys;
        self
    }

    /// Sets the keys that confirm and the keys that type a space.
    /// `Enter` and `Space` confirm, and no key types a space, by default.
    #[must_use]
    pub fn with_input_keys(
        mut self,
        select: Bindings,
        space: Bindings,
    ) -> Self {
        self.select = select;
        self.space = space;
        self
    }
}

/// `Enter` and `Space`, the keys that confirm without `[hotkeys]`.
pub(super) fn default_select() -> Bindings {
    let plain = |named| Binding {
        mods: Mods::default(),
        code: Code::Named(named),
    };

    Bindings(vec![plain(Named::Enter), plain(Named::Space)])
}

#[cfg(test)]
#[path = "prompt_keys_tests.rs"]
mod tests;
