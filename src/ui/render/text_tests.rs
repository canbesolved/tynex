// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for text measuring, missing glyphs, truncation and drawing.

use super::*;

const FONT: &[u8] = include_bytes!("../../../tests/fonts/dejavu_sans.ttf");
const SIZE: f32 = 16.0;

fn font() -> Face {
    Face::new(FONT, 0).expect("bundled test font")
}

fn text() -> Text {
    Text::new(font())
}

fn style(highlights: &[Range<usize>]) -> Style<'_> {
    Style {
        size: SIZE,
        color: Rgba(255, 255, 255, 255),
        highlight: Rgba(255, 0, 0, 255),
        highlights,
    }
}

#[test]
fn widths_add_up() {
    let t = text();
    let one = t.width("o", SIZE);

    assert!(one > 0.0);
    assert!((t.width("oo", SIZE) - 2.0 * one).abs() < 0.01);
    assert!(t.width("Canción", SIZE) > 0.0); // DejaVu has accents
    assert!(t.width("", SIZE).abs() < f32::EPSILON);
}

#[test]
fn missing_characters_are_boxes() {
    let mut t = text();
    let (w, h) = (40, 24);
    let mut data = vec![0; w * h * 4];
    let mut canvas = Canvas::new(&mut data, w as u32, h as u32).unwrap();
    let box_w = t.width("微", SIZE); // Not in DejaVu Sans

    assert!(box_w > 0.0);
    assert!((t.width("信", SIZE) - box_w).abs() < f32::EPSILON);
    t.draw(&mut canvas, "微", (2.0, 18.0), 40.0, &style(&[]));
    assert!(data.chunks(4).any(|px| px[3] > 0)); // The box is drawn
}

#[test]
fn short_text_is_not_cut() {
    assert_eq!(text().fit("Kate", SIZE, 500.0), None);
}

#[test]
fn long_text_is_cut_at_safe_points() {
    let t = text();
    let title = "cafe\u{301} cafe\u{301} cafe\u{301} cafe\u{301}";
    let full = t.width(title, SIZE);
    let ellipsis = t.width(ELLIPSIS, SIZE).ceil() as u32;

    // Below the ellipsis width nothing fits; the ellipsis alone is drawn.
    for max in (ellipsis..full as u32).map(|m| m as f32) {
        let cut = t.fit(title, SIZE, max).expect("must be cut");

        assert!(cut_points(title).any(|p| p == cut), "cut {cut} at {max}");
        assert!(t.width(&title[..cut], SIZE) + t.width(ELLIPSIS, SIZE) <= max);
    }
}

#[test]
fn drawing_uses_text_and_highlight_colors() {
    let (w, h) = (80, 24);
    let mut data = vec![0; w * h * 4];
    let mut canvas = Canvas::new(&mut data, w as u32, h as u32).unwrap();
    let mut t = text();
    let split = (2.0 + t.width("H", SIZE)) as usize; // Between the two Hs
    let first_h = std::slice::from_ref(&(0..1));

    canvas.clear(Rgba(0, 0, 0, 255));
    t.draw(&mut canvas, "HH", (2.0, 18.0), 80.0, &style(first_h));

    let lit = |x: usize, y: usize| {
        let i = (y * w + x) * 4;

        (data[i] > 200, data[i + 1] > 200) // (red, green) channels lit
    };
    let all = |xs: std::ops::Range<usize>| {
        xs.flat_map(|x| (0..h).map(move |y| (x, y)))
            .map(|(x, y)| lit(x, y))
    };

    // First H only red, second H only white.
    assert!(all(0..split).any(|p| p == (true, false)));
    assert!(all(0..split).all(|p| p != (true, true)));
    assert!(all(split..w).any(|p| p == (true, true)));
    assert!(all(split..w).all(|p| p != (true, false)));
}
