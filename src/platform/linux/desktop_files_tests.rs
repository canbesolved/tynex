// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Desktop file tests, with made-up apps under the system temp dir.

use std::fs;

use super::*;
use crate::core::window::WindowId;

const VIEWER: &str = "\
# Made-up app
[Desktop Entry]
Type=Application
GenericName=Image Viewer
Name[de]=Bildbetrachter
Name = Pixel\\sViewer
Name[de_CH]=Bildli
Name[es]=Visor

[Desktop Action new]
Name=New Window
Name[fr]=Nouvelle fenêtre
";

fn keys(list: &[&str]) -> Vec<String> {
    list.iter().map(|&k| k.to_owned()).collect()
}

fn window(app_id: &str, app_name: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId(app_id.into()),
        app_id: app_id.into(),
        app_name: app_name.into(),
        title: String::new(),
        icon: None,
    }
}

#[test]
fn locale_keys_follow_the_spec_order() {
    assert_eq!(
        locale_keys("sr_RS.UTF-8@latin"),
        keys(&["sr_RS@latin", "sr_RS", "sr@latin", "sr"])
    );
    assert_eq!(locale_keys("de_CH.UTF-8"), keys(&["de_CH", "de"]));
    assert_eq!(locale_keys("es"), keys(&["es"]));
    assert_eq!(locale_keys("C.UTF-8"), keys(&[]));
    assert_eq!(locale_keys("POSIX"), keys(&[]));
    assert_eq!(locale_keys(""), keys(&[]));
}

#[test]
fn name_takes_the_best_locale() {
    let name = |locale| entry_name(VIEWER, &locale_keys(locale));

    assert_eq!(name("de_CH.UTF-8").as_deref(), Some("Bildli"));
    assert_eq!(name("de_AT.UTF-8").as_deref(), Some("Bildbetrachter"));
    assert_eq!(name("es_ES.UTF-8").as_deref(), Some("Visor"));
    assert_eq!(name("C").as_deref(), Some("Pixel Viewer"));
}

#[test]
fn only_the_desktop_entry_group_counts() {
    let fr = locale_keys("fr_FR.UTF-8");
    let action_only = "[Desktop Action new]\nName=New Window\n";

    assert_eq!(entry_name(VIEWER, &fr).as_deref(), Some("Pixel Viewer"));
    assert_eq!(entry_name(action_only, &[]), None);
    assert_eq!(entry_name("[Desktop Entry]\nName=\n", &[]), None);
}

#[test]
fn escapes_are_decoded() {
    assert_eq!(unescape(r"a\sb\tc\\d\"), "a b\tc\\d\\");
}

#[test]
fn files_are_found_by_app_id_and_cached() {
    let base = std::env::temp_dir()
        .join(format!("tynex_desktop_files_{}", std::process::id()));
    let (home, system) = (base.join("home"), base.join("system"));

    for (dir, name) in [(&home, "Mine"), (&system, "System")] {
        fs::create_dir_all(dir.join("applications")).unwrap();
        fs::write(
            dir.join("applications/org.example.Pixel.desktop"),
            format!("[Desktop Entry]\nName={name}\n"),
        )
        .unwrap();
    }

    let mut files = DesktopFiles::with(vec![home.clone(), system], "C");
    let mut list = [
        window("org.example.Pixel", "pixel"),
        window("unknown", "unknown"),
        window("../applications/org.example.Pixel", "class"),
    ];

    files.name(&mut list);
    assert_eq!(list[0].app_name, "Mine"); // First data dir wins
    assert_eq!(list[1].app_name, "unknown"); // Class stays
    assert_eq!(list[2].app_name, "class"); // Refused id

    // Cached: a removed file is not read again.
    fs::remove_dir_all(&home).unwrap();
    list[0].app_name.clear();
    files.name(&mut list);
    assert_eq!(list[0].app_name, "Mine");
    fs::remove_dir_all(base).unwrap();
}
