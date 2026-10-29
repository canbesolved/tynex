// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hint text in the input bar.

use super::{draw_popup, measured};
use crate::core::state::State;
use crate::ui::draw::{Metrics, Paint};
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::glyphs::Face;
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

const FONT: &[u8] = include_bytes!("../../tests/fonts/dejavu_sans.ttf");

fn metrics(color: Rgba) -> Metrics {
    Metrics {
        font_size: 14.0,
        width: 300.0,
        max_rows: 3,
        scale: 1.0,
        title_bar: false,
        show_icons: false,
        hint_color: color,
    }
}

fn render(state: &State, color: Rgba) -> Vec<u8> {
    let m = metrics(color);
    let theme = Theme::default();
    let size = measured(&state.view(), &m, &theme);
    let mut data = vec![0; (size.width * size.height * 4) as usize];
    let mut canvas = Canvas::new(&mut data, size.width, size.height).unwrap();
    let mut text = Text::new(Face::new(FONT, 0).unwrap());
    let mut icons = Icons::new(Box::new(NoIcons));
    let mut paint = Paint {
        canvas: &mut canvas,
        text: &mut text,
        icons: &mut icons,
        theme: &theme,
    };

    draw_popup(&state.view(), &m, &mut paint);
    data
}

#[test]
fn empty_query_draws_the_hint_in_its_color() {
    let color = Rgba(255, 0, 0, 255);
    let mut hinted = State::new(Vec::new());

    hinted.hint = "WWW".into();

    let pixels = render(&hinted, color);
    let mut typed = hinted.clone();

    typed.query = "x".into();

    let mut hidden = State::new(Vec::new());

    hidden.query = "x".into();
    assert_ne!(pixels, render(&State::new(Vec::new()), color));
    assert!(pixels.chunks(4).any(|px| px[0] > px[1] && px[0] > px[2]));
    assert_eq!(render(&typed, color), render(&hidden, color));
}

#[test]
fn loading_dots_use_the_hint_color() {
    let color = Rgba(255, 0, 0, 255);
    let mut hinted = State::new(Vec::new());

    hinted.hint = "...".into();

    let pixels = render(&hinted, color);

    assert!(pixels.chunks(4).any(|px| px[0] > px[1] && px[0] > px[2]));
}
