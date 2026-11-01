// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Which cached window shows a browser window.

use crate::core::browser_tabs::TabInfo;
use crate::core::window::{WindowId, WindowInfo};

/// `KWin` window that shows browser window `window` of a source.
///
/// `windows` is the cache, most recently used first. `tabs` is the
/// source's list from the popup. `target` is the tab that was
/// activated. `window` is its browser window, from the reply.
#[must_use]
pub fn browser_window(
    windows: &[WindowInfo],
    pid: u32,
    tabs: &[TabInfo],
    target: i32,
    window: i32,
) -> Option<WindowId> {
    let mut candidates = windows.iter().filter(|w| w.pid == Some(pid));
    let first = candidates.next()?;

    let Some(second) = candidates.next() else {
        return Some(first.id.clone());
    };

    let mut titles = Vec::new();

    if let Some(active) = tabs.iter().find(|t| t.window == window && t.active) {
        push_title(&mut titles, &active.title);
    }
    if let Some(tab) = tabs.iter().find(|t| t.id == target) {
        push_title(&mut titles, &tab.title);
    }

    std::iter::once(first)
        .chain(std::iter::once(second))
        .chain(candidates)
        .find(|w| titles.iter().any(|title| w.title.starts_with(title)))
        .map(|w| w.id.clone())
}

/// Keeps a non-empty title once.
fn push_title(titles: &mut Vec<String>, title: &str) {
    if !title.is_empty() && !titles.iter().any(|have| have == title) {
        titles.push(title.to_owned());
    }
}
