// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Per-user native messaging manifests, written on daemon start.

use std::fs;
use std::path::Path;

use serde_json::json;

use super::channels::{Channel, SPECS, Spec};
use crate::config::{Browser, BrowserTabsConfig, Browsers};
use crate::platform::linux::paths::write_atomic;

/// Extension id derived from the public key in the manifest.
const EXTENSION_ID: &str = "dpkepmbcncmpdefbicacdmlipejaijpj";

const HOST: &str = "io.github.canbesolved.tynex";
const FILE: &str = "io.github.canbesolved.tynex.json";

/// File every Chromium profile folder has; a bare folder is not an
/// installed browser.
const MARKER: &str = "Local State";

/// Makes per-user manifests match `config`. `home` is the user's home.
///
/// Returns whether a selected browser is installed. The socket is
/// opened only then.
#[must_use]
pub fn apply(config: &BrowserTabsConfig, home: &Path, exe: &Path) -> bool {
    sync(config, home, exe, Path::new("/"))
}

/// `root` is `/` in production and a temp folder in tests.
#[must_use]
pub(super) fn sync(
    config: &BrowserTabsConfig,
    home: &Path,
    exe: &Path,
    root: &Path,
) -> bool {
    let Some(exe) = exe.to_str() else {
        log::error!("browser tabs: executable path is not text");
        return false;
    };
    let mut used = Vec::new();

    for spec in SPECS {
        let hosts = home
            .join(".config")
            .join(spec.folder)
            .join("NativeMessagingHosts");
        let file = hosts.join(FILE);

        if !wanted(config, spec.browser) {
            remove_ours(&file); // Left from an earlier selection
            continue;
        }
        match channel(spec, home, root) {
            Channel::Missing => remove_ours(&file),
            Channel::System => {
                remove_ours(&file);
                used.push(spec);
            }
            Channel::User => {
                write(&hosts, &file, exe);
                used.push(spec);
            }
        }
    }
    report(config, &used);
    !used.is_empty()
}

/// Probes one selected channel and logs it.
fn channel(spec: &Spec, home: &Path, root: &Path) -> Channel {
    let installed = home.join(".config").join(spec.folder).join(MARKER);
    let installed = installed.is_file();
    let system = root.join(spec.system).join(FILE).is_file();

    let (state, text) = match (installed, system) {
        (false, _) => (Channel::Missing, "not installed"),
        (true, true) => (Channel::System, "installed, system-wide host used"),
        (true, false) => (Channel::User, "installed"),
    };

    log::debug!("browser tabs: {}: {text}", spec.name);
    state
}

/// Logs the outcome: off, the channels used, listed browsers missing.
fn report(config: &BrowserTabsConfig, used: &[&Spec]) {
    if !config.enabled {
        log::info!("browser tabs: off in the config");
        return;
    }
    if let Browsers::List(list) = &config.browsers {
        for browser in list
            .iter()
            .filter(|b| !used.iter().any(|s| s.browser == **b))
        {
            log::warn!("browser tabs: {browser:?} is listed but not installed");
        }
    }
    let names_used: Vec<_> = used.iter().map(|s| s.name).collect();

    log::info!("browser tabs: using {}", names(&names_used));
}

/// Removes `file` if this daemon wrote it.
fn remove_ours(file: &Path) {
    if !ours(file) {
        return;
    }
    match fs::remove_file(file) {
        Ok(()) => log::info!("browser tabs: removed {}", file.display()),
        Err(e) => log::warn!("browser tabs: {}: {e}", file.display()),
    }
}

/// `Brave, Chrome`, or `none`.
fn names(list: &[&str]) -> String {
    if list.is_empty() {
        "none".to_owned()
    } else {
        list.join(", ")
    }
}

fn wanted(config: &BrowserTabsConfig, browser: Browser) -> bool {
    if !config.enabled {
        return false;
    }
    match &config.browsers {
        Browsers::Auto => true,
        Browsers::List(list) => list.contains(&browser),
    }
}

fn write(dir: &Path, file: &Path, exe: &str) {
    let body = json!({
        "name": HOST,
        "description": "tynex tab bridge",
        "path": exe,
        "type": "stdio",
        "allowed_origins": [format!("chrome-extension://{EXTENSION_ID}/")],
    });
    let Ok(bytes) = serde_json::to_vec_pretty(&body) else {
        return;
    };

    if fs::read(file).ok().as_deref() == Some(bytes.as_slice()) {
        log::debug!("browser tabs: {} is up to date", file.display());
        return;
    }
    match fs::create_dir_all(dir).and_then(|()| write_atomic(dir, FILE, &bytes))
    {
        Ok(_) => log::info!("browser tabs: wrote {}", file.display()),
        Err(e) => log::warn!("browser tabs: {}: {e}", file.display()),
    }
}

/// A manifest this daemon wrote: our file name and a `tynex` path.
fn ours(path: &Path) -> bool {
    let Ok(text) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return false;
    };

    value
        .get("path")
        .and_then(|p| p.as_str())
        .is_some_and(|exe| {
            Path::new(exe).file_name().is_some_and(|n| n == "tynex")
        })
}

#[cfg(test)]
#[path = "register_tests.rs"]
mod tests;
