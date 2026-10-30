// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Dropdown: drawing, and opening over the title list.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use super::{Panel, draw, panel};
use crate::config::TaskbarMode;
use crate::core::event::Click;
use crate::core::state::State;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics, Paint};
use crate::ui::frames;
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::glyphs::Face;
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::taskbar::icons::hit;

const FONT: &[u8] = include_bytes!("../../../tests/fonts/dejavu_sans.ttf");

const M: Metrics = Metrics {
    font_size: 14.0,
    width: 300.0,
    max_rows: 19,
    list_rows: 4,
    first_row: None,
    scale: 1.0,
    taskbar: TaskbarMode::Both,
    show_icons: false,
    hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
    icon_size: 24.0,
    border: 4.0,
};

/// One app with `n` windows, titled so they sort by number.
fn state(n: usize) -> State {
    let ids: Vec<_> = (0..n).map(|i| WindowId(format!("w{i:02}"))).collect();
    let rows = ids.iter().map(|id| Row {
        target: RowTarget::Window(id.clone()),
        app_id: "kwrite".into(),
        app_name: "KWrite".into(),
        icon: None,
        title: id.0.clone(),
        title_color: None,
        highlights: Vec::new(),
    });
    let mut state = State::new(rows.collect());

    state.icons = vec![AppIcon {
        app_id: "kwrite".into(),
        icon: None,
        windows: ids,
    }];
    state
}

fn text() -> Text {
    Text::new(Face::new(FONT, 0).unwrap())
}

#[test]
fn the_title_list_sits_under_the_icons_without_spare_room() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let state = state(19);
    let under = frames::icon_bottom(&state.view(), &M, &grid);
    let titles = 2.0 * grid.pad + 4.0 * grid.row_h;
    let height = frames::measured(&state.view(), &M, &theme).height;

    assert_eq!(height, (under + 3.0 + titles).round() as u32);
}

#[test]
fn the_dropdown_stays_over_the_title_list_and_scrolls() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let mut state = state(19);

    state.open_dropdown();
    state.dropdown = Some(10);

    let mut place = super::Place::default();

    place.align(10, 4, 19);

    let open = panel(&state.view(), &M, &grid, &text(), place.first()).unwrap();
    let height = frames::measured(&state.view(), &M, &theme).height;

    assert_eq!(open.titles.len(), 4);
    assert_eq!((open.first, open.selected), (7, 3));
    assert_eq!(open.titles[3], "w10");
    assert!(open.y + open.h <= height as f32);

    place.align(8, 4, 19);
    assert_eq!(place.first(), 7);
}

#[test]
fn a_click_on_a_scrolled_dropdown_picks_that_window() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let mut state = state(19);

    state.open_dropdown();
    state.dropdown = Some(10);

    let mut place = super::Place::default();

    place.align(10, 4, 19);

    let open = panel(&state.view(), &M, &grid, &text(), place.first()).unwrap();
    let click = hit(
        &state.view(),
        &M,
        &grid,
        open.x + 5.0,
        open.y + 1.5 * open.row_h,
        1.0,
        Some(&open),
    );

    assert_eq!(click, Click::Menu(8));
}

#[test]
fn last_dropdown_title_survives_a_short_reveal() {
    let theme = Theme::default();
    let grid = Grid::new(&M, &theme);
    let h = 3.0 * grid.row_h;
    let panel = Panel {
        x: 10.0,
        y: 10.0,
        w: 120.0,
        h,
        row_h: grid.row_h,
        titles: vec!["one".into(), "two".into(), "three".into()],
        selected: 2,
        first: 0,
    };
    let mut data = vec![0; 200 * 200 * 4];
    let mut canvas = Canvas::new(&mut data, 200, 200).unwrap();
    let mut text = text();
    let mut icons = Icons::new(Box::new(NoIcons));
    let mut paint = Paint {
        canvas: &mut canvas,
        text: &mut text,
        icons: &mut icons,
        theme: &theme,
    };

    draw(&mut paint, &grid, &panel, (h - 0.5) / h, 1.0);

    let at = ((10.0 + 2.5 * grid.row_h) as usize * 200 + 40) * 4;

    assert!(data[at + 2] > data[at], "last row was skipped");
}
