// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `~/.local/share/tynex/stats.tsv`: the usage scores, read on start
//! and written after a change (ADR-0095).

use crate::core::usage::Usage;
use crate::platform::linux::data_file;

/// File name.
const FILE: &str = "stats.tsv";

/// Reads the scores. A missing or unreadable file gives none.
#[must_use]
pub fn load() -> Usage {
    match data_file::read(FILE) {
        Ok(text) => Usage::parse(&text),
        Err(e) => {
            log::warn!("cannot read the usage file: {e}");
            Usage::default()
        }
    }
}

/// Writes `text`, the scores. Problems are logged.
pub fn save(text: &str) {
    if let Err(e) = data_file::write(FILE, text) {
        log::warn!("cannot write the usage file: {e}");
    }
}
