// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hotkey notation: parsing and matching key presses.

use super::*;

fn parse(text: &str) -> Binding {
    Binding::parse(text).unwrap()
}

fn char_key(text: char, latin: Option<char>) -> Key {
    Key::Char { text, latin }
}

#[test]
fn parses_a_combination_with_any_case_and_spaces() {
    let expected = Binding {
        mods: Mods::META,
        code: Code::Named(Named::Space),
    };

    assert_eq!(parse("Meta+Space"), expected);
    assert_eq!(parse("Meta + Space"), expected);
    assert_eq!(parse("meta+space"), expected);
    assert_eq!(parse("Super+Space"), expected);
    assert_eq!(parse("Win+Space"), expected);
}

#[test]
fn parses_modifiers_and_aliases() {
    assert_eq!(
        parse("Control+Alt+Shift+T"),
        Binding {
            mods: Mods::CTRL.with(Mods::ALT).with(Mods::SHIFT),
            code: Code::Char('t'),
        }
    );
    assert_eq!(parse("Esc").code, Code::Named(Named::Escape));
    assert_eq!(parse("Return").code, Code::Named(Named::Enter));
    assert_eq!(parse("ArrowUp").code, parse("Up").code);
}

#[test]
fn parses_single_characters_and_function_keys() {
    assert_eq!(parse("."), Binding::plain(Code::Char('.')));
    assert_eq!(parse(","), Binding::plain(Code::Char(',')));
    assert_eq!(parse("+"), Binding::plain(Code::Char('+')));
    assert_eq!(parse(" "), Binding::plain(Code::Named(Named::Space)));
    assert_eq!(parse("F2").code, Code::Named(Named::F(2)));
    assert_eq!(parse("f24").code, Code::Named(Named::F(24)));
    assert_eq!(parse("Ctrl++").code, Code::Char('+'));
}

#[test]
fn parses_a_modifier_alone_as_a_tap() {
    assert_eq!(
        parse("ShiftLeft"),
        Binding::plain(Code::Tap(Side::ShiftLeft))
    );
    assert_eq!(parse("metaright").code, Code::Tap(Side::MetaRight));
}

#[test]
fn rejects_bad_bindings() {
    for text in [
        "",
        "F25",
        "F0",
        "Hyper+A",
        "Ctrl+",
        "Ctrl+ShiftLeft",
        "Shift+.",
        "Shift+a",
        "Ctrl+Foo",
        "\u{1}",
    ] {
        assert!(Binding::parse(text).is_err(), "{text:?} parsed");
    }
}

#[test]
fn a_character_matches_the_typed_character_or_latin_letter() {
    let dot = parse(".");
    let r = parse("r");

    assert!(dot.matches(char_key('.', None)));
    assert!(!dot.matches(char_key(',', None)));
    assert!(r.matches(char_key('к', Some('r'))));
    assert!(!parse("R").matches(char_key('r', Some('r'))));
    assert!(parse("R").matches(char_key('R', Some('r'))));
}

#[test]
fn named_keys_match_their_key() {
    assert!(parse("Space").matches(char_key(' ', None)));
    assert!(parse("Enter").matches(Key::Enter));
    assert!(parse("Escape").matches(Key::Esc));
    assert!(parse("Shift+Tab").matches(Key::BackTab));
    assert!(parse("Ctrl+Left").matches(Key::WordLeft));
    assert!(!parse("Enter").matches(Key::Tab));
}

#[test]
fn combinations_match_combo_keys_only() {
    let f2 = parse("F2");
    let ctrl_r = parse("Ctrl+R");

    assert!(f2.matches(Key::Combo(f2)));
    assert!(ctrl_r.matches(Key::Combo(parse("ctrl+r"))));
    assert!(!ctrl_r.matches(char_key('r', Some('r'))));
    assert!(!parse("ShiftLeft").matches(Key::Combo(parse("ShiftRight"))));
}

#[test]
fn any_binding_in_a_list_fires() {
    let keys = Bindings(vec![parse("Enter"), parse("Space")]);

    assert!(keys.matches(Key::Enter));
    assert!(keys.matches(char_key(' ', None)));
    assert!(!keys.matches(Key::Tab));
    assert!(!Bindings::default().matches(Key::Enter));
}

#[test]
fn an_unbound_combination_falls_back_to_its_plain_key() {
    assert_eq!(plain(Key::Combo(parse("Ctrl+Enter"))), Some(Key::Enter));
    assert_eq!(
        plain(Key::Combo(parse("Ctrl+Space"))),
        Some(char_key(' ', None))
    );
    assert_eq!(plain(Key::Combo(parse("Ctrl+R"))), None);
    assert_eq!(plain(Key::Combo(parse("F2"))), None);
    assert_eq!(plain(Key::Up), Some(Key::Up));
}
