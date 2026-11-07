// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Linux backend: Wayland presenter, D-Bus, event loop, fonts, paths,
//! desktop files.

pub mod apps;
pub mod counter_file;
pub mod data_file;
pub mod dbus;
pub mod dbus_ipc;
pub mod desktop_files;
pub mod evdev;
pub mod event_loop;
pub mod fonts;
pub mod hint;
pub mod hint_file;
mod hint_proc;
pub mod icon_theme;
pub mod key_file;
pub mod paths;
pub mod signals;
pub mod usage_file;
pub mod wayland;
