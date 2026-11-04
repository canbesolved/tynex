// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The hints folder and its default script, prepared on start
//! (ADR-0092).

use std::fs::{self, Permissions};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::config::{HINT_TEXT, HintConfig};
use crate::platform::linux::paths;

/// Folder of hint scripts under the home folder. It matches the
/// default `input.hint.text`, so it is not moved by `$XDG_DATA_HOME`.
const HINTS_DIR: &str = ".local/share/tynex/hints";

/// Script the default `input.hint.text` runs.
const DEFAULT: &str = "default.sh";

/// `extensions/hint_templates/default.sh`, built into the binary. The
/// other templates there stay out of it; users copy them by hand.
const DEFAULT_SCRIPT: &str =
    include_str!("../../../extensions/hint_templates/default.sh");

/// Prepares the hints folder for `hint`. Problems are logged.
pub fn prepare(hint: &HintConfig) {
    let Some(home) = std::env::home_dir() else {
        return;
    };
    let runs_default = hint.enabled && hint.text == HINT_TEXT;

    match prepare_in(&home, runs_default) {
        Ok(Some(path)) => log::info!("created hint script {}", path.display()),
        Ok(None) => {}
        Err(e) => log::warn!("cannot prepare the hint folder: {e}"),
    }
}

/// Creates `home/HINTS_DIR` when it is missing, with `default.sh`. In
/// an existing folder, writes `default.sh` only when `runs_default`.
/// Nothing at its path, even a link, is replaced, so edits stay.
/// Returns the path when the script was written.
fn prepare_in(home: &Path, runs_default: bool) -> io::Result<Option<PathBuf>> {
    let dir = home.join(HINTS_DIR);
    let missing = dir.symlink_metadata().is_err();

    fs::create_dir_all(&dir)?;
    if !(missing || runs_default)
        || dir.join(DEFAULT).symlink_metadata().is_ok()
    {
        return Ok(None);
    }

    let path = paths::write_atomic(&dir, DEFAULT, DEFAULT_SCRIPT.as_bytes())?;

    fs::set_permissions(&path, Permissions::from_mode(0o755))?;
    Ok(Some(path))
}

#[cfg(test)]
#[path = "hint_file_tests.rs"]
mod tests;
