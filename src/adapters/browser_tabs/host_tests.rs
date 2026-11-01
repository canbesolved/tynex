// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Host relay and the argument check. No browser is started.

use std::ffi::OsString;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;

use super::{Relay, is_host};
use crate::adapters::browser_tabs::frame::{FrameReader, encode};

fn dir() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "tynex-host-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));

    std::fs::create_dir_all(&path).unwrap();
    path
}

fn frames(bytes: &[u8]) -> Vec<String> {
    FrameReader::new(1 << 20)
        .push(bytes)
        .unwrap()
        .iter()
        .map(|f| String::from_utf8_lossy(f).into_owned())
        .collect()
}

#[test]
fn argument_shape() {
    let id = "chrome-extension://abcdefghijklmnopabcdefghijklmnop/";

    assert!(is_host([OsString::from(id)]));
    assert!(!is_host([OsString::from("chrome-extension://abc/")]));
    assert!(!is_host([OsString::from(id.replace('a', "z"))]));
    assert!(!is_host([OsString::from(id), OsString::from("daemon")]));
    assert!(!is_host(Vec::<OsString>::new()));
}

#[test]
fn connects_relays_and_reports_status() {
    let dir = dir();
    let mut relay = Relay::new(&dir);

    assert!(!relay.try_connect());
    let listener = UnixListener::bind(dir.join("tabs.sock")).unwrap();

    assert!(relay.try_connect());
    relay
        .on_stdin(&encode(br#"{"v":1,"type":"tabs"}"#))
        .unwrap();
    relay.flush();

    let mut daemon = listener.accept().unwrap().0;
    let mut buf = [0u8; 256];
    let n = daemon.read(&mut buf).unwrap();
    let sent = frames(&buf[..n]);

    assert!(sent[0].contains(r#""type":"hello""#));
    assert!(sent[1].contains(r#""type":"tabs""#));
    daemon
        .write_all(&encode(br#"{"v":1,"type":"list"}"#))
        .unwrap();
    relay.on_socket();
    drop(daemon);
    relay.on_socket();
    assert!(relay.socket().is_none());

    let out = frames(&std::mem::take(&mut relay.to_browser));

    assert!(out[0].contains(r#""daemon":true"#));
    assert!(out[1].contains(r#""type":"list""#));
    assert!(out[2].contains(r#""daemon":false"#));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn frames_are_dropped_while_disconnected_and_bad_ones_fail() {
    let mut relay = Relay::new(&dir());

    relay.on_stdin(&encode(b"{}")).unwrap();
    assert!(!relay.wants_write());
    assert!(relay.on_stdin(&0u32.to_ne_bytes()).is_err());
}
