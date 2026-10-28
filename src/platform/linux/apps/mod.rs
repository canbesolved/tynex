// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Installed apps from `.desktop` files: scan once, spawn `Exec`.

mod exec;
mod parse;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use rustix::process;

use crate::config::LauncherMode;
use crate::core::app::AppInfo;
use crate::platform::linux::apps::exec::expand_exec;
use crate::platform::linux::apps::parse::scan;
use crate::platform::linux::desktop_files::locale_keys;
use crate::platform::linux::paths;

/// One catalog entry: core info plus the data needed to start it.
#[derive(Debug, Clone)]
pub(super) struct Entry {
    pub info: AppInfo,
    pub exec: String,   // Raw Exec= value
    pub path: PathBuf,  // Desktop file path (`%k`)
    pub terminal: bool, // Terminal=true
}

/// Installed apps, scanned once at daemon start.
pub struct Apps {
    entries: HashMap<String, Entry>, // By app id
    children: Vec<Child>,            // Spawned, reaped on wake
}

impl Apps {
    /// Scans XDG application dirs. Returns an empty catalog when the
    /// launcher is off.
    #[must_use]
    pub fn scan(mode: LauncherMode) -> Self {
        if mode == LauncherMode::Off {
            return Self {
                entries: HashMap::new(),
                children: Vec::new(),
            };
        }

        let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .find_map(|var| std::env::var(var).ok().filter(|v| !v.is_empty()))
            .unwrap_or_default();
        let entries = scan(
            &paths::data_dirs(),
            &locale_keys(&locale),
            mode == LauncherMode::GuiOnly,
        );

        log::info!("launcher catalog: {} apps", entries.len());
        Self {
            entries,
            children: Vec::new(),
        }
    }

    /// Returns the pure app list for the input module.
    #[must_use]
    pub fn list(&self) -> Vec<AppInfo> {
        let mut list: Vec<_> =
            self.entries.values().map(|e| e.info.clone()).collect();

        list.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
        list
    }

    /// Starts the app of `id`.
    ///
    /// # Errors
    ///
    /// Returns a message when the id is unknown, `Exec` is empty, a
    /// terminal app has no `xdg-terminal-exec`, or spawn fails.
    pub fn launch(&mut self, id: &str) -> Result<(), String> {
        let entry = self
            .entries
            .get(id)
            .ok_or_else(|| format!("unknown app {id}"))?;
        let argv = expand_exec(
            &entry.exec,
            &entry.info.name,
            entry.info.icon.as_deref(),
            &entry.path,
        )?;

        if argv.is_empty() {
            return Err(format!("empty Exec for {id}"));
        }

        let mut cmd = if entry.terminal {
            terminal_command(&argv)?
        } else {
            let mut c = Command::new(&argv[0]);

            c.args(&argv[1..]);
            c
        };

        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        // Own session so the daemon is not the controlling parent.
        unsafe {
            cmd.pre_exec(|| {
                process::setsid()?;
                Ok(())
            });
        }

        let child =
            cmd.spawn().map_err(|e| format!("cannot start {id}: {e}"))?;

        log::info!("launch {id}");
        self.children.push(child);
        Ok(())
    }

    /// Reaps finished children. Call on each event-loop wake.
    pub fn reap(&mut self) {
        self.children.retain_mut(|child| match child.try_wait() {
            Ok(None) => true,
            Ok(Some(status)) => {
                log::debug!("child exited: {status}");
                false
            }
            Err(e) => {
                log::warn!("cannot wait on child: {e}");
                false
            }
        });
    }
}

/// Builds a command that opens `argv` in a terminal.
fn terminal_command(argv: &[String]) -> Result<Command, String> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    let has = path.to_string_lossy().split(':').any(|dir| {
        std::path::Path::new(dir)
            .join("xdg-terminal-exec")
            .is_file()
    });

    if !has {
        return Err("Terminal=true but xdg-terminal-exec is not on PATH".into());
    }

    let mut cmd = Command::new("xdg-terminal-exec");

    cmd.args(argv);
    Ok(cmd)
}

// `CommandExt::pre_exec` lives in std::os::unix.
use std::os::unix::process::CommandExt;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
