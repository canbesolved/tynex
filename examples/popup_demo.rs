// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Shows the popup with the mock windows.
//!
//! Run in a Wayland session: `cargo run --example popup_demo`. Type to
//! search, `Enter` to pick a window, arrows to move, `Esc` to close.
//! Without input the popup closes after a few seconds. Fonts come from
//! fontconfig, with fallback for scripts the default font lacks.

use std::time::{Duration, Instant};

use rustix::event::{PollFd, PollFlags, Timespec, poll};
use tynex::adapters::mock::MockSource;
use tynex::config::{AppearanceConfig, Font};
use tynex::core::event::{Action, Event};
use tynex::core::order::SortBy;
use tynex::core::queue::channel;
use tynex::core::switcher::{HotkeyAction, Switcher};
use tynex::core::traits::WindowSource;
use tynex::modes::prompt::PromptMode;
use tynex::platform::linux::fonts::default_face;
use tynex::platform::linux::icon_theme::IconTheme;
use tynex::platform::linux::wayland::{PresenterEvent, WaylandPresenter};
use tynex::ui::painter::Painter;
use tynex::ui::render::icons::Icons;
use tynex::ui::render::text::Text;

const SHOW_FOR: Duration = Duration::from_secs(10);

type App = Switcher<PromptMode>;

/// Sizes the popup for the current view and draws it.
fn redraw(presenter: &mut WaylandPresenter, app: &App, painter: &mut Painter) {
    let Some(view) = app.view() else { return };
    let scale = presenter.scale();

    presenter.show(painter.size(&view, scale));
    presenter.draw(|canvas| painter.draw(canvas, &view, scale));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = WaylandPresenter::connect()?;
    let text = Text::new(default_face(&Font::Auto)?);
    let icons = Icons::new(Box::new(IconTheme::default()));
    let mut painter = Painter::new(
        text,
        icons,
        AppearanceConfig {
            font: Font::Auto,
            font_size: 14,
            width: 480,
            max_rows: 19,
            title_bar: true,
            show_app: true,
            strip_app_name: false,
        },
    );
    let (tx, rx) = channel();
    let mut source = MockSource::new();
    let mut app =
        Switcher::new(HotkeyAction::Hide, true, SortBy::Recently, PromptMode);

    source.start(tx)?;
    while let Some(event) = rx.recv() {
        app.handle(event);
    }
    app.handle(Event::Show);
    redraw(&mut presenter, &app, &mut painter);

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
                    redraw(&mut presenter, &app, &mut painter);
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
