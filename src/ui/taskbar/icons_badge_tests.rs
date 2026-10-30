// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Window count badge on the icon strip.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use super::icons::{layout, strip_height};
use crate::config::TaskbarMode;
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

const FONT: &[u8] = include_bytes!("../../../tests/fonts/dejavu_sans.ttf");

fn metrics() -> Metrics {
    Metrics {
        font_size: 14.0,
        width: 300.0,
        max_rows: 3,
        list_rows: 3,
        first_row: None,
        scale: 1.0,
        taskbar: TaskbarMode::Iconbar,
        show_icons: false,
        hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
        icon_size: 24.0,
        border: 4.0,
    }
}

fn state(ids: &[&str]) -> State {
    let rows = ids.iter().map(|id| Row {
        target: RowTarget::Window(WindowId((*id).into())),
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: (*id).into(),
        title_color: None,
        highlights: Vec::new(),
    });
    let mut state = State::new(rows.collect());

    state.icons = vec![AppIcon {
        app_id: "kate".into(),
        icon: None,
        windows: ids.iter().map(|id| WindowId((*id).into())).collect(),
    }];
    state
}

/// Draws the popup and returns the pixel at `(x, y)`.
fn pixel(state: &State, metrics: &Metrics, x: f32, y: f32) -> [u8; 4] {
    let theme = Theme::default();
    let size = frames::measured(&state.view(), metrics, &theme);
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

    frames::draw_popup(&state.view(), metrics, &mut paint);

    let i = ((y as u32 * size.width + x as u32) * 4) as usize;

    [data[i], data[i + 1], data[i + 2], data[i + 3]]
}

#[test]
fn the_count_is_a_badge_on_the_icon() {
    let metrics = metrics();
    let grid = Grid::new(&metrics, &Theme::default());
    let one = state(&["w"]);
    let two = state(&["a", "b"]);
    let placed = layout(&two.view(), &metrics, &grid);
    let top = frames::icon_top(&grid, &metrics);
    let x = placed.x + placed.size / 2.0 - 3.5; // Badge, left of the digit
    let y = top + placed.size - 4.5;
    let badged = pixel(&two, &metrics, x, y);

    assert_eq!(strip_height(&two.view(), &metrics, &grid), placed.size);
    assert_eq!(badged, [0x30, 0x36, 0x3d, 0xff]); // Theme border
    assert_ne!(pixel(&one, &metrics, x, y), badged);
}

#[test]
fn the_badge_is_a_rounded_square() {
    let metrics = metrics();
    let grid = Grid::new(&metrics, &Theme::default());
    let two = state(&["a", "b"]);
    let placed = layout(&two.view(), &metrics, &grid);
    let top = frames::icon_top(&grid, &metrics);
    let h = (placed.size * 0.3 * 1.25).round(); // Badge side
    let corner = pixel(
        &two,
        &metrics,
        placed.x + (placed.size - h) / 2.0 + 1.0,
        top + placed.size - h + 1.0,
    );

    assert!(corner[0] > 0x20, "corner is cut like a circle: {corner:?}");
}
