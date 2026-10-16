// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Reading the config file, or writing the default one on first start.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::Path;

use crate::config::{Config, ConfigError, DEFAULT_TOML, parse};

/// Result of loading the config file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Loaded {
    Existing(Config), // Read from the file
    Created(Config),  // File was missing; the default was written
}

/// Loads `path`, or writes the default file there when it is missing.
///
/// An existing file is never changed.
///
/// # Errors
///
/// Returns read, write and parse problems; messages do not include the
/// path, so the caller adds it.
pub fn load_or_create(path: &Path) -> Result<Loaded, Vec<ConfigError>> {
    match fs::read_to_string(path) {
        Ok(text) => parse(&text, &Path::is_file).map(Loaded::Existing),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            write_default(path).map_err(|e| {
                vec![io_error("cannot write the default file", &e)]
            })?;
            Ok(Loaded::Created(Config::default()))
        }
        Err(e) => Err(vec![io_error("cannot read the file", &e)]),
    }
}

/// Writes `DEFAULT_TOML` to `path`, creating parent directories.
fn write_default(path: &Path) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    // `create_new` never overwrites a file created in the meantime.
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(DEFAULT_TOML.as_bytes())
}

fn io_error(what: &str, e: &std::io::Error) -> ConfigError {
    ConfigError {
        position: None,
        message: format!("{what}: {e}"),
    }
}
