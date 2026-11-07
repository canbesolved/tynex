// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Files of the daemon in `~/.local/share/tynex/`: read in part when
//! too big, written atomically and private. The usage stats and the
//! qs counter use them.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::platform::linux::paths;

/// Folder under the home folder, next to the hints.
const DIR: &str = ".local/share/tynex";

/// Largest file read, in bytes. A bigger one is read in part.
const MAX_BYTES: u64 = 1 << 20;

/// Reads `name`. A missing file, or no home folder, reads as empty.
///
/// # Errors
///
/// Fails when the file exists but cannot be read.
pub fn read(name: &str) -> io::Result<String> {
    match std::env::home_dir() {
        Some(home) => read_in(&home, name),
        None => Ok(String::new()),
    }
}

/// Writes `text` to `name` with mode `0600`, creating the folder.
///
/// # Errors
///
/// Fails with no home folder, or when the file cannot be written.
pub fn write(name: &str, text: &str) -> io::Result<PathBuf> {
    let home = std::env::home_dir()
        .ok_or_else(|| io::Error::other("no home folder"))?;

    write_in(&home, name, text)
}

fn read_in(home: &Path, name: &str) -> io::Result<String> {
    let mut text = String::new();

    match File::open(home.join(DIR).join(name)) {
        Ok(file) => file.take(MAX_BYTES).read_to_string(&mut text)?,
        Err(e) if e.kind() == io::ErrorKind::NotFound => 0,
        Err(e) => return Err(e),
    };
    Ok(text)
}

fn write_in(home: &Path, name: &str, text: &str) -> io::Result<PathBuf> {
    let dir = home.join(DIR);

    fs::create_dir_all(&dir)?;
    paths::write_atomic(&dir, name, text.as_bytes())
}

#[cfg(test)]
#[path = "data_file_tests.rs"]
mod tests;
