// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Catalog scan tests with a temporary applications tree.

use std::fs;
use std::path::Path;

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
