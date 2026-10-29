// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Canvas fills, strokes and coverage blending.

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
fn stroke_leaves_the_center_clear() {
    let mut data = vec![0; 20 * 20 * 4];
    let mut canvas = Canvas::new(&mut data, 20, 20).unwrap();

    canvas.clear(BG);
    canvas.stroke_rounded_rect(
        Area {
            x: 4.0,
            y: 4.0,
            w: 12.0,
            h: 12.0,
        },
        2.0,
        2.0,
        FG,
    );
    assert_eq!(pixel(&data, 20, 10, 10), [0, 0, 0, 255]);
    assert_ne!(pixel(&data, 20, 4, 10), [0, 0, 0, 255]);
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
