// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Output bar row colors: a band behind each row of a colored type.

use super::tests::{M, pixel, render_themed, row_center, state};
use crate::core::row_color::Tint;
use crate::core::view::Rgb;
use crate::ui::render::theme::Theme;

/// Pixel near the right end of output row `n`, past its title.
fn row_pixel(theme: &Theme, n: usize) -> [u8; 4] {
    let mut typed = state(2);

    typed.query = "w".into();
    typed.output = typed.rows.clone();

    let (data, size) = render_themed(&typed, &M, theme);

    pixel(&data, size, size.width as f32 - 20.0, row_center(n))
}

#[test]
fn window_rows_get_their_band() {
    let mut theme = Theme::default();
    let plain = row_pixel(&theme, 1);

    theme.rows.window = Some(Tint {
        color: Rgb::new(0xff, 0, 0),
        alpha: 0xff,
    });
    assert_eq!(row_pixel(&theme, 1), [0xff, 0, 0, 0xff]);
    assert_ne!(plain, [0xff, 0, 0, 0xff]);

    theme.rows.window = None;
    theme.rows.tab = Some(Tint {
        color: Rgb::new(0xff, 0, 0),
        alpha: 0xff,
    });
    assert_eq!(row_pixel(&theme, 1), plain); // Not a tab row
}
