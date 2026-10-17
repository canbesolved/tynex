// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Requests from the CLI to the daemon. Portable; each platform module
//! provides the transport (D-Bus on Linux).

use std::fmt;

use crate::core::event::ModeKind;

/// A request sent by `tynex show`, `hide` or `quit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Request {
    Show(Option<ModeKind>), // `None`: the mode from the config
    Hide,
    Quit,
}

/// Why a request did not reach the daemon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcError {
    NotRunning,
    Failed(String),
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRunning => f.write_str(
                "the daemon is not running; start it with `tynex daemon`",
            ),
            Self::Failed(why) => write!(f, "request failed: {why}"),
        }
    }
}

impl std::error::Error for IpcError {}
