// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon strip placement, spare room and hit testing.

use super::{layout, shows, spare_height, strip_height};
use crate::core::event::Click;
use crate::core::state::State;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics};
use crate::ui::icon_bar::hit;
use crate::ui::render::canvas::Rgba;
use crate::ui::render::glyphs::Face;
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::{frames, icon_menu};

const FONT: &[u8] = include_bytes!("../../tests/fonts/dejavu_sans.ttf");

fn metrics(width: f32) -> Metrics {
    Metrics {
        font_size: 14.0,
        width,
        max_rows: 3,
        scale: 1.0,
        title_bar: true,
        show_icons: false,
        hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
        icon_bar: true,
        icon_size: 24.0,
    }
}

fn icon(app: &str, windows: &[&str]) -> AppIcon {
    AppIcon {
        app_id: app.into(),
        icon: None,
        windows: windows.iter().map(|id| WindowId((*id).into())).collect(),
    }
}

fn row(id: &str, title: &str) -> Row {
    Row {
        target: RowTarget::Window(WindowId(id.into())),
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: title.into(),
        title_color: None,
        highlights: Vec::new(),
    }
}

fn state(icons: Vec<AppIcon>, titles: &[(&str, &str)]) -> State {
    let mut state =
        State::new(titles.iter().map(|(id, title)| row(id, title)).collect());

    state.icons = icons;
    state
}

#[test]
fn a_short_row_is_centered() {
    let metrics = metrics(300.0);
    let grid = Grid::new(&metrics, &Theme::default());
    let state = state(vec![icon("kate", &["w"])], &[("w", "File")]);
    let placed = layout(&state.view(), &metrics, &grid);
    let inner = 300.0 - 2.0 * grid.pad;

    assert!((placed.x - (grid.pad + (inner - placed.size) / 2.0)).abs() < 0.1);
}

#[test]
fn a_wide_row_shrinks_and_stays_clickable() {
    let metrics = metrics(200.0);
    let grid = Grid::new(&metrics, &Theme::default());
    let icons = (0..12).map(|i| icon(&format!("app{i}"), &["w"])).collect();
    let state = state(icons, &[("w", "File")]);
    let placed = layout(&state.view(), &metrics, &grid);

    assert!(placed.size < 24.0);

    let y = grid.pad + grid.row_h + placed.size / 2.0;
    let hit = hit(
        &state.view(),
        &metrics,
        &grid,
        &Text::new(Face::new(FONT, 0).unwrap()),
        placed.x + placed.size / 2.0,
        y,
        1.0,
    );

    assert_eq!(hit, Click::Icon(0));
}

#[test]
fn spare_height_stays_when_the_dropdown_opens() {
    let metrics = metrics(300.0);
    let theme = Theme::default();
    let grid = Grid::new(&metrics, &theme);
    let mut state = state(
        vec![icon("firefox", &["a", "b"])],
        &[("a", "Mail"), ("b", "Notes")],
    );
    let closed = frames::measured(&state.view(), &metrics, &theme);

    state.open_dropdown();

    let open = frames::measured(&state.view(), &metrics, &theme);

    assert_eq!(closed, open);
    assert!(spare_height(&state.view(), &metrics, &grid) > 0.0);
}

#[test]
fn a_query_hides_the_strip() {
    let metrics = metrics(300.0);
    let grid = Grid::new(&metrics, &Theme::default());
    let mut state = state(vec![icon("kate", &["w"])], &[("w", "File")]);

    assert!(shows(&state.view(), &metrics));
    state.insert('a');
    assert!(!shows(&state.view(), &metrics));
    assert_eq!(strip_height(&state.view(), &metrics, &grid), 0.0);
    assert_eq!(spare_height(&state.view(), &metrics, &grid), 0.0);
}

#[test]
fn a_long_title_is_capped() {
    let metrics = metrics(480.0);
    let grid = Grid::new(&metrics, &Theme::default());
    let mut state = state(
        vec![icon("firefox", &["a", "b"])],
        &[("a", "Mail"), ("b", &"Notes ".repeat(80))],
    );
    let text = Text::new(Face::new(FONT, 0).unwrap());

    state.open_dropdown();

    let panel =
        icon_menu::panel(&state.view(), &metrics, &grid, &text).unwrap();

    assert!(panel.w <= 450.0 + 2.0 * grid.gap + 0.5);
    assert!(
        panel.w < icon_bar_width(&text, &state.view().window_rows()[1].title)
    );
}

/// Returns the uncapped title width plus the row padding.
fn icon_bar_width(text: &Text, title: &str) -> f32 {
    text.width(title, 14.0) + 2.0 * 8.0
}
