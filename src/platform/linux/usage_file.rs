// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `~/.local/share/tynex/usage_stats.tsv`: the usage scores, read on
//! start and written after a change (ADR-0095).

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::core::usage::Usage;
use crate::platform::linux::paths;

/// Folder of the file under the home folder, next to the hints.
const DIR: &str = ".local/share/tynex";

/// File name.
const FILE: &str = "usage_stats.tsv";

/// Largest file read, in bytes. A bigger one is read in part.
const MAX_BYTES: u64 = 1 << 20;

/// Reads the scores. A missing or unreadable file gives none.
#[must_use]
pub fn load() -> Usage {
    let Some(home) = std::env::home_dir() else {
        return Usage::default();
    };

    match read(&home) {
        Ok(text) => Usage::parse(&text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Usage::default(),
        Err(e) => {
            log::warn!("cannot read the usage file: {e}");
            Usage::default()
        }
    }
}

/// Writes `text`, the scores. Problems are logged.
pub fn save(text: &str) {
    let Some(home) = std::env::home_dir() else {
        return;
    };

    if let Err(e) = write(&home, text) {
        log::warn!("cannot write the usage file: {e}");
    }
}

fn read(home: &Path) -> io::Result<String> {
    let mut text = String::new();

    File::open(home.join(DIR).join(FILE))?
        .take(MAX_BYTES)
        .read_to_string(&mut text)?;
    Ok(text)
}

fn write(home: &Path, text: &str) -> io::Result<PathBuf> {
    let dir = home.join(DIR);

    fs::create_dir_all(&dir)?;
    paths::write_atomic(&dir, FILE, text.as_bytes())
}

#[cfg(test)]
#[path = "usage_file_tests.rs"]
mod tests;
