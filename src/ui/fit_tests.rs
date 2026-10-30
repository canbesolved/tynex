// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The popup fits the screen: lists shrink first, then the dropdown room.

use super::fit;
use crate::config::TaskbarMode;
use crate::core::state::State;
use crate::core::view::{AppIcon, Row, RowTarget};
use crate::core::window::WindowId;
use crate::ui::draw::Metrics;
use crate::ui::frames::measured;
use crate::ui::render::canvas::Rgba;
use crate::ui::render::theme::Theme;

/// Metrics like a 1536x864 screen at 1.25 with large icons.
fn metrics() -> Metrics {
    Metrics {
        font_size: 14.0,
        width: 480.0,
        max_rows: 19,
        list_rows: 19,
        first_row: None,
        scale: 1.25,
        taskbar: TaskbarMode::Both,
        show_icons: true,
        hint_color: Rgba(0x8d, 0xa7, 0x87, 0xff),
        icon_size: 46.0,
        border: 4.0,
    }
}

/// `n` windows; the first `same` belong to one app.
fn state(n: usize, same: usize) -> State {
    let ids: Vec<_> = (0..n).map(|i| WindowId(format!("w{i}"))).collect();
    let rows = ids.iter().map(|id| Row {
        target: RowTarget::Window(id.clone()),
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: id.0.clone(),
        title_color: None,
        highlights: Vec::new(),
    });
    let mut state = State::new(rows.collect());

    state.icons = vec![AppIcon {
        app_id: "konsole".into(),
        icon: None,
        windows: ids[..same].to_vec(),
    }];
    state
}

#[test]
fn many_windows_shrink_the_title_list_to_the_screen() {
    let theme = Theme::default();
    let state = state(31, 12);
    let full = metrics();
    let screen = 1080;

    assert!(measured(&state.view(), &full, &theme).height > screen);

    let fitted = fit(&state.view(), full, &theme, Some(screen));
    let height = measured(&state.view(), &fitted, &theme).height;

    assert!(height <= screen - 60, "{height}");
    assert!(fitted.list_rows < 19);
    assert_eq!(fitted.max_rows, 19); // The dropdown room is kept
}

#[test]
fn a_popup_that_fits_is_unchanged() {
    let theme = Theme::default();
    let state = state(3, 2);

    assert_eq!(fit(&state.view(), metrics(), &theme, Some(1080)), metrics());
    assert_eq!(fit(&state.view(), metrics(), &theme, None), metrics());
}

#[test]
fn a_tiny_screen_shrinks_the_dropdown_room_too() {
    let theme = Theme::default();
    let state = state(31, 19);
    let fitted = fit(&state.view(), metrics(), &theme, Some(600));

    assert_eq!(fitted.list_rows, 1);
    assert!(fitted.max_rows < 19);
}
