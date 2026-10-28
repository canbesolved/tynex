// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Scan XDG `applications/` trees and parse desktop entries.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::app::AppInfo;
use crate::platform::linux::apps::Entry;
use crate::platform::linux::key_file::{self, entries, unescape};

/// Scans each data dir's `applications/` tree. First id wins.
pub(super) fn scan(
    data_dirs: &[PathBuf],
    locales: &[String],
    gui_only: bool,
) -> HashMap<String, Entry> {
    let mut out = HashMap::new();

    for dir in data_dirs {
        let apps = dir.join("applications");

        scan_dir(&apps, &apps, locales, gui_only, &mut out);
    }
    out
}

/// Walks `dir` for `.desktop` files. `root` is the applications
/// directory; relative paths become ids with `/` as `-`.
fn scan_dir(
    dir: &Path,
    root: &Path,
    locales: &[String],
    gui_only: bool,
    out: &mut HashMap<String, Entry>,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            scan_dir(&path, root, locales, gui_only, out);
            continue;
        }
        if path.extension().and_then(OsStr::to_str) != Some("desktop") {
            continue;
        }

        let Ok(rel) = path.strip_prefix(root) else {
            continue;
        };
        let id = desktop_id(rel);

        if id.is_empty() || out.contains_key(&id) {
            continue;
        }

        let Some(text) = key_file::read(&path) else {
            continue;
        };
        if let Some(entry) = parse_app(&text, &id, &path, locales, gui_only) {
            out.insert(id, entry);
        }
    }
}

/// Desktop file id: path under `applications/` with `/` as `-`.
fn desktop_id(rel: &Path) -> String {
    let text = rel.to_string_lossy();
    let stem = text.strip_suffix(".desktop").unwrap_or(&text);

    stem.replace('/', "-")
}

/// Parses a `[Desktop Entry]` into a catalog entry, or `None` when it
/// must stay hidden.
fn parse_app(
    text: &str,
    id: &str,
    path: &Path,
    locales: &[String],
    gui_only: bool,
) -> Option<Entry> {
    let mut name: Option<(usize, &str)> = None;
    let mut icon = None;
    let mut exec = None;
    let mut entry_type = None;
    let mut no_display = false;
    let mut hidden = false;
    let mut terminal = false;

    for (_, key, value) in
        entries(text).filter(|&(group, ..)| group == "Desktop Entry")
    {
        match key {
            "Type" => entry_type = entry_type.or(Some(value)),
            "Exec" => exec = exec.or(Some(value)),
            "Icon" => icon = icon.or(Some(value)),
            "NoDisplay" => no_display |= is_true(value),
            "Hidden" => hidden |= is_true(value),
            "Terminal" => terminal |= is_true(value),
            key => {
                let rank = match key.strip_prefix("Name") {
                    Some("") => Some(locales.len()),
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
        }
    }

    if entry_type != Some("Application") || no_display || hidden {
        return None;
    }
    if gui_only && terminal {
        return None;
    }

    let exec = unescape(exec?);
    if exec.trim().is_empty() {
        return None;
    }

    let name = name
        .map(|(_, v)| unescape(v))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| id.to_owned());
    let icon = icon.map(unescape).filter(|v| !v.is_empty());

    Some(Entry {
        info: AppInfo {
            id: id.to_owned(),
            name,
            icon,
        },
        exec,
        path: path.to_owned(),
        terminal,
    })
}

fn is_true(value: &str) -> bool {
    value.eq_ignore_ascii_case("true")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_id_replaces_slashes() {
        assert_eq!(desktop_id(Path::new("kde/foo.desktop")), "kde-foo");
    }

    #[test]
    fn skips_hidden_and_nodisplay() {
        let text = "[Desktop Entry]\nType=Application\nName=X\n\
                    Exec=x\nNoDisplay=true\n";

        assert!(
            parse_app(text, "x", Path::new("x.desktop"), &[], false).is_none()
        );
    }

    #[test]
    fn gui_only_skips_terminal() {
        let text = "[Desktop Entry]\nType=Application\nName=T\n\
                    Exec=t\nTerminal=true\n";

        assert!(
            parse_app(text, "t", Path::new("t.desktop"), &[], true).is_none()
        );
        assert!(
            parse_app(text, "t", Path::new("t.desktop"), &[], false).is_some()
        );
    }
}
