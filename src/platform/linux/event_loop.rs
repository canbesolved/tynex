// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The daemon's single loop: one `poll` over the Wayland and D-Bus fds.
//! The timeout is the next key repeat, otherwise none, so an idle daemon
//! never wakes up.

use std::time::{Duration, Instant};

use crate::adapters::Source;
use crate::adapters::browser_tabs::Socket;
use crate::core::event::{Action, Event};
use crate::core::queue::EventRx;
use crate::core::switcher::Switcher;
use crate::core::traits::WindowSource;
use crate::modes::prompt::PromptMode;
use crate::platform::linux::apps::Apps;
use crate::platform::linux::dbus::Bus;
use crate::platform::linux::hint::Hint;
use crate::platform::linux::signals::Signals;
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
    pub tabs: Option<Socket>,
    pub signals: Option<Signals>, // SIGINT, SIGTERM, SIGHUP quit
    pub wait: Duration,           // For the tab lists before a show
    pub hold: Option<Instant>,    // The first draw waits until then
}

/// Runs until a `Quit` request arrives.
///
/// # Errors
///
/// Fails when the Wayland or D-Bus connection is lost.
pub fn run(p: &mut Parts) -> Result<(), String> {
    loop {
        fds::drive_tabs(p); // Before the flushes below send its output
        while let Some(event) = p.rx.recv() {
            if !handle(p, event) {
                return quit(p);
            }
        }

        fds::release(p);
        drive_hint(p);
        if p.painter.needs_frame(Instant::now()) {
            redraw(p);
        }
        p.presenter.flush().map_err(|e| e.to_string())?;
        p.bus.flush(); // Calls such as an activation
        p.apps.reap();
        fds::wait(p)?;

        for event in p.presenter.dispatch().map_err(|e| e.to_string())? {
            match event {
                PresenterEvent::Key(key) => {
                    handle(p, Event::Key(key));
                }
                PresenterEvent::Click { x, y } => point(p, x, y, false),
                PresenterEvent::Hover { x, y } => point(p, x, y, true),
                PresenterEvent::Scroll { notches, dy } => {
                    scroll(p, notches, dy);
                }
                PresenterEvent::Configured
                | PresenterEvent::Scale(_)
                | PresenterEvent::Redraw => {
                    redraw(p);
                }
                PresenterEvent::Closed | PresenterEvent::FocusLost => {
                    handle(p, Event::Hide);
                }
            }
        }
        p.bus.dispatch().map_err(|e| e.to_string())?;
        if let Some(name) = p.signals.as_ref().and_then(Signals::take) {
            log::info!("{name} received, quitting");
            handle(p, Event::Quit);
            return quit(p);
        }
    }
}

/// Hides the popup and sends the last requests. The caller then
/// stops the window source, like after `tynex quit`.
fn quit(p: &mut Parts) -> Result<(), String> {
    p.presenter.hide();
    p.presenter.flush().map_err(|e| e.to_string())
}

/// Converts a surface coordinate to physical pixels.
fn px(value: f64, scale: f32) -> f32 {
    #[allow(clippy::cast_possible_truncation)]
    let value = value as f32;

    value * scale
}

/// Hit-tests a pointer event and feeds it to the switcher.
fn point(p: &mut Parts, x: f64, y: f64, hover: bool) {
    let scale = p.presenter.scale();
    let hit = p
        .app
        .view()
        .map(|view| p.painter.hit(&view, scale, px(x, scale), px(y, scale)));

    if let Some(hit) = hit {
        let event = if hover {
            Event::Hover(hit)
        } else {
            Event::Click(hit)
        };

        handle(p, event);
    }
}

/// Scrolls the list on screen and redraws when it moved.
fn scroll(p: &mut Parts, notches: i32, dy: f64) {
    let scale = p.presenter.scale();
    let moved = p
        .app
        .view()
        .is_some_and(|view| p.painter.scroll(&view, scale, notches, dy));

    if moved {
        redraw(p);
    }
}

#[path = "event_loop_fds.rs"]
mod fds;

/// Feeds one event to the switcher and applies the result. Returns false
/// on `Quit`.
fn handle(p: &mut Parts, event: Event) -> bool {
    let quit = event == Event::Quit;
    let windows = matches!(event, Event::Windows(_));
    let open = p.app.view().is_some();

    if matches!(event, Event::Show) && !open {
        let text = p.hint.on_show(Instant::now());

        p.app.set_hint(text);
        if let Some(tabs) = &mut p.tabs {
            tabs.list();
            if !p.wait.is_zero() && tabs.listing() {
                p.hold = Some(Instant::now() + p.wait);
            }
        }
    }

    let action = p.app.handle(event);

    if open && p.app.view().is_none() {
        p.hold = None;
        p.hint.on_hide();
        p.painter.hidden();
        if let Some(tabs) = &mut p.tabs {
            tabs.hide();
        }
    }

    if windows {
        let list = p.app.windows(); // Titles are row texts now

        p.painter.prepare(list, p.presenter.scale());
        log::debug!("window list updated, {} windows", list.len());
    }

    if let Some(text) = p.app.usage_to_save() {
        crate::platform::linux::usage_file::save(&text);
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
        Action::ActivateTab(tab) => {
            p.presenter.hide();
            if let Some(tabs) = &mut p.tabs {
                tabs.activate(&tab);
            }
        }
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
    if p.hold.is_some() {
        return; // The tab lists come first
    }
    let Some(view) = p.app.view() else { return };
    let scale = p.presenter.scale();
    let painter = &mut p.painter;

    painter.set_screen(p.presenter.screen_height(), p.presenter.anchor());
    p.presenter.show(painter.size(&view, scale));
    p.presenter
        .draw(|canvas| painter.draw(canvas, &view, scale));
}
