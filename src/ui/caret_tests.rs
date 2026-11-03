// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Caret drawn in the input bar.

use super::tests::{FONT, M, pixel, render, state};
use crate::core::window::WindowId;
use crate::ui::draw::Grid;
use crate::ui::input_bar::scroll_start;
use crate::ui::render::glyphs::Face;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

#[test]
fn input_bar_draws_a_beam_at_the_cursor() {
    let g = Grid::new(&M, &Theme::default());
    let (data, size) = render(&state(0));
    let text = Text::new(Face::new(FONT, 0).unwrap());
    let (ascent, descent) = text.metrics(g.font);
    let left = g.pad + g.gap;
    let mark = text.width(">", g.font);
    let space = text.width(" ", g.font);
    let x = (left + mark + g.gap + space).round();
    let y = (g.pad + g.row_h / 2.0 + f32::midpoint(ascent, descent)).round();
    let caret = [0xe6, 0xed, 0xf3, 0xff];

    assert_eq!(pixel(&data, size, x, y), caret);
    assert_eq!(pixel(&data, size, x, y - 1.0), caret);
    assert_ne!(pixel(&data, size, x + 3.0, y), caret);

    let mut s = state(0);

    s.query = "ab".into();

    let start = render(&s);

    s.cursor = 2;

    let end = render(&s);
    let x1 = (left + mark + g.gap + space + text.width("ab", g.font)).round();

    assert_ne!(pixel(&start.0, size, x1, y), caret);
    assert_eq!(pixel(&end.0, size, x1, y), caret);
}

#[test]
fn a_long_title_being_renamed_scrolls_to_show_the_beam() {
    let g = Grid::new(&M, &Theme::default());
    let mut s = state(1);

    s.rows[0].title =
        "A very long window title that is wider than the row".into();
    s.output = s.rows.clone();
    s.start_rename(&WindowId("w0".into()));

    let (data, size) = render(&s);
    let text = Text::new(Face::new(FONT, 0).unwrap());
    let title = &s.rows[0].title;
    let left = g.pad + g.gap + g.icon + g.gap;
    let right = size.width as f32 - g.pad - g.gap;
    let room = right - left - 1.0;
    let start =
        scroll_start(title, title.len(), room, |part| text.width(part, g.font));
    let x = (left + text.width(&title[start..], g.font)).round();
    let top = crate::ui::frames::list_top(&s.view(), &M, &g);
    let (ascent, descent) = text.metrics(g.font);
    let y = (top + g.row_h / 2.0 + f32::midpoint(ascent, descent)).round();
    let caret = [0xe6, 0xed, 0xf3, 0xff];

    assert!(start > 0, "the title must scroll");
    assert!(x < right, "the beam is inside the row");
    assert_eq!(pixel(&data, size, x, y - 1.0), caret);
    assert_eq!(pixel(&data, size, x, y - 4.0), caret);
}
