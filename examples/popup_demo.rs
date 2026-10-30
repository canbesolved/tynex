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
use tynex::config::{Font, HintConfig, MainConfig, TaskbarConfig, TaskbarMode};
use tynex::config::{LauncherConfig, LauncherMode, OutputConfig};
use tynex::core::event::{Action, Event};
use tynex::core::order::SortBy;
use tynex::core::queue::channel;
use tynex::core::switcher::{HotkeyAction, Switcher};
use tynex::core::traits::WindowSource;
use tynex::modes::prompt::PromptMode;
use tynex::modules::input::Input;
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

fn painter(text: Text, icons: Icons) -> Painter {
    let main = MainConfig {
        desktop: tynex::config::Desktop::Mock,
        font: Font::Auto,
        font_size: 14,
        width: 480,
    };
    let output = OutputConfig { max_rows: 19 };
    let taskbar = TaskbarConfig {
        mode: TaskbarMode::Both,
        ..TaskbarConfig::default()
    };

    Painter::new(
        text,
        icons,
        &main,
        &output,
        &HintConfig::default(),
        &taskbar,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = WaylandPresenter::connect()?;
    let mut painter = painter(
        Text::new(default_face(&Font::Auto)?),
        Icons::new(Box::new(IconTheme::default())),
    );
    let (tx, rx) = channel();
    let mut source = MockSource::new();
    let mut app = Switcher::new(
        HotkeyAction::Hide,
        true,
        SortBy::Recently,
        PromptMode::new(
            Input::new(
                LauncherConfig {
                    mode: LauncherMode::Off,
                    ..LauncherConfig::default()
                },
                Vec::new(),
            ),
            TaskbarMode::Both,
            19,
        ),
    );

    source.start(tx)?;
    while let Some(event) = rx.recv() {
        app.handle(event);
    }
    app.handle(Event::Show);
    redraw(&mut presenter, &app, &mut painter);
    session(&mut presenter, &mut app, &mut painter)
}

/// Runs the popup until it closes or the demo time is up.
fn session(
    presenter: &mut WaylandPresenter,
    app: &mut App,
    painter: &mut Painter,
) -> Result<(), Box<dyn std::error::Error>> {
    let end = Instant::now() + SHOW_FOR;

    while Instant::now() < end {
        presenter.flush()?;

        let anim = painter.next_frame(Instant::now());
        let wake = presenter
            .next_deadline()
            .into_iter()
            .chain(anim)
            .fold(end, std::cmp::Ord::min);
        let timeout =
            Timespec::try_from(wake.saturating_duration_since(Instant::now()))?;

        {
            let fd = presenter.fd();
            let mut fds = [PollFd::new(&fd, PollFlags::IN)];

            poll(&mut fds, Some(&timeout))?;
        }

        if painter.needs_frame(Instant::now()) {
            redraw(presenter, app, painter);
        }

        for event in presenter.dispatch()? {
            let action = match event {
                PresenterEvent::Closed | PresenterEvent::FocusLost => {
                    Action::Close
                }
                PresenterEvent::Key(key) => app.handle(Event::Key(key)),
                PresenterEvent::Click { x, y } => {
                    click(presenter, app, painter, x, y, false)
                }
                PresenterEvent::Hover { x, y } => {
                    click(presenter, app, painter, x, y, true)
                }
                PresenterEvent::Scale(_)
                | PresenterEvent::Configured
                | PresenterEvent::Redraw => Action::Redraw,
            };

            match action {
                Action::Activate(id) => {
                    println!("activate {}", id.0);
                    return close(presenter);
                }
                Action::Launch(id) => {
                    println!("launch {id}");
                    return close(presenter);
                }
                Action::Close => return close(presenter),
                Action::Redraw => redraw(presenter, app, painter),
                Action::None => {}
            }
        }
    }
    close(presenter)
}

/// Returns the action for a pointer event at surface `(x, y)`.
fn click(
    presenter: &WaylandPresenter,
    app: &mut App,
    painter: &Painter,
    x: f64,
    y: f64,
    hover: bool,
) -> Action {
    let scale = presenter.scale();
    let hit = app
        .view()
        .map(|view| painter.hit(&view, scale, px(x, scale), px(y, scale)));
    let event = |hit| {
        if hover {
            Event::Hover(hit)
        } else {
            Event::Click(hit)
        }
    };

    hit.map_or(Action::None, |hit| app.handle(event(hit)))
}

/// Converts a surface coordinate to physical pixels.
fn px(value: f64, scale: f32) -> f32 {
    #[allow(clippy::cast_possible_truncation)]
    let value = value as f32;

    value * scale
}

/// Hides the popup and sends the request before exiting.
fn close(
    presenter: &mut WaylandPresenter,
) -> Result<(), Box<dyn std::error::Error>> {
    presenter.hide();
    presenter.flush()?;
    Ok(())
}
