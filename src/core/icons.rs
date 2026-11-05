// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Running apps for the icon bar, one icon per app id.

use crate::core::app::AppInfo;
use crate::core::browser_tabs::TabList;
use crate::core::places::Place;
use crate::core::view::{AppIcon, Row, RowTarget, TabIcon};
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
                    tab: None,
                },
            }
        })
        .collect();

    icons.append(&mut running);
    icons
}

/// Puts the `previous` place right after the `pinned` icons and returns
/// its index (ADR-0097). A window's app icon moves there; a pinned one
/// stays and is returned. A tab gets an icon of its own, with its
/// favicon; before its list is known, its browser's icon is used.
pub fn put_previous(
    icons: &mut Vec<AppIcon>,
    pinned: usize,
    previous: Option<&Place>,
    windows: &[WindowInfo],
    lists: &[TabList],
) -> Option<usize> {
    let app_of = |found: Option<&WindowInfo>| found.map(|w| w.app_id.clone());
    let app = match previous? {
        Place::Window(id) => app_of(windows.iter().find(|w| w.id == *id)),
        Place::Tab { tab, pid, .. } => {
            let listed = lists.iter().find(|l| l.source == tab.source);
            let info = listed.and_then(|list| {
                list.tabs.iter().find(|t| t.id == tab.id).map(|t| (list, t))
            });

            if let Some((list, info)) = info {
                let at = pinned.min(icons.len());

                icons.insert(
                    at,
                    AppIcon {
                        app_id: String::new(),
                        icon: Some(list.icon.clone()),
                        windows: Vec::new(),
                        tab: Some(TabIcon {
                            tab: tab.clone(),
                            key: info.icon.clone(),
                            fallback: list.icon.clone(),
                        }),
                    },
                );
                return Some(at);
            }
            app_of(windows.iter().find(|w| w.pid == Some(*pid)))
        }
    }?;
    let at = icons
        .iter()
        .position(|icon| icon.tab.is_none() && icon.app_id == app)?;

    if at < pinned {
        return Some(at);
    }
    let icon = icons.remove(at);

    icons.insert(pinned, icon);
    Some(pinned)
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
                tab: None,
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
            tab: None,
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
