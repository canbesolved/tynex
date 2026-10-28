// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Fuzzy ranking of windows with `frizbee` (ADR-0022). The wrapper keeps
//! the pre-1.0 API in one place.

use std::cmp::Reverse;
use std::ops::Range;

use frizbee::{Config, Matcher};

use crate::modules::search::fold::Folded;

/// A window that matches the query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    pub index: usize,                  // Index into the ranked items
    pub score: u16,                    // Higher is better
    pub highlights: Vec<Range<usize>>, // Byte ranges in the title
}

/// Ranks `items` (title, app) for `query`, best first; equal scores
/// keep the order of `items`. An empty query keeps every item.
///
/// Smart case, one typo per four characters once the query has at
/// least five characters (shorter queries allow none), and an ASCII
/// query also matches accented letters.
#[must_use]
pub fn rank(query: &str, items: &[(&str, &str)]) -> Vec<Hit> {
    if query.is_empty() {
        return (0..items.len())
            .map(|index| Hit {
                index,
                score: 0,
                highlights: Vec::new(),
            })
            .collect();
    }

    let mut matcher =
        Matcher::new(query, &Config::default().max_typos(Some(typos(query))));
    let titles: Vec<Folded> = items
        .iter()
        .map(|(title, _)| {
            if query.is_ascii() {
                Folded::new(title)
            } else {
                Folded::plain(title)
            }
        })
        .collect();
    let texts: Vec<&str> = titles.iter().map(|f| f.text.as_str()).collect();
    let apps: Vec<&str> = items.iter().map(|&(_, app)| app).collect();
    let mut scores = vec![None; items.len()];
    let mut hits: Vec<Hit> = Vec::new();

    for m in matcher.match_list_indices(&texts) {
        let i = m.index as usize;

        scores[i] = Some(m.score);
        hits.push(Hit {
            index: i,
            score: m.score,
            highlights: titles[i].ranges(items[i].0, &m.indices),
        });
    }
    // One `frizbee` entry point only: each adds about 1 MB (ADR-0044).
    for m in matcher.match_list_indices(&apps) {
        let i = m.index as usize;

        match scores[i] {
            Some(score) if score >= m.score => {}
            Some(_) => {
                scores[i] = Some(m.score);
                if let Some(hit) = hits.iter_mut().find(|h| h.index == i) {
                    hit.score = m.score;
                }
            }
            None => {
                scores[i] = Some(m.score);
                hits.push(Hit {
                    index: i,
                    score: m.score,
                    highlights: Vec::new(), // Only the title is drawn
                });
            }
        }
    }
    hits.sort_by_key(|h| (Reverse(scores[h.index]), h.index));
    hits
}

/// Typos allowed for `query`: none under five characters, then one
/// per four characters (`code` is exact-ish; `relaese` still works).
fn typos(query: &str) -> u16 {
    let n = query.chars().count();

    if n < 5 {
        0
    } else {
        u16::try_from(n / 4).unwrap_or(u16::MAX)
    }
}

#[cfg(test)]
#[path = "fuzzy_tests.rs"]
mod tests;
