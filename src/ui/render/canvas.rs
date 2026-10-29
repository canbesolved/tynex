// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Canvas over a borrowed RGBA pixel buffer (premultiplied alpha).

use tiny_skia::{
    Color, FillRule, Paint, Path, PathBuilder, Pixmap, PixmapMut, PixmapPaint,
    PremultipliedColorU8, Rect, Stroke, Transform,
};

/// Color with straight (not premultiplied) alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

/// Rectangle in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Area {
    #[must_use]
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

/// Drawing surface over a presenter's buffer, row by row, 4 bytes each.
pub struct Canvas<'a> {
    pixmap: PixmapMut<'a>,
}

impl<'a> Canvas<'a> {
    /// Wraps `data`; returns `None` if its size does not match.
    #[must_use]
    pub fn new(data: &'a mut [u8], width: u32, height: u32) -> Option<Self> {
        PixmapMut::from_bytes(data, width, height).map(|pixmap| Self { pixmap })
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.pixmap.width()
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.pixmap.height()
    }

    /// Fills the whole canvas.
    pub fn clear(&mut self, color: Rgba) {
        self.pixmap.fill(to_color(color));
    }

    /// Fills `area`; does nothing for an empty area.
    pub fn fill_rect(&mut self, area: Area, color: Rgba) {
        if let Some(rect) = Rect::from_xywh(area.x, area.y, area.w, area.h) {
            self.pixmap.fill_rect(
                rect,
                &paint(color),
                Transform::identity(),
                None,
            );
        }
    }

    /// Fills `area` with corners of `radius`.
    pub fn fill_rounded_rect(&mut self, area: Area, radius: f32, color: Rgba) {
        if let Some(path) = rounded(area, radius) {
            self.pixmap.fill_path(
                &path,
                &paint(color),
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }

    /// Strokes `area` with a line of `width` and corners of `radius`.
    pub fn stroke_rounded_rect(
        &mut self,
        area: Area,
        radius: f32,
        width: f32,
        color: Rgba,
    ) {
        let stroke = Stroke {
            width,
            ..Stroke::default()
        };

        if let Some(path) = rounded(area, radius) {
            self.pixmap.stroke_path(
                &path,
                &paint(color),
                &stroke,
                Transform::identity(),
                None,
            );
        }
    }

    /// Draws `image` with its top-left at `(x, y)`, over the canvas.
    pub fn draw_image(&mut self, image: &Pixmap, (x, y): (i32, i32)) {
        self.pixmap.draw_pixmap(
            x,
            y,
            image.as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }

    /// Blends `color` through a coverage mask whose top-left is `(x, y)`.
    ///
    /// `coverage` holds `width` bytes per row, 0 to 255 each.
    pub fn blend_coverage(
        &mut self,
        (x, y): (i32, i32),
        width: usize,
        coverage: &[u8],
        color: Rgba,
    ) {
        let (cw, ch) = (self.width() as i32, self.height() as i32);
        let pixels = self.pixmap.pixels_mut();

        for (i, &cov) in coverage.iter().enumerate() {
            let (px, py) = (x + (i % width) as i32, y + (i / width) as i32);

            if cov == 0 || px < 0 || py < 0 || px >= cw || py >= ch {
                continue;
            }

            let dst = &mut pixels[(py * cw + px) as usize];

            *dst = over(*dst, color, cov);
        }
    }
}

fn to_color(c: Rgba) -> Color {
    Color::from_rgba8(c.0, c.1, c.2, c.3)
}

fn paint(color: Rgba) -> Paint<'static> {
    let mut paint = Paint::default();

    paint.set_color(to_color(color));
    paint.anti_alias = true;
    paint
}

/// Builds a rounded rectangle; corners are cubic circle approximations.
fn rounded(a: Area, radius: f32) -> Option<Path> {
    let r = radius.min(a.w / 2.0).min(a.h / 2.0).max(0.0);
    let k = r * 0.552_284_8; // Cubic handle length for a quarter circle
    let (x0, y0, x1, y1) = (a.x, a.y, a.x + a.w, a.y + a.h);
    let mut pb = PathBuilder::new();

    pb.move_to(x0 + r, y0);
    pb.line_to(x1 - r, y0);
    pb.cubic_to(x1 - r + k, y0, x1, y0 + r - k, x1, y0 + r);
    pb.line_to(x1, y1 - r);
    pb.cubic_to(x1, y1 - r + k, x1 - r + k, y1, x1 - r, y1);
    pb.line_to(x0 + r, y1);
    pb.cubic_to(x0 + r - k, y1, x0, y1 - r + k, x0, y1 - r);
    pb.line_to(x0, y0 + r);
    pb.cubic_to(x0, y0 + r - k, x0 + r - k, y0, x0 + r, y0);
    pb.close();
    pb.finish()
}

/// Source-over blend of `color` at `coverage` onto premultiplied `dst`.
fn over(
    dst: PremultipliedColorU8,
    color: Rgba,
    coverage: u8,
) -> PremultipliedColorU8 {
    let a = u32::from(color.3) * u32::from(coverage) / 255;
    let mix = |src: u8, d: u8| {
        (u32::from(src) * a / 255 + u32::from(d) * (255 - a) / 255) as u8
    };

    PremultipliedColorU8::from_rgba(
        mix(color.0, dst.red()),
        mix(color.1, dst.green()),
        mix(color.2, dst.blue()),
        (a + u32::from(dst.alpha()) * (255 - a) / 255) as u8,
    )
    .unwrap_or(dst)
}

#[cfg(test)]
#[path = "canvas_tests.rs"]
mod tests;
