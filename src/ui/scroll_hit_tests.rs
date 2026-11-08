// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Pointer hits on the list on screen: the title list, scrolled, and
//! the output rows while a query is typed.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use super::hit;
use crate::config::TaskbarMode;
use crate::core::event::Click;
use crate::core::state::State;
use crate::core::view::{Row, RowTarget};
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics};
use crate::ui::frames;
use crate::ui::render::canvas::Rgba;
use crate::ui::render::theme::Theme;

const M: Metrics = Metrics {
    font_size: 14.0,
    width: 300.0,
    max_rows: 19,
    list_rows: 4,
    first_row: Some(5),
    scale: 1.0,
    taskbar: TaskbarMode::Titlebar,
    show_icons: false,
    hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
    icon_size: 24.0,
    border: 4.0,
};

fn state(n: usize) -> State {
    let rows = (0..n)
        .map(|i| Row {
            target: RowTarget::Window(WindowId(format!("w{i:02}"))),
            app_id: "kwrite".into(),
            app_name: "KWrite".into(),
            icon: None,
            title: format!("w{i:02}"),
            title_color: None,
            highlights: Vec::new(),
        })
        .collect();

    State::new(rows)
}

#[test]
fn a_click_on_a_scrolled_list_picks_that_window() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let state = state(12);
    let top = frames::list_top(&state.view(), &M, &grid);
    let click = hit(
        &state.view(),
        &M,
        &grid,
        grid.pad + 4.0,
        top + 0.5 * grid.row_h,
    );

    assert_eq!(click, Click::Row(5));
}

#[test]
fn the_scrollbar_is_not_a_row() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let state = state(12);
    let top = frames::list_top(&state.view(), &M, &grid);
    let inner = M.width - 2.0 * grid.pad;
    let click = hit(
        &state.view(),
        &M,
        &grid,
        grid.pad + inner - 1.0,
        top + 0.5 * grid.row_h,
    );

    assert_eq!(click, Click::Elsewhere);
}

#[test]
fn a_typed_query_hits_the_output_rows() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let mut typed = state(3);

    typed.query = "w".into();
    typed.output = typed.rows.clone();
    let top = frames::list_top(&typed.view(), &M, &grid);
    let at = |row: f32| {
        hit(
            &typed.view(),
            &M,
            &grid,
            grid.pad + 4.0,
            top + row * grid.row_h,
        )
    };

    assert_eq!(at(1.5), Click::Row(1)); // Three rows fit: no scrolling
    assert_eq!(at(3.5), Click::Elsewhere); // Below the last row
}
