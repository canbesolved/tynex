// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tab row titles and search order.

use std::cmp::Reverse;

use crate::core::browser_tabs::{TabInfo, TabList, TabRef};
use crate::core::text::sanitize_title;
use crate::core::view::{Row, RowIcon, RowTarget};
use crate::core::window::WindowInfo;
use crate::modules::search::fuzzy::rank;
use crate::modules::search::starts_with_query;

/// Most tabs kept from one browser.
const MAX_TABS: usize = 10_000;

/// Longest title kept, in chars. The extension cuts too.
const MAX_TITLE: usize = 256;

/// Longest URL kept, in chars.
const MAX_URL: usize = 1024;

/// Theme icon names of the supported browsers, by display name.
const ICONS: &[(&str, &str)] = &[
    ("Brave", "brave-browser"),
    ("Google Chrome", "google-chrome"),
];

/// Keeps the newest `MAX_TABS` tabs, with clean and capped text.
#[must_use]
pub fn clean(mut tabs: Vec<TabInfo>) -> Vec<TabInfo> {
    tabs.sort_unstable_by_key(|tab| Reverse(tab.last));
    tabs.truncate(MAX_TABS);
    for tab in &mut tabs {
        tab.title = capped(&tab.title, MAX_TITLE);
        tab.url = capped(&tab.url, MAX_URL);
    }
    tabs
}

/// Icon when the favicon is missing: the icon of a window of that
/// browser, else the browser's theme icon name.
#[must_use]
pub fn fallback(windows: &[WindowInfo], pid: u32, browser: &str) -> String {
    windows
        .iter()
        .find(|w| pid != 0 && w.pid == Some(pid))
        .and_then(|w| w.icon.clone())
        .or_else(|| {
            ICONS
                .iter()
                .find(|(name, _)| *name == browser)
                .map(|(_, icon)| (*icon).to_owned())
        })
        .unwrap_or_else(|| browser.to_lowercase().replace(' ', "-"))
}

/// Row title: `<title> — <host>`, or the URL when that is the title.
#[must_use]
pub fn row_title(tab: &TabInfo) -> String {
    let title = tab.title.as_str();
    let url = tab.url.as_str();

    if title.is_empty() || title == url {
        return url.to_owned();
    }
    match host_label(url) {
        Some(host) => format!("{title} — {host}"),
        None => title.to_owned(),
    }
}

/// Matching tab rows, best first. Empty when `query` is empty.
#[must_use]
pub fn rows(query: &str, lists: &[TabList]) -> Vec<Row> {
    if query.is_empty() {
        return Vec::new();
    }

    let flat: Vec<(&TabList, &TabInfo, String)> = lists
        .iter()
        .flat_map(|list| {
            list.tabs.iter().map(move |tab| (list, tab, row_title(tab)))
        })
        .collect();
    let items: Vec<(&str, &str)> = flat
        .iter()
        .map(|(_, _, title)| (title.as_str(), ""))
        .collect();
    let mut hits = rank(query, &items);

    hits.sort_by(|a, b| {
        let lead = |index: usize| starts_with_query(&flat[index].2, query);
        let last = |index: usize| flat[index].1.last;

        lead(b.index).cmp(&lead(a.index)).then_with(|| {
            b.score
                .cmp(&a.score)
                .then_with(|| last(b.index).cmp(&last(a.index)))
        })
    });

    hits.into_iter()
        .map(|hit| {
            let (list, tab, title) = &flat[hit.index];

            Row {
                target: RowTarget::Tab(TabRef {
                    source: list.source,
                    id: tab.id,
                }),
                app_id: String::new(),
                app_name: list.browser.clone(),
                icon: Some(RowIcon::Tab {
                    source: list.source,
                    key: tab.icon.clone(),
                    fallback: list.icon.clone(),
                }),
                title: title.clone(),
                title_color: None,
                highlights: hit.highlights,
            }
        })
        .collect()
}

/// `text` sanitized like a window title, at most `max` chars.
fn capped(text: &str, max: usize) -> String {
    let clean = sanitize_title(text);

    match clean.char_indices().nth(max) {
        Some((end, _)) => clean[..end].to_owned(),
        None => clean,
    }
}

/// Host without a leading `www.`, or the scheme when there is no host.
fn host_label(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = host.rsplit_once('@').map_or(host, |(_, host)| host);

    if host.is_empty() {
        return Some(scheme.to_owned());
    }
    Some(host.strip_prefix("www.").unwrap_or(host).to_owned())
}
