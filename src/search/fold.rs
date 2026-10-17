// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Accent folding (`Café` to `Cafe`), with a map back to the original
//! bytes so highlights land on the real letters.

use std::ops::Range;

use crate::core::text::is_combining_mark;

/// Base letters, one per code point from U+00C0, `.` for none.
const LATIN_1: &str =
    "AAAAAA.CEEEEIIIIDNOOOOO.OUUUUY..aaaaaa.ceeeeiiiidnooooo.ouuuuy.y";

/// Base letters from U+0100.
const EXTENDED_A: &str = concat!(
    "AaAaAaCcCcCcCcDdDdEeEeEeEeEeGgGgGgGgHhHhIiIiIiIiIi..JjKk.LlLlLlL",
    "lLlNnNnNn...OoOoOo..RrRrRrSsSsSsSsTtTtTtUuUuUuUuUuUuWwYyYZzZzZzs",
);

/// Base letters from U+1E00.
const EXTENDED_ADDITIONAL: &str = concat!(
    "AaBbBbBbCcDdDdDdDdDdEeEeEeEeEeFfGgHhHhHhHhHhIiIiKkKkKkLlLlLlLlMm",
    "MmMmNnNnNnNnOoOoOoOoPpPpRrRrRrRrSsSsSsSsSsTtTtTtTtUuUuUuUuUuVvVv",
    "WwWwWwWwWwXxXxYyZzZzZzhtwy......AaAaAaAaAaAaAaAaAaAaAaAaEeEeEeEe",
    "EeEeEeEeIiIiOoOoOoOoOoOoOoOoOoOoOoOoUuUuUuUuUuUuUuYyYyYyYy......",
);

/// Tables by first code point. Generated from Unicode NFD
/// decompositions, plus letters with a stroke (`Ø`, `Đ`, `Ł`, ...).
const TABLES: [(u32, &str); 3] = [
    (0xC0, LATIN_1),
    (0x100, EXTENDED_A),
    (0x1E00, EXTENDED_ADDITIONAL),
];

/// A title prepared for matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folded {
    pub text: String,   // What the matcher sees
    starts: Vec<usize>, // Original byte offset of each byte of `text`
}

impl Folded {
    /// Folds accented Latin letters to ASCII and drops combining marks.
    #[must_use]
    pub fn new(title: &str) -> Self {
        Self::build(title, true)
    }

    /// Keeps `title` as is, for queries with non-ASCII letters.
    #[must_use]
    pub fn plain(title: &str) -> Self {
        Self::build(title, false)
    }

    fn build(title: &str, fold: bool) -> Self {
        let mut text = String::with_capacity(title.len());
        let mut starts = Vec::with_capacity(title.len());

        for (at, c) in title.char_indices() {
            match fold.then(|| base(c)) {
                Some(_) if is_combining_mark(c) => {} // Dropped
                Some(Some(letter)) => {
                    text.push(letter);
                    starts.push(at);
                }
                _ => {
                    text.push(c);
                    starts.extend(std::iter::repeat_n(at, c.len_utf8()));
                }
            }
        }
        Self { text, starts }
    }

    /// Turns byte offsets in `text` into sorted, merged byte ranges of
    /// `original`, each covering whole characters and their marks.
    #[must_use]
    pub fn ranges(&self, original: &str, offsets: &[u32]) -> Vec<Range<usize>> {
        let mut starts: Vec<usize> = offsets
            .iter()
            .filter_map(|&o| self.starts.get(usize::try_from(o).ok()?))
            .copied()
            .collect();
        let mut ranges: Vec<Range<usize>> = Vec::new();

        starts.sort_unstable();
        starts.dedup();
        for start in starts {
            let end = char_end(original, start);

            match ranges.last_mut() {
                Some(last) if last.end >= start => last.end = end,
                _ => ranges.push(start..end),
            }
        }
        ranges
    }
}

/// Returns the ASCII base letter of an accented Latin letter.
fn base(c: char) -> Option<char> {
    TABLES.iter().find_map(|&(first, table)| {
        let i = usize::try_from(u32::from(c).checked_sub(first)?).ok()?;
        let letter = *table.as_bytes().get(i)?;

        (letter != b'.').then_some(char::from(letter))
    })
}

/// End of the character at `start`, including combining marks after it.
fn char_end(text: &str, start: usize) -> usize {
    let mut chars = text[start..].char_indices();
    let first = chars.next().map_or(0, |(_, c)| c.len_utf8());

    chars
        .take_while(|&(_, c)| is_combining_mark(c))
        .last()
        .map_or(start + first, |(i, c)| start + i + c.len_utf8())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Byte ranges as `(start, end)` pairs.
    fn spans(ranges: &[Range<usize>]) -> Vec<(usize, usize)> {
        ranges.iter().map(|r| (r.start, r.end)).collect()
    }

    #[test]
    fn tables_cover_their_blocks() {
        assert_eq!(LATIN_1.len(), 0x40);
        assert_eq!(EXTENDED_A.len(), 0x80);
        assert_eq!(EXTENDED_ADDITIONAL.len(), 0x100);
    }

    #[test]
    fn accents_fold_to_ascii() {
        assert_eq!(Folded::new("Café Zürich").text, "Cafe Zurich");
        assert_eq!(Folded::new("Łódź Ørsted").text, "Lodz Orsted");
        assert_eq!(Folded::new("Ệ ×").text, "E ×");
        assert_eq!(Folded::new("cafe\u{301}").text, "cafe");
    }

    #[test]
    fn other_scripts_stay_unchanged() {
        assert_eq!(Folded::new("微信 Ω").text, "微信 Ω");
        assert_eq!(Folded::plain("Café").text, "Café");
    }

    #[test]
    fn ranges_map_back_to_original_letters() {
        let title = "Café";
        let folded = Folded::new(title);

        assert_eq!(spans(&folded.ranges(title, &[3, 0, 1])), [(0, 2), (3, 5)]);
        assert_eq!(spans(&Folded::plain(title).ranges(title, &[3])), [(3, 5)]);
    }

    #[test]
    fn ranges_include_combining_marks() {
        let title = "cafe\u{301}!";

        assert_eq!(spans(&Folded::new(title).ranges(title, &[3])), [(3, 6)]);
    }
}
