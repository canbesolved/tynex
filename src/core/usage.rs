// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Usage scores: how often a window's app, an app or a tab was opened
//! through tynex, from 0 to 100 (ADR-0095). Pure logic; the platform
//! reads and writes the file.

use std::collections::{HashMap, HashSet};

use crate::core::browser_tabs::TabList;
use crate::core::view::{Row, RowTarget};

/// Highest score. Going past it halves every score.
pub const MAX: u8 = 100;

/// What a score belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Item {
    App(String), // App id: its windows and its launch rows
    Tab { browser: String, url: String },
}

/// Scores of every item used so far.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Usage {
    scores: HashMap<Item, u8>,
    changed: bool, // Not saved since the last change
}

impl Usage {
    /// Reads the file text. Malformed lines are skipped.
    #[must_use]
    pub fn parse(text: &str) -> Self {
        let scores = text
            .lines()
            .filter_map(|line| {
                let mut parts = line.split('\t');
                let score = parts.next()?.parse::<u32>().ok()?;
                let score = u8::try_from(score.min(u32::from(MAX))).ok()?;
                let item = match (parts.next()?, parts.next()?, parts.next()) {
                    ("app", id, None) if !id.is_empty() => {
                        Item::App(id.to_owned())
                    }
                    ("tab", browser, Some(url)) if !url.is_empty() => {
                        Item::Tab {
                            browser: browser.to_owned(),
                            url: url.to_owned(),
                        }
                    }
                    _ => return None,
                };

                (score > 0).then_some((item, score))
            })
            .collect();

        Self {
            scores,
            changed: false,
        }
    }

    /// Returns the file text, one `score<TAB>kind<TAB>key` line per
    /// item, highest score first.
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut items: Vec<_> = self.scores.iter().collect();

        items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        items
            .into_iter()
            .map(|(item, score)| match item {
                Item::App(id) => format!("{score}\tapp\t{id}\n"),
                Item::Tab { browser, url } => {
                    format!("{score}\ttab\t{browser}\t{url}\n")
                }
            })
            .collect()
    }

    /// Returns the score of `item`, 0 when it was never used.
    #[must_use]
    pub fn score(&self, item: &Item) -> u8 {
        self.scores.get(item).copied().unwrap_or(0)
    }

    /// Adds one use of `item`. Past `MAX`, every score is halved, so
    /// old uses fade and the order stays.
    pub fn record(&mut self, item: Item) {
        if !storable(&item) {
            return;
        }
        let score = self.scores.entry(item).or_insert(0);

        *score += 1;
        if *score > MAX {
            for score in self.scores.values_mut() {
                *score /= 2;
            }
            self.scores.retain(|_, score| *score > 0);
        }
        self.changed = true;
    }

    /// Drops apps for which `exists` is false.
    pub fn retain_apps(&mut self, exists: impl Fn(&str) -> bool) {
        self.retain(|item| match item {
            Item::App(id) => exists(id),
            Item::Tab { .. } => true,
        });
    }

    /// Drops tabs of `browser` that no list of it has open.
    pub fn retain_tabs(&mut self, browser: &str, lists: &[TabList]) {
        let open: HashSet<&str> = lists
            .iter()
            .filter(|list| list.browser == browser)
            .flat_map(|list| list.tabs.iter().map(|tab| tab.url.as_str()))
            .collect();

        self.retain(|item| match item {
            Item::Tab { browser: b, url } => {
                b != browser || open.contains(&**url)
            }
            Item::App(_) => true,
        });
    }

    /// Returns true once after a change, so the caller saves the file.
    pub fn take_changed(&mut self) -> bool {
        std::mem::take(&mut self.changed)
    }

    fn retain(&mut self, keep: impl Fn(&Item) -> bool) {
        let before = self.scores.len();

        self.scores.retain(|item, _| keep(item));
        self.changed |= self.scores.len() != before;
    }
}

/// The item a row opens. `rows` are the open windows; `lists` the tabs.
#[must_use]
pub fn item(
    target: &RowTarget,
    rows: &[Row],
    lists: &[TabList],
) -> Option<Item> {
    match target {
        RowTarget::Window(_) => rows
            .iter()
            .find(|row| row.target == *target)
            .map(|row| Item::App(row.app_id.clone())),
        RowTarget::Launch(id) => Some(Item::App(id.clone())),
        RowTarget::Tab(tab) => {
            let list = lists.iter().find(|l| l.source == tab.source)?;
            let info = list.tabs.iter().find(|t| t.id == tab.id)?;

            Some(Item::Tab {
                browser: list.browser.clone(),
                url: info.url.clone(),
            })
        }
    }
}

/// An item whose key fits one line of the file.
fn storable(item: &Item) -> bool {
    let clean = |text: &str| !text.contains(['\t', '\n', '\r']);

    match item {
        Item::App(id) => !id.is_empty() && clean(id),
        Item::Tab { browser, url } => {
            !url.is_empty() && clean(browser) && clean(url)
        }
    }
}

#[cfg(test)]
#[path = "usage_tests.rs"]
mod tests;
