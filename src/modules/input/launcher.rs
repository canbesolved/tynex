// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Groups matching windows and launch rows. No I/O.

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use crate::config::NewWindowConfig;
use crate::core::app::AppInfo;
use crate::core::title::short_app_id;
use crate::core::view::{Row, RowTarget};
use crate::modules::search::app_haystack;
use crate::modules::search::fuzzy::{Hit, rank};

/// Builds the output list for a non-empty query with the launcher on.
///
/// A running app whose name or id matches lists all of its windows,
/// then a new-window row when that row is enabled. A title-only match
/// lists those windows, then the row when the app is installed. An
/// app that is not running and matches is one launch row.
#[must_use]
pub fn merge(
    query: &str,
    windows: &[Row],
    apps: &[AppInfo],
    new_window: &NewWindowConfig,
) -> Vec<Row> {
    let win_apps: Vec<String> = windows
        .iter()
        .map(|r| app_haystack(&r.app_name, &r.app_id))
        .collect();
    let win_items: Vec<(&str, &str)> = windows
        .iter()
        .zip(&win_apps)
        .map(|(r, app)| (r.title.as_str(), app.as_str()))
        .collect();
    let win_hits = rank(query, &win_items);
    let app_items: Vec<(&str, &str)> = apps
        .iter()
        .map(|a| (a.name.as_str(), short_app_id(&a.id)))
        .collect();
    let app_hits = rank(query, &app_items);
    let groups = collect_groups(windows, apps, &win_hits, &app_hits);

    emit(windows, apps, &win_hits, &app_hits, groups, new_window)
}

fn collect_groups<'a>(
    windows: &'a [Row],
    apps: &'a [AppInfo],
    win_hits: &'a [Hit],
    app_hits: &'a [Hit],
) -> Vec<(String, Group<'a>)> {
    let mut by_app: HashMap<String, Group<'a>> = HashMap::new();

    for hit in win_hits {
        let row = &windows[hit.index];
        let group = by_app.entry(row.app_id.clone()).or_default();

        group.score = group.score.max(hit.score);
        group.window_hits.push(hit);
        if hit.highlights.is_empty() {
            group.app_field = true;
        }
    }
    for hit in app_hits {
        let app = &apps[hit.index];
        let group = by_app.entry(app.id.clone()).or_default();

        group.score = group.score.max(hit.score);
        group.app_hit = true;
    }

    let mut groups: Vec<_> = by_app.into_iter().collect();

    // Better score first; on a tie, running apps before launch-only,
    // then app id for stability.
    groups.sort_by(|(id_a, a), (id_b, b)| {
        Reverse(a.score).cmp(&Reverse(b.score)).then_with(|| {
            let run = |id: &str| windows.iter().any(|w| w.app_id == id);

            run(id_b).cmp(&run(id_a)).then_with(|| id_a.cmp(id_b))
        })
    });
    groups
}

fn emit(
    windows: &[Row],
    apps: &[AppInfo],
    win_hits: &[Hit],
    app_hits: &[Hit],
    groups: Vec<(String, Group<'_>)>,
    new_window: &NewWindowConfig,
) -> Vec<Row> {
    let app_matched: HashSet<&str> =
        app_hits.iter().map(|h| apps[h.index].id.as_str()).collect();
    let catalog: HashMap<&str, &AppInfo> =
        apps.iter().map(|a| (a.id.as_str(), a)).collect();
    let mut output = Vec::new();
    let mut used_windows = HashSet::new();

    for (app_id, group) in groups {
        let take_all = group.app_hit
            || group.app_field
            || app_matched.contains(app_id.as_str());

        if take_all {
            for (i, row) in windows.iter().enumerate() {
                if row.app_id == app_id {
                    push_window(&mut output, row, win_hits_for(win_hits, i));
                    used_windows.insert(i);
                }
            }
        } else {
            for hit in &group.window_hits {
                push_window(
                    &mut output,
                    &windows[hit.index],
                    Some(hit.highlights.clone()),
                );
                used_windows.insert(hit.index);
            }
        }
        if let Some(app) = catalog.get(app_id.as_str()).copied() {
            let running = windows.iter().any(|r| r.app_id == app.id);

            if !running || new_window.enabled {
                output.push(launch_row(app, running, new_window));
            }
        }
    }
    for hit in win_hits {
        if !used_windows.contains(&hit.index) {
            push_window(
                &mut output,
                &windows[hit.index],
                Some(hit.highlights.clone()),
            );
        }
    }
    output
}

#[derive(Default)]
struct Group<'a> {
    score: u16,
    window_hits: Vec<&'a Hit>,
    app_hit: bool,   // Catalog name or id matched
    app_field: bool, // A window hit the app column only
}

fn win_hits_for(
    hits: &[Hit],
    index: usize,
) -> Option<Vec<std::ops::Range<usize>>> {
    hits.iter()
        .find(|h| h.index == index)
        .map(|h| h.highlights.clone())
}

fn push_window(
    output: &mut Vec<Row>,
    row: &Row,
    highlights: Option<Vec<std::ops::Range<usize>>>,
) {
    let mut row = row.clone();

    row.highlights = highlights.unwrap_or_default();
    output.push(row);
}

fn launch_row(
    app: &AppInfo,
    running: bool,
    new_window: &NewWindowConfig,
) -> Row {
    Row {
        target: RowTarget::Launch(app.id.clone()),
        app_id: app.id.clone(),
        app_name: app.name.clone(),
        icon: app.icon.clone(),
        title: if running {
            new_window.text.clone()
        } else {
            app.name.clone()
        },
        title_color: running.then_some(new_window.text_color),
        highlights: Vec::new(),
    }
}

#[cfg(test)]
#[path = "launcher_tests.rs"]
mod tests;
