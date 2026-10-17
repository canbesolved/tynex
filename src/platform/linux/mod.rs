// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Linux backend: Wayland presenter, D-Bus, event loop, fonts, paths,
//! desktop files.

pub mod dbus;
pub mod dbus_ipc;
pub mod desktop_files;
pub mod event_loop;
pub mod fonts;
pub mod paths;
pub mod wayland;
