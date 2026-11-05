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

/// Default `hotkeys.select`.
const SELECT: [&str; 2] = ["Enter", "Space"];

/// Default `hotkeys.space`: a left Shift tapped alone.
const SPACE: &str = "ShiftLeft";

/// The `[hotkeys]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeysConfig {
    pub action: HotkeyAction,
    pub rename: Bindings, // Edits the selected window title
    pub pinned: Vec<Bindings>, // The n-th opens the n-th pinned app
    pub select: Bindings, // Confirms the selected match
    pub space: Bindings,  // Types a space in the query
}

/// Returns the default bindings of `hotkeys.rename`.
pub(crate) fn default_rename() -> Bindings {
    defaults(&[RENAME])
}

/// Returns the default bindings of `hotkeys.select`.
pub(crate) fn default_select() -> Bindings {
    defaults(&SELECT)
}

/// Returns the default bindings of `hotkeys.space`.
pub(crate) fn default_space() -> Bindings {
    defaults(&[SPACE])
}

/// Parses built-in bindings.
fn defaults(keys: &[&str]) -> Bindings {
    Bindings(keys.iter().filter_map(|k| Binding::parse(k).ok()).collect())
}

/// Returns the default `hotkeys.pinned`: `Alt+1` to `Alt+9`.
pub(crate) fn default_pinned() -> Vec<Bindings> {
    (1..=9)
        .filter_map(|n| Binding::parse(&format!("Alt+{n}")).ok())
        .map(|b| Bindings(vec![b]))
        .collect()
}

/// Checks the `[hotkeys]` table. Every bad binding is reported.
pub(super) fn check(
    raw: &RawHotkeys,
    err: &mut impl FnMut(Range<usize>, String),
) -> HotkeysConfig {
    let rename = keys("rename", raw.rename.as_ref(), default_rename, err);
    let select = keys("select", raw.select.as_ref(), default_select, err);
    let space = keys("space", raw.space.as_ref(), default_space, err);
    let pinned = pinned(raw.pinned.as_ref(), err);
    let span = |value: &Option<Spanned<toml::Value>>| {
        value.as_ref().map_or(0..0, Spanned::span)
    };
    let mut actions = vec![
        ("rename".to_owned(), &rename, span(&raw.rename)),
        ("select".to_owned(), &select, span(&raw.select)),
        ("space".to_owned(), &space, span(&raw.space)),
    ];

    for (n, keys) in pinned.iter().enumerate() {
        actions.push((format!("pinned[{}]", n + 1), keys, span(&raw.pinned)));
    }
    conflicts(&actions, err);
    HotkeysConfig {
        action: match raw.action {
            RawHotkeyAction::Hide => HotkeyAction::Hide,
            RawHotkeyAction::Last => HotkeyAction::Last,
        },
        rename,
        pinned,
        select,
        space,
    }
}

/// Checks `hotkeys.<name>`, or returns `default()` when it is not set.
fn keys(
    name: &str,
    value: Option<&Spanned<toml::Value>>,
    default: fn() -> Bindings,
    err: &mut impl FnMut(Range<usize>, String),
) -> Bindings {
    value.map_or_else(default, |value| {
        bindings(name, value.get_ref(), value.span(), err)
    })
}

/// Checks `hotkeys.pinned`: a list with one entry per pinned app, each
/// a binding, a list of bindings, `""` or `[]`.
fn pinned(
    value: Option<&Spanned<toml::Value>>,
    err: &mut impl FnMut(Range<usize>, String),
) -> Vec<Bindings> {
    let Some(value) = value else {
        return default_pinned();
    };
    let span = value.span();

    if let toml::Value::Array(items) = value.get_ref() {
        items
            .iter()
            .map(|item| bindings("pinned", item, span.clone(), err))
            .collect()
    } else {
        err(
            span,
            "`hotkeys.pinned` must be a list with one key per pinned app"
                .into(),
        );
        Vec::new()
    }
}

/// Checks one `hotkeys.<name>` value: a binding, a list of bindings,
/// `""` or `[]`. Errors point at `span`.
fn bindings(
    name: &str,
    value: &toml::Value,
    span: Range<usize>,
    err: &mut impl FnMut(Range<usize>, String),
) -> Bindings {
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

    match value {
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
    actions: &[(String, &Bindings, Range<usize>)],
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
