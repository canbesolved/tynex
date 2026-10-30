// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon strip placement, spare room and hit testing.

use super::{hit, layout, shows, spare_height, strip_height};
use crate::config::TaskbarMode;
use crate::core::event::Click;
use crate::core::state::State;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::ui::draw::{Grid, Metrics, Paint};
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::glyphs::Face;
use crate::ui::render::icons::{Icons, NoIcons};
use crate::ui::render::text::Text;
use crate::ui::render::theme::Theme;
use crate::ui::{frames, taskbar::menu as icon_menu};

const FONT: &[u8] = include_bytes!("../../../tests/fonts/dejavu_sans.ttf");

fn metrics(width: f32) -> Metrics {
    Metrics {
        font_size: 14.0,
        width,
        max_rows: 3,
        list_rows: 3,
        first_row: None,
        scale: 1.0,
        taskbar: TaskbarMode::Both,
        show_icons: false,
        hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
        icon_size: 24.0,
        border: 4.0,
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

    let y = frames::icon_top(&grid, &metrics) + placed.size / 2.0;
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
    let metrics = Metrics {
        taskbar: TaskbarMode::Iconbar,
        ..metrics(300.0)
    };
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
    let under = frames::icon_bottom(&state.view(), &metrics, &grid);

    assert!((panel.y - under - 3.0).abs() < 0.1);
    assert!(
        panel.w < icon_bar_width(&text, &state.view().window_rows()[1].title)
    );
}

/// Returns the uncapped title width plus the row padding.
fn icon_bar_width(text: &Text, title: &str) -> f32 {
    text.width(title, 14.0) + 2.0 * 8.0
}

fn pixel(data: &[u8], width: u32, x: f32, y: f32) -> [u8; 4] {
    let i = ((y as u32 * width + x as u32) * 4) as usize;
    [data[i], data[i + 1], data[i + 2], data[i + 3]]
}

fn drawn(state: &State, metrics: &Metrics) -> (Vec<u8>, u32) {
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
    (data, size.width)
}

#[test]
fn selection_is_a_border_around_the_icon() {
    let metrics = metrics(300.0);
    let grid = Grid::new(&metrics, &Theme::default());
    let state = state(vec![icon("kate", &["w"])], &[("w", "File")]);
    let placed = layout(&state.view(), &metrics, &grid);
    let top = frames::icon_top(&grid, &metrics);
    let (data, width) = drawn(&state, &metrics);
    let mid_y = top + placed.size / 2.0;
    let center = pixel(&data, width, placed.x + placed.size / 2.0, mid_y);
    let edge = pixel(&data, width, placed.x - 2.0, mid_y);

    assert!(edge[2] > center[2], "edge {edge:?} center {center:?}");
    assert!(center[2] < 40, "center was filled: {center:?}");
}

#[test]
fn a_clear_gap_separates_the_icon_frame() {
    let metrics = metrics(300.0);
    let grid = Grid::new(&metrics, &Theme::default());
    let state = state(vec![icon("kate", &["w"])], &[("w", "File")]);
    let (data, width) = drawn(&state, &metrics);
    let mid = width as f32 / 2.0;
    let frame = 2.0 * grid.pad + grid.row_h;
    let gap = pixel(&data, width, mid, frame + 2.0);
    let body = pixel(&data, width, mid, frame + 8.0);
    let under = frames::icon_bottom(&state.view(), &metrics, &grid);
    let split = pixel(&data, width, mid, under + 2.0);

    assert_eq!(gap[3], 0, "{gap:?}");
    assert!(body[3] > 0, "{body:?}");
    assert_eq!(split[3], 0, "{split:?}");
}

#[test]
fn a_clear_gap_separates_the_lower_block() {
    let theme = Theme::default();
    let border = theme.border;
    let open = state(vec![], &[("w", "File")]);
    let mut typed = open.clone();

    typed.query = "f".into();

    for (mode, state) in
        [(TaskbarMode::Titlebar, &open), (TaskbarMode::Off, &typed)]
    {
        let metrics = Metrics {
            taskbar: mode,
            ..metrics(300.0)
        };
        let grid = Grid::new(&metrics, &theme);
        let (data, width) = drawn(state, &metrics);
        let mid = width as f32 / 2.0;
        let y = 2.0 * grid.pad + grid.row_h;

        assert_eq!(pixel(&data, width, mid, y + 1.0)[3], 0);
        assert_eq!(
            pixel(&data, width, mid, y + 3.0),
            [border.0, border.1, border.2, border.3]
        );
    }
}
