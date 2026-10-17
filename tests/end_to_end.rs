// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Config, mock source, event queue and switcher working together.

use tynex::adapters::mock::MockSource;
use tynex::adapters::{self, Kind, Source};
use tynex::config;
use tynex::core::event::{Action, Event, Key, ModeKind};
use tynex::core::queue::{EventRx, channel};
use tynex::core::switcher::Switcher;
use tynex::core::traits::WindowSource;
use tynex::core::window::WindowId;
use tynex::modes::hotkey::HotkeyMode;
use tynex::modes::search::SearchMode;

type App = Switcher<HotkeyMode, SearchMode>;

/// The registry's pick for `desktop`, built without a session bus.
fn mock(desktop: config::Desktop) -> Source {
    assert_eq!(adapters::pick(desktop, None), Ok(Kind::Mock));
    Source::Mock(MockSource::new())
}

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
    let mut source = mock(config.desktop);
    let (tx, rx) = channel();
    let mut sw = Switcher::new(
        &config.keys,
        config.hotkey_action,
        config.appearance.title.clone(),
        config.sort_by,
        HotkeyMode,
        SearchMode,
    );

    source.start(tx).expect("mock starts");
    pump(&rx, &mut sw);

    // The previous window gets `a`; rows follow their letters.
    let start = [("Café".into(), Some('a')), ("Música".into(), Some('s'))];

    assert_eq!(first_rows(&mut sw, 2), start);

    // Letters and rows stay in place while switching (`sort_by = "key"`).
    switch(&mut sw, &mut source, &rx, 'a');
    switch(&mut sw, &mut source, &rx, 's');
    assert_eq!(first_rows(&mut sw, 2), start);

    let Source::Mock(mock) = &source else {
        unreachable!("mock source")
    };

    assert_eq!(
        mock.activations(),
        [WindowId("mock-02".into()), WindowId("mock-03".into())]
    );
}

#[test]
fn titles_from_the_source_are_formatted() {
    let text = "desktop = \"mock\"\n[appearance]\n\
                title_format = \"{app} : {title}\"\n";
    let config = config::parse(text, &|_| false).unwrap();
    let mut source = mock(config.desktop);
    let (tx, rx) = channel();
    let mut sw = Switcher::new(
        &config.keys,
        config.hotkey_action,
        config.appearance.title.clone(),
        config.sort_by,
        HotkeyMode,
        SearchMode,
    );

    source.start(tx).unwrap();
    pump(&rx, &mut sw);
    sw.handle(Event::Show(config.mode));

    let titles: Vec<_> =
        sw.view().unwrap().rows().map(|r| r.title.clone()).collect();

    // Sanitized, then " — Okular" is stripped.
    assert!(titles.contains(&"Okular : manual.pdf fdp.exe".to_owned()));
    assert!(titles.contains(&"Firefox : Café".to_owned()));
}
