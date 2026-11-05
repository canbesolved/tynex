// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The title list while nothing is typed: the history from the
//! previous place on, windows and tabs, then the other windows, with
//! the current place last like the active window. A browser window is
//! listed as its active tab (ADR-0097).

use crate::core::browser_tabs::{TabInfo, TabList, TabRef};
use crate::core::places::Place;
use crate::core::title::shows_tab;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::modules::browser_tabs::tab_row;

/// Rows of the title list. `rows` are the open windows in row order,
/// the active one last; `history` is newest first.
#[must_use]
pub fn titles(rows: &[Row], history: &[Place], lists: &[TabList]) -> Vec<Row> {
    let mut out = Vec::with_capacity(rows.len());
    let mut windows: Vec<&WindowId> = Vec::new(); // Already listed
    let mut tabs: Vec<TabRef> = Vec::new();

    for place in history.iter().skip(1) {
        match place {
            Place::Window(id) => {
                if let Some(row) = window_row(rows, id)
                    && !windows.contains(&id)
                {
                    windows.push(id);
                    out.push(row.clone());
                }
            }
            Place::Tab { tab, window, .. } => {
                let Some((list, info)) = find(lists, tab) else {
                    continue;
                };

                if !tabs.contains(tab) {
                    tabs.push(tab.clone());
                    out.push(tab_row(list, info));
                }
                if info.active
                    && let Some(id) = window
                {
                    windows.push(id);
                }
            }
        }
    }
    for row in rows {
        let RowTarget::Window(id) = &row.target else {
            continue;
        };

        if windows.contains(&id) {
            continue;
        }
        match active_tab(lists, id, &row.title) {
            Some((list, info)) => {
                let tab = TabRef {
                    source: list.source,
                    id: info.id,
                };

                if !tabs.contains(&tab) {
                    tabs.push(tab);
                    out.push(tab_row(list, info));
                }
            }
            None => out.push(row.clone()),
        }
    }
    out
}

/// The row of window `id`.
fn window_row<'a>(rows: &'a [Row], id: &WindowId) -> Option<&'a Row> {
    rows.iter()
        .find(|row| matches!(&row.target, RowTarget::Window(w) if w == id))
}

/// The list and the tab of `tab`, while its browser is listed.
fn find<'a>(
    lists: &'a [TabList],
    tab: &TabRef,
) -> Option<(&'a TabList, &'a TabInfo)> {
    let list = lists.iter().find(|list| list.source == tab.source)?;
    let info = list.tabs.iter().find(|t| t.id == tab.id)?;

    Some((list, info))
}

/// The active tab that browser window `id`, titled `title`, shows.
fn active_tab<'a>(
    lists: &'a [TabList],
    id: &WindowId,
    title: &str,
) -> Option<(&'a TabList, &'a TabInfo)> {
    lists
        .iter()
        .filter(|list| list.windows.contains(id))
        .find_map(|list| {
            list.tabs
                .iter()
                .find(|t| t.active && shows_tab(title, &t.title))
                .map(|t| (list, t))
        })
}

#[cfg(test)]
#[path = "titles_tests.rs"]
mod tests;
