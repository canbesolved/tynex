// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Manifest registration against a temporary home.

use std::fs;
use std::path::Path;

use super::sync;
use crate::config::{Browser, BrowserTabsConfig, Browsers};

fn home() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "tynex-reg-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));

    install(&path, "google-chrome");
    path
}

/// A profile folder as the browser leaves it: with `Local State`.
fn install(home: &Path, folder: &str) {
    let dir = home.join(".config").join(folder);

    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("Local State"), b"{}").unwrap();
}

fn config(enabled: bool, browsers: Browsers) -> BrowserTabsConfig {
    BrowserTabsConfig {
        enabled,
        browsers,
        ..BrowserTabsConfig::default()
    }
}

fn file(home: &Path) -> std::path::PathBuf {
    home.join(".config/google-chrome/NativeMessagingHosts")
        .join("io.github.canbesolved.tynex.json")
}

#[test]
fn writes_only_for_an_existing_browser_folder() {
    let home = home();
    let root = home.join("root");

    assert!(sync(
        &config(true, Browsers::Auto),
        &home,
        Path::new("/usr/bin/tynex"),
        &root,
    ));
    let text = fs::read_to_string(file(&home)).unwrap();

    assert!(text.contains("/usr/bin/tynex"));
    assert!(!home.join(".config/BraveSoftware").exists());
    let before = fs::metadata(file(&home)).unwrap().modified().unwrap();

    assert!(sync(
        &config(true, Browsers::Auto),
        &home,
        Path::new("/usr/bin/tynex"),
        &root,
    ));
    let after = fs::metadata(file(&home)).unwrap().modified().unwrap();

    assert_eq!(before, after);
    assert!(sync(
        &config(true, Browsers::Auto),
        &home,
        Path::new("/opt/tynex"),
        &root,
    ));
    assert!(
        fs::read_to_string(file(&home))
            .unwrap()
            .contains("/opt/tynex")
    );
    assert!(!sync(
        &config(false, Browsers::Auto),
        &home,
        Path::new("/opt/tynex"),
        &root,
    ));
    assert!(!file(&home).exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn a_system_manifest_and_a_foreign_file_win() {
    let home = home();
    let root = home.join("root");
    let system = root.join("etc/opt/chrome/native-messaging-hosts");

    fs::create_dir_all(&system).unwrap();
    fs::write(system.join("io.github.canbesolved.tynex.json"), b"{}").unwrap();
    assert!(sync(
        &config(true, Browsers::List(vec![Browser::Chrome])),
        &home,
        Path::new("/usr/bin/tynex"),
        &root,
    ));
    assert!(!file(&home).exists());

    let foreign = file(&home);
    fs::create_dir_all(foreign.parent().unwrap()).unwrap();
    fs::write(&foreign, r#"{"path":"/usr/bin/other"}"#).unwrap();
    assert!(!sync(
        &config(false, Browsers::Auto),
        &home,
        Path::new("/usr/bin/tynex"),
        &root,
    ));
    assert!(foreign.is_file());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn a_missing_listed_browser_does_not_count() {
    let home = home();
    let root = home.join("root");

    assert!(!sync(
        &config(true, Browsers::List(vec![Browser::Brave])),
        &home,
        Path::new("/usr/bin/tynex"),
        &root,
    ));
    assert!(!file(&home).exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn channels_count_and_a_bare_folder_does_not() {
    let home = home();
    let root = home.join("root");
    let bare = home.join(".config/BraveSoftware/Brave-Browser");
    let stale =
        bare.join("NativeMessagingHosts/io.github.canbesolved.tynex.json");

    fs::create_dir_all(stale.parent().unwrap()).unwrap();
    fs::write(&stale, r#"{"path":"/usr/bin/tynex"}"#).unwrap();
    install(&home, "BraveSoftware/Brave-Origin");
    assert!(sync(
        &config(true, Browsers::List(vec![Browser::Brave])),
        &home,
        Path::new("/usr/bin/tynex"),
        &root,
    ));
    assert!(!stale.exists());
    assert!(
        home.join(".config/BraveSoftware/Brave-Origin/NativeMessagingHosts")
            .join("io.github.canbesolved.tynex.json")
            .is_file()
    );
    assert!(!file(&home).exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn chromium_uses_its_own_folders() {
    let home = home();
    let root = home.join("root");
    let user = home
        .join(".config/chromium/NativeMessagingHosts")
        .join("io.github.canbesolved.tynex.json");
    let chromium = config(true, Browsers::List(vec![Browser::Chromium]));

    install(&home, "chromium");
    assert!(sync(&chromium, &home, Path::new("/usr/bin/tynex"), &root));
    assert!(user.is_file());
    assert!(!file(&home).exists());

    let system = root.join("etc/chromium/native-messaging-hosts");

    fs::create_dir_all(&system).unwrap();
    fs::write(system.join("io.github.canbesolved.tynex.json"), b"{}").unwrap();
    assert!(sync(&chromium, &home, Path::new("/usr/bin/tynex"), &root));
    assert!(!user.exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn an_unselected_browser_loses_only_our_manifest() {
    let home = home();
    let root = home.join("root");
    let exe = Path::new("/usr/bin/tynex");

    install(&home, "BraveSoftware/Brave-Browser");
    assert!(sync(&config(true, Browsers::Auto), &home, exe, &root));
    assert!(file(&home).is_file());
    assert!(sync(
        &config(true, Browsers::List(vec![Browser::Brave])),
        &home,
        exe,
        &root,
    ));
    assert!(!file(&home).exists());
    assert!(home.join(".config/google-chrome/Local State").is_file());
    let _ = fs::remove_dir_all(home);
}
