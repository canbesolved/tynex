// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Text rules for window titles in any language and script.

/// Cleans a title once, when it arrives from an adapter.
///
/// Control characters become spaces, bidi override and isolate controls
/// are removed, whitespace runs collapse to one space and the ends are
/// trimmed. Letters, case and accents are never changed.
#[must_use]
pub fn sanitize_title(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut space = false;

    for c in raw.chars() {
        if is_bidi_control(c) {
            continue;
        }

        if c.is_control() || c.is_whitespace() {
            space = !out.is_empty(); // Never leading
            continue;
        }

        if space {
            out.push(' ');
            space = false;
        }
        out.push(c);
    }
    out
}

/// Returns the byte offsets where `text` may be cut.
///
/// Offsets are on `char` boundaries and never between a base letter and
/// its combining marks. The first offset is 0, the last is `text.len()`.
pub fn cut_points(text: &str) -> impl Iterator<Item = usize> + '_ {
    text.char_indices()
        .filter(|&(_, c)| !is_combining_mark(c))
        .map(|(i, _)| i)
        .chain(std::iter::once(text.len()))
}

/// Returns true for bidi override and isolate controls.
fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

/// Returns true for combining diacritical marks.
#[must_use]
pub fn is_combining_mark(c: char) -> bool {
    matches!(
        c,
        '\u{0300}'..='\u{036F}'
            | '\u{1AB0}'..='\u{1AFF}'
            | '\u{1DC0}'..='\u{1DFF}'
            | '\u{20D0}'..='\u{20FF}'
            | '\u{FE20}'..='\u{FE2F}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controls_become_spaces() {
        assert_eq!(sanitize_title("a\tb\nc\u{7f}d\u{85}e"), "a b c d e");
    }

    #[test]
    fn bidi_controls_are_removed() {
        assert_eq!(sanitize_title("abc\u{202E}def\u{2066}g"), "abcdefg");
    }

    #[test]
    fn whitespace_collapses_and_trims() {
        assert_eq!(sanitize_title("  a \u{a0}\t b  "), "a b");
        assert_eq!(sanitize_title(" \t\u{202E} "), "");
    }

    #[test]
    fn letters_stay_unchanged() {
        for title in ["Música — Dolphin", "微信", "Café — Firefox"] {
            assert_eq!(sanitize_title(title), title);
        }
        assert_eq!(sanitize_title("e\u{301}"), "e\u{301}");
    }

    #[test]
    fn cut_points_skip_combining_marks() {
        let text = "e\u{301}x"; // e at 0, U+0301 at 1, x at 3
        assert_eq!(cut_points(text).collect::<Vec<_>>(), [0, 3, 4]);
    }

    #[test]
    fn cut_points_follow_char_boundaries() {
        let text = "微信"; // Two 3-byte chars
        assert_eq!(cut_points(text).collect::<Vec<_>>(), [0, 3, 6]);
        assert_eq!(cut_points("").collect::<Vec<_>>(), [0]);
    }
}
