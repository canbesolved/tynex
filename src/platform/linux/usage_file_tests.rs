// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The usage file is private and round-trips.

use std::os::unix::fs::PermissionsExt;

use super::*;

#[test]
fn the_file_is_written_private_and_read_back() {
    let home = std::env::temp_dir()
        .join(format!("tynex_usage_{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);

    assert_eq!(read(&home).unwrap_err().kind(), io::ErrorKind::NotFound);

    let path = write(&home, "2\tapp\tkate\n").unwrap();
    let mode = fs::metadata(&path).unwrap().permissions().mode();

    assert_eq!(path, home.join(DIR).join(FILE));
    assert_eq!(mode & 0o777, 0o600);
    assert_eq!(read(&home).unwrap(), "2\tapp\tkate\n");
    fs::remove_dir_all(&home).unwrap();
}
