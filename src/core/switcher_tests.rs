// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for the switcher, with minimal stand-in modes.

use super::*;
use crate::core::window::WindowId;

/// Letter keys activate; `Esc` closes; arrows move.
struct Letters;

impl Mode for Letters {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        match key {
            Key::Char { text, .. } => s
                .id_for_key(text)
                .map_or(Action::None, |id| Action::Activate(id.clone())),
            Key::Down => {
                s.select_next();
                Action::Redraw
            }
            Key::Esc => Action::Close,
            _ => Action::None,
        }
    }
}

/// Typing filters titles by substring.
struct Filter;

impl Mode for Filter {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        if let Key::Char { text, .. } = key {
            s.query.push(text);
            self.refresh(s);
        }
        Action::Redraw
    }

    fn refresh(&mut self, s: &mut State) {
        let query = s.query.clone();

        s.shown = (0..s.rows.len())
            .filter(|&i| s.rows[i].title.contains(&query))
            .collect();
    }
}

fn window(id: &str, title: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.into()),
        app_id: id.into(),
        title: title.into(),
        icon: None,
    }
}

/// Windows whose ids are also their titles, most recent first.
fn list(ids: &[&str]) -> Vec<WindowInfo> {
    ids.iter().map(|id| window(id, id)).collect()
}

fn switcher(ids: &[&str]) -> Switcher<Letters, Filter> {
    let mut sw = Switcher::new("asd", Letters, Filter);

    assert_eq!(sw.handle(Event::Windows(list(ids))), Action::None);
    sw
}

fn shown(sw: &Switcher<Letters, Filter>) -> Vec<(String, Option<char>)> {
    sw.view()
        .expect("popup is open")
        .rows()
        .map(|r| (r.id.0.clone(), r.key))
        .collect()
}

fn letter(c: char) -> Event {
    Event::Key(Key::Char {
        text: c,
        latin: None,
    })
}

#[test]
fn hidden_until_shown() {
    let mut sw = switcher(&["a", "b"]);

    assert!(sw.view().is_none());
    assert_eq!(sw.handle(letter('a')), Action::None);
}

#[test]
fn show_moves_active_window_last() {
    let mut sw = switcher(&["active", "prev", "older"]);

    assert_eq!(sw.handle(Event::Show(ModeKind::Hotkey)), Action::Redraw);
    assert_eq!(
        shown(&sw),
        [
            ("prev".into(), Some('a')),
            ("older".into(), Some('s')),
            ("active".into(), Some('d')),
        ]
    );
}

#[test]
fn letter_activates_and_closes() {
    let mut sw = switcher(&["active", "prev"]);

    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(
        sw.handle(letter('a')),
        Action::Activate(WindowId("prev".into()))
    );
    assert!(sw.view().is_none());
}

#[test]
fn titles_are_sanitized_on_arrival() {
    let mut sw = Switcher::new("a", Letters, Filter);

    sw.handle(Event::Windows(vec![window("x", " a\u{202E}\tb ")]));
    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(sw.view().unwrap().rows().next().unwrap().title, "a b");
}

#[test]
fn tab_switches_mode_and_refreshes() {
    let mut sw = switcher(&["x", "firefox", "kate"]);

    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(sw.handle(Event::Key(Key::Tab)), Action::Redraw);
    sw.handle(letter('k'));
    assert_eq!(sw.view().unwrap().mode, ModeKind::Search);
    assert_eq!(shown(&sw), [("kate".into(), Some('s'))]);
}

#[test]
fn update_while_open_keeps_keys_and_selection() {
    let mut sw = switcher(&["x", "a", "b"]);

    sw.handle(Event::Show(ModeKind::Hotkey));
    sw.handle(Event::Key(Key::Down)); // Select "b"

    assert_eq!(
        sw.handle(Event::Windows(list(&["b", "x", "a"]))),
        Action::Redraw
    );
    assert_eq!(
        shown(&sw),
        [
            ("x".into(), Some('d')),
            ("a".into(), Some('a')),
            ("b".into(), Some('s')),
        ]
    );
    assert_eq!(sw.view().unwrap().selected, 2);
}

#[test]
fn letters_freed_while_open_are_reused_after_close() {
    let mut sw = Switcher::new("asdfh", Letters, Filter);

    sw.handle(Event::Windows(list(&["x", "a", "b"])));
    sw.handle(Event::Show(ModeKind::Hotkey)); // a, b, x get a, s, d

    // "b" closes, "n1" opens: it gets "f", not the freed "s".
    sw.handle(Event::Windows(list(&["n1", "a", "x"])));
    assert_eq!(
        shown(&sw),
        [
            ("a".into(), Some('a')),
            ("x".into(), Some('d')),
            ("n1".into(), Some('f')),
        ]
    );

    // Two more windows: the most recent gets "h", then none are left.
    sw.handle(Event::Windows(list(&["n3", "n2", "n1", "a", "x"])));
    assert_eq!(
        shown(&sw),
        [
            ("n2".into(), None),
            ("n1".into(), Some('f')),
            ("a".into(), Some('a')),
            ("x".into(), Some('d')),
            ("n3".into(), Some('h')),
        ]
    );

    // After closing, the held "s" goes to "n2".
    sw.handle(Event::Hide);
    sw.handle(Event::Show(ModeKind::Hotkey));
    assert_eq!(shown(&sw)[0], ("n2".into(), Some('s')));
}

#[test]
fn hide_and_quit_close_only_when_open() {
    let mut sw = switcher(&["a"]);

    assert_eq!(sw.handle(Event::Hide), Action::None);
    sw.handle(Event::Show(ModeKind::Search));
    assert_eq!(sw.handle(Event::Quit), Action::Close);
    assert!(sw.view().is_none());
}

#[test]
fn empty_list_shows_empty_view() {
    let mut sw = switcher(&[]);

    sw.handle(Event::Show(ModeKind::Hotkey));
    assert!(sw.view().unwrap().is_empty());
    assert_eq!(sw.handle(Event::Key(Key::Down)), Action::Redraw);
}

/// Switches to search mode on `/`, like the hotkey mode.
struct Slash;

impl Mode for Slash {
    fn on_key(&mut self, _key: Key, s: &mut State) -> Action {
        s.mode = ModeKind::Search;
        Action::Redraw
    }
}

#[test]
fn mode_switch_from_a_mode_refreshes_the_new_mode() {
    let mut sw = Switcher::new("asd", Slash, Filter);
    let list = ["x", "firefox", "kate"].iter().map(|id| window(id, id));

    sw.handle(Event::Windows(list.collect()));
    sw.handle(Event::Show(ModeKind::Hotkey));
    sw.state.as_mut().unwrap().query = "fire".into();
    assert_eq!(sw.handle(letter('/')), Action::Redraw);
    assert_eq!(sw.view().unwrap().mode, ModeKind::Search);
    assert_eq!(sw.view().unwrap().len(), 1); // Filter refreshed
}
