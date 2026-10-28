// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! App names and icons from desktop files (freedesktop Desktop Entry
//! spec).

use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::window::WindowInfo;
use crate::platform::linux::key_file::{self, entries, unescape};
use crate::platform::linux::paths;

/// What a desktop file says about an app.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Entry {
    name: Option<String>, // Localized `Name`
    icon: Option<String>, // `Icon`: theme name or absolute path
}

/// Looks up apps by desktop file id, cached for the daemon's life.
pub struct DesktopFiles {
    dirs: Vec<PathBuf>,   // `applications/`, best first
    locales: Vec<String>, // `Name[...]` keys, best first
    entries: HashMap<String, Option<Entry>>, // By app id, misses too
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
            entries: HashMap::new(),
        }
    }

    /// Sets the app name and icon of each window from its desktop file.
    /// Without a `Name`, the window keeps its app name; without a
    /// desktop file, the app id is tried as icon name.
    pub fn fill(&mut self, windows: &mut [WindowInfo]) {
        for window in windows {
            match self.lookup(&window.app_id) {
                Some(entry) => {
                    if let Some(name) = entry.name {
                        window.app_name = name;
                    }
                    window.icon = entry.icon;
                }
                None if !window.app_id.is_empty() => {
                    window.icon = Some(window.app_id.clone());
                }
                None => {}
            }
        }
    }

    fn lookup(&mut self, app_id: &str) -> Option<Entry> {
        if let Some(entry) = self.entries.get(app_id) {
            return entry.clone();
        }

        let entry = find(&self.dirs, app_id)
            .map(|text| parse_entry(&text, &self.locales));

        log::debug!("desktop entry of {app_id:?}: {entry:?}");
        self.entries.insert(app_id.to_owned(), entry.clone());
        entry
    }
}

/// Reads `<app_id>.desktop` from the first dir that has it. Ids that
/// could leave the dir are refused.
fn find(dirs: &[PathBuf], app_id: &str) -> Option<String> {
    if app_id.is_empty() || app_id.starts_with('.') || app_id.contains('/') {
        return None;
    }

    let file = format!("{app_id}.desktop");

    dirs.iter().find_map(|dir| key_file::read(&dir.join(&file)))
}

/// Reads `Name`, in the first of `locales` it has, else untranslated,
/// and `Icon` from the `[Desktop Entry]` group of `text`.
fn parse_entry(text: &str, locales: &[String]) -> Entry {
    let mut name: Option<(usize, &str)> = None; // Locale rank, value
    let mut icon = None;

    for (_, key, value) in
        entries(text).filter(|&(group, ..)| group == "Desktop Entry")
    {
        if key == "Icon" {
            icon = icon.or(Some(value));
            continue;
        }

        let rank = match key.strip_prefix("Name") {
            Some("") => Some(locales.len()), // Untranslated, last
            Some(tag) => tag
                .strip_prefix('[')
                .and_then(|t| t.strip_suffix(']'))
                .and_then(|t| locales.iter().position(|l| l == t)),
            None => None,
        };

        if let Some(rank) = rank
            && name.is_none_or(|(old, _)| rank < old)
        {
            name = Some((rank, value));
        }
    }

    let text = |value: Option<&str>| {
        value.map(unescape).filter(|value| !value.is_empty())
    };

    Entry {
        name: text(name.map(|(_, value)| value)),
        icon: text(icon),
    }
}

/// Returns the `Name[...]` tags to try for a POSIX locale such as
/// `sr_RS.UTF-8@latin`, best first: `sr_RS@latin`, `sr_RS`, `sr@latin`,
/// `sr`. `C` and `POSIX` have none.
pub(crate) fn locale_keys(locale: &str) -> Vec<String> {
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
