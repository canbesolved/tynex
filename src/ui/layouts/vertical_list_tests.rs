// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Tests for the vertical list: sizes, badges, selection and scrolling.

use super::*;
use crate::core::state::State;
use crate::core::window::WindowId;
use crate::ui::render::canvas::Canvas;
use crate::ui::render::glyphs::{Face, NoFallback};
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;

const FONT: &[u8] = include_bytes!("../../../tests/fonts/dejavu_sans.ttf");
const M: Metrics = Metrics {
    font_size: 14.0,
    width: 300.0,
    max_rows: 3,
    scale: 1.0,
};

fn state(mode: ModeKind, n: usize) -> State {
    let rows = (0..n)
        .map(|i| Row {
            id: WindowId(format!("w{i}")),
            key: char::from_digit(10 + i as u32, 36), // a, b, c, …
            app_id: String::new(),
            app_name: String::new(),
            icon: None,
            title: format!("Window {i}"),
            highlights: Vec::new(),
        })
        .collect();

    State::new(mode, rows)
}

/// Draws `state` and returns the RGBA buffer and its size.
fn render(state: &State) -> (Vec<u8>, Size) {
    let theme = Theme::default();
    let size = VerticalList.size(&state.view(), &M, &theme);
    let mut data = vec![0; (size.width * size.height * 4) as usize];
    let mut canvas = Canvas::new(&mut data, size.width, size.height).unwrap();
    let mut text = Text::new(Face::new(FONT, 0).unwrap(), Box::new(NoFallback));
    let mut icons = Icons::new(Box::new(NoIcons));
    let mut paint = Paint {
        canvas: &mut canvas,
        text: &mut text,
        icons: &mut icons,
        theme: &theme,
    };

    VerticalList.draw(&state.view(), &M, &mut paint);
    (data, size)
}

fn pixel(data: &[u8], size: Size, x: f32, y: f32) -> [u8; 4] {
    let i = ((y as u32 * size.width + x as u32) * 4) as usize;

    [data[i], data[i + 1], data[i + 2], data[i + 3]]
}

/// Center of row `n` of the list body, in hotkey mode.
fn row_center(n: usize) -> f32 {
    let g = Grid::new(&M, &Theme::default());

    g.pad + g.row_h * (n as f32 + 0.5)
}

#[test]
fn height_follows_rows_up_to_max() {
    let theme = Theme::default();
    let size =
        |n| VerticalList.size(&state(ModeKind::Hotkey, n).view(), &M, &theme);
    let g = Grid::new(&M, &theme);

    assert_eq!(size(2).width, 300);
    assert_eq!(size(2).height, (2.0 * g.pad + 2.0 * g.row_h) as u32);
    assert_eq!(size(9).height, size(3).height); // Capped at max_rows
    assert_eq!(size(0).height, size(1).height); // Never collapses
}

#[test]
fn search_mode_adds_a_search_bar() {
    let theme = Theme::default();
    let hotkey =
        VerticalList.size(&state(ModeKind::Hotkey, 2).view(), &M, &theme);
    let search =
        VerticalList.size(&state(ModeKind::Search, 2).view(), &M, &theme);
    let g = Grid::new(&M, &theme);

    assert_eq!(search.height - hotkey.height, (g.row_h + 1.0) as u32);
}

#[test]
fn selected_row_is_highlighted() {
    let mut s = state(ModeKind::Hotkey, 3);

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
fn badges_only_in_hotkey_mode() {
    let g = Grid::new(&M, &Theme::default());
    let corner = g.pad + g.gap + g.badge / 2.0; // Badge center, hotkey row 0
    let badge = |mode| {
        let (data, size) = render(&state(mode, 2));
        let y = if mode == ModeKind::Search {
            g.row_h + 1.0
        } else {
            0.0
        };

        pixel(&data, size, corner + g.badge / 3.0, y + row_center(0))
    };
    let theme = Theme::default();

    assert_eq!(
        badge(ModeKind::Hotkey)[..3],
        [theme.badge.0, theme.badge.1, theme.badge.2]
    );
    assert_ne!(
        badge(ModeKind::Search)[..3],
        [theme.badge.0, theme.badge.1, theme.badge.2]
    );
}

#[test]
fn list_scrolls_to_keep_selection_visible() {
    let mut s = state(ModeKind::Hotkey, 6);

    s.selected = 4; // Rows 2 to 4 are shown; row 4 is the last one

    let (data, size) = render(&s);
    let x = size.width as f32 - 20.0;

    assert_ne!(
        pixel(&data, size, x, row_center(2)),
        pixel(&data, size, x, row_center(0))
    );
}
