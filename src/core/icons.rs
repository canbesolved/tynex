// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Running apps for the icon bar, one icon per app id.

use crate::core::app::AppInfo;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::{WindowId, WindowInfo};

/// Builds the strip: `pinned` apps first in their order, running or
/// not, then the other running apps, most recently used first.
///
/// A pinned app that is not running has no windows and the icon of
/// its desktop file.
#[must_use]
pub fn strip(pinned: &[AppInfo], windows: &[WindowInfo]) -> Vec<AppIcon> {
    let mut running = running(windows);
    let mut icons: Vec<AppIcon> = pinned
        .iter()
        .map(|app| {
            match running.iter().position(|icon| icon.app_id == app.id) {
                Some(at) => {
                    let mut icon = running.remove(at);

                    icon.icon = icon.icon.or_else(|| app.icon.clone());
                    icon
                }
                None => AppIcon {
                    app_id: app.id.clone(),
                    icon: app.icon.clone(),
                    windows: Vec::new(),
                },
            }
        })
        .collect();

    icons.append(&mut running);
    icons
}

/// Groups `windows` by app id, most recently used app first.
///
/// The icon is taken from the newest window of that app. Window ids
/// stay in most recently used order.
#[must_use]
pub fn running(windows: &[WindowInfo]) -> Vec<AppIcon> {
    let mut icons: Vec<AppIcon> = Vec::new();

    for window in windows {
        if let Some(icon) = icons.iter_mut().find(|i| i.app_id == window.app_id)
        {
            icon.windows.push(window.id.clone());
        } else {
            icons.push(AppIcon {
                app_id: window.app_id.clone(),
                icon: window.icon.clone(),
                windows: vec![window.id.clone()],
            });
        }
    }
    icons
}

/// Returns the rows of `icon`, alphabetically by title.
///
/// Comparison ignores case. A tie keeps the original title order.
#[must_use]
pub fn ordered<'a>(icon: &AppIcon, rows: &'a [Row]) -> Vec<&'a Row> {
    let mut found: Vec<&Row> = icon
        .windows
        .iter()
        .filter_map(|id| row_for(rows, id))
        .collect();

    found.sort_by(|a, b| {
        fold(&a.title)
            .cmp(&fold(&b.title))
            .then(a.title.cmp(&b.title))
    });
    found
}

/// Returns the row whose target is window `id`.
fn row_for<'a>(rows: &'a [Row], id: &WindowId) -> Option<&'a Row> {
    rows.iter().find(
        |row| matches!(&row.target, RowTarget::Window(window) if window == id),
    )
}

/// Returns `text` in lowercase for alphabetical order.
fn fold(text: &str) -> String {
    text.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::view::RowTarget;

    fn window(id: &str, app: &str) -> WindowInfo {
        WindowInfo {
            id: WindowId(id.into()),
            app_id: app.into(),
            app_name: app.into(),
            title: id.into(),
            icon: None,
            pid: None,
        }
    }

    fn row(id: &str, title: &str) -> Row {
        Row {
            target: RowTarget::Window(WindowId(id.into())),
            app_id: String::new(),
            app_name: String::new(),
            icon: None,
            title: title.into(),
            title_color: None,
            highlights: Vec::new(),
        }
    }

    #[test]
    fn groups_by_app_in_mru_order() {
        let icons = running(&[
            window("w1", "firefox"),
            window("w2", "kate"),
            window("w3", "firefox"),
        ]);

        assert_eq!(icons.len(), 2);
        assert_eq!(icons[0].app_id, "firefox");
        assert_eq!(icons[0].windows.len(), 2);
        assert_eq!(icons[1].app_id, "kate");
        assert_eq!(icons[1].windows.len(), 1);
    }

    #[test]
    fn dropdown_titles_are_alphabetical() {
        let icon = AppIcon {
            app_id: "firefox".into(),
            icon: None,
            windows: vec![
                WindowId("w1".into()),
                WindowId("w2".into()),
                WindowId("w3".into()),
            ],
        };
        let rows =
            vec![row("w1", "Release"), row("w2", "café"), row("w3", "Café")];
        let titles: Vec<_> = ordered(&icon, &rows)
            .into_iter()
            .map(|r| r.title.as_str())
            .collect();

        assert_eq!(titles, ["Café", "café", "Release"]);
    }

    fn app(id: &str, icon: &str) -> AppInfo {
        AppInfo {
            id: id.into(),
            name: id.into(),
            icon: Some(icon.into()),
        }
    }

    #[test]
    fn pinned_apps_come_first_running_or_not() {
        let pinned = [app("kate", "kate-icon"), app("gimp", "gimp-icon")];
        let icons = strip(
            &pinned,
            &[
                window("w1", "firefox"),
                window("w2", "kate"),
                window("w3", "firefox"),
            ],
        );
        let ids: Vec<_> = icons.iter().map(|i| i.app_id.as_str()).collect();

        assert_eq!(ids, ["kate", "gimp", "firefox"]);
        assert_eq!(icons[0].windows, [WindowId("w2".into())]);
        assert_eq!(icons[0].icon.as_deref(), Some("kate-icon"));
        assert_eq!(icons[1].windows, []);
        assert_eq!(icons[1].icon.as_deref(), Some("gimp-icon"));
        assert_eq!(icons[2].windows.len(), 2);
        assert_eq!(strip(&[], &[window("w1", "kate")]).len(), 1);
    }
}
