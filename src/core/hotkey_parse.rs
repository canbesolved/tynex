// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hotkey text to `Binding`: `"Ctrl+Shift+T"`, `"F2"`, `"."`.

use super::{Binding, Code, Mods, Named, Side};

const NAMED: &[(&str, Named)] = &[
    ("space", Named::Space),
    ("enter", Named::Enter),
    ("return", Named::Enter),
    ("tab", Named::Tab),
    ("escape", Named::Escape),
    ("esc", Named::Escape),
    ("backspace", Named::Backspace),
    ("delete", Named::Delete),
    ("del", Named::Delete),
    ("insert", Named::Insert),
    ("ins", Named::Insert),
    ("home", Named::Home),
    ("end", Named::End),
    ("pageup", Named::PageUp),
    ("pagedown", Named::PageDown),
    ("arrowup", Named::ArrowUp),
    ("up", Named::ArrowUp),
    ("arrowdown", Named::ArrowDown),
    ("down", Named::ArrowDown),
    ("arrowleft", Named::ArrowLeft),
    ("left", Named::ArrowLeft),
    ("arrowright", Named::ArrowRight),
    ("right", Named::ArrowRight),
];

const SIDES: &[(&str, Side)] = &[
    ("shiftleft", Side::ShiftLeft),
    ("shiftright", Side::ShiftRight),
    ("controlleft", Side::ControlLeft),
    ("controlright", Side::ControlRight),
    ("altleft", Side::AltLeft),
    ("altright", Side::AltRight),
    ("metaleft", Side::MetaLeft),
    ("metaright", Side::MetaRight),
];

impl Binding {
    /// Parses one binding, for example `"Ctrl+Shift+T"` or `"."`.
    ///
    /// # Errors
    ///
    /// Returns a message naming the part that is not valid.
    pub fn parse(text: &str) -> Result<Self, String> {
        if text.is_empty() {
            return Err("it is empty".into());
        }
        if text.trim().is_empty() {
            return Ok(Self::plain(Code::Named(Named::Space)));
        }

        let parts: Vec<&str> = text.split('+').map(str::trim).collect();
        let (mod_parts, key) = match parts.as_slice() {
            [.., "", ""] => (&parts[..parts.len() - 2], "+"),
            [rest @ .., key] => (rest, *key),
            [] => return Err("it is empty".into()),
        };
        let mut mods = Mods::default();

        for part in mod_parts {
            add_mod(&mut mods, part)?;
        }
        let code = code_of(key)?;

        match code {
            Code::Tap(_) if !mods.is_empty() => {
                Err(format!("`{key}` is a modifier and must be alone"))
            }
            Code::Char(c)
                if mods == Mods::SHIFT
                    || mods.has(Mods::SHIFT) && !c.is_ascii_alphabetic() =>
            {
                Err(format!("write the character Shift types, not `{text}`"))
            }
            Code::Char(c) if !mods.is_empty() => Ok(Self {
                mods,
                code: Code::Char(c.to_ascii_lowercase()),
            }),
            code => Ok(Self { mods, code }),
        }
    }

    /// Returns a binding with no modifier.
    pub(super) fn plain(code: Code) -> Self {
        Self {
            mods: Mods::default(),
            code,
        }
    }
}

/// Adds the modifier `name` to `mods`.
fn add_mod(mods: &mut Mods, name: &str) -> Result<(), String> {
    let added = match name.to_ascii_lowercase().as_str() {
        "ctrl" | "control" => Mods::CTRL,
        "alt" => Mods::ALT,
        "shift" => Mods::SHIFT,
        "meta" | "super" | "win" => Mods::META,
        "" => return Err("it has an empty part around `+`".into()),
        _ => return Err(format!("`{name}` is not a modifier")),
    };

    *mods = mods.with(added);
    Ok(())
}

/// Parses the key part of a binding.
fn code_of(name: &str) -> Result<Code, String> {
    let lower = name.to_ascii_lowercase();
    let mut chars = name.chars();

    if let (Some(c), None) = (chars.next(), chars.next()) {
        return if c.is_control() {
            Err(format!("`{name}` is not a key"))
        } else {
            Ok(Code::Char(c))
        };
    }
    if let Some((_, named)) = NAMED.iter().find(|(n, _)| *n == lower) {
        return Ok(Code::Named(*named));
    }
    if let Some((_, side)) = SIDES.iter().find(|(n, _)| *n == lower) {
        return Ok(Code::Tap(*side));
    }
    match lower.strip_prefix('f').and_then(|n| n.parse::<u8>().ok()) {
        Some(n @ 1..=24) => Ok(Code::Named(Named::F(n))),
        _ if name.is_empty() => Err("it has no key after `+`".into()),
        _ => Err(format!("`{name}` is not a key name")),
    }
}
