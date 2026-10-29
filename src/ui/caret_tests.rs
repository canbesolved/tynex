// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Caret drawn in the input bar.

use super::tests::{FONT, M, pixel, render, state};
use crate::ui::draw::Grid;
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
