// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `output.colors`: off by default, `#RRGGBB` or `#RRGGBBAA`.

use crate::config::parse;
use crate::core::row_color::{RowColors, Tint};
use crate::core::view::Rgb;

#[test]
fn colors_are_off_by_default() {
    let config = parse("", &|_| false).expect("empty config");

    assert_eq!(config.output.colors, RowColors::default());
}

#[test]
fn colors_parse_per_type() {
    let text = "[output]\ncolors.window = \"#a3d9a5\"\n\
                colors.tab = \"#f2e2a040\"\ncolors.app = \"\"\n";
    let colors = parse(text, &|_| false).expect("valid config").output.colors;

    assert_eq!(
        colors.window,
        Some(Tint {
            color: Rgb::new(0xa3, 0xd9, 0xa5),
            alpha: 0xff
        })
    );
    assert_eq!(colors.tab.map(|t| t.alpha), Some(0x40));
    assert_eq!(colors.app, None);
}

#[test]
fn bad_colors_and_keys_are_errors() {
    let msg = parse("[output]\ncolors.tab = \"yellow\"\n", &|_| false)
        .expect_err("not a color");

    assert!(
        msg[0].to_string().contains("`output.colors.tab`"),
        "{msg:?}"
    );

    let msg = parse("[output]\ncolors.bookmark = \"\"\n", &|_| false)
        .expect_err("unknown type");

    assert!(
        msg[0].to_string().contains("unknown key `bookmark`"),
        "{msg:?}"
    );
}
