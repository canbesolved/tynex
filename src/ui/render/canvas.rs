// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Canvas over a borrowed RGBA pixel buffer (premultiplied alpha).

use tiny_skia::{
    Color, FillRule, Paint, Path, PathBuilder, Pixmap, PixmapMut, PixmapPaint,
    PremultipliedColorU8, Rect, Transform,
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
mod tests {
    use super::*;

    const BG: Rgba = Rgba(0, 0, 0, 255);
    const FG: Rgba = Rgba(200, 100, 50, 255);

    fn pixel(data: &[u8], width: usize, x: usize, y: usize) -> [u8; 4] {
        let i = (y * width + x) * 4;

        [data[i], data[i + 1], data[i + 2], data[i + 3]]
    }

    #[test]
    fn rect_and_clear() {
        let mut data = vec![0; 4 * 4 * 4];
        let mut canvas = Canvas::new(&mut data, 4, 4).unwrap();

        canvas.clear(BG);
        canvas.fill_rect(
            Area {
                x: 1.0,
                y: 1.0,
                w: 2.0,
                h: 2.0,
            },
            FG,
        );
        assert_eq!(pixel(&data, 4, 0, 0), [0, 0, 0, 255]);
        assert_eq!(pixel(&data, 4, 1, 1), [200, 100, 50, 255]);
    }

    #[test]
    fn rounded_corners_stay_clear() {
        let mut data = vec![0; 20 * 20 * 4];
        let mut canvas = Canvas::new(&mut data, 20, 20).unwrap();
        let area = Area {
            x: 0.0,
            y: 0.0,
            w: 20.0,
            h: 20.0,
        };

        canvas.clear(BG);
        canvas.fill_rounded_rect(area, 8.0, FG);
        assert_eq!(pixel(&data, 20, 0, 0), [0, 0, 0, 255]);
        assert_eq!(pixel(&data, 20, 10, 10), [200, 100, 50, 255]);
    }

    #[test]
    fn coverage_blends_and_clips() {
        let mut data = vec![0; 2 * 2 * 4];
        let mut canvas = Canvas::new(&mut data, 2, 2).unwrap();

        canvas.clear(BG);
        canvas.blend_coverage((1, 0), 2, &[255, 255, 128, 0], FG);
        assert_eq!(pixel(&data, 2, 1, 0), [200, 100, 50, 255]);
        assert_eq!(pixel(&data, 2, 0, 1), [0, 0, 0, 255]); // Clipped at x = 2
        assert_eq!(pixel(&data, 2, 1, 1), [100, 50, 25, 255]);
    }

    #[test]
    fn wrong_buffer_size_is_rejected() {
        assert!(Canvas::new(&mut [0; 15], 2, 2).is_none());
    }
}
