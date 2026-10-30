// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Config, mock source, event queue and switcher working together.

use tynex::adapters::mock::MockSource;
use tynex::adapters::{self, Kind, Source};
use tynex::config::{self, Config};
use tynex::core::event::{Action, Event, Key};
use tynex::core::queue::{EventRx, channel};
use tynex::core::switcher::Switcher;
use tynex::core::traits::WindowSource;
use tynex::core::window::WindowId;
use tynex::modes::prompt::PromptMode;
use tynex::modules::input::Input;

type App = Switcher<PromptMode>;

/// The registry's pick for `desktop`, built without a session bus.
fn mock(desktop: config::Desktop) -> Source {
    assert_eq!(adapters::pick(desktop, None), Ok(Kind::Mock));
    Source::Mock(MockSource::new())
}

/// A switcher built from `config`, fed by a started mock source.
fn start(config: &Config) -> (App, Source, EventRx) {
    let mut source = mock(config.main.desktop);
    let (tx, rx) = channel();
    let mut sw = Switcher::new(
        config.hotkeys.action,
        config.taskbar.titles.strip_app_name,
        config.taskbar.titles.sort_by,
        PromptMode::new(
            Input::new(config.input_bar.launcher.clone(), Vec::new()),
            config.taskbar.mode,
            config.output_bar.max_rows,
        ),
    );

    source.start(tx).expect("mock starts");
    pump(&rx, &mut sw);
    (sw, source, rx)
}

/// Feeds all queued events to the switcher.
fn pump(rx: &EventRx, sw: &mut App) {
    while let Some(event) = rx.recv() {
        sw.handle(event);
    }
}

/// Opens the popup, types `query`, presses `Enter` and activates the
/// result.
fn switch(sw: &mut App, source: &mut Source, rx: &EventRx, query: &str) {
    sw.handle(Event::Show);
    for c in query.chars() {
        sw.handle(Event::Key(Key::Char {
            text: c,
            latin: None,
        }));
    }

    match sw.handle(Event::Key(Key::Enter)) {
        Action::Activate(id) => source.activate(&id).expect("known window"),
        other => panic!("expected activation, got {other:?}"),
    }
    pump(rx, sw);
}

#[test]
fn mock_source_drives_the_switcher() {
    let config =
        config::parse("[main]\ndesktop = \"mock\"\n", &|_| false).unwrap();
    let (mut sw, mut source, rx) = start(&config);

    // Firefox has two windows, so the first Enter opens its dropdown
    // and the next Enter activates the first title, Café (mock-02).
    sw.handle(Event::Show);
    assert_eq!(sw.handle(Event::Key(Key::Enter)), Action::Redraw);
    match sw.handle(Event::Key(Key::Enter)) {
        Action::Activate(id) => source.activate(&id).expect("known window"),
        other => panic!("expected activation, got {other:?}"),
    }
    pump(&rx, &mut sw);
    // The previous app now has one window, so Enter activates it.
    switch(&mut sw, &mut source, &rx, "");
    // Typing picks the best match.
    switch(&mut sw, &mut source, &rx, "musica");

    let Source::Mock(mock) = &source else {
        unreachable!("mock source")
    };

    assert_eq!(
        mock.activations(),
        ["mock-02", "mock-01", "mock-03"].map(|id| WindowId(id.into()))
    );
}

#[test]
fn titles_keep_the_app_name_by_default() {
    let config =
        config::parse("[main]\ndesktop = \"mock\"\n", &|_| false).unwrap();
    let (mut sw, ..) = start(&config);

    sw.handle(Event::Show);
    assert_eq!(
        sw.view().unwrap().rows().next().unwrap().title,
        "Café — Firefox"
    );
}

#[test]
fn titles_from_the_source_are_stripped() {
    let text = "[main]\ndesktop = \"mock\"\n\
                [title_bar]\nstrip_app_name = true\n";
    let config = config::parse(text, &|_| false).unwrap();
    let (mut sw, ..) = start(&config);

    sw.handle(Event::Show);

    let rows: Vec<_> = sw
        .view()
        .unwrap()
        .rows()
        .map(|r| (r.app_name.clone(), r.title.clone()))
        .collect();

    // Sanitized, then " — Okular" is stripped; the name stays apart.
    assert!(rows.contains(&("Okular".into(), "manual.pdf fdp.exe".into())));
    assert!(rows.contains(&("Firefox".into(), "Café".into())));
}
