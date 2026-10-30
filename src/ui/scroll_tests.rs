// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Wheel scrolling, selection follow and the scrollbar.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use super::{Viewport, first_row, follow};
use crate::config::TaskbarMode;
use crate::core::state::State;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics, Paint};
use crate::ui::frames::{draw_popup, measured};
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::glyphs::Face;
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;

const FONT: &[u8] = include_bytes!("../../tests/fonts/dejavu_sans.ttf");

const M: Metrics = Metrics {
    font_size: 14.0,
    width: 300.0,
    max_rows: 3,
    list_rows: 3,
    first_row: None,
    scale: 1.0,
    taskbar: TaskbarMode::Titlebar,
    show_icons: false,
    hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
    icon_size: 24.0,
    border: 4.0,
};

fn state(n: usize) -> State {
    let rows = (0..n).map(|i| Row {
        target: RowTarget::Window(WindowId(format!("w{i}"))),
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: format!("Window {i}"),
        title_color: None,
        highlights: Vec::new(),
    });

    State::new(rows.collect())
}

#[test]
fn a_new_selection_moves_the_rows_the_least() {
    assert_eq!(follow(0, 1, 3), 0);
    assert_eq!(follow(0, 5, 3), 3);
    assert_eq!(follow(4, 2, 3), 2);
}

#[test]
fn the_wheel_scrolls_without_moving_the_selection() {
    let mut s = state(10);
    let mut list = Viewport::default();

    s.selected = 1;
    assert_eq!(list.place(&s.view(), M).first_row, Some(0));
    assert!(list.scroll(&s.view(), &M, 1, 0.0));
    assert_eq!(list.place(&s.view(), M).first_row, Some(3));
    assert_eq!(s.selected, 1);

    assert!(list.scroll(&s.view(), &M, 5, 0.0));
    assert_eq!(list.place(&s.view(), M).first_row, Some(7)); // Last page
    assert!(!list.scroll(&s.view(), &M, 1, 0.0));

    s.selected = 2; // A key moved the selection off the rows
    assert_eq!(list.place(&s.view(), M).first_row, Some(2));
}

#[test]
fn smooth_scrolling_adds_up_to_whole_rows() {
    let s = state(10);
    let mut list = Viewport::default();

    list.place(&s.view(), M);
    assert!(!list.scroll(&s.view(), &M, 0, 0.6));
    assert!(list.scroll(&s.view(), &M, 0, 0.6));
    assert_eq!(list.place(&s.view(), M).first_row, Some(1));
    assert!(list.scroll(&s.view(), &M, 0, -1.2)); // 0.2 left from above
    assert_eq!(list.place(&s.view(), M).first_row, Some(0));
}

#[test]
fn without_a_set_row_the_list_follows_the_selection() {
    assert_eq!(first_row(&M, 5, 10), 3);
    assert_eq!(first_row(&M, 9, 10), 7);
    assert_eq!(
        first_row(
            &Metrics {
                first_row: Some(9),
                ..M
            },
            0,
            10
        ),
        7
    );
}

/// Draws `n` windows and returns the pixel at the scrollbar column,
/// at the middle of the first row.
fn bar_pixel(windows: usize) -> [u8; 4] {
    let theme = Theme::default();
    let s = state(windows);
    let size = measured(&s.view(), &M, &theme);
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

    draw_popup(&s.view(), &M, &mut paint);

    let grid = Grid::new(&M, &theme);
    let col = (size.width as f32 - grid.pad - 1.5) as u32;
    let line = (size.height as f32 - grid.pad - 2.5 * grid.row_h) as u32;
    let at = ((line * size.width + col) * 4) as usize;

    [data[at], data[at + 1], data[at + 2], data[at + 3]]
}

#[test]
fn a_long_list_shows_a_scrollbar() {
    let bar = bar_pixel(10);
    let none = bar_pixel(3);

    assert!(bar[0] > none[0] + 0x10, "bar {bar:?} none {none:?}");
}
