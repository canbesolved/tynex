// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Icon decoding and cache tests, with images made in the test.

use std::cell::Cell;
use std::rc::Rc;

use tiny_skia::Color;

use super::*;

const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg"
    width="16" height="8"><rect width="16" height="8" fill="red"/></svg>"#;

/// Lookup that returns `path` and counts its calls.
struct Counting {
    path: PathBuf,
    calls: Rc<Cell<u32>>,
}

impl IconLookup for Counting {
    fn find(&mut self, _icon: &str, _size: u32) -> Option<PathBuf> {
        self.calls.set(self.calls.get() + 1);
        Some(self.path.clone())
    }
}

/// A fresh dir under the system temp dir.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("tynex_icons_{name}_{}", std::process::id()));

    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Alpha of pixel `(x, y)`.
fn alpha(image: &Pixmap, x: u32, y: u32) -> u8 {
    image.pixel(x, y).unwrap().alpha()
}

#[test]
fn svg_is_fitted_and_centered() {
    let image = render_svg(SVG.as_bytes(), 32).unwrap();

    assert_eq!((image.width(), image.height()), (32, 32));
    assert_eq!(alpha(&image, 16, 16), 255); // Inside the 32 x 16 band
    assert_eq!(alpha(&image, 16, 2), 0); // Above it
}

#[test]
fn png_is_scaled_to_the_row_size() {
    let mut source = Pixmap::new(4, 4).unwrap();

    source.fill(Color::from_rgba8(0, 0, 255, 255));

    let image = fit_png(&source, 12).unwrap();

    assert_eq!((image.width(), image.height()), (12, 12));
    assert_eq!(alpha(&image, 6, 6), 255);
}

#[test]
fn files_load_by_extension() {
    let dir = scratch("load");
    let (png, svg, xpm) =
        (dir.join("a.png"), dir.join("a.svg"), dir.join("a.xpm"));
    let mut source = Pixmap::new(2, 2).unwrap();

    source.fill(Color::WHITE);
    fs::write(&png, source.encode_png().unwrap()).unwrap();
    fs::write(&svg, SVG).unwrap();
    fs::write(&xpm, "/* XPM */").unwrap();

    assert!(load(&png, 8).is_some());
    assert!(load(&svg, 8).is_some());
    assert!(load(&xpm, 8).is_none());
    assert!(load(&dir.join("missing.png"), 8).is_none());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn images_and_misses_are_cached() {
    let dir = scratch("cache");
    let calls = Rc::new(Cell::new(0));
    let path = dir.join("app.svg");
    let mut icons = Icons::new(Box::new(Counting {
        path: path.clone(),
        calls: calls.clone(),
    }));

    icons.prepare("app", 16); // Miss: no file yet
    fs::write(&path, SVG).unwrap();
    icons.prepare("app", 16);
    assert!(icons.image("app", 16).is_none());
    assert!(icons.image("app", 24).is_some()); // New size, new lookup
    assert_eq!(calls.get(), 2);
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn absolute_paths_skip_the_lookup() {
    let dir = scratch("absolute");
    let path = dir.join("app.svg");
    let mut icons = Icons::new(Box::new(NoIcons));

    fs::write(&path, SVG).unwrap();
    assert!(icons.image(path.to_str().unwrap(), 16).is_some());
    assert!(icons.image("app", 16).is_none());
    fs::remove_dir_all(dir).unwrap();
}
