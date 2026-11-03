// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Keysyms to `core::Key`: plain keys, combinations and modifier taps.

use xkbcommon::xkb::{self, keysyms};

use crate::core::event::Key;
use crate::core::hotkey::{Binding, Code, Mods, Named, Side};

/// Maps `Left` and `Right` to a word move while Control is held.
pub(super) fn word_move(key: Key, ctrl: bool) -> Key {
    match key {
        Key::Left if ctrl => Key::WordLeft,
        Key::Right if ctrl => Key::WordRight,
        other => other,
    }
}

/// Maps a keysym and its text to a `core::Key`.
pub(super) fn to_key(
    sym: u32,
    text: Option<&str>,
    latin: Option<char>,
) -> Option<Key> {
    match sym {
        keysyms::KEY_Return | keysyms::KEY_KP_Enter => Some(Key::Enter),
        keysyms::KEY_Escape => Some(Key::Esc),
        keysyms::KEY_Tab => Some(Key::Tab),
        keysyms::KEY_ISO_Left_Tab => Some(Key::BackTab), // Shift+Tab
        keysyms::KEY_Up | keysyms::KEY_KP_Up => Some(Key::Up),
        keysyms::KEY_Down | keysyms::KEY_KP_Down => Some(Key::Down),
        keysyms::KEY_Left | keysyms::KEY_KP_Left => Some(Key::Left),
        keysyms::KEY_Right | keysyms::KEY_KP_Right => Some(Key::Right),
        keysyms::KEY_Home | keysyms::KEY_KP_Home => Some(Key::Home),
        keysyms::KEY_End | keysyms::KEY_KP_End => Some(Key::End),
        keysyms::KEY_BackSpace => Some(Key::Backspace),
        keysyms::KEY_Delete | keysyms::KEY_KP_Delete => Some(Key::Delete),
        _ => {
            let mut chars = text?.chars();
            let c = chars.next().filter(|c| !c.is_control())?;

            chars
                .next()
                .is_none()
                .then_some(Key::Char { text: c, latin })
        }
    }
}

/// Returns the binding of a press that the plain keys cannot carry: a
/// key held with Ctrl, Alt or Meta, or a key with no plain variant
/// such as `F2`. `None` keeps the plain key.
pub(super) fn combo(
    sym: u32,
    mods: Mods,
    latin: Option<char>,
) -> Option<Binding> {
    let held =
        mods.has(Mods::CTRL) || mods.has(Mods::ALT) || mods.has(Mods::META);
    let code = match named(sym) {
        Some(
            n @ (Named::F(_) | Named::Insert | Named::PageUp | Named::PageDown),
        ) => Code::Named(n),
        Some(n) if held => Code::Named(n),
        None if held => Code::Char(latin.or_else(|| sym_char(sym))?),
        _ => return None,
    };

    Some(Binding { mods, code })
}

/// Returns the modifier side of `sym`, for taps.
pub(super) fn side(sym: u32) -> Option<Side> {
    Some(match sym {
        keysyms::KEY_Shift_L => Side::ShiftLeft,
        keysyms::KEY_Shift_R => Side::ShiftRight,
        keysyms::KEY_Control_L => Side::ControlLeft,
        keysyms::KEY_Control_R => Side::ControlRight,
        keysyms::KEY_Alt_L => Side::AltLeft,
        keysyms::KEY_Alt_R => Side::AltRight,
        keysyms::KEY_Super_L | keysyms::KEY_Meta_L => Side::MetaLeft,
        keysyms::KEY_Super_R | keysyms::KEY_Meta_R => Side::MetaRight,
        _ => return None,
    })
}

/// Returns the modifiers in effect in `state`.
pub(super) fn mods(state: &xkb::State) -> Mods {
    [
        (xkb::MOD_NAME_CTRL, Mods::CTRL),
        (xkb::MOD_NAME_ALT, Mods::ALT),
        (xkb::MOD_NAME_SHIFT, Mods::SHIFT),
        (xkb::MOD_NAME_LOGO, Mods::META),
    ]
    .into_iter()
    .filter(|(name, _)| {
        state.mod_name_is_active(name, xkb::STATE_MODS_EFFECTIVE)
    })
    .fold(Mods::default(), |all, (_, one)| all.with(one))
}

/// Returns the named key of `sym`, if it has one.
fn named(sym: u32) -> Option<Named> {
    Some(match sym {
        keysyms::KEY_space | keysyms::KEY_KP_Space => Named::Space,
        keysyms::KEY_Return | keysyms::KEY_KP_Enter => Named::Enter,
        keysyms::KEY_Tab | keysyms::KEY_ISO_Left_Tab => Named::Tab,
        keysyms::KEY_Escape => Named::Escape,
        keysyms::KEY_BackSpace => Named::Backspace,
        keysyms::KEY_Delete | keysyms::KEY_KP_Delete => Named::Delete,
        keysyms::KEY_Insert | keysyms::KEY_KP_Insert => Named::Insert,
        keysyms::KEY_Home | keysyms::KEY_KP_Home => Named::Home,
        keysyms::KEY_End | keysyms::KEY_KP_End => Named::End,
        keysyms::KEY_Prior | keysyms::KEY_KP_Prior => Named::PageUp,
        keysyms::KEY_Next | keysyms::KEY_KP_Next => Named::PageDown,
        keysyms::KEY_Up | keysyms::KEY_KP_Up => Named::ArrowUp,
        keysyms::KEY_Down | keysyms::KEY_KP_Down => Named::ArrowDown,
        keysyms::KEY_Left | keysyms::KEY_KP_Left => Named::ArrowLeft,
        keysyms::KEY_Right | keysyms::KEY_KP_Right => Named::ArrowRight,
        keysyms::KEY_F1..=keysyms::KEY_F24 => {
            Named::F(u8::try_from(sym - keysyms::KEY_F1 + 1).ok()?)
        }
        _ => return None,
    })
}

/// Returns the printable character of `sym`, lowercased.
fn sym_char(sym: u32) -> Option<char> {
    char::from_u32(xkb::keysym_to_utf32(sym.into()))
        .filter(|c| !c.is_control() && *c != '\0')
        .map(|c| c.to_ascii_lowercase())
}

/// Returns the letter of `code` in the first layout where it is an ASCII
/// letter, so letter keys work with any active layout.
pub(super) fn latin(keymap: &xkb::Keymap, code: xkb::Keycode) -> Option<char> {
    (0..keymap.num_layouts_for_key(code)).find_map(|layout| {
        let sym = *keymap.key_get_syms_by_level(code, layout, 0).first()?;
        let c = char::from_u32(xkb::keysym_to_utf32(sym))?;

        c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase())
    })
}
