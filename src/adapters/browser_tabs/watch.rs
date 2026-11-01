// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! inotify watch on the runtime folder, to see the tabs socket appear.

use std::mem::MaybeUninit;
use std::path::Path;

use rustix::fd::OwnedFd;
use rustix::fs::inotify::{self, CreateFlags, WatchFlags};

/// Watches `dir` for new and renamed-in files. `None` without inotify.
pub fn open(dir: &Path) -> Option<OwnedFd> {
    let fd =
        inotify::init(CreateFlags::CLOEXEC | CreateFlags::NONBLOCK).ok()?;

    inotify::add_watch(&fd, dir, WatchFlags::CREATE | WatchFlags::MOVED_TO)
        .ok()?;
    Some(fd)
}

/// Reads every pending event. Any of them is a reason to try a
/// connect; the folder holds only `tynex` files.
pub fn drain(fd: &OwnedFd) {
    let mut buf = [MaybeUninit::uninit(); 4096];
    let mut reader = inotify::Reader::new(fd, &mut buf);

    while reader.next().is_ok() {}
}
