// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Scrolling an edited title so the cursor stays visible.

use super::scroll_start;

/// Every character is 10 pixels wide.
fn width(text: &str) -> f32 {
    text.chars().map(|_| 10.0).sum()
}

#[test]
fn a_text_that_fits_does_not_scroll() {
    assert_eq!(scroll_start("Notes", 5, 50.0, width), 0);
    assert_eq!(scroll_start("", 0, 50.0, width), 0);
}

#[test]
fn a_long_text_scrolls_so_the_cursor_shows() {
    let text = "A very long window title";

    assert_eq!(scroll_start(text, text.len(), 50.0, width), 19);
    assert_eq!(scroll_start(text, 6, 50.0, width), 1);
    assert_eq!(scroll_start(text, 3, 50.0, width), 0);
}

#[test]
fn scrolling_stops_on_character_boundaries() {
    let text = "Café über naïve";

    let start = scroll_start(text, text.len(), 45.0, width);

    assert!(text.is_char_boundary(start));
    assert_eq!(&text[start..], "aïve");
}
