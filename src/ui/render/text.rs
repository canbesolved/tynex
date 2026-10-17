// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Text in one font: measuring, truncation and glyph drawing.

use std::collections::HashMap;
use std::ops::Range;

use skrifa::GlyphId;

use crate::core::text::cut_points;
use crate::ui::render::canvas::{Canvas, Rgba};
use crate::ui::render::glyphs::{Face, Raster, rasterize};

const ELLIPSIS: &str = "…";

/// How to draw one line of text.
#[derive(Debug, Clone, Copy)]
pub struct Style<'a> {
    pub size: f32,
    pub color: Rgba,
    pub highlight: Rgba,
    pub highlights: &'a [Range<usize>], // Byte ranges drawn in `highlight`
}

/// One glyph placed on a line.
struct Placed {
    id: GlyphId,
    x: f32,   // Pen position before the glyph
    end: f32, // Pen position after it, without the next kerning
    byte: usize,
}

/// Text renderer with a glyph cache. Characters the font lacks are drawn
/// as its missing-glyph box.
pub struct Text {
    face: Face,
    masks: HashMap<(GlyphId, u32), Option<Raster>>, // By glyph and size
}

impl Text {
    #[must_use]
    pub fn new(face: Face) -> Self {
        Self {
            face,
            masks: HashMap::new(),
        }
    }

    /// Returns the font's ascent and descent at `size` pixels.
    #[must_use]
    pub fn metrics(&self, size: f32) -> (f32, f32) {
        self.face.metrics(size)
    }

    /// Returns the advance width of `text` at `size` pixels.
    #[must_use]
    pub fn width(&self, text: &str, size: f32) -> f32 {
        self.layout(text, size).1
    }

    /// Returns where to cut `text` so it and an ellipsis fit `max`.
    ///
    /// `None` means the whole text fits. Cuts are safe cut points.
    #[must_use]
    pub fn fit(&self, text: &str, size: f32, max: f32) -> Option<usize> {
        let (placed, total) = self.layout(text, size);

        if total <= max {
            return None;
        }

        let room = max - self.width(ELLIPSIS, size);
        let width_to = |cut: usize| {
            let kept = placed.iter().take_while(|p| p.byte < cut);

            kept.last().map_or(0.0, |p| p.end)
        };

        Some(
            cut_points(text)
                .take_while(|&c| width_to(c) <= room)
                .last()
                .unwrap_or(0),
        )
    }

    /// Draws `text` with its baseline at `(x, y)`, truncated to `max`.
    pub fn draw(
        &mut self,
        canvas: &mut Canvas<'_>,
        text: &str,
        (x, y): (f32, f32),
        max: f32,
        style: &Style<'_>,
    ) {
        let end = self.fit(text, style.size, max);
        let shown = &text[..end.unwrap_or(text.len())];
        let (placed, width) = self.layout(shown, style.size);

        for p in &placed {
            let lit = style.highlights.iter().any(|r| r.contains(&p.byte));
            let color = if lit { style.highlight } else { style.color };

            self.blit(canvas, p, (x, y), style.size, color);
        }

        if end.is_some() {
            for p in &self.layout(ELLIPSIS, style.size).0 {
                self.blit(canvas, p, (x + width, y), style.size, style.color);
            }
        }
    }

    /// Places each glyph and returns the total advance.
    fn layout(&self, text: &str, size: f32) -> (Vec<Placed>, f32) {
        let mut placed = Vec::with_capacity(text.len());
        let mut x = 0.0;

        // No kerning and no shaping: glyphs follow their advances.
        for (byte, c) in text.char_indices() {
            let id = self.face.glyph(c).unwrap_or(GlyphId::NOTDEF);
            let end = x + self.face.advance(id, size);

            placed.push(Placed { id, x, end, byte });
            x = end;
        }
        (placed, x)
    }

    /// Draws one placed glyph through its cached raster.
    fn blit(
        &mut self,
        canvas: &mut Canvas<'_>,
        p: &Placed,
        (x, y): (f32, f32),
        size: f32,
        color: Rgba,
    ) {
        let face = &self.face;
        let mask = self
            .masks
            .entry((p.id, size.to_bits()))
            .or_insert_with(|| rasterize(face, p.id, size));

        if let Some(m) = mask {
            let origin = ((x + p.x).round() as i32, y.round() as i32);
            let at = (origin.0 + m.left, origin.1 + m.top);

            canvas.blend_coverage(at, m.width, &m.coverage, color);
        }
    }
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
