// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Calls to `KWin`: runner check, script loading and window activation.

use std::path::Path;

use dbus::Message;

use crate::core::traits::SourceError;
use crate::core::window::WindowId;
use crate::platform::linux::dbus::Bus;

/// `KWin`'s bus name.
pub const SERVICE: &str = "org.kde.KWin";

/// `KWin`'s window search, which activates a window by id.
const RUNNER: (&str, &str) = ("/WindowsRunner", "org.kde.krunner1");

/// `KWin`'s script manager.
const SCRIPTING: (&str, &str) = ("/Scripting", "org.kde.kwin.Scripting");

/// Name of the loaded script in `KWin`.
const PLUGIN: &str = "tynex";

fn error(e: impl std::fmt::Display) -> SourceError {
    SourceError(format!("KWin: {e}"))
}

/// Builds a call to `(path, interface)` on `KWin`.
fn method((path, interface): (&str, &str), member: &str) -> Message {
    Message::new_method_call(SERVICE, path, interface, member)
        .expect("valid KWin names")
}

/// Fails unless `KWin`'s window search is on the bus.
///
/// # Errors
///
/// Names the missing interface.
pub fn check_runner(bus: &Bus) -> Result<(), SourceError> {
    let introspect = Message::new_method_call(
        SERVICE,
        RUNNER.0,
        "org.freedesktop.DBus.Introspectable",
        "Introspect",
    )
    .expect("valid names");
    let found = bus
        .call_blocking(introspect)
        .ok()
        .and_then(|reply| reply.read1::<&str>().ok().map(str::to_owned))
        .is_some_and(|xml| xml.contains(&format!("\"{}\"", RUNNER.1)));

    if found {
        Ok(())
    } else {
        Err(error(format_args!(
            "no {} at {}; tynex needs KWin from Plasma 6",
            RUNNER.1, RUNNER.0
        )))
    }
}

/// Loads and starts the script at `file`, replacing a stale one left by
/// an earlier daemon.
///
/// # Errors
///
/// Fails when `KWin` refuses the script.
pub fn load(bus: &Bus, file: &Path) -> Result<(), SourceError> {
    let file = file
        .to_str()
        .ok_or_else(|| error("script path is not UTF-8"))?;

    unload(bus)?;

    let reply = bus
        .call_blocking(method(SCRIPTING, "loadScript").append2(file, PLUGIN))
        .map_err(error)?;

    if reply.read1::<i32>().map_err(error)? < 0 {
        return Err(error("the script was not loaded"));
    }
    bus.call_blocking(method(SCRIPTING, "start"))
        .map(drop)
        .map_err(error)
}

/// Unloads the script, if loaded.
///
/// # Errors
///
/// Fails when `KWin` does not answer.
pub fn unload(bus: &Bus) -> Result<(), SourceError> {
    bus.call_blocking(method(SCRIPTING, "unloadScript").append1(PLUGIN))
        .map(drop)
        .map_err(error)
}

/// Asks `KWin` to activate window `id` without waiting; a failure is
/// logged when its reply arrives.
///
/// # Errors
///
/// Fails when the call cannot be sent.
pub fn activate(bus: &Bus, id: &WindowId) -> Result<(), SourceError> {
    let msg = method(RUNNER, "Run").append2(match_id(id), "");
    let name = id.0.clone();

    bus.call(msg, move |reply| {
        if let Err(e) = reply {
            log::warn!("cannot activate {name}: {e}");
        }
    })
    .map_err(error)
}

/// Runner match id of window `id`: action 0 (activate), then the UUID.
fn match_id(id: &WindowId) -> String {
    format!("0_{{{}}}", id.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_id_wraps_the_uuid() {
        let id = WindowId("0b4c7e2a-1f3d-4c5b-9a6e-7d8f9a0b1c2d".into());

        assert_eq!(match_id(&id), "0_{0b4c7e2a-1f3d-4c5b-9a6e-7d8f9a0b1c2d}");
    }
}
