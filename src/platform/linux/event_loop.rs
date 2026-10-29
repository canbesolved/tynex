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
use crate::modes::prompt::PromptMode;
use crate::platform::linux::apps::Apps;
use crate::platform::linux::dbus::Bus;
use crate::platform::linux::hint::Hint;
use crate::platform::linux::wayland::{PresenterEvent, WaylandPresenter};
use crate::ui::painter::Painter;

/// The switcher with its modes.
pub type App = Switcher<PromptMode>;

/// Everything the loop drives.
pub struct Parts {
    pub app: App,
    pub source: Source,
    pub presenter: WaylandPresenter,
    pub bus: Bus,
    pub painter: Painter,
    pub apps: Apps,
    pub hint: Hint,
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

        drive_hint(p);
        p.presenter.flush().map_err(|e| e.to_string())?;
        p.bus.flush(); // Calls such as an activation
        p.apps.reap();
        wait(p)?;

        for event in p.presenter.dispatch().map_err(|e| e.to_string())? {
            match event {
                PresenterEvent::Key(key) => {
                    handle(p, Event::Key(key));
                }
                PresenterEvent::Configured | PresenterEvent::Scale(_) => {
                    redraw(p);
                }
                PresenterEvent::Closed | PresenterEvent::FocusLost => {
                    handle(p, Event::Hide);
                }
            }
        }
        p.bus.dispatch().map_err(|e| e.to_string())?;
    }
}

/// Blocks until a fd is readable or the next key repeat is due.
fn wait(p: &Parts) -> Result<(), String> {
    let deadline = earliest(p.presenter.next_deadline(), p.hint.deadline());
    let timeout = deadline
        .map(|at| {
            Timespec::try_from(at.saturating_duration_since(Instant::now()))
        })
        .transpose()
        .map_err(|e| e.to_string())?;
    let wayland = p.presenter.fd();
    let dbus = p.bus.fd();
    let hint = p.hint.stdout_fd();
    let n = 2 + usize::from(hint.is_some());
    let extra = hint.unwrap_or(wayland);
    let mut fds = [
        PollFd::new(&wayland, PollFlags::IN),
        PollFd::new(&dbus, PollFlags::IN),
        PollFd::new(&extra, PollFlags::IN),
    ];

    match poll(&mut fds[..n], timeout.as_ref()) {
        Ok(_) | Err(rustix::io::Errno::INTR) => Ok(()),
        Err(e) => Err(e.to_string()),
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

/// Feeds one event to the switcher and applies the result. Returns false
/// on `Quit`.
fn handle(p: &mut Parts, event: Event) -> bool {
    let quit = event == Event::Quit;
    let windows = matches!(event, Event::Windows(_));
    let open = p.app.view().is_some();

    if matches!(event, Event::Show) && !open {
        let text = p.hint.on_show(Instant::now());

        p.app.set_hint(text);
    }

    let action = p.app.handle(event);

    if open && p.app.view().is_none() {
        p.hint.on_hide();
    }

    if windows {
        let list = p.app.windows(); // Titles are row texts now

        p.painter.prepare(list, p.presenter.scale());
        log::debug!("window list updated, {} windows", list.len());
    }

    match action {
        Action::Redraw => redraw(p),
        Action::Activate(id) => {
            p.presenter.hide();
            log::info!("activate {}", id.0);
            if let Err(e) = p.source.activate(&id) {
                log::warn!("cannot activate {}: {e}", id.0);
            }
        }
        Action::Launch(id) => {
            p.presenter.hide();
            if let Err(e) = p.apps.launch(&id) {
                log::warn!("cannot launch {id}: {e}");
            }
        }
        Action::Close => p.presenter.hide(),
        Action::None => {}
    }
    !quit
}

/// Applies a finished command or the next dot frame, then redraws.
fn drive_hint(p: &mut Parts) {
    let open = p.app.view().is_some();
    let query_empty = p.app.view().is_some_and(|v| v.query.is_empty());
    let Some(text) = p.hint.poll(Instant::now(), open, query_empty) else {
        return;
    };
    let same = p.app.view().is_some_and(|v| v.hint == text);

    p.app.set_hint(text);
    if open && query_empty && !same {
        redraw(p);
    }
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
