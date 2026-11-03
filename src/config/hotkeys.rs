// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `[hotkeys]` key bindings: one binding or a list (ADR-0087).

use std::ops::Range;

use toml::Spanned;

use crate::config::raw::{RawHotkeyAction, RawHotkeys};
use crate::core::hotkey::{Binding, Bindings};
use crate::core::switcher::HotkeyAction;

/// Default `hotkeys.rename`.
pub(crate) const RENAME: &str = ".";

/// The `[hotkeys]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeysConfig {
    pub action: HotkeyAction,
    pub rename: Bindings, // Edits the selected window title
}

/// Returns the default bindings of `hotkeys.rename`.
pub(crate) fn default_rename() -> Bindings {
    Bindings(Binding::parse(RENAME).into_iter().collect())
}

/// Checks the `[hotkeys]` table. Every bad binding is reported.
pub(super) fn check(
    raw: &RawHotkeys,
    err: &mut impl FnMut(Range<usize>, String),
) -> HotkeysConfig {
    let rename = bindings("rename", raw.rename.as_ref(), default_rename(), err);
    let rename_span = raw.rename.as_ref().map_or(0..0, Spanned::span);

    conflicts(&[("rename", &rename, rename_span)], err);
    HotkeysConfig {
        action: match raw.action {
            RawHotkeyAction::Hide => HotkeyAction::Hide,
            RawHotkeyAction::Last => HotkeyAction::Last,
        },
        rename,
    }
}

/// Checks `hotkeys.<name>`: a binding, a list of bindings, `""` or
/// `[]`. A missing key uses `default`.
pub(super) fn bindings(
    name: &str,
    value: Option<&Spanned<toml::Value>>,
    default: Bindings,
    err: &mut impl FnMut(Range<usize>, String),
) -> Bindings {
    let Some(value) = value else {
        return default;
    };
    let span = value.span();
    let mut one = |text: &str| match text {
        "" => None,
        text => Binding::parse(text)
            .map_err(|why| {
                err(
                    span.clone(),
                    format!("`hotkeys.{name}` = {text:?} is not a key: {why}"),
                );
            })
            .ok(),
    };

    match value.get_ref() {
        toml::Value::String(text) => Bindings(one(text).into_iter().collect()),
        toml::Value::Array(items) => {
            let mut list = Vec::new();

            for item in items {
                let toml::Value::String(text) = item else {
                    err(span.clone(), list_message(name));
                    break;
                };

                list.extend(one(text));
            }
            Bindings(list)
        }
        _ => {
            err(span.clone(), list_message(name));
            Bindings::default()
        }
    }
}

/// Reports a binding used by two actions, naming both keys.
pub(super) fn conflicts(
    actions: &[(&str, &Bindings, Range<usize>)],
    err: &mut impl FnMut(Range<usize>, String),
) {
    for (i, (name, keys, span)) in actions.iter().enumerate() {
        for (other, other_keys, _) in &actions[i + 1..] {
            if keys.0.iter().any(|b| other_keys.0.contains(b)) {
                err(
                    span.clone(),
                    format!(
                        "`hotkeys.{name}` and `hotkeys.{other}` use the same \
                         key, expected different keys"
                    ),
                );
            }
        }
    }
}

fn list_message(name: &str) -> String {
    format!("`hotkeys.{name}` must be a key, a list of keys, or \"\"")
}
