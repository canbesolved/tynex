// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Portable drawing: no OS types.
//!
//! UI modules are the input bar, the output bar, the taskbar and the
//! info bar. Each can be changed on its own. The input bar is always
//! on. The output bar shows the result of the input. The taskbar shows
//! running apps, open windows, or both. The info bar comes later.

pub mod draw;
pub mod fit;
pub mod frames;
pub mod input_bar;
pub mod output_bar;
pub mod painter;
pub mod render;
pub mod scroll;
pub mod slide;
pub mod taskbar;

pub use draw::Size;
