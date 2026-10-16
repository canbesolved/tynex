// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! tynex: efficient task switcher.

/// Program entry point.
fn main() {
    tynex::logger::init();
    tynex::logger::install_panic_hook();
    println!("tynex {}", env!("CARGO_PKG_VERSION"));
}
