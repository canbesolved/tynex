// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Row colors: parsing and the color of each row type.

use super::*;
use crate::core::browser_tabs::{SourceId, TabRef};
use crate::core::window::WindowId;

fn row(target: RowTarget, title_color: Option<Rgb>) -> Row {
    Row {
        target,
        app_id: String::new(),
        app_name: String::new(),
        icon: None,
        title: String::new(),
        title_color,
        highlights: Vec::new(),
    }
}

fn tint(r: u8, alpha: u8) -> Tint {
    Tint {
        color: Rgb::new(r, 0, 0),
        alpha,
    }
}

#[test]
fn parses_opaque_and_translucent_colors() {
    assert_eq!(Tint::parse("#010000"), Some(tint(1, 0xff)));
    assert_eq!(Tint::parse("#01000040"), Some(tint(1, 0x40)));
    assert_eq!(Tint::parse(""), None);
    assert_eq!(Tint::parse("#0100"), None);
    assert_eq!(Tint::parse("#010000+f"), None);
    assert_eq!(Tint::parse("#01000é"), None);
}

#[test]
fn each_row_type_has_its_color() {
    let colors = RowColors {
        window: Some(tint(1, 0xff)),
        tab: Some(tint(2, 0xff)),
        app: Some(tint(3, 0xff)),
    };
    let window = row(RowTarget::Window(WindowId("w".into())), None);
    let tab = row(
        RowTarget::Tab(TabRef {
            source: SourceId(1),
            id: 7,
        }),
        None,
    );
    let app = row(RowTarget::Launch("a".into()), None);
    let new_window =
        row(RowTarget::Launch("a".into()), Some(Rgb::new(9, 9, 9)));

    assert_eq!(colors.of(&window), colors.window);
    assert_eq!(colors.of(&tab), colors.tab);
    assert_eq!(colors.of(&app), colors.app);
    assert_eq!(colors.of(&new_window), None);
    assert_eq!(RowColors::default().of(&window), None);
}
