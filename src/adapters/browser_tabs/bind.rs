// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Binding the tabs socket: single instance, stale files, atomic rename.

use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

/// File name hosts connect to.
pub const NAME: &str = "tabs.sock";

/// Bound here first, then renamed to `NAME` once it listens.
const TEMP: &str = ".tabs.sock.new";

/// Longest socket path: `sun_path` minus its final NUL.
const PATH_MAX: usize = 107;

/// Binds `dir/tabs.sock`. `None` means tabs stay off.
///
/// The socket is bound under a temporary name, set to mode `0600` and
/// renamed, so a host never sees it before it listens.
pub fn listen(dir: &Path) -> Option<(PathBuf, UnixListener)> {
    let path = dir.join(NAME);
    let temp = dir.join(TEMP);

    if temp.as_os_str().len() > PATH_MAX {
        log::error!("browser tabs: socket path is too long");
        return None;
    }
    if !free(&path) || !free(&temp) {
        return None;
    }
    let listener = UnixListener::bind(&temp).and_then(|listener| {
        listener.set_nonblocking(true)?;
        fs::set_permissions(&temp, fs::Permissions::from_mode(0o600))?;
        fs::rename(&temp, &path)?;
        Ok(listener)
    });

    match listener {
        Ok(listener) => {
            log::debug!("browser tabs: listening on {}", path.display());
            Some((path, listener))
        }
        Err(e) => {
            log::warn!("browser tabs: socket: {e}");
            let _ = fs::remove_file(&temp);
            None
        }
    }
}

/// True when `path` can be bound: missing, or a stale socket of this
/// user, which is removed. A live socket means another daemon.
fn free(path: &Path) -> bool {
    match UnixStream::connect(path) {
        Ok(_) => {
            log::warn!("browser tabs: socket in use by another daemon");
            false
        }
        Err(e) if e.kind() == ErrorKind::NotFound => true,
        Err(e) if e.kind() == ErrorKind::ConnectionRefused && stale(path) => {
            log::debug!("browser tabs: removing stale {}", path.display());
            fs::remove_file(path).is_ok()
        }
        Err(e) => {
            log::warn!("browser tabs: socket: {e}");
            false
        }
    }
}

/// A socket file owned by this user, not a symlink or a regular file.
fn stale(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|meta| {
        meta.file_type().is_socket()
            && meta.uid() == rustix::process::getuid().as_raw()
    })
}
