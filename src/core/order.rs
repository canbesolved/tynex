// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row order in the popup: `sort_by` in the config.

use crate::core::keys::Pins;
use crate::core::window::WindowInfo;

/// How rows are ordered.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortBy {
    #[default]
    Key, // Letter order in `keys`; rows never move
    App,      // Grouped by app name, then by letter
    Recently, // Most recently used first, active window last
}

/// Returns `windows` (most recently used first) in row order.
///
/// Every order starts from the most recently used order with the active
/// window moved last, and sorts stably from there: windows without a
/// letter keep that order and come after those with one.
#[must_use]
pub fn order<'a>(
    windows: &'a [WindowInfo],
    pins: &Pins,
    sort: SortBy,
) -> Vec<&'a WindowInfo> {
    let mut rows: Vec<&WindowInfo> =
        windows.iter().skip(1).chain(windows.first()).collect();
    let letter = |w: &WindowInfo| pins.rank(&w.id).unwrap_or(usize::MAX);

    match sort {
        SortBy::Key => rows.sort_by_key(|w| letter(w)),
        SortBy::App => {
            rows.sort_by_cached_key(|w| (w.app_name.to_lowercase(), letter(w)));
        }
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

    /// Ids of `windows` in `sort` order, with `keys` handed out first.
    fn ids(windows: &[WindowInfo], keys: &str, sort: SortBy) -> Vec<String> {
        let mut pins = Pins::new(keys);

        pins.update(windows, false);
        order(windows, &pins, sort)
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
        assert_eq!(
            ids(&sample(), "asd", SortBy::Recently),
            ["w2", "w3", "w4", "w1"]
        );
    }

    #[test]
    fn key_follows_letters_then_recent_use() {
        let mut windows = sample();
        let mut pins = Pins::new("asdf");

        pins.update(&windows, false); // w2 a, w3 s, w4 d, w1 f
        windows.swap(0, 3); // w4 is active now: letters stay
        windows.remove(1); // w2 closes: `a` is free
        windows.push(window("w5", "Kate"));
        pins.update(&windows, false); // w5 takes `a`

        let rows: Vec<_> = order(&windows, &pins, SortBy::Key)
            .iter()
            .map(|w| w.id.0.as_str())
            .collect();

        assert_eq!(rows, ["w5", "w3", "w4", "w1"]);
        assert_eq!(ids(&sample(), "a", SortBy::Key), ["w2", "w3", "w4", "w1"]);
    }

    #[test]
    fn app_groups_by_name_ignoring_case() {
        assert_eq!(
            ids(&sample(), "asdf", SortBy::App),
            ["w4", "w2", "w3", "w1"] // Dolphin, firefox, Kate (s), Kate (f)
        );
    }
}
