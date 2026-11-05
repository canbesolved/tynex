// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Puts tab matches between window matches and launcher rows, and
//! drops a window row that its active tab row already stands for.

use crate::core::browser_tabs::{TabInfo, TabList};
use crate::core::title::shows_tab;
use crate::core::view::{Row, RowTarget};

/// Window rows, then tab rows, then launcher rows. A browser window
/// whose active tab is in `tabs` is left out: the tab row raises it
/// too (ADR-0093).
#[must_use]
pub fn join(windows: Vec<Row>, tabs: Vec<Row>, lists: &[TabList]) -> Vec<Row> {
    let active = active_tabs(&tabs, lists);
    let mut out = Vec::with_capacity(windows.len() + tabs.len());
    let mut launch = Vec::new();

    for row in windows {
        if matches!(row.target, RowTarget::Launch(_)) {
            launch.push(row);
        } else if !shown_by_tab(&row, &active) {
            out.push(row);
        }
    }
    out.extend(tabs);
    out.extend(launch);
    out
}

/// Active tabs among the tab rows, with their lists.
fn active_tabs<'a>(
    tabs: &[Row],
    lists: &'a [TabList],
) -> Vec<(&'a TabList, &'a TabInfo)> {
    tabs.iter()
        .filter_map(|row| match &row.target {
            RowTarget::Tab(tab) => Some(tab),
            _ => None,
        })
        .filter_map(|tab| {
            let list = lists.iter().find(|l| l.source == tab.source)?;
            let info = list.tabs.iter().find(|t| t.id == tab.id)?;

            info.active.then_some((list, info))
        })
        .collect()
}

/// True when `row` is a window of a tab list's browser that shows one
/// of the `active` tabs.
fn shown_by_tab(row: &Row, active: &[(&TabList, &TabInfo)]) -> bool {
    let RowTarget::Window(id) = &row.target else {
        return false;
    };

    active.iter().any(|(list, tab)| {
        list.windows.contains(id) && shows_tab(&row.title, &tab.title)
    })
}

#[cfg(test)]
#[path = "browser_tabs_tests.rs"]
mod tests;
