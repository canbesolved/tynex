// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The daemon's single loop: one `poll` over the Wayland and D-Bus fds.
//! The timeout is the next key repeat, otherwise none, so an idle daemon
//! never wakes up.

use std::time::Instant;

use rustix::event::{PollFd, PollFlags, Timespec, poll};

use crate::adapters::Source;
use crate::core::event::{Action, Event};
use crate::core::queue::EventRx;
use crate::core::switcher::Switcher;
use crate::core::traits::WindowSource;
use crate::modes::hotkey::HotkeyMode;
use crate::platform::linux::dbus::Bus;
use crate::platform::linux::wayland::{PresenterEvent, WaylandPresenter};
use crate::ui::painter::Painter;

/// The switcher with its modes. Search mode is a stand-in for now.
pub type App = Switcher<HotkeyMode, HotkeyMode>;

/// Everything the loop drives.
pub struct Parts {
    pub app: App,
    pub source: Source,
    pub presenter: WaylandPresenter,
    pub bus: Bus,
    pub painter: Painter,
    pub rx: EventRx,
}

/// Runs until a `Quit` request arrives.
///
/// # Errors
///
/// Fails when the Wayland or D-Bus connection is lost.
pub fn run(p: &mut Parts) -> Result<(), String> {
    loop {
        while let Some(event) = p.rx.recv() {
            if !handle(p, event) {
                p.presenter.hide();
                return p.presenter.flush().map_err(|e| e.to_string());
            }
        }

        p.presenter.flush().map_err(|e| e.to_string())?;
        p.bus.flush(); // Calls such as an activation
        wait(p)?;

        for event in p.presenter.dispatch().map_err(|e| e.to_string())? {
            match event {
                PresenterEvent::Key(key) => {
                    handle(p, Event::Key(key));
                }
                PresenterEvent::Configured | PresenterEvent::Scale(_) => {
                    redraw(p);
                }
                PresenterEvent::Closed => {
                    handle(p, Event::Hide);
                }
            }
        }
        p.bus.dispatch().map_err(|e| e.to_string())?;
    }
}

/// Blocks until a fd is readable or the next key repeat is due.
fn wait(p: &Parts) -> Result<(), String> {
    let timeout = p
        .presenter
        .next_deadline()
        .map(|at| {
            Timespec::try_from(at.saturating_duration_since(Instant::now()))
        })
        .transpose()
        .map_err(|e| e.to_string())?;
    let (wayland, dbus) = (p.presenter.fd(), p.bus.fd());
    let mut fds = [
        PollFd::new(&wayland, PollFlags::IN),
        PollFd::new(&dbus, PollFlags::IN),
    ];

    match poll(&mut fds, timeout.as_ref()) {
        Ok(_) | Err(rustix::io::Errno::INTR) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// Feeds one event to the switcher and applies the result. Returns false
/// on `Quit`.
fn handle(p: &mut Parts, event: Event) -> bool {
    let quit = event == Event::Quit;

    if let Event::Windows(list) = &event {
        p.painter.prepare(list);
        log::debug!("window list updated, {} windows", list.len());
    }

    match p.app.handle(event) {
        Action::Redraw => redraw(p),
        Action::Activate(id) => {
            p.presenter.hide();
            log::info!("activate {}", id.0);
            if let Err(e) = p.source.activate(&id) {
                log::warn!("cannot activate {}: {e}", id.0);
            }
        }
        Action::Close => p.presenter.hide(),
        Action::None => {}
    }
    !quit
}

/// Sizes the popup for the current view and draws it, if it is open.
fn redraw(p: &mut Parts) {
    let Some(view) = p.app.view() else { return };
    let scale = p.presenter.scale();
    let painter = &mut p.painter;

    p.presenter.show(painter.size(&view, scale));
    p.presenter
        .draw(|canvas| painter.draw(canvas, &view, scale));
}
