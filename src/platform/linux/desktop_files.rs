// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! App names from desktop files (freedesktop Desktop Entry spec).

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::core::window::WindowInfo;
use crate::platform::linux::paths;

/// Largest desktop file read; real ones are well under this.
const MAX_SIZE: u64 = 1 << 20;

/// Looks up app names by desktop file id, cached for the daemon's life.
pub struct DesktopFiles {
    dirs: Vec<PathBuf>,   // `applications/`, best first
    locales: Vec<String>, // `Name[...]` keys, best first
    names: HashMap<String, Option<String>>, // By app id, misses too
}

impl Default for DesktopFiles {
    /// Reads the XDG data dirs and the message locale from the
    /// environment; files are read on first use of each app id.
    fn default() -> Self {
        let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .find_map(|var| std::env::var(var).ok().filter(|v| !v.is_empty()))
            .unwrap_or_default();

        Self::with(paths::data_dirs(), &locale)
    }
}

impl DesktopFiles {
    fn with(data_dirs: Vec<PathBuf>, locale: &str) -> Self {
        Self {
            dirs: data_dirs
                .into_iter()
                .map(|dir| dir.join("applications"))
                .collect(),
            locales: locale_keys(locale),
            names: HashMap::new(),
        }
    }

    /// Sets the app name of each window whose app id has a desktop file
    /// with a `Name`; other windows keep theirs.
    pub fn name(&mut self, windows: &mut [WindowInfo]) {
        for window in windows {
            if let Some(name) = self.lookup(&window.app_id) {
                window.app_name = name;
            }
        }
    }

    fn lookup(&mut self, app_id: &str) -> Option<String> {
        if let Some(name) = self.names.get(app_id) {
            return name.clone();
        }

        let name = find(&self.dirs, app_id)
            .and_then(|text| entry_name(&text, &self.locales));

        log::debug!("app name of {app_id:?}: {name:?}");
        self.names.insert(app_id.to_owned(), name.clone());
        name
    }
}

/// Reads `<app_id>.desktop` from the first dir that has it. Ids that
/// could leave the dir are refused.
fn find(dirs: &[PathBuf], app_id: &str) -> Option<String> {
    if app_id.is_empty() || app_id.starts_with('.') || app_id.contains('/') {
        return None;
    }

    let file = format!("{app_id}.desktop");

    dirs.iter().find_map(|dir| read(&dir.join(&file)))
}

fn read(path: &Path) -> Option<String> {
    let mut bytes = Vec::new();

    File::open(path)
        .ok()?
        .take(MAX_SIZE)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// Returns `Name` from the `[Desktop Entry]` group of `text`, in the
/// first of `locales` it has, else untranslated.
fn entry_name(text: &str, locales: &[String]) -> Option<String> {
    let mut in_entry = false;
    let mut best: Option<(usize, &str)> = None; // Locale rank, value

    for line in text.lines().map(str::trim) {
        if line.starts_with('[') {
            in_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_entry {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let rank = match key.trim_end().strip_prefix("Name") {
            Some("") => Some(locales.len()), // Untranslated, last
            Some(tag) => tag
                .strip_prefix('[')
                .and_then(|t| t.strip_suffix(']'))
                .and_then(|t| locales.iter().position(|l| l == t)),
            None => None,
        };

        if let Some(rank) = rank
            && best.is_none_or(|(old, _)| rank < old)
        {
            best = Some((rank, value.trim_start()));
        }
    }
    best.map(|(_, value)| unescape(value))
        .filter(|name| !name.is_empty())
}

/// Decodes `\s`, `\n`, `\t`, `\r` and `\\`.
fn unescape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('s') => out.push(' '),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// Returns the `Name[...]` tags to try for a POSIX locale such as
/// `sr_RS.UTF-8@latin`, best first: `sr_RS@latin`, `sr_RS`, `sr@latin`,
/// `sr`. `C` and `POSIX` have none.
fn locale_keys(locale: &str) -> Vec<String> {
    let (base, modifier) = match locale.split_once('@') {
        Some((base, m)) => (base, Some(m)),
        None => (locale, None),
    };
    let base = base.split('.').next().unwrap_or_default(); // No encoding
    let (lang, country) = match base.split_once('_') {
        Some((lang, c)) => (lang, Some(c)),
        None => (base, None),
    };

    if lang.is_empty() || lang == "C" || lang == "POSIX" {
        return Vec::new();
    }

    let mut keys = Vec::new();

    if let (Some(c), Some(m)) = (country, modifier) {
        keys.push(format!("{lang}_{c}@{m}"));
    }
    if let Some(c) = country {
        keys.push(format!("{lang}_{c}"));
    }
    if let Some(m) = modifier {
        keys.push(format!("{lang}@{m}"));
    }
    keys.push(lang.to_owned());
    keys
}

#[cfg(test)]
#[path = "desktop_files_tests.rs"]
mod tests;
