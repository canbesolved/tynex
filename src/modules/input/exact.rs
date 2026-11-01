// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Exact matches first: rows that contain the query as typed move
//! above rows that only match fuzzily, whatever their source.

use std::cmp::Reverse;

use crate::core::title::short_app_id;
use crate::core::view::{Row, RowTarget};
use crate::modules::search::{app_haystack, contains_query};

/// Moves exact rows above fuzzy ones. The sort is stable, so each band
/// keeps its order: windows, tabs, launch rows, leading matches first.
pub fn exact_first(query: &str, rows: Vec<Row>) -> Vec<Row> {
    let own: Vec<bool> = rows.iter().map(|row| own(query, row)).collect();
    let exact: Vec<bool> = rows
        .iter()
        .zip(&own)
        .map(|(row, &hit)| hit || with_its_windows(&rows, &own, row))
        .collect();
    let mut keyed: Vec<(bool, Row)> = exact.into_iter().zip(rows).collect();

    keyed.sort_by_key(|(hit, _)| Reverse(*hit));
    keyed.into_iter().map(|(_, row)| row).collect()
}

/// The row's own text contains the query: the title, and for windows
/// and apps the app name or short id.
fn own(query: &str, row: &Row) -> bool {
    contains_query(&row.title, query)
        || match &row.target {
            RowTarget::Window(_) => {
                contains_query(&app_haystack(&row.app_name, &row.app_id), query)
            }
            RowTarget::Launch(id) => {
                contains_query(&row.app_name, query)
                    || contains_query(short_app_id(id), query)
            }
            RowTarget::Tab(_) => false, // The title holds the host too
        }
}

/// A new-window row stays with its app's exact windows.
fn with_its_windows(rows: &[Row], exact: &[bool], row: &Row) -> bool {
    matches!(row.target, RowTarget::Launch(_))
        && rows.iter().zip(exact).any(|(other, &hit)| {
            hit && matches!(other.target, RowTarget::Window(_))
                && other.app_id == row.app_id
        })
}

#[cfg(test)]
#[path = "exact_tests.rs"]
mod tests;
