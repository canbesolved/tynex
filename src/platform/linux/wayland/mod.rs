// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Wayland presenter: a centered layer-shell popup drawn on the CPU.

// Surface sizes convert between float scales and integer pixels on
// purpose; values stay within the popup size.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

mod buffer;
mod error;
mod input;
mod keyboard;
mod state;
mod surface;

use std::fmt;
use std::os::fd::{AsFd, BorrowedFd};

use wayland_client::globals::registry_queue_init;
use wayland_client::protocol::wl_shm::Format;
use wayland_client::{Connection, EventQueue};

use crate::platform::linux::wayland::buffer::Buffers;
pub use crate::platform::linux::wayland::error::PresenterError;
use crate::platform::linux::wayland::error::{error, missing};
use crate::platform::linux::wayland::keyboard::Keyboard;
pub use crate::platform::linux::wayland::state::PresenterEvent;
use crate::platform::linux::wayland::state::Wl;
use crate::ui::Size;
use crate::ui::render::canvas::Canvas;

/// The popup on a Wayland compositor with `wlr-layer-shell`.
pub struct WaylandPresenter {
    conn: Connection,
    queue: EventQueue<Wl>,
    wl: Wl,
    size: Size, // Physical size of the popup
}

impl WaylandPresenter {
    /// Connects to `$WAYLAND_DISPLAY` and binds the needed globals.
    ///
    /// # Errors
    ///
    /// Fails without a Wayland session or without `wlr-layer-shell`.
    pub fn connect() -> Result<Self, PresenterError> {
        let conn = Connection::connect_to_env()
            .map_err(|e| error("cannot connect to Wayland", e))?;
        let (globals, mut queue) = registry_queue_init::<Wl>(&conn)
            .map_err(|e| error("cannot read Wayland globals", e))?;
        let qh = queue.handle();
        let mut wl = Wl {
            compositor: globals
                .bind(&qh, 4..=6, ())
                .map_err(missing("wl_compositor"))?,
            shm: globals.bind(&qh, 1..=1, ()).map_err(missing("wl_shm"))?,
            layer_shell: globals
                .bind(&qh, 1..=4, ())
                .map_err(missing("zwlr_layer_shell_v1"))?,
            viewporter: globals.bind(&qh, 1..=1, ()).ok(),
            fractional: globals.bind(&qh, 1..=1, ()).ok(),
            _seat: globals.bind(&qh, 4..=7, ()).ok(), // 4+ sends repeat info
            keyboard_proxy: None,
            keyboard: Keyboard::new(),
            formats: Vec::new(),
            scale: 1.0,
            popup: None,
            buffers: None,
            events: Vec::new(),
        };

        // Without a viewport, fractional scale cannot be applied.
        if wl.viewporter.is_none() {
            wl.fractional = None;
        }
        queue
            .roundtrip(&mut wl)
            .map_err(|e| error("Wayland roundtrip failed", e))?;
        Ok(Self {
            conn,
            queue,
            wl,
            size: Size {
                width: 0,
                height: 0,
            },
        })
    }

    /// Returns the connection fd for the poll loop.
    #[must_use]
    pub fn fd(&self) -> BorrowedFd<'_> {
        self.conn.as_fd()
    }

    /// Returns the output scale: 1.0, or fractional like 1.25.
    #[must_use]
    pub fn scale(&self) -> f32 {
        self.wl.scale
    }

    /// Returns true while the popup surface exists.
    #[must_use]
    pub fn is_shown(&self) -> bool {
        self.wl.popup.is_some()
    }

    /// Shows the popup at `size` physical pixels, or resizes it.
    pub fn show(&mut self, size: Size) {
        let qh = self.queue.handle();

        self.size = size;
        if self.wl.popup.is_none() {
            self.wl.popup = Some(surface::create(&self.wl, &qh));
        }
        if let Some(popup) = &self.wl.popup {
            let (w, h) = surface::logical(size, self.wl.scale);

            popup.layer.set_size(w, h);
            if !popup.configured {
                popup.surface.commit();
            }
        }
    }

    /// Draws a frame with `paint` and commits it.
    ///
    /// Returns false when the popup is not configured yet or both buffers
    /// are still held by the compositor; draw again after the next event.
    pub fn draw(&mut self, paint: impl FnOnce(&mut Canvas<'_>)) -> bool {
        let Some(popup) = self.wl.popup.as_ref().filter(|p| p.configured)
        else {
            return false;
        };
        let (width, height) = (self.size.width.max(1), self.size.height.max(1));
        let format = surface::format(&self.wl.formats);

        if self
            .wl
            .buffers
            .as_ref()
            .is_none_or(|b| (b.width, b.height) != (width, height))
        {
            let qh = self.queue.handle();

            self.wl.buffers =
                Buffers::new(&self.wl.shm, &qh, (width, height), format).ok();
        }

        let Some(buffers) = &mut self.wl.buffers else {
            return false;
        };
        let Some((i, pixels)) = buffers.free() else {
            return false;
        };
        let Some(mut canvas) = Canvas::new(pixels, width, height) else {
            return false;
        };

        paint(&mut canvas);
        if format == Format::Argb8888 {
            // Memory order B, G, R, A: swap red and blue.
            pixels
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .for_each(|px| px.swap(0, 2));
        }
        surface::commit(popup, buffers.buffer(i), self.size, self.wl.scale);
        buffers.busy[i] = true;
        true
    }

    /// Returns when a held key repeats next; use it as the poll timeout.
    #[must_use]
    pub fn next_deadline(&self) -> Option<std::time::Instant> {
        self.wl.keyboard.next_repeat()
    }

    /// Destroys the popup and frees its buffers. The connection stays.
    pub fn hide(&mut self) {
        self.wl.keyboard.stop();
        if let Some(popup) = self.wl.popup.take() {
            surface::destroy(popup);
        }
        self.wl.buffers = None;
    }

    /// Sends queued requests to the compositor. Call before polling.
    ///
    /// # Errors
    ///
    /// Fails when the connection is lost.
    pub fn flush(&mut self) -> Result<(), PresenterError> {
        self.queue
            .flush()
            .map_err(|e| error("Wayland connection lost", e))
    }

    /// Reads available events, and key repeats that are due, and returns
    /// what the caller must handle.
    ///
    /// # Errors
    ///
    /// Fails when the connection is lost or a protocol error occurs.
    pub fn dispatch(&mut self) -> Result<Vec<PresenterEvent>, PresenterError> {
        let lost = |e: &dyn fmt::Display| error("Wayland connection lost", e);

        self.queue
            .dispatch_pending(&mut self.wl)
            .map_err(|e| lost(&e))?;
        if let Some(guard) = self.queue.prepare_read() {
            match guard.read() {
                Ok(_) => {}
                Err(wayland_client::backend::WaylandError::Io(e))
                    if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(lost(&e)),
            }
        }
        self.queue
            .dispatch_pending(&mut self.wl)
            .map_err(|e| lost(&e))?;
        if let Some(key) = self.wl.keyboard.tick(std::time::Instant::now()) {
            self.wl.events.push(PresenterEvent::Key(key));
        }
        Ok(std::mem::take(&mut self.wl.events))
    }
}
