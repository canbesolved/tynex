// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Data files are private, round-trip, and read as empty when missing.

use std::os::unix::fs::PermissionsExt;

use super::*;

#[test]
fn the_file_is_written_private_and_read_back() {
    let home = std::env::temp_dir()
        .join(format!("tynex_data_file_{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);

    assert_eq!(read_in(&home, "stats.tsv").unwrap(), "");

    let path = write_in(&home, "stats.tsv", "2\tapp\tkate\n").unwrap();
    let mode = fs::metadata(&path).unwrap().permissions().mode();

    assert_eq!(path, home.join(DIR).join("stats.tsv"));
    assert_eq!(mode & 0o777, 0o600);
    assert_eq!(read_in(&home, "stats.tsv").unwrap(), "2\tapp\tkate\n");
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn a_folder_in_its_place_is_an_error() {
    let home = std::env::temp_dir()
        .join(format!("tynex_data_dir_{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);

    fs::create_dir_all(home.join(DIR).join("stats.tsv")).unwrap();
    assert!(read_in(&home, "stats.tsv").is_err());
    fs::remove_dir_all(&home).unwrap();
}
