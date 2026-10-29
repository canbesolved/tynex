// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hint command tests.

use std::thread;
use std::time::{Duration, Instant};

use super::{Hint, frame, next_update};

fn hint(command: &str, update: Duration) -> Hint {
    Hint {
        command: command.into(),
        update,
        last: String::new(),
        step: 0,
        tick: None,
        repeat: None,
        reap: None,
        child: None,
    }
}

#[test]
fn dots_then_repeat() {
    assert_eq!(frame(0), ".");
    assert_eq!(frame(1), "..");
    assert_eq!(frame(2), "...");
    assert_eq!(frame(3), "");
    assert_eq!(frame(4), ".");

    let exit = Instant::now();

    assert_eq!(next_update(0, exit), None);
    assert_eq!(next_update(3, exit), Some(exit + Duration::from_secs(3)));
}

#[test]
fn spawn_returns_before_sleep_ends() {
    let start = Instant::now();
    let mut hint = hint("sleep 30", Duration::ZERO);
    let text = hint.on_show(Instant::now());

    assert_eq!(text, ".");
    assert!(start.elapsed() < Duration::from_secs(2));
    assert!(hint.child.is_some());
}

#[test]
fn finished_command_yields_the_first_line() {
    let mut hint = hint(r"printf 'hi\nthere\n'", Duration::from_secs(30));

    assert_eq!(hint.on_show(Instant::now()), ".");

    let line = wait_until(&mut hint, |text| text == "hi");

    assert_eq!(line.as_deref(), Some("hi"));
    assert!(hint.child.is_none());
    assert!(hint.repeat.is_some());
}

#[test]
fn missing_command_keeps_an_empty_hint() {
    let mut hint = hint("tynex-no-such-bin", Duration::ZERO);

    assert_eq!(hint.on_show(Instant::now()), ".");
    wait_until(&mut hint, |_| false);
    assert!(hint.child.is_none());
    assert_eq!(hint.last, "");
}

/// Polls until `done` or two seconds pass. Returns the last string.
fn wait_until(hint: &mut Hint, done: impl Fn(&str) -> bool) -> Option<String> {
    let start = Instant::now();
    let mut last = None;

    while start.elapsed() < Duration::from_secs(2) {
        if let Some(text) = hint.poll(Instant::now(), true, true) {
            last = Some(text);
        }
        if last.as_deref().is_some_and(&done) || hint.child.is_none() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    last
}
