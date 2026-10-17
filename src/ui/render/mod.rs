// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! CPU renderer: draws into a plain pixel buffer.

// Pixel math converts between float positions and integer pixels on
// purpose; values stay within the surface and glyph sizes.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

pub mod canvas;
pub mod glyphs;
pub mod icons;
pub mod text;
pub mod theme;
