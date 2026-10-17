// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! KDE Plasma 6 window source: a `KWin` script pushes the window list, and
//! `KWin`'s window search activates windows.

mod bridge;
mod kwin;
mod parse;

use std::fs;
use std::path::PathBuf;

use crate::core::event::ModeKind;
use crate::core::queue::EventTx;
use crate::core::traits::{SourceError, WindowSource};
use crate::core::window::WindowId;
use crate::platform::linux::dbus::Bus;
use crate::platform::linux::paths;

/// The `KWin` script, written to the runtime directory on start.
const SCRIPT: &str = include_str!("tynex.js");

/// Window source backed by `KWin`.
pub struct KdeSource {
    bus: Bus,
    mode: ModeKind,          // Mode the shortcut opens
    script: Option<PathBuf>, // Script file, while loaded
}

impl KdeSource {
    /// Creates the source; nothing is sent to `KWin` before `start`.
    #[must_use]
    pub fn new(bus: Bus, mode: ModeKind) -> Self {
        Self {
            bus,
            mode,
            script: None,
        }
    }
}

impl WindowSource for KdeSource {
    fn start(&mut self, tx: EventTx) -> Result<(), SourceError> {
        kwin::check_runner(&self.bus)?;

        let kwin = self
            .bus
            .name_owner(kwin::SERVICE)
            .map_err(|e| SourceError(format!("KWin: {e}")))?;
        let dir = paths::runtime_dir().map_err(SourceError)?;
        let file = paths::write_atomic(&dir, "tynex.js", SCRIPT.as_bytes())
            .map_err(|e| {
                SourceError(format!("cannot write the KWin script: {e}"))
            })?;

        bridge::serve(&self.bus, tx, kwin, self.mode);
        kwin::load(&self.bus, &file)?;
        self.script = Some(file);
        log::info!("KWin script loaded");
        Ok(())
    }

    fn activate(&mut self, id: &WindowId) -> Result<(), SourceError> {
        kwin::activate(&self.bus, id)
    }

    fn stop(&mut self) {
        let Some(file) = self.script.take() else {
            return;
        };

        if let Err(e) = kwin::unload(&self.bus) {
            log::warn!("cannot unload the KWin script: {e}");
        }
        let _ = fs::remove_file(file);
    }
}
