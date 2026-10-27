// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row order in the popup: `sort_by` in the config.

use crate::core::window::WindowInfo;

/// How rows are ordered.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortBy {
    App, // Grouped by app name, then by recent use
    #[default]
    Recently, // Most recently used first, active window last
}

/// Returns `windows` (most recently used first) in row order.
///
/// Every order starts from the most recently used order with the active
/// window moved last, and sorts stably from there.
#[must_use]
pub fn order(windows: &[WindowInfo], sort: SortBy) -> Vec<&WindowInfo> {
    let mut rows: Vec<&WindowInfo> =
        windows.iter().skip(1).chain(windows.first()).collect();

    match sort {
        SortBy::App => rows.sort_by_cached_key(|w| w.app_name.to_lowercase()),
        SortBy::Recently => {}
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::window::WindowId;

    fn window(id: &str, app: &str) -> WindowInfo {
        WindowInfo {
            id: WindowId(id.into()),
            app_id: String::new(),
            app_name: app.into(),
            title: String::new(),
            icon: None,
        }
    }

    /// Ids of `windows` in `sort` order.
    fn ids(windows: &[WindowInfo], sort: SortBy) -> Vec<String> {
        order(windows, sort)
            .iter()
            .map(|w| w.id.0.clone())
            .collect()
    }

    /// Most recently used first: `w1` is active.
    fn sample() -> Vec<WindowInfo> {
        vec![
            window("w1", "Kate"),
            window("w2", "firefox"),
            window("w3", "Kate"),
            window("w4", "Dolphin"),
        ]
    }

    #[test]
    fn recently_puts_the_active_window_last() {
        assert_eq!(ids(&sample(), SortBy::Recently), ["w2", "w3", "w4", "w1"]);
    }

    #[test]
    fn app_groups_by_name_ignoring_case() {
        assert_eq!(
            ids(&sample(), SortBy::App),
            ["w4", "w2", "w3", "w1"] // Dolphin, firefox, Kate, Kate (active)
        );
    }
}
