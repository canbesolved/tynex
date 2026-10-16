// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Config, mock source, event queue and switcher working together.

use tynex::adapters::{self, Source};
use tynex::config;
use tynex::core::event::{Action, Event, Key, ModeKind};
use tynex::core::queue::{EventRx, channel};
use tynex::core::state::State;
use tynex::core::switcher::Switcher;
use tynex::core::traits::{Mode, WindowSource};
use tynex::core::window::WindowId;

/// Minimal hotkey mode: a row letter activates its window.
struct Letters;

impl Mode for Letters {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action {
        match key {
            Key::Char { text, .. } => s
                .id_for_key(text)
                .map_or(Action::None, |id| Action::Activate(id.clone())),
            _ => Action::Close,
        }
    }
}

type App = Switcher<Letters, Letters>;

/// Feeds all queued events to the switcher.
fn pump(rx: &EventRx, sw: &mut App) {
    while let Some(event) = rx.recv() {
        sw.handle(event);
    }
}

/// Opens the popup and presses `letter`, then activates the result.
fn switch(sw: &mut App, source: &mut Source, rx: &EventRx, letter: char) {
    sw.handle(Event::Show(ModeKind::Hotkey));

    let key = Key::Char {
        text: letter,
        latin: None,
    };

    match sw.handle(Event::Key(key)) {
        Action::Activate(id) => source.activate(&id).expect("known window"),
        other => panic!("expected activation, got {other:?}"),
    }
    pump(rx, sw);
}

fn first_rows(sw: &mut App, n: usize) -> Vec<(String, Option<char>)> {
    sw.handle(Event::Show(ModeKind::Hotkey));

    let rows = sw.view().expect("popup is open").rows();
    let first = rows.take(n).map(|r| (r.title.clone(), r.key)).collect();

    sw.handle(Event::Hide);
    first
}

#[test]
fn mock_source_drives_the_switcher() {
    let text = "desktop = \"mock\"\nkeys = \"asdf\"\n";
    let config = config::parse(text, &|_| false).expect("valid config");
    let mut source = adapters::select(config.desktop, None).expect("mock");
    let (tx, rx) = channel();
    let mut sw = Switcher::new(config.keys, Letters, Letters);

    source.start(tx).expect("mock starts");
    pump(&rx, &mut sw);

    // The active window (mock-01) is last; the previous one gets `a`.
    assert_eq!(
        first_rows(&mut sw, 2),
        [
            ("Café — Firefox".into(), Some('a')),
            ("Документы — Dolphin".into(), Some('s')),
        ]
    );

    // Pressing `a` twice toggles between the two most recent windows.
    switch(&mut sw, &mut source, &rx, 'a');
    switch(&mut sw, &mut source, &rx, 'a');

    let Source::Mock(mock) = &source;

    assert_eq!(
        mock.activations(),
        [WindowId("mock-02".into()), WindowId("mock-01".into())]
    );
}

#[test]
fn titles_from_the_source_are_sanitized() {
    let config = config::parse("desktop = \"mock\"\n", &|_| false).unwrap();
    let mut source = adapters::select(config.desktop, None).unwrap();
    let (tx, rx) = channel();
    let mut sw = Switcher::new(config.keys, Letters, Letters);

    source.start(tx).unwrap();
    pump(&rx, &mut sw);
    sw.handle(Event::Show(config.mode));

    let titles: Vec<_> =
        sw.view().unwrap().rows().map(|r| r.title.clone()).collect();

    assert!(titles.contains(&"manual.pdf fdp.exe — Okular".to_owned()));
}
