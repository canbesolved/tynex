// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for the popup: sizes, rows, selection and scrolling.

use super::{draw_popup, measured};
use crate::core::state::State;
use crate::core::view::Row;
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics, Paint, Size};
use crate::ui::render::canvas::Canvas;
use crate::ui::render::glyphs::Face;
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

const FONT: &[u8] = include_bytes!("../../tests/fonts/dejavu_sans.ttf");
const M: Metrics = Metrics {
    font_size: 14.0,
    width: 300.0,
    max_rows: 3,
    scale: 1.0,
    title_bar: true,
    show_app_icon: false,
};

fn state(n: usize) -> State {
    let rows = (0..n)
        .map(|i| Row {
            target: crate::core::view::RowTarget::Window(WindowId(format!(
                "w{i}"
            ))),
            app_id: String::new(),
            app_name: String::new(),
            icon: None,
            title: format!("Window {i}"),
            highlights: Vec::new(),
        })
        .collect();

    State::new(rows)
}

/// Draws `state` and returns the RGBA buffer and its size.
fn render(state: &State) -> (Vec<u8>, Size) {
    render_with(state, &M)
}

/// Draws `state` with metrics `m`.
fn render_with(state: &State, m: &Metrics) -> (Vec<u8>, Size) {
    let theme = Theme::default();
    let size = measured(&state.view(), m, &theme);
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

    draw_popup(&state.view(), m, &mut paint);
    (data, size)
}

fn pixel(data: &[u8], size: Size, x: f32, y: f32) -> [u8; 4] {
    let i = ((y as u32 * size.width + x as u32) * 4) as usize;

    [data[i], data[i + 1], data[i + 2], data[i + 3]]
}

/// Center of row `n` of the list, below the input bar.
fn row_center(n: usize) -> f32 {
    let g = Grid::new(&M, &Theme::default());

    g.pad + g.row_h + 1.0 + g.row_h * (n as f32 + 0.5)
}

#[test]
fn height_follows_rows_up_to_max() {
    let theme = Theme::default();
    let size = |n| measured(&state(n).view(), &M, &theme);
    let g = Grid::new(&M, &theme);

    assert_eq!(size(2).width, 300);
    assert_eq!(size(2).height, (2.0 * g.pad + 3.0 * g.row_h + 1.0) as u32);
    assert_eq!(size(9).height, size(3).height); // Capped at max_rows
    assert_eq!(size(0).height, size(1).height); // Never collapses
}

#[test]
fn without_title_bar_rows_wait_for_a_query() {
    let theme = Theme::default();
    let m = Metrics {
        title_bar: false,
        ..M
    };
    let g = Grid::new(&m, &theme);
    let mut s = state(2);

    let input = measured(&s.view(), &m, &theme);

    assert_eq!(input.height, (2.0 * g.pad + g.row_h) as u32);
    s.query = "w".into();
    assert_eq!(
        measured(&s.view(), &m, &theme),
        measured(&state(2).view(), &M, &theme)
    );
}

#[test]
fn typing_hides_the_title_bar_until_the_query_is_cleared() {
    let theme = Theme::default();
    let off = Metrics {
        title_bar: false,
        ..M
    };
    let mut s = state(3);

    s.output.truncate(1); // One match, three open windows

    let open = measured(&s.view(), &M, &theme);
    let input_only = measured(&s.view(), &off, &theme);

    s.query = "w".into();

    let typed = measured(&s.view(), &M, &theme);

    assert!(typed.height < open.height);
    assert_eq!(typed, measured(&s.view(), &off, &theme));

    s.query.clear();
    assert_eq!(measured(&s.view(), &M, &theme), open);
    assert_eq!(measured(&s.view(), &off, &theme), input_only);
}

#[test]
fn selected_row_is_highlighted() {
    let mut s = state(3);

    s.selected = 1;

    let (data, size) = render(&s);
    let x = size.width as f32 - 20.0; // Right end of a row, no text there

    assert_ne!(
        pixel(&data, size, x, row_center(1)),
        pixel(&data, size, x, row_center(0))
    );
    assert_eq!(
        pixel(&data, size, x, row_center(0)),
        pixel(&data, size, x, row_center(2))
    );
}

#[test]
fn list_scrolls_to_keep_selection_visible() {
    let mut s = state(6);

    s.selected = 4; // Rows 2 to 4 are shown; row 4 is the last one

    let (data, size) = render(&s);
    let x = size.width as f32 - 20.0;

    assert_ne!(
        pixel(&data, size, x, row_center(2)),
        pixel(&data, size, x, row_center(0))
    );
}

#[test]
fn app_column_only_with_show_app_icon() {
    let mut s = state(1);

    s.rows[0].app_name = "WWWW".into();

    let app = Theme::default().app_text;
    let app_pixels = |m: &Metrics| {
        let (data, _) = render_with(&s, m);

        data.chunks(4)
            .filter(|px| px[..3] == [app.0, app.1, app.2])
            .count()
    };
    let shown = Metrics {
        show_app_icon: true,
        ..M
    };

    assert!(app_pixels(&shown) > 0);
    assert_eq!(app_pixels(&M), 0);
}

#[test]
fn output_bar_never_draws_the_app_column() {
    let mut s = state(1);

    s.query = "w".into();
    s.rows[0].app_name = "WWWW".into();
    s.output = s.rows.clone();

    let app = Theme::default().app_text;
    let metrics = Metrics {
        title_bar: false,
        show_app_icon: true,
        ..M
    };
    let (data, _) = render_with(&s, &metrics);
    let app_pixels = data
        .chunks(4)
        .filter(|px| px[..3] == [app.0, app.1, app.2])
        .count();

    assert_eq!(app_pixels, 0);
}
