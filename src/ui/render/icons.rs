// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! App icons: file lookup through the platform, PNG and SVG decoding,
//! and a cache of images at row size.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use resvg::usvg;
use tiny_skia::{FilterQuality, Pixmap, PixmapPaint, Transform};

use crate::ui::render::canvas::Canvas;

/// Finds the file of an icon name, for example in an icon theme.
pub trait IconLookup {
    /// Returns the file of `icon` closest to `size` pixels.
    fn find(&mut self, icon: &str, size: u32) -> Option<PathBuf>;
}

/// Lookup that finds nothing: only absolute icon paths load.
#[derive(Debug, Default)]
pub struct NoIcons;

impl IconLookup for NoIcons {
    fn find(&mut self, _icon: &str, _size: u32) -> Option<PathBuf> {
        None
    }
}

/// Icon images by name and size; misses are cached too.
pub struct Icons {
    lookup: Box<dyn IconLookup>,
    images: HashMap<(String, u32), Option<Pixmap>>,
}

impl Icons {
    #[must_use]
    pub fn new(lookup: Box<dyn IconLookup>) -> Self {
        Self {
            lookup,
            images: HashMap::new(),
        }
    }

    /// Loads `icon` at `size` pixels, so drawing never reads files.
    pub fn prepare(&mut self, icon: &str, size: u32) {
        self.image(icon, size);
    }

    /// Draws `icon` as a `size` pixel square at `(x, y)`; draws nothing
    /// when it cannot be found or decoded.
    pub fn draw(
        &mut self,
        canvas: &mut Canvas<'_>,
        icon: &str,
        size: u32,
        (x, y): (i32, i32),
    ) {
        if let Some(image) = self.image(icon, size) {
            canvas.draw_image(image, (x, y));
        }
    }

    fn image(&mut self, icon: &str, size: u32) -> Option<&Pixmap> {
        let key = (icon.to_owned(), size);

        if !self.images.contains_key(&key) {
            let path = if Path::new(icon).is_absolute() {
                Some(PathBuf::from(icon))
            } else {
                self.lookup.find(icon, size)
            };
            let image = path.as_deref().and_then(|p| load(p, size));

            if image.is_none() {
                log::debug!("no icon {icon:?} at {size} px");
            }
            self.images.insert(key.clone(), image);
        }
        self.images[&key].as_ref()
    }
}

/// Decodes a PNG or SVG file into a `size` pixel square.
fn load(path: &Path, size: u32) -> Option<Pixmap> {
    let data = fs::read(path).ok()?;

    match path.extension()?.to_str()? {
        "png" => fit_png(&Pixmap::decode_png(&data).ok()?, size),
        "svg" => render_svg(&data, size),
        _ => None, // XPM and others are not supported
    }
}

/// Scales `image` into a `size` square, centered, keeping its aspect.
fn fit_png(image: &Pixmap, size: u32) -> Option<Pixmap> {
    let mut out = Pixmap::new(size, size)?;
    let transform = fit(image.width() as f32, image.height() as f32, size);
    let paint = PixmapPaint {
        quality: FilterQuality::Bicubic,
        ..PixmapPaint::default()
    };

    out.draw_pixmap(0, 0, image.as_ref(), &paint, transform, None);
    Some(out)
}

/// Renders an SVG into a `size` square, centered, keeping its aspect.
fn render_svg(data: &[u8], size: u32) -> Option<Pixmap> {
    let tree = usvg::Tree::from_data(data, &usvg::Options::default()).ok()?;
    let mut out = Pixmap::new(size, size)?;
    let transform = fit(tree.size().width(), tree.size().height(), size);

    resvg::render(&tree, transform, &mut out.as_mut());
    Some(out)
}

/// Returns the transform that fits a `w` x `h` image in a `size` square.
fn fit(w: f32, h: f32, size: u32) -> Transform {
    let square = size as f32;
    let scale = square / w.max(h).max(1.0);

    Transform::from_scale(scale, scale)
        .post_translate((square - w * scale) / 2.0, (square - h * scale) / 2.0)
}

#[cfg(test)]
#[path = "icons_tests.rs"]
mod tests;
