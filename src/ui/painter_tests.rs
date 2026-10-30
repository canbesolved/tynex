// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for the popup: sizes, rows, selection and scrolling.

use super::{draw_popup, measured};
use crate::config::TaskbarMode;
use crate::core::state::State;
use crate::core::view::Row;
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics, Paint, Size};
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::glyphs::Face;
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

pub(in crate::ui) const FONT: &[u8] =
    include_bytes!("../../tests/fonts/dejavu_sans.ttf");
pub(in crate::ui) const M: Metrics = Metrics {
    font_size: 14.0,
    width: 300.0,
    max_rows: 3,
    scale: 1.0,
    taskbar: TaskbarMode::Titlebar,
    show_icons: true,
    hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
    icon_size: 24.0,
    border: 4.0,
};

pub(in crate::ui) fn state(n: usize) -> State {
    let rows = (0..n)
        .map(|i| Row {
            target: crate::core::view::RowTarget::Window(WindowId(format!(
                "w{i}"
            ))),
            app_id: String::new(),
            app_name: String::new(),
            icon: None,
            title: format!("Window {i}"),
            title_color: None,
            highlights: Vec::new(),
        })
        .collect();

    State::new(rows)
}

/// Draws `state` and returns the RGBA buffer and its size.
pub(in crate::ui) fn render(state: &State) -> (Vec<u8>, Size) {
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

pub(in crate::ui) fn pixel(data: &[u8], size: Size, x: f32, y: f32) -> [u8; 4] {
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
        taskbar: TaskbarMode::Off,
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
        taskbar: TaskbarMode::Off,
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
fn titles_only_starts_where_the_icon_slot_was() {
    let s = state(1);
    let g = Grid::new(&M, &Theme::default());
    let x = g.pad + g.gap + 4.0; // Inside the icon slot
    let y = row_center(0);
    let icons = Metrics {
        show_icons: true,
        ..M
    };
    let titles = Metrics {
        show_icons: false,
        ..M
    };
    let (with_slot, size) = render_with(&s, &icons);
    let (text_only, _) = render_with(&s, &titles);

    assert_ne!(pixel(&with_slot, size, x, y), pixel(&text_only, size, x, y));
}

#[test]
fn output_bar_keeps_the_icon_slot() {
    let open = state(1);
    let mut typed = state(1);

    typed.query = "w".into();
    typed.rows[0].app_name = "WWWW".into();
    typed.output = typed.rows.clone();

    let g = Grid::new(&M, &Theme::default());
    let x = g.pad + g.gap + 4.0;
    let y = row_center(0);
    let title = render(&open);
    let output = render_with(
        &typed,
        &Metrics {
            taskbar: TaskbarMode::Off,
            ..M
        },
    );

    assert_eq!(
        pixel(&title.0, title.1, x, y),
        pixel(&output.0, output.1, x, y)
    );
}
