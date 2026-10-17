// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon theme tests, with made-up themes under the system temp dir.

use std::fs;

use super::*;

/// Theme files: `(path under the root, contents)`.
const FILES: [(&str, &str); 11] = [
    (
        "mine/index.theme",
        "[Icon Theme]\nInherits=parent\n\
         Directories=apps/16,apps/48,apps/scalable\n\
         [apps/16]\nSize=16\nType=Fixed\n\
         [apps/48]\nSize=48\nType=Fixed\n\
         [apps/scalable]\nSize=48\nType=Scalable\nMinSize=8\nMaxSize=512\n",
    ),
    ("mine/apps/16/a.png", ""),
    ("mine/apps/48/a.png", ""),
    ("mine/apps/scalable/b.svg", ""),
    (
        "parent/index.theme",
        "[Icon Theme]\nInherits=hicolor,mine\nDirectories=apps/32\n\
         [apps/32]\nSize=32\n",
    ),
    ("parent/apps/32/c.png", ""),
    (
        "hicolor/index.theme",
        "[Icon Theme]\nDirectories=apps/48\n[apps/48]\nSize=48\n",
    ),
    ("hicolor/apps/48/a.png", ""), // Hidden by `mine`
    ("hicolor/apps/48/d.png", ""),
    ("hicolor/apps/48/d.svg", ""), // PNG first
    ("../pixmaps/e.svg", ""),
];

/// Builds the themes under a fresh dir; returns the lookup and the dir.
fn themes(name: &str) -> (IconTheme, PathBuf) {
    let base = std::env::temp_dir()
        .join(format!("tynex_icon_theme_{name}_{}", std::process::id()));
    let root = base.join("icons");

    for (path, text) in FILES {
        let file = root.join(path);

        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }
    (
        IconTheme::with([root], std::slice::from_ref(&base), "mine".into()),
        base,
    )
}

/// Returns the file found for `icon`, relative to `base`.
fn found(theme: &mut IconTheme, base: &Path, icon: &str, size: u32) -> String {
    theme.find(icon, size).map_or_else(String::new, |path| {
        path.strip_prefix(base)
            .unwrap()
            .to_string_lossy()
            .into_owned()
    })
}

#[test]
fn closest_size_wins() {
    let (mut theme, base) = themes("size");
    let mut find = |icon, size| found(&mut theme, &base, icon, size);

    assert_eq!(find("a", 48), "icons/mine/apps/48/a.png");
    assert_eq!(find("a", 20), "icons/mine/apps/16/a.png");
    assert_eq!(find("a", 40), "icons/mine/apps/48/a.png");
    assert_eq!(find("b", 100), "icons/mine/apps/scalable/b.svg");
    fs::remove_dir_all(base).unwrap();
}

#[test]
fn parents_then_hicolor_then_pixmaps() {
    let (mut theme, base) = themes("chain");
    let mut find = |icon, size| found(&mut theme, &base, icon, size);

    assert_eq!(find("c", 48), "icons/parent/apps/32/c.png");
    assert_eq!(find("d", 48), "icons/hicolor/apps/48/d.png");
    assert_eq!(find("e", 48), "pixmaps/e.svg");
    assert_eq!(find("zzz", 48), "");
    assert_eq!(find("../hicolor/apps/48/d", 48), "");

    // `parent` inherits `mine` again: each theme loads once.
    assert_eq!(theme.themes.map(|t| t.len()), Some(3));
    fs::remove_dir_all(base).unwrap();
}

#[test]
fn directory_sizes() {
    let dir = |keys: &[(&str, &str)]| SubDir::new("d", keys).unwrap();
    let threshold = dir(&[("Size", "32")]);
    let scaled = dir(&[("Size", "22"), ("Scale", "2"), ("Type", "Fixed")]);

    assert_eq!((threshold.min, threshold.max), (30, 34));
    assert_eq!((scaled.min, scaled.max), (44, 44));
    assert_eq!(threshold.distance(31), 0);
    assert_eq!(threshold.distance(24), 6);
    assert_eq!(threshold.distance(40), 6);
    assert_eq!(SubDir::new("d", &[("Size", "x")]), None);
}
