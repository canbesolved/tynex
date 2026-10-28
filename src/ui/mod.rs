// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Portable drawing: no OS types.
//!
//! UI modules are the input bar, the output bar, the title bar, the
//! info bar and the icon bar. Each can be changed on its own. The
//! input bar is always on. The output bar shows the result of the
//! input. The info bar and the icon bar come later.

pub mod draw;
pub mod input_bar;
pub mod output_bar;
pub mod painter;
pub mod render;
pub mod title_bar;

pub use draw::Size;
