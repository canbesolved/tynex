// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The poll set: Wayland, D-Bus, the hint pipe, signals and the tabs
//! socket.

use std::time::Instant;

use rustix::event::{PollFd, PollFlags, Timespec, poll};

use super::{Parts, handle};
use crate::adapters::browser_tabs::Socket;
use crate::core::browser_tabs::TabsUpdate;
use crate::core::event::Event;

/// Blocks until a fd is readable or the next deadline is due.
pub(super) fn wait(p: &Parts) -> Result<(), String> {
    let deadline = earliest(
        earliest(p.presenter.next_deadline(), p.hint.deadline()),
        p.painter.next_frame(Instant::now()),
    );
    let timeout = deadline
        .map(|at| {
            Timespec::try_from(at.saturating_duration_since(Instant::now()))
        })
        .transpose()
        .map_err(|e| e.to_string())?;
    let wayland = p.presenter.fd();
    let dbus = p.bus.fd();
    let hint = p.hint.stdout_fd();
    let tab_fds = p.tabs.as_ref().map(Socket::fds).unwrap_or_default();
    let mut fds = vec![
        PollFd::new(&wayland, PollFlags::IN),
        PollFd::new(&dbus, PollFlags::IN),
    ];

    if let Some(hint) = &hint {
        fds.push(PollFd::new(hint, PollFlags::IN));
    }
    if let Some(signals) = &p.signals {
        fds.push(PollFd::new(signals, PollFlags::IN));
    }
    for (fd, write) in &tab_fds {
        let flags = if *write {
            PollFlags::IN | PollFlags::OUT
        } else {
            PollFlags::IN
        };

        fds.push(PollFd::new(fd, flags));
    }

    match poll(&mut fds, timeout.as_ref()) {
        Ok(_) | Err(rustix::io::Errno::INTR) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// Reads and writes the tabs socket and feeds what arrived. Favicon
/// pixels go to the painter; the core only sees that they changed.
pub(super) fn drive_tabs(p: &mut Parts) {
    let Some(batch) = p.tabs.as_mut().map(Socket::drive) else {
        return;
    };

    let rejected = batch
        .icons
        .iter()
        .filter(|i| !p.painter.store_tab_icon(i.source, &i.key, &i.png))
        .count();

    if rejected > 0 {
        log::debug!("browser tabs: {rejected} favicons rejected (size, cap)");
    }
    for update in batch.updates {
        if let TabsUpdate::Gone(source) = update {
            p.painter.drop_tab_icons(source);
        }
        handle(p, Event::Tabs(update));
    }
}

/// Returns the sooner of two optional deadlines.
fn earliest(a: Option<Instant>, b: Option<Instant>) -> Option<Instant> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, b) => b,
    }
}
