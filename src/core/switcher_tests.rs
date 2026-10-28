// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for the switcher, with a minimal stand-in mode.

use super::*;
use crate::core::window::WindowId;

/// Typing filters titles by substring; `Enter` activates; `Esc` closes;
/// arrows move.
pub(super) struct Filter;

impl Mode for Filter {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        match key {
            Key::Char { text, .. } => {
                s.query.push(text);
                self.refresh(s);
                Action::Redraw
            }
            Key::Down => {
                s.select_next();
                Action::Redraw
            }
            Key::Enter => s
                .selected_id()
                .map_or(Action::None, |id| Action::Activate(id.clone())),
            Key::Esc => Action::Close,
            _ => Action::None,
        }
    }

    fn refresh(&mut self, s: &mut State) {
        let query = s.query.clone();

        s.output = s
            .rows
            .iter()
            .filter(|r| r.title.contains(&query))
            .cloned()
            .collect();
        s.selected = 0;
    }
}

fn window(id: &str, app: &str, title: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId(id.into()),
        app_id: id.into(),
        app_name: app.into(),
        title: title.into(),
        icon: None,
    }
}

/// Windows whose ids are also their titles, most recent first.
pub(super) fn list(ids: &[&str]) -> Vec<WindowInfo> {
    ids.iter().map(|id| window(id, "", id)).collect()
}

/// A hidden switcher in most recently used order.
pub(super) fn build(action: HotkeyAction) -> Switcher<Filter> {
    Switcher::new(action, true, SortBy::Recently, Filter)
}

fn switcher(ids: &[&str]) -> Switcher<Filter> {
    let mut sw = build(HotkeyAction::Hide);

    assert_eq!(sw.handle(Event::Windows(list(ids))), Action::None);
    sw
}

fn shown(sw: &Switcher<Filter>) -> Vec<String> {
    sw.view()
        .expect("popup is open")
        .rows()
        .map(|r| match &r.target {
            crate::core::view::RowTarget::Window(id) => id.0.clone(),
            crate::core::view::RowTarget::Launch(id) => id.clone(),
        })
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
fn show_moves_active_window_last_and_selects_the_previous() {
    let mut sw = switcher(&["active", "prev", "older"]);

    assert_eq!(sw.handle(Event::Show), Action::Redraw);
    assert_eq!(shown(&sw), ["prev", "older", "active"]);
    assert_eq!(sw.view().unwrap().selected, 0);
}

#[test]
fn enter_right_after_show_activates_the_previous_window() {
    let mut sw = switcher(&["active", "prev"]);

    sw.handle(Event::Show);
    assert_eq!(
        sw.handle(Event::Key(Key::Enter)),
        Action::Activate(WindowId("prev".into()))
    );
    assert!(sw.view().is_none());
}

#[test]
fn app_order_selects_the_previous_window() {
    let mut sw = Switcher::new(HotkeyAction::Hide, true, SortBy::App, Filter);

    sw.handle(Event::Windows(vec![
        window("active", "Kate", "a"),
        window("prev", "Kate", "p"),
        window("older", "Dolphin", "o"),
    ]));
    sw.handle(Event::Show);
    assert_eq!(shown(&sw), ["older", "prev", "active"]);
    assert_eq!(sw.view().unwrap().selected, 1);
}

#[test]
fn titles_are_sanitized_on_arrival() {
    let mut sw = build(HotkeyAction::Hide);

    sw.handle(Event::Windows(vec![window("x", "", " a\u{202E}\tb ")]));
    sw.handle(Event::Show);
    assert_eq!(sw.view().unwrap().rows().next().unwrap().title, "a b");
}

#[test]
fn keys_go_to_the_mode() {
    let mut sw = switcher(&["x", "firefox", "kate"]);

    sw.handle(Event::Show);
    assert_eq!(sw.handle(letter('k')), Action::Redraw);
    assert_eq!(shown(&sw), ["kate"]);
    assert_eq!(sw.handle(Event::Key(Key::Tab)), Action::None);
}

#[test]
fn update_while_open_keeps_query_and_selection() {
    let mut sw = switcher(&["x", "a", "b"]);

    sw.handle(Event::Show);
    sw.handle(Event::Key(Key::Down)); // Select "b"

    assert_eq!(
        sw.handle(Event::Windows(list(&["b", "x", "a"]))),
        Action::Redraw
    );
    assert_eq!(shown(&sw), ["x", "a", "b"]);
    assert_eq!(sw.view().unwrap().selected, 2);

    sw.handle(letter('a'));
    sw.handle(Event::Windows(list(&["b", "a", "x"])));
    assert_eq!(shown(&sw), ["a"]); // Query kept and applied
}

#[test]
fn hide_and_quit_close_only_when_open() {
    let mut sw = switcher(&["a"]);

    assert_eq!(sw.handle(Event::Hide), Action::None);
    sw.handle(Event::Show);
    assert_eq!(sw.handle(Event::Quit), Action::Close);
    assert!(sw.view().is_none());
}

#[test]
fn empty_list_shows_empty_view() {
    let mut sw = switcher(&[]);

    sw.handle(Event::Show);
    assert!(sw.view().unwrap().is_empty());
    assert_eq!(sw.handle(Event::Key(Key::Down)), Action::Redraw);
    assert_eq!(sw.handle(Event::Key(Key::Enter)), Action::None);
}
