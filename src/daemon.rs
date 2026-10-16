// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `tynex daemon`: startup checks and wiring.

use std::path::Path;
use std::process::ExitCode;

use crate::config::{Config, Loaded, load_or_create};

/// Exit code for an invalid config (`EX_CONFIG` from `sysexits.h`).
pub const EX_CONFIG: u8 = 78;

/// Runs the daemon until it is told to quit.
#[cfg(target_os = "linux")]
#[must_use]
pub fn run() -> ExitCode {
    let Some(path) = crate::platform::linux::paths::config_file() else {
        log::error!("cannot find the config directory: no home directory");
        return ExitCode::from(EX_CONFIG);
    };
    let config = match load_config(&path) {
        Ok(config) => config,
        Err(errors) => {
            for e in &errors {
                log::error!("{e}");
            }
            return ExitCode::from(EX_CONFIG);
        }
    };

    log::info!("config loaded, desktop {:?}", config.desktop);
    // The event loop and D-Bus come next; until then the daemon exits.
    ExitCode::SUCCESS
}

/// Loads the config at `path`, writing the default file if missing.
///
/// # Errors
///
/// Returns one message per problem, each starting with the path.
pub fn load_config(path: &Path) -> Result<Config, Vec<String>> {
    match load_or_create(path) {
        Ok(Loaded::Existing(config)) => Ok(config),
        Ok(Loaded::Created(config)) => {
            log::warn!("created default config {}", path.display());
            Ok(config)
        }
        Err(errors) => Err(errors
            .iter()
            .map(|e| match e.position {
                Some(_) => format!("{}:{e}", path.display()),
                None => format!("{}: {e}", path.display()),
            })
            .collect()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn errors_start_with_the_path() {
        let dir = std::env::temp_dir()
            .join(format!("tynex_daemon_test_{}", std::process::id()));
        let path = dir.join("tynex.toml");

        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, "adaptr = \"kde\"\n").unwrap();

        let errors = load_config(&path).unwrap_err();

        assert!(
            errors[0]
                .starts_with(&format!("{}:1:1: unknown key", path.display()))
        );
        fs::write(&path, "").unwrap();
        assert_eq!(load_config(&path), Ok(Config::default()));
        fs::remove_dir_all(dir).unwrap();
    }
}
