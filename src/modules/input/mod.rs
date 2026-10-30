// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Input sub-modules: search of open windows, then the launcher.

pub mod launcher;

use crate::config::{LauncherConfig, LauncherMode, NewWindowConfig};
use crate::core::app::AppInfo;
use crate::core::state::State;
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
            s.output = s.rows.clone();
            s.selected = 0;
            return;
        }

        if self.mode == LauncherMode::Off || self.apps.is_empty() {
            search_windows(s);
            return;
        }

        s.output =
            launcher::merge(&s.query, &s.rows, &self.apps, &self.new_window);
        s.selected = 0;
    }
}

/// Filters open windows into `output` and sets title highlights.
fn search_windows(s: &mut State) {
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
    s.output = hits
        .iter()
        .map(|h| {
            let mut row = s.rows[h.index].clone();

            row.highlights.clone_from(&h.highlights);
            row
        })
        .collect();
    s.selected = 0;
}
