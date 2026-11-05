// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hotkey bindings: `Modifier+Key` text with W3C UI Events key names
//! (ADR-0087), and matching against `Key` presses. Parsing is in
//! `hotkey_parse.rs`.

use crate::core::event::Key;

#[path = "hotkey_parse.rs"]
mod parse;

/// Modifiers held with a key, as a bit set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Mods(u8);

impl Mods {
    pub const CTRL: Self = Self(1);
    pub const ALT: Self = Self(2);
    pub const SHIFT: Self = Self(4);
    pub const META: Self = Self(8);

    /// Returns these modifiers and `other`.
    #[must_use]
    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns true when every modifier of `other` is held.
    #[must_use]
    pub const fn has(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Returns true when no modifier is held.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// A key without a printable character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Named {
    Space,
    Enter,
    Tab,
    Escape,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    F(u8), // F1 to F24
}

/// One side of a modifier, bound alone as a tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    MetaLeft,
    MetaRight,
}

/// The key part of a binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    Char(char), // The character the key types
    Named(Named),
    Tap(Side), // Pressed and released with no other key
}

/// A key and the modifiers held with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binding {
    pub mods: Mods,
    pub code: Code,
}

/// Bindings of one action. Any of them fires it; empty is off.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bindings(pub Vec<Binding>);

impl Bindings {
    /// Returns true when `key` fires one of the bindings.
    #[must_use]
    pub fn matches(&self, key: Key) -> bool {
        self.0.iter().any(|b| b.matches(key))
    }

    /// Returns true when `key`, or its plain key, fires one of the
    /// bindings: `Ctrl+Enter` fires `Enter` when nothing took it.
    #[must_use]
    pub fn takes(&self, key: Key) -> bool {
        self.matches(key) || plain(key).is_some_and(|k| self.matches(k))
    }
}

impl Binding {
    /// Returns true when `key` fires this binding.
    #[must_use]
    pub fn matches(&self, key: Key) -> bool {
        if let Key::Combo(combo) = key {
            return combo == *self;
        }
        match (self.mods.is_empty(), self.code) {
            (true, Code::Char(c)) => match key {
                Key::Char { text, latin } => {
                    text == c || (c.is_ascii_lowercase() && latin == Some(c))
                }
                _ => false,
            },
            (true, Code::Named(Named::Space)) => {
                matches!(key, Key::Char { text: ' ', .. })
            }
            (true, Code::Named(named)) => plain_named(named) == Some(key),
            (false, Code::Named(Named::Tab)) => {
                self.mods == Mods::SHIFT && key == Key::BackTab
            }
            (false, Code::Named(named)) => {
                self.mods == Mods::CTRL && word_move(named) == Some(key)
            }
            _ => false,
        }
    }
}

/// Returns `key` without the modifiers of a combination that no
/// binding took: `Ctrl+Enter` still confirms. `None` for a key with no
/// plain meaning, such as `Ctrl+R` or `F2`.
#[must_use]
pub fn plain(key: Key) -> Option<Key> {
    match key {
        Key::Combo(Binding {
            code: Code::Named(Named::Space),
            ..
        }) => Some(Key::Char {
            text: ' ',
            latin: None,
        }),
        Key::Combo(Binding {
            code: Code::Named(named),
            ..
        }) => plain_named(named),
        Key::Combo(_) => None,
        key => Some(key),
    }
}

/// Returns the `Key` a named key gives without modifiers, if any.
fn plain_named(named: Named) -> Option<Key> {
    Some(match named {
        Named::Enter => Key::Enter,
        Named::Tab => Key::Tab,
        Named::Escape => Key::Esc,
        Named::Backspace => Key::Backspace,
        Named::Delete => Key::Delete,
        Named::Home => Key::Home,
        Named::End => Key::End,
        Named::ArrowUp => Key::Up,
        Named::ArrowDown => Key::Down,
        Named::ArrowLeft => Key::Left,
        Named::ArrowRight => Key::Right,
        _ => return None,
    })
}

/// Returns the word move that `Ctrl` with `named` gives, if any.
fn word_move(named: Named) -> Option<Key> {
    match named {
        Named::ArrowLeft => Some(Key::WordLeft),
        Named::ArrowRight => Some(Key::WordRight),
        _ => None,
    }
}

#[cfg(test)]
#[path = "hotkey_tests.rs"]
mod tests;
