// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Favicons sent by a browser, decoded once and dropped with the source.

use std::collections::HashMap;

use tiny_skia::{FilterQuality, Pixmap, PixmapPaint, Transform};

use crate::core::browser_tabs::SourceId;
use crate::ui::render::canvas::Canvas;

/// Largest PNG accepted from the extension.
const MAX_BYTES: usize = 16 * 1024;

/// Largest side, in pixels, before the icon is rejected.
const MAX_SIDE: u32 = 64;

/// Icons kept per browser connection.
const MAX_ICONS: usize = 512;

/// Decoded favicons, per source, by icon key.
#[derive(Debug, Default)]
pub struct TabIcons {
    sources: HashMap<SourceId, HashMap<String, Pixmap>>,
}

impl TabIcons {
    /// Decodes `png` and keeps it. A bad, large or over-cap image is
    /// dropped, and the row then uses the browser icon.
    pub fn insert(&mut self, source: SourceId, key: &str, png: &[u8]) -> bool {
        if png.len() > MAX_BYTES || key.is_empty() {
            return false;
        }
        let icons = self.sources.entry(source).or_default();

        if icons.contains_key(key) {
            return true;
        }
        if icons.len() >= MAX_ICONS {
            return false;
        }
        let Some(image) = decode(png) else {
            return false;
        };

        icons.insert(key.to_owned(), image);
        true
    }

    /// Draws the favicon as a `size` square, if this source has `key`.
    pub fn draw(
        &self,
        canvas: &mut Canvas<'_>,
        source: SourceId,
        key: &str,
        size: u32,
        (x, y): (i32, i32),
    ) -> bool {
        let Some(fitted) = self
            .sources
            .get(&source)
            .and_then(|icons| icons.get(key))
            .and_then(|image| fit(image, size))
        else {
            return false;
        };

        canvas.draw_image(&fitted, (x, y));
        true
    }

    /// Drops every icon of `source`.
    pub fn drop_source(&mut self, source: SourceId) {
        self.sources.remove(&source);
    }

    /// How many icons `source` has stored.
    #[must_use]
    pub fn count(&self, source: SourceId) -> usize {
        self.sources.get(&source).map_or(0, HashMap::len)
    }
}

/// Rejects a PNG that is too wide or that does not decode.
fn decode(png: &[u8]) -> Option<Pixmap> {
    let image = Pixmap::decode_png(png).ok()?;

    if image.width() > MAX_SIDE || image.height() > MAX_SIDE {
        return None;
    }
    Some(image)
}

/// Scales `image` into a `size` square, centered.
fn fit(image: &Pixmap, size: u32) -> Option<Pixmap> {
    let mut out = Pixmap::new(size, size)?;
    let edge = size as f32;
    let w = image.width() as f32;
    let h = image.height() as f32;
    let scale = edge / w.max(h).max(1.0);
    let transform = Transform::from_scale(scale, scale)
        .post_translate((edge - w * scale) / 2.0, (edge - h * scale) / 2.0);
    let paint = PixmapPaint {
        quality: FilterQuality::Bicubic,
        ..PixmapPaint::default()
    };

    out.draw_pixmap(0, 0, image.as_ref(), &paint, transform, None);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::TabIcons;
    use crate::core::browser_tabs::SourceId;

    fn png(w: u32, h: u32) -> Vec<u8> {
        tiny_skia::Pixmap::new(w, h).unwrap().encode_png().unwrap()
    }

    #[test]
    fn keeps_a_small_png_and_rejects_the_rest() {
        let mut icons = TabIcons::default();
        let source = SourceId(1);

        assert!(icons.insert(source, "k", &png(16, 16)));
        assert!(!icons.insert(source, "wide", &png(65, 1)));
        assert!(!icons.insert(source, "bad", b"not a png"));
        assert!(!icons.insert(source, "big", &vec![0; 16 * 1024 + 1]));
        assert_eq!(icons.count(source), 1);
        icons.drop_source(source);
        assert_eq!(icons.count(source), 0);
    }

    #[test]
    fn the_cap_is_512() {
        let mut icons = TabIcons::default();
        let source = SourceId(2);
        let bytes = png(1, 1);

        for i in 0..512 {
            assert!(icons.insert(source, &i.to_string(), &bytes));
        }
        assert!(!icons.insert(source, "more", &bytes));
        assert_eq!(icons.count(source), 512);
    }
}
