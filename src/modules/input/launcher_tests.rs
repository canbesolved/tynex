// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Launcher grouping tests.

use super::*;
use crate::core::window::WindowId;

fn window(id: &str, app_id: &str, app_name: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(WindowId(id.into())),
        app_id: app_id.into(),
        app_name: app_name.into(),
        icon: None,
        title: title.into(),
        highlights: Vec::new(),
    }
}

fn app(id: &str, name: &str) -> AppInfo {
    AppInfo {
        id: id.into(),
        name: name.into(),
        icon: Some(id.into()),
    }
}

fn titles(rows: &[Row]) -> Vec<&str> {
    rows.iter().map(|r| r.title.as_str()).collect()
}

fn targets(rows: &[Row]) -> Vec<RowTarget> {
    rows.iter().map(|r| r.target.clone()).collect()
}

#[test]
fn running_app_lists_windows_then_launch() {
    let windows = vec![
        window("w1", "firefox", "Firefox", "Release notes"),
        window("w2", "firefox", "Firefox", "Café"),
        window("w3", "kate", "Kate", "notes.md"),
    ];
    let apps = vec![app("firefox", "Firefox"), app("kate", "Kate")];
    let out = merge("fire", &windows, &apps);

    assert_eq!(titles(&out), ["Release notes", "Café", " ↳  New window"]);
    assert!(matches!(
        out.last().map(|r| &r.target),
        Some(RowTarget::Launch(id)) if id == "firefox"
    ));
}

#[test]
fn not_running_app_is_one_launch_row() {
    let windows = vec![window("w1", "kate", "Kate", "notes.md")];
    let apps = vec![app("firefox", "Firefox"), app("kate", "Kate")];
    let out = merge("fire", &windows, &apps);

    assert_eq!(titles(&out), ["Firefox"]);
    assert_eq!(targets(&out), [RowTarget::Launch("firefox".into())]);
}

#[test]
fn window_without_desktop_entry_stays_a_window() {
    let windows = vec![window("w1", "custom", "Custom", "Scratch")];
    let out = merge("scratch", &windows, &[]);

    assert_eq!(titles(&out), ["Scratch"]);
    assert_eq!(targets(&out), [RowTarget::Window(WindowId("w1".into()))]);
}

#[test]
fn code_does_not_rank_rustdesk_above_code() {
    // Reverse-DNS ids used to match `code` via letters in
    // `org.rustdesk.RustDesk` (with one typo).
    let windows =
        vec![window("w1", "org.rustdesk.RustDesk", "RustDesk", "Remote")];
    let apps = vec![
        app("org.rustdesk.RustDesk", "RustDesk"),
        app("code", "Code - OSS"),
    ];
    let out = merge("code", &windows, &apps);

    assert_eq!(titles(&out), ["Code - OSS"]);
    assert_eq!(targets(&out), [RowTarget::Launch("code".into())]);
}

#[test]
fn running_app_beats_idle_app_on_equal_score() {
    let windows = vec![window(
        "w1",
        "cursor",
        "Cursor",
        "Git: Changes - tynex - Cursor",
    )];
    let apps = vec![
        app("calf", "Calf Plugin Pack for JACK"),
        app("cursor", "Cursor"),
        app("chromium", "Chromium Web Browser"),
    ];
    let out = merge("c", &windows, &apps);

    assert_eq!(out[0].app_id, "cursor");
    assert!(matches!(out[0].target, RowTarget::Window(_)));
}
