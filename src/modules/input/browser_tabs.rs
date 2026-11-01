// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Puts tab matches between window matches and launcher rows.

use crate::core::view::{Row, RowTarget};

/// Window rows, then tab rows, then launcher rows.
#[must_use]
pub fn join(windows: Vec<Row>, tabs: Vec<Row>) -> Vec<Row> {
    let mut out = Vec::with_capacity(windows.len() + tabs.len());
    let mut launch = Vec::new();

    for row in windows {
        if matches!(row.target, RowTarget::Launch(_)) {
            launch.push(row);
        } else {
            out.push(row);
        }
    }
    out.extend(tabs);
    out.extend(launch);
    out
}
