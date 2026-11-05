// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Input sub-modules: search of open windows, then the launcher.

pub mod browser_tabs;
mod exact;
pub mod launcher;
mod titles;

use crate::config::{LauncherConfig, LauncherMode, NewWindowConfig};
use crate::core::app::AppInfo;
use crate::core::state::State;
use crate::core::usage;
use crate::core::view::Row;
use crate::modules::search::app_haystack;
use crate::modules::search::fuzzy::rank;
use crate::modules::search::starts_with_query;

/// Rebuilds the output list from the query, open windows and apps.
#[derive(Debug, Clone)]
pub struct Input {
    mode: LauncherMode,
    new_window: NewWindowConfig,
    apps: Vec<AppInfo>,
}

impl Input {
    /// Creates an input module. `apps` is empty when the launcher is
    /// off.
    #[must_use]
    pub fn new(launcher: LauncherConfig, apps: Vec<AppInfo>) -> Self {
        Self {
            mode: launcher.mode,
            new_window: launcher.new_window,
            apps,
        }
    }

    /// Recomputes `output` and highlights from `query` and `rows`.
    pub fn refresh(&self, s: &mut State) {
        for row in &mut s.rows {
            row.highlights.clear();
        }

        if s.query.is_empty() {
            s.output = match &s.history {
                Some(history) => titles::titles(&s.rows, history, &s.tab_lists),
                None => s.rows.clone(),
            };
            s.selected = 0;
            return;
        }

        let windows = if self.mode == LauncherMode::Off || self.apps.is_empty()
        {
            search_windows(s)
        } else {
            launcher::merge(&s.query, &s.rows, &self.apps, &self.new_window)
        };
        let tabs = crate::modules::browser_tabs::rows(&s.query, &s.tab_lists);

        let joined = browser_tabs::join(windows, tabs, &s.tab_lists);
        let score = |row: &Row| {
            usage::item(&row.target, &s.rows, &s.tab_lists)
                .map_or(0, |item| s.usage.score(&item))
        };

        s.output = exact::exact_first(&s.query, joined, score);
        s.selected = 0;
    }
}

/// Filters open windows and sets title highlights.
fn search_windows(s: &mut State) -> Vec<Row> {
    let apps: Vec<String> = s
        .rows
        .iter()
        .map(|r| app_haystack(&r.app_name, &r.app_id))
        .collect();
    let items: Vec<(&str, &str)> = s
        .rows
        .iter()
        .zip(&apps)
        .map(|(r, app)| (r.title.as_str(), app.as_str()))
        .collect();
    let mut hits = rank(&s.query, &items);

    // A title or app name that starts with the query stays above a
    // match that begins later in the text.
    hits.sort_by(|a, b| {
        let lead = |index: usize| {
            let row = &s.rows[index];

            starts_with_query(&row.title, &s.query)
                || starts_with_query(&row.app_name, &s.query)
        };

        lead(b.index).cmp(&lead(a.index))
    });
    for hit in &hits {
        s.rows[hit.index].highlights.clone_from(&hit.highlights);
    }
    hits.iter()
        .map(|h| {
            let mut row = s.rows[h.index].clone();

            row.highlights.clone_from(&h.highlights);
            row
        })
        .collect()
}

#[cfg(test)]
#[path = "usage_order_tests.rs"]
mod usage_order_tests;
