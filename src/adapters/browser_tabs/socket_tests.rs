// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Socket cases. Each test uses its own temporary folder.

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::time::Duration;

use super::Socket;
use crate::adapters::browser_tabs::frame;
use crate::core::browser_tabs::{SourceId, TabRef, TabsUpdate};

fn dir() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "tynex-tabs-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));

    std::fs::create_dir_all(&path).unwrap();
    path
}

fn connect(dir: &std::path::Path) -> UnixStream {
    let stream = UnixStream::connect(dir.join("tabs.sock")).unwrap();

    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    stream
}

fn send(stream: &mut UnixStream, json: &str) {
    stream.write_all(&frame::encode(json.as_bytes())).unwrap();
}

fn take(stream: &mut UnixStream) -> String {
    let mut len = [0u8; 4];

    stream.read_exact(&mut len).unwrap();
    let mut buf = vec![0; u32::from_ne_bytes(len) as usize];

    stream.read_exact(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

fn closed(stream: &mut UnixStream) -> bool {
    matches!(stream.read(&mut [0u8; 1]), Ok(0))
}

/// A socket with one host that said hello.
fn ready() -> (PathBuf, Socket, UnixStream) {
    let dir = dir();
    let mut socket = Socket::bind(&dir).unwrap();
    let mut client = connect(&dir);

    send(&mut client, r#"{"v":1,"type":"hello","pid":7}"#);
    socket.drive();
    (dir, socket, client)
}

#[test]
fn binds_by_rename_with_mode_0600() {
    use std::os::unix::fs::PermissionsExt;

    let dir = dir();
    let socket = Socket::bind(&dir).unwrap();
    let mode = std::fs::metadata(dir.join("tabs.sock"))
        .unwrap()
        .permissions();

    assert_eq!(mode.mode() & 0o777, 0o600);
    assert!(!dir.join(".tabs.sock.new").exists());
    drop(socket);
    assert!(!dir.join("tabs.sock").exists());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_first_frame_other_than_hello_closes() {
    let dir = dir();
    let mut socket = Socket::bind(&dir).unwrap();
    let mut bad_json = connect(&dir);
    let mut no_hello = connect(&dir);

    send(&mut bad_json, "nope");
    send(&mut no_hello, r#"{"v":1,"type":"tabs","req":1}"#);
    let batch = socket.drive();

    assert_eq!(batch.updates.len(), 2);
    assert!(closed(&mut bad_json) && closed(&mut no_hello));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn eof_reports_gone() {
    let (dir, mut socket, client) = ready();

    drop(client);
    let batch = socket.drive();

    assert_eq!(batch.updates, [TabsUpdate::Gone(SourceId(1))]);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn ninth_connection_is_refused() {
    let dir = dir();
    let mut socket = Socket::bind(&dir).unwrap();
    let _clients: Vec<_> = (0..8).map(|_| connect(&dir)).collect();

    socket.drive();
    let mut extra = connect(&dir);

    socket.drive();
    assert!(closed(&mut extra));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn stale_socket_is_replaced_and_a_live_one_is_kept() {
    let dir = dir();
    let listener = UnixListener::bind(dir.join("tabs.sock")).unwrap();

    assert!(Socket::bind(&dir).is_none());
    drop(listener);
    assert!(Socket::bind(&dir).is_some());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn list_on_show_reaches_late_hellos_and_drops_late_replies() {
    let (dir, mut socket, mut client) = ready();

    socket.list();
    socket.drive();
    assert!(take(&mut client).contains(r#""req":1"#));
    let mut late = connect(&dir);

    send(&mut late, r#"{"v":1,"type":"hello","pid":8}"#);
    socket.drive();
    assert!(take(&mut late).contains(r#""type":"list""#));
    socket.hide();
    send(
        &mut client,
        r#"{"v":1,"type":"tabs","req":1,"browser":"Brave","tabs":[]}"#,
    );
    assert_eq!(socket.drive().updates, []);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn only_the_pending_reply_counts() {
    let (dir, mut socket, mut client) = ready();
    let tab = TabRef {
        source: SourceId(1),
        id: 5,
    };

    socket.activate(&tab);
    socket.drive();
    assert!(take(&mut client).contains(r#""id":5"#));
    send(
        &mut client,
        r#"{"v":1,"type":"activated","req":99,"ok":true,"window":1}"#,
    );
    assert_eq!(socket.drive().updates, []);
    send(
        &mut client,
        r#"{"v":1,"type":"activated","req":1,"ok":false,"window":-1}"#,
    );
    let done = TabsUpdate::Activated {
        source: SourceId(1),
        ok: false,
        window: -1,
    };

    assert_eq!(socket.drive().updates, [done]);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn icons_are_decoded_and_reported_once() {
    let (dir, mut socket, mut client) = ready();

    send(
        &mut client,
        r#"{"v":1,"type":"icons","icons":[{"key":"k","png":"YQ=="},
        {"key":"bad","png":"!"}]}"#,
    );
    let batch = socket.drive();

    assert_eq!(batch.updates, [TabsUpdate::Icons]);
    assert_eq!(batch.icons.len(), 1);
    assert_eq!(batch.icons[0].png, b"a");
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn an_oversized_frame_closes() {
    let (dir, mut socket, mut client) = ready();

    client
        .write_all(&(32 * 1024 * 1024 + 1u32).to_ne_bytes())
        .unwrap();
    assert_eq!(socket.drive().updates, [TabsUpdate::Gone(SourceId(1))]);
    assert!(closed(&mut client));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn listing_lasts_until_every_asked_host_answers() {
    let (dir, mut socket, mut client) = ready();

    assert!(!socket.listing());
    socket.list();
    socket.drive();
    assert!(socket.listing());
    assert!(take(&mut client).contains(r#""req":1"#));
    send(
        &mut client,
        r#"{"v":1,"type":"tabs","req":1,"browser":"Brave","tabs":[]}"#,
    );
    socket.drive();
    assert!(!socket.listing());

    socket.list();
    assert!(socket.listing());
    drop(client); // A host that leaves counts as answered
    socket.drive();
    assert!(!socket.listing());
    let _ = std::fs::remove_dir_all(dir);
}
