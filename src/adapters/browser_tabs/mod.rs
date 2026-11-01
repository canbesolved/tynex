// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Browser tabs bridge over a Unix socket.
//!
//! `start` is the daemon side. `is_host` and `run_host` are the process
//! the browser starts; its manifest cannot pass `daemon`.

mod base64;
mod bind;
mod channels;
mod conn;
mod frame;
mod host;
mod msg;
mod pump;
mod register;
mod socket;
mod watch;

use std::io::Write;

pub use host::is_host;
pub use socket::{Batch, IconPng, Socket};

use crate::config::BrowserTabsConfig;
use crate::platform::linux::paths::runtime_dir;

/// Syncs the host manifests, then opens the socket when tabs are on
/// and a selected browser is installed. Manifests are synced on every
/// start, so turning tabs off removes them.
#[must_use]
pub fn start(config: &BrowserTabsConfig) -> Option<Socket> {
    let (Some(home), Ok(exe)) = (std::env::home_dir(), std::env::current_exe())
    else {
        log::warn!("browser tabs: no home folder or executable path");
        return None;
    };

    if !register::apply(config, &home, &exe) {
        return None;
    }
    let socket = match runtime_dir() {
        Ok(dir) => Socket::bind(&dir),
        Err(e) => {
            log::warn!("browser tabs: {e}");
            None
        }
    };

    match &socket {
        Some(_) => log::info!("browser tabs: socket ready, waiting for hosts"),
        None => log::warn!("browser tabs: socket not opened, tabs stay off"),
    }
    socket
}

/// Runs the host until the browser closes stdin. Returns the exit code.
#[must_use]
pub fn run_host() -> u8 {
    log::debug!("browser tabs: host started by the browser");
    match runtime_dir() {
        Ok(dir) => pump::run(&dir),
        Err(e) => {
            log::warn!("browser tabs: host: {e}");
            idle()
        }
    }
}

/// Without a runtime folder: reports the daemon as down and waits for
/// the browser to close stdin, so the extension does not respawn us.
fn idle() -> u8 {
    let mut relay = host::Relay::new(std::path::Path::new(""));

    relay.status(false);
    if std::io::stdout().write_all(&relay.to_browser).is_err() {
        return 0;
    }
    let _ = std::io::stdout().flush();
    let _ = std::io::copy(&mut std::io::stdin(), &mut std::io::sink());
    0
}
