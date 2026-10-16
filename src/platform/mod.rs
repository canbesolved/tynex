// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Per-OS backends. OS types never leave this module or `adapters`.

#[cfg(target_os = "linux")]
pub mod linux;
