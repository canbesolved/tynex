// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hints folder: created with the default script, edits kept.

use super::*;

/// A fresh home folder under the system temp dir.
fn home(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("tynex_hint_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);

    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_new_folder_gets_the_default_script_only() {
    let home = home("new");
    let dir = home.join(HINTS_DIR);
    let path = prepare_in(&home, false).unwrap().expect("written");
    let mode = fs::metadata(&path).unwrap().permissions().mode();

    assert_eq!(path, dir.join(DEFAULT));
    assert_eq!(fs::read_to_string(&path).unwrap(), DEFAULT_SCRIPT);
    assert_eq!(mode & 0o777, 0o755);
    assert!(DEFAULT_SCRIPT.starts_with("#!/usr/bin/env bash\n"));
    assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn edits_are_kept_and_a_deleted_script_returns_only_if_used() {
    let home = home("kept");
    let path = home.join(HINTS_DIR).join(DEFAULT);

    prepare_in(&home, false).unwrap();
    fs::write(&path, "edited").unwrap();
    assert_eq!(prepare_in(&home, true).unwrap(), None);
    assert_eq!(fs::read_to_string(&path).unwrap(), "edited");

    fs::remove_file(&path).unwrap();
    assert_eq!(prepare_in(&home, false).unwrap(), None);
    assert!(!path.exists());
    assert_eq!(prepare_in(&home, true).unwrap(), Some(path));
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn the_default_text_runs_the_default_script() {
    assert_eq!(HINT_TEXT, format!("~/{HINTS_DIR}/{DEFAULT}"));
}
