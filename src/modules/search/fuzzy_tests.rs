// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Ranking tests. A `frizbee` upgrade that changes behavior fails here.

use super::*;

/// Made-up windows: (title, app id).
const WINDOWS: [(&str, &str); 5] = [
    ("Release notes — Firefox", "firefox"),
    ("~/projects/demo : bash", "org.kde.konsole"),
    ("Café — Firefox", "firefox"),
    ("Música — Dolphin", "org.kde.dolphin"),
    ("Ñandú", "org.telegram.desktop"),
];

/// Indices of the hits for `query`, best first.
fn order(query: &str) -> Vec<usize> {
    rank(query, &WINDOWS).into_iter().map(|h| h.index).collect()
}

/// Highlights of hit `index` for `query`, as `(start, end)` pairs.
fn highlights(query: &str, index: usize) -> Vec<(usize, usize)> {
    rank(query, &WINDOWS)
        .into_iter()
        .find(|h| h.index == index)
        .expect("a hit")
        .highlights
        .iter()
        .map(|r| (r.start, r.end))
        .collect()
}

#[test]
fn empty_query_keeps_every_window_in_order() {
    assert_eq!(order(""), [0, 1, 2, 3, 4]);
}

#[test]
fn best_match_comes_first() {
    assert_eq!(order("notes")[0], 0);
    assert_eq!(order("demo")[0], 1); // Matches with a typo follow
}

#[test]
fn equal_scores_keep_the_given_order() {
    let hits = order("firefox");

    assert_eq!(hits[..2], [0, 2]);
}

#[test]
fn app_id_matches_without_highlights() {
    assert_eq!(order("konsole"), [1]);
    assert_eq!(highlights("konsole", 1), []);
}

#[test]
fn smart_case() {
    assert_eq!(order("Cafe")[0], 2);
    assert_eq!(order("CAFE"), []);
}

#[test]
fn accents_fold_and_highlight_the_real_letters() {
    // "Café": C 0..1, a 1..2, f 2..3, é 3..5.
    assert_eq!(highlights("cafe", 2), [(0, 5)]);
    assert_eq!(highlights("café", 2), [(0, 5)]);

    // A query with accents is not folded: `è` is a typo for `é`.
    assert_eq!(highlights("cafè", 2), [(0, 3)]);
}

#[test]
fn non_ascii_matches_with_unicode_case() {
    assert_eq!(order("ñandú"), [4]);
    assert_eq!(order("músi"), [3]);
    assert_eq!(highlights("músi", 3), [(0, 5)]);
}

#[test]
fn one_typo_per_four_characters() {
    assert_eq!(order("relaese")[0], 0); // 7 characters, 1 typo
    assert_eq!(order("rlz"), []); // 3 characters, no typo
}
