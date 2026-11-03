// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Catalog scan tests with a temporary applications tree.

use std::fs;
use std::path::{Path, PathBuf};

use super::{Apps, Entry};
use crate::config::LauncherMode;
use crate::core::app::AppInfo;
use crate::platform::linux::apps::parse::scan;

fn write_desktop(dir: &Path, rel: &str, body: &str) {
    let path = dir.join(rel);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

#[test]
fn first_id_wins_across_dirs() {
    let root = std::env::temp_dir()
        .join(format!("tynex_apps_test_{}", std::process::id()));
    let a = root.join("a/applications");
    let b = root.join("b/applications");

    write_desktop(
        &a,
        "demo.desktop",
        "[Desktop Entry]\nType=Application\nName=First\nExec=first\n",
    );
    write_desktop(
        &b,
        "demo.desktop",
        "[Desktop Entry]\nType=Application\nName=Second\nExec=second\n",
    );

    let entries = scan(&[root.join("a"), root.join("b")], &[], false);

    assert_eq!(entries["demo"].info.name, "First");
    fs::remove_dir_all(root).unwrap();
}

fn entry(id: &str, terminal: bool) -> (String, Entry) {
    let info = AppInfo {
        id: id.into(),
        name: id.into(),
        icon: Some(format!("{id}-icon")),
    };

    (
        id.into(),
        Entry {
            info,
            exec: id.into(),
            path: PathBuf::from(format!("{id}.desktop")),
            terminal,
        },
    )
}

fn catalog(mode: LauncherMode) -> Apps {
    Apps {
        entries: [entry("kate", false), entry("htop", true)].into(),
        children: Vec::new(),
        mode,
    }
}

fn ids(list: &[AppInfo]) -> Vec<&str> {
    list.iter().map(|a| a.id.as_str()).collect()
}

#[test]
fn the_launcher_list_follows_its_mode() {
    assert_eq!(ids(&catalog(LauncherMode::All).list()), ["htop", "kate"]);
    assert_eq!(ids(&catalog(LauncherMode::GuiOnly).list()), ["kate"]);
    assert_eq!(ids(&catalog(LauncherMode::Off).list()), Vec::<&str>::new());
}

#[test]
fn pinned_apps_include_terminal_and_unknown_ids() {
    let apps = catalog(LauncherMode::Off);
    let pinned = apps.pinned(&["htop".into(), "gone".into()]);

    assert_eq!(ids(&pinned), ["htop", "gone"]);
    assert_eq!(pinned[0].icon.as_deref(), Some("htop-icon"));
    assert_eq!(pinned[1].icon, None);
}

#[test]
fn nothing_is_scanned_without_launcher_or_pins() {
    assert_eq!(Apps::scan(LauncherMode::Off, &[]).entries.len(), 0);
}
