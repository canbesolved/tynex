// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Shows the popup with the mock windows for a few seconds.
//!
//! Run in a Wayland session: `cargo run --example popup_demo`. The
//! keyboard is not handled yet; the popup closes by itself.

use std::time::{Duration, Instant};

use rustix::event::{PollFd, PollFlags, Timespec, poll};
use tynex::adapters::mock::MockSource;
use tynex::core::event::{Event, ModeKind};
use tynex::core::keys::DEFAULT_KEYS;
use tynex::core::queue::channel;
use tynex::core::switcher::Switcher;
use tynex::core::traits::WindowSource;
use tynex::modes::hotkey::HotkeyMode;
use tynex::platform::linux::wayland::{PresenterEvent, WaylandPresenter};
use tynex::ui::layouts::vertical_list::VerticalList;
use tynex::ui::layouts::{Layout, Metrics, Paint};
use tynex::ui::render::glyphs::{Face, NoFallback};
use tynex::ui::render::text::Text;
use tynex::ui::render::theme::Theme;

static FONT: &[u8] = include_bytes!("../tests/fonts/dejavu_sans.ttf");
const SHOW_FOR: Duration = Duration::from_secs(4);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = WaylandPresenter::connect()?;
    let mut text =
        Text::new(Face::new(FONT, 0).ok_or("bad font")?, Box::new(NoFallback));
    let theme = Theme::default();
    let (tx, rx) = channel();
    let mut source = MockSource::new();
    let mut switcher = Switcher::new(DEFAULT_KEYS, HotkeyMode, HotkeyMode);

    source.start(tx)?;
    while let Some(event) = rx.recv() {
        switcher.handle(event);
    }
    switcher.handle(Event::Show(ModeKind::Hotkey));

    let metrics = |scale| Metrics {
        font_size: 14.0,
        width: 480.0,
        max_rows: 19,
        scale,
    };
    let view = switcher.view().ok_or("popup is not open")?;

    presenter.show(VerticalList.size(
        &view,
        &metrics(presenter.scale()),
        &theme,
    ));

    let end = Instant::now() + SHOW_FOR;

    while Instant::now() < end {
        presenter.flush()?;

        let left = end.saturating_duration_since(Instant::now());
        let timeout = Timespec::try_from(left)?;
        {
            let fd = presenter.fd();
            let mut fds = [PollFd::new(&fd, PollFlags::IN)];

            poll(&mut fds, Some(&timeout))?;
        }

        for event in presenter.dispatch()? {
            match event {
                PresenterEvent::Closed => return Ok(()),
                PresenterEvent::Scale(_) | PresenterEvent::Configured => {
                    let m = metrics(presenter.scale());

                    presenter.show(VerticalList.size(&view, &m, &theme));
                    presenter.draw(|canvas| {
                        let mut paint = Paint {
                            canvas,
                            text: &mut text,
                            theme: &theme,
                        };

                        VerticalList.draw(&view, &m, &mut paint);
                    });
                }
            }
        }
    }
    presenter.hide();
    presenter.flush()?;
    Ok(())
}
