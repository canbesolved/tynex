// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Window count badge on an icon of the strip.

use super::icons::Layout;
use crate::ui::draw::Paint;
use crate::ui::render::canvas::Area;
use crate::ui::render::text::Style;

/// Window count font size, as a fraction of the icon side.
const BADGE_RATIO: f32 = 0.3;

/// Draws the window count in a rounded square at the icon's bottom.
pub(super) fn draw(
    paint: &mut Paint<'_, '_>,
    placed: &Layout,
    x: f32,
    top: f32,
    n: usize,
) {
    let label = n.to_string();
    let size = (placed.size * BADGE_RATIO).max(1.0);
    let width = paint.text.width(&label, size);
    let h = (size * 1.25).min(placed.size);
    let w = (width + h * 0.4).clamp(h, placed.size);
    let back =
        Area::new(x + (placed.size - w) / 2.0, top + placed.size - h, w, h);
    let style = Style {
        size,
        color: paint.theme.text,
        highlight: paint.theme.highlight,
        highlights: &[],
    };
    let (ascent, descent) = paint.text.metrics(size);
    let base = (back.y + h / 2.0 + f32::midpoint(ascent, descent)).round();
    let left = x + (placed.size - width) / 2.0;

    paint
        .canvas
        .fill_rounded_rect(back, h / 4.0, paint.theme.border);
    paint
        .text
        .draw(paint.canvas, &label, (left, base), width.max(1.0), &style);
}
