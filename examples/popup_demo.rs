// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Shows the popup with the mock windows.
//!
//! Run in a Wayland session: `cargo run --example popup_demo`. Press a
//! letter or `Enter` to pick a window, arrows to move, `Esc` to close.
//! Without input the popup closes after a few seconds. Fonts come from
//! fontconfig, with fallback for scripts the default font lacks.

use std::time::{Duration, Instant};

use rustix::event::{PollFd, PollFlags, Timespec, poll};
use tynex::adapters::mock::MockSource;
use tynex::config::Font;
use tynex::core::event::{Action, Event, ModeKind};
use tynex::core::keys::DEFAULT_KEYS;
use tynex::core::queue::channel;
use tynex::core::switcher::{HotkeyAction, Switcher};
use tynex::core::traits::WindowSource;
use tynex::modes::hotkey::HotkeyMode;
use tynex::platform::linux::fonts::{FcFallback, default_face};
use tynex::platform::linux::wayland::{PresenterEvent, WaylandPresenter};
use tynex::ui::layouts::vertical_list::VerticalList;
use tynex::ui::layouts::{Layout, Metrics, Paint};
use tynex::ui::render::text::Text;
use tynex::ui::render::theme::Theme;

const SHOW_FOR: Duration = Duration::from_secs(10);

type App = Switcher<HotkeyMode, HotkeyMode>;

/// Sizes the popup for the current view and draws it.
fn redraw(
    presenter: &mut WaylandPresenter,
    app: &App,
    text: &mut Text,
    theme: &Theme,
) {
    let Some(view) = app.view() else { return };
    let m = Metrics {
        font_size: 14.0,
        width: 480.0,
        max_rows: 19,
        scale: presenter.scale(),
    };

    presenter.show(VerticalList.size(&view, &m, theme));
    presenter.draw(|canvas| {
        let mut paint = Paint {
            canvas,
            text,
            theme,
        };

        VerticalList.draw(&view, &m, &mut paint);
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = WaylandPresenter::connect()?;
    let mut text =
        Text::new(default_face(&Font::Auto)?, Box::new(FcFallback::default()));
    let theme = Theme::default();
    let (tx, rx) = channel();
    let mut source = MockSource::new();
    let mut app =
        Switcher::new(DEFAULT_KEYS, HotkeyAction::Hide, HotkeyMode, HotkeyMode);

    source.start(tx)?;
    while let Some(event) = rx.recv() {
        app.handle(event);
    }
    app.handle(Event::Show(ModeKind::Hotkey));
    for row in app.view().ok_or("popup is not open")?.rows() {
        text.prepare(&row.title); // Font fallback before the first frame
    }
    redraw(&mut presenter, &app, &mut text, &theme);

    let end = Instant::now() + SHOW_FOR;

    while Instant::now() < end {
        presenter.flush()?;

        let wake = presenter.next_deadline().map_or(end, |d| d.min(end));
        let timeout =
            Timespec::try_from(wake.saturating_duration_since(Instant::now()))?;

        {
            let fd = presenter.fd();
            let mut fds = [PollFd::new(&fd, PollFlags::IN)];

            poll(&mut fds, Some(&timeout))?;
        }

        for event in presenter.dispatch()? {
            let action = match event {
                PresenterEvent::Closed | PresenterEvent::FocusLost => {
                    Action::Close
                }
                PresenterEvent::Key(key) => app.handle(Event::Key(key)),
                PresenterEvent::Scale(_) | PresenterEvent::Configured => {
                    Action::Redraw
                }
            };

            match action {
                Action::Activate(id) => {
                    println!("activate {}", id.0);
                    return close(&mut presenter);
                }
                Action::Close => return close(&mut presenter),
                Action::Redraw => {
                    redraw(&mut presenter, &app, &mut text, &theme);
                }
                Action::None => {}
            }
        }
    }
    close(&mut presenter)
}

/// Hides the popup and sends the request before exiting.
fn close(
    presenter: &mut WaylandPresenter,
) -> Result<(), Box<dyn std::error::Error>> {
    presenter.hide();
    presenter.flush()?;
    Ok(())
}
