// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Presenter errors.

use std::fmt;

use wayland_client::globals::BindError;

/// Error from the Wayland connection or a missing protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenterError(pub String);

impl fmt::Display for PresenterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PresenterError {}

/// Wraps `e` with a description of what failed.
pub(super) fn error(what: &str, e: impl fmt::Display) -> PresenterError {
    PresenterError(format!("{what}: {e}"))
}

/// Maps a failed bind of a required global to a clear error.
pub(super) fn missing(
    name: &'static str,
) -> impl Fn(BindError) -> PresenterError {
    move |_| PresenterError(format!("the compositor lacks `{name}`"))
}
