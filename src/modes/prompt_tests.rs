// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Prompt mode key tests.

use super::*;
use crate::core::view::Row;
use crate::core::window::WindowId;

fn state() -> State {
    let rows = [("w1", "Release notes"), ("w2", "Café"), ("w3", "Kate")]
        .into_iter()
        .map(|(id, title)| Row {
            id: WindowId(id.into()),
            app_id: String::new(),
            app_name: String::new(),
            icon: None,
            title: title.into(),
            highlights: Vec::new(),
        })
        .collect();
    let mut s = State::new(rows);

    PromptMode.refresh(&mut s);
    s
}

fn typed(s: &mut State, text: &str) {
    for c in text.chars() {
        PromptMode.on_key(
            Key::Char {
                text: c,
                latin: None,
            },
            s,
        );
    }
}

fn titles(s: &State) -> Vec<String> {
    s.view().rows().map(|r| r.title.clone()).collect()
}

#[test]
fn typing_filters_and_selects_the_best_match() {
    let mut s = state();

    typed(&mut s, "cafe");
    assert_eq!(titles(&s), ["Café"]);
    assert_eq!(s.rows[1].highlights.first(), Some(&(0..5)));
    assert_eq!(s.rows[1].highlights.len(), 1);
    assert_eq!(
        PromptMode.on_key(Key::Enter, &mut s),
        Action::Activate(WindowId("w2".into()))
    );
}

#[test]
fn backspace_widens_the_filter() {
    let mut s = state();

    typed(&mut s, "kz"); // Too short for a typo
    assert_eq!(titles(&s), [] as [&str; 0]);
    PromptMode.on_key(Key::Backspace, &mut s);
    assert_eq!(titles(&s), ["Kate"]);
}

#[test]
fn esc_clears_the_query_then_closes() {
    let mut s = state();

    typed(&mut s, "kate");
    assert_eq!(PromptMode.on_key(Key::Esc, &mut s), Action::Redraw);
    assert_eq!(s.query, "");
    assert_eq!(titles(&s).len(), 3);
    assert!(s.rows.iter().all(|r| r.highlights.is_empty()));
    assert_eq!(PromptMode.on_key(Key::Esc, &mut s), Action::Close);
}

#[test]
fn enter_without_matches_does_nothing() {
    let mut s = state();

    typed(&mut s, "zzzz");
    assert_eq!(PromptMode.on_key(Key::Enter, &mut s), Action::None);
}

#[test]
fn app_names_match() {
    let mut s = state();

    s.rows[0].app_name = "Configuración".into(); // Not in the title
    typed(&mut s, "configuración");
    assert_eq!(titles(&s), ["Release notes"]);
}
