// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! XDG paths and private runtime files, from the standard library only.

use std::ffi::OsString;
use std::fs::{self, DirBuilder, Metadata, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

/// Returns `<config dir>/tynex/tynex.toml`, or `None` without a home.
#[must_use]
pub fn config_file() -> Option<PathBuf> {
    config_file_from(std::env::var_os("XDG_CONFIG_HOME"), std::env::home_dir())
}

/// Uses an absolute `$XDG_CONFIG_HOME`, else `~/.config`, per the XDG
/// spec, which says relative values are ignored.
fn config_file_from(
    xdg: Option<OsString>,
    home: Option<PathBuf>,
) -> Option<PathBuf> {
    let dir = xdg
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| home.map(|h| h.join(".config")))?;

    Some(dir.join("tynex").join("tynex.toml"))
}

/// Returns the XDG data dirs, most important first: `$XDG_DATA_HOME`
/// or `~/.local/share`, then `$XDG_DATA_DIRS` or
/// `/usr/local/share:/usr/share`.
#[must_use]
pub fn data_dirs() -> Vec<PathBuf> {
    data_dirs_from(
        std::env::var_os("XDG_DATA_HOME"),
        std::env::var_os("XDG_DATA_DIRS"),
        std::env::home_dir(),
    )
}

/// Skips relative paths, as the XDG spec says.
fn data_dirs_from(
    xdg_home: Option<OsString>,
    xdg_dirs: Option<OsString>,
    home: Option<PathBuf>,
) -> Vec<PathBuf> {
    let first = xdg_home
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| home.map(|h| h.join(".local/share")));
    let dirs = xdg_dirs
        .filter(|d| !d.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".into());

    first
        .into_iter()
        .chain(std::env::split_paths(&dirs).filter(|p| p.is_absolute()))
        .collect()
}

/// Returns `$XDG_RUNTIME_DIR/tynex/`, created with mode `0700` if
/// missing. There is no fallback to `/tmp`.
///
/// # Errors
///
/// Fails when `$XDG_RUNTIME_DIR` is unset or either directory is not a
/// private directory of the user.
pub fn runtime_dir() -> Result<PathBuf, String> {
    let uid = rustix::process::getuid().as_raw();

    runtime_dir_from(std::env::var_os("XDG_RUNTIME_DIR"), uid)
}

fn runtime_dir_from(
    xdg: Option<OsString>,
    uid: u32,
) -> Result<PathBuf, String> {
    let base = xdg
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .ok_or("XDG_RUNTIME_DIR is not set to an absolute path")?;

    private(&base, fs::metadata(&base), uid)?;

    let dir = base.join("tynex");

    match DirBuilder::new().mode(0o700).create(&dir) {
        Err(e) if e.kind() != io::ErrorKind::AlreadyExists => {
            return Err(format!("{}: {e}", dir.display()));
        }
        _ => {}
    }
    private(&dir, fs::symlink_metadata(&dir), uid)?;
    Ok(dir)
}

/// Accepts a real directory owned by `uid` that others cannot access.
fn private(
    path: &Path,
    meta: io::Result<Metadata>,
    uid: u32,
) -> Result<(), String> {
    let meta = meta.map_err(|e| format!("{}: {e}", path.display()))?;
    let why = if meta.file_type().is_symlink() {
        "is a symlink"
    } else if !meta.is_dir() {
        "is not a directory"
    } else if meta.uid() != uid {
        "is not owned by the user"
    } else if meta.mode() & 0o077 != 0 {
        "is open to other users"
    } else {
        return Ok(());
    };

    Err(format!("{} {why}", path.display()))
}

/// Writes `contents` to `dir/name` atomically: a new file with mode
/// `0600`, renamed over the target. Returns the target path.
///
/// # Errors
///
/// Fails when the file cannot be created, written or renamed.
pub fn write_atomic(
    dir: &Path,
    name: &str,
    contents: &[u8],
) -> io::Result<PathBuf> {
    let target = dir.join(name);
    let tmp = dir.join(format!(".{name}.{}", std::process::id()));
    let _ = fs::remove_file(&tmp); // Left over from a crash

    let result = OpenOptions::new()
        .write(true)
        .create_new(true) // O_EXCL: never follows a planted symlink
        .mode(0o600)
        .open(&tmp)
        .and_then(|mut file| file.write_all(contents))
        .and_then(|()| fs::rename(&tmp, &target));

    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result.map(|()| target)
}

#[cfg(test)]
#[path = "paths_tests.rs"]
mod tests;
