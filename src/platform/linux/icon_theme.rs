// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon files from freedesktop icon themes (Icon Theme spec).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::platform::linux::key_file::{self, entries, value};
use crate::platform::linux::paths;
use crate::ui::render::icons::IconLookup;

/// Icon file types, in the spec's order of preference.
const EXTENSIONS: [&str; 2] = ["png", "svg"];

/// Theme of a Plasma setup that never changed it.
const DEFAULT_THEME: &str = "breeze";

/// Theme every lookup ends with.
const FALLBACK_THEME: &str = "hicolor";

/// Sizes one theme directory accepts, in pixels at scale 1.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SubDir {
    path: String,
    min: u32,
    max: u32,
}

impl SubDir {
    /// Reads `Size`, `Scale`, `Type`, `MinSize`, `MaxSize` and
    /// `Threshold`; `None` without a valid `Size`.
    fn new(path: &str, keys: &[(&str, &str)]) -> Option<Self> {
        let get = |key| keys.iter().find(|&&(k, _)| k == key).map(|e| e.1);
        let num = |key| get(key).and_then(|v| v.parse::<u32>().ok());
        let size = num("Size")?;
        let scale = num("Scale").unwrap_or(1).max(1);
        let (min, max) = match get("Type").unwrap_or("Threshold") {
            "Fixed" => (size, size),
            "Scalable" => (
                num("MinSize").unwrap_or(size),
                num("MaxSize").unwrap_or(size),
            ),
            _ => {
                let t = num("Threshold").unwrap_or(2);

                (size.saturating_sub(t), size + t)
            }
        };

        Some(Self {
            path: path.to_owned(),
            min: min * scale,
            max: max * scale,
        })
    }

    /// Returns how far `size` is from the sizes this directory accepts.
    fn distance(&self, size: u32) -> u32 {
        self.min.saturating_sub(size) + size.saturating_sub(self.max)
    }
}

/// One installed theme.
#[derive(Debug, Default)]
struct Theme {
    dirs: Vec<PathBuf>, // `<icon root>/<name>` that exist
    subdirs: Vec<SubDir>,
}

impl Theme {
    /// Returns the file of `icon` closest to `size`, preferring the
    /// directory order of the index on ties.
    fn find(&self, icon: &str, size: u32) -> Option<PathBuf> {
        let mut subdirs: Vec<&SubDir> = self.subdirs.iter().collect();

        subdirs.sort_by_key(|s| s.distance(size)); // Stable
        subdirs.iter().find_map(|sub| {
            self.dirs
                .iter()
                .find_map(|dir| file(&dir.join(&sub.path), icon))
        })
    }
}

/// Finds icon files in the user's theme, its parents, `hicolor`, then
/// the `pixmaps` dirs. Themes load on the first lookup.
pub struct IconTheme {
    roots: Vec<PathBuf>,   // `~/.icons`, then `<data dir>/icons`
    pixmaps: Vec<PathBuf>, // `<data dir>/pixmaps`
    name: String,
    themes: Option<Vec<Theme>>, // The theme, its parents, `hicolor`
}

impl Default for IconTheme {
    /// Uses the theme from KDE's `kdeglobals`, else `breeze`.
    fn default() -> Self {
        let data = paths::data_dirs();
        let home = std::env::home_dir().map(|h| h.join(".icons"));
        let name = paths::config_dirs()
            .iter()
            .filter_map(|dir| key_file::read(&dir.join("kdeglobals")))
            .find_map(|text| value(&text, "Icons", "Theme").map(str::to_owned))
            .unwrap_or_else(|| DEFAULT_THEME.to_owned());

        log::debug!("icon theme {name:?}");
        Self::with(
            home.into_iter().chain(data.iter().map(|d| d.join("icons"))),
            &data,
            name,
        )
    }
}

impl IconTheme {
    fn with(
        roots: impl IntoIterator<Item = PathBuf>,
        data: &[PathBuf],
        name: String,
    ) -> Self {
        Self {
            roots: roots.into_iter().collect(),
            pixmaps: data.iter().map(|d| d.join("pixmaps")).collect(),
            name,
            themes: None,
        }
    }

    /// Loads `name` and its parents, depth first, then `hicolor`.
    fn load(&self) -> Vec<Theme> {
        let mut themes = Vec::new();
        let mut seen = Vec::new();
        let mut todo = vec![self.name.clone(), FALLBACK_THEME.to_owned()];

        while !todo.is_empty() {
            let name = todo.remove(0);

            if seen.contains(&name) {
                continue;
            }

            let (theme, mut parents) = self.load_one(&name);

            parents.retain(|p| p != FALLBACK_THEME); // Always last
            todo.splice(0..0, parents); // Parents before the rest
            seen.push(name);
            themes.push(theme);
        }
        themes
    }

    /// Reads the first `index.theme` of `name`; returns the theme and
    /// its `Inherits` list.
    fn load_one(&self, name: &str) -> (Theme, Vec<String>) {
        let dirs: Vec<PathBuf> = self
            .roots
            .iter()
            .map(|root| root.join(name))
            .filter(|dir| dir.is_dir())
            .collect();
        let Some(text) = dirs
            .iter()
            .find_map(|dir| key_file::read(&dir.join("index.theme")))
        else {
            return (Theme::default(), Vec::new());
        };
        let (subdirs, parents) = parse_index(&text);

        (Theme { dirs, subdirs }, parents)
    }
}

impl IconLookup for IconTheme {
    fn find(&mut self, icon: &str, size: u32) -> Option<PathBuf> {
        if icon.is_empty() || icon.contains('/') {
            return None;
        }

        let themes = match &self.themes {
            Some(themes) => themes,
            None => self.themes.insert(self.load()),
        };

        themes
            .iter()
            .find_map(|theme| theme.find(icon, size))
            .or_else(|| self.pixmaps.iter().find_map(|dir| file(dir, icon)))
    }
}

/// Returns `dir/icon.png` or `dir/icon.svg`, whichever exists first.
fn file(dir: &Path, icon: &str) -> Option<PathBuf> {
    EXTENSIONS
        .iter()
        .map(|ext| dir.join(format!("{icon}.{ext}")))
        .find(|path| path.is_file())
}

/// Returns the directories of an `index.theme` and its `Inherits`.
fn parse_index(text: &str) -> (Vec<SubDir>, Vec<String>) {
    let mut groups: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();

    for (group, key, value) in entries(text) {
        groups.entry(group).or_default().push((key, value));
    }

    let head = groups.get("Icon Theme").map_or(&[][..], Vec::as_slice);
    let list = |key| {
        head.iter()
            .filter(move |&&(k, _)| k == key)
            .flat_map(|(_, v)| v.split(','))
            .map(str::trim)
            .filter(|v| !v.is_empty())
    };
    let subdirs = list("Directories")
        .chain(list("ScaledDirectories"))
        .filter_map(|dir| SubDir::new(dir, groups.get(dir)?))
        .collect();
    let parents = list("Inherits").map(str::to_owned).collect();

    (subdirs, parents)
}

#[cfg(test)]
#[path = "icon_theme_tests.rs"]
mod tests;
