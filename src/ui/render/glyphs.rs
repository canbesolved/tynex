// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Font faces and glyph rasterizing with `skrifa` and `tiny-skia`.

use skrifa::instance::{LocationRef, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::{FontRef, GlyphId, MetadataProvider};
use tiny_skia::{FillRule, Mask, PathBuilder, Transform};

/// A font face. Font data lives as long as the program, for example a
/// memory-mapped file that is never unmapped.
#[derive(Clone)]
pub struct Face(FontRef<'static>);

impl Face {
    /// Parses face `index` of a font file or collection (`.ttc`).
    #[must_use]
    pub fn new(data: &'static [u8], index: u32) -> Option<Self> {
        FontRef::from_index(data, index).ok().map(Self)
    }

    /// Returns the glyph for `c`, if the face has one.
    pub(super) fn glyph(&self, c: char) -> Option<GlyphId> {
        self.0.charmap().map(c)
    }

    /// Returns the advance width of `id` at `size` pixels.
    pub(super) fn advance(&self, id: GlyphId, size: f32) -> f32 {
        self.0
            .glyph_metrics(Size::new(size), LocationRef::default())
            .advance_width(id)
            .unwrap_or(0.0)
    }

    /// Returns ascent and descent (negative) at `size` pixels.
    pub(super) fn metrics(&self, size: f32) -> (f32, f32) {
        let m = self.0.metrics(Size::new(size), LocationRef::default());

        (m.ascent, m.descent)
    }
}

/// A rasterized glyph: offset from the pen position and coverage rows.
pub(super) struct Raster {
    pub left: i32,
    pub top: i32,
    pub width: usize,
    pub coverage: Vec<u8>,
}

/// Rasterizes glyph `id` at `size` pixels; `None` for empty glyphs.
pub(super) fn rasterize(face: &Face, id: GlyphId, size: f32) -> Option<Raster> {
    let glyph = face.0.outline_glyphs().get(id)?;
    let mut pen = Pen(PathBuilder::new());
    let settings =
        DrawSettings::unhinted(Size::new(size), LocationRef::default());

    glyph.draw(settings, &mut pen).ok()?;

    let path = pen.0.finish()?;
    let b = path.bounds();
    let (left, top) = (b.left().floor(), b.top().floor());
    let width = (b.right().ceil() - left) as u32;
    let height = (b.bottom().ceil() - top) as u32;
    let mut mask = Mask::new(width.max(1), height.max(1))?;

    mask.fill_path(
        &path,
        FillRule::Winding,
        true,
        Transform::from_translate(-left, -top),
    );
    Some(Raster {
        left: left as i32,
        top: top as i32,
        width: mask.width() as usize,
        coverage: mask.data().to_vec(),
    })
}

/// Collects an outline into a path, flipping y to point down.
struct Pen(PathBuilder);

impl OutlinePen for Pen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, -y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, -y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.0.quad_to(cx0, -cy0, x, -y);
    }

    fn curve_to(
        &mut self,
        cx0: f32,
        cy0: f32,
        cx1: f32,
        cy1: f32,
        x: f32,
        y: f32,
    ) {
        self.0.cubic_to(cx0, -cy0, cx1, -cy1, x, -y);
    }

    fn close(&mut self) {
        self.0.close();
    }
}
