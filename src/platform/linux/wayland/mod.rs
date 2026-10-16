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
mod state;
mod surface;

use std::fmt;
use std::os::fd::{AsFd, BorrowedFd};

use wayland_client::globals::{BindError, registry_queue_init};
use wayland_client::protocol::wl_shm::Format;
use wayland_client::{Connection, EventQueue};

use crate::platform::linux::wayland::buffer::Buffers;
pub use crate::platform::linux::wayland::state::PresenterEvent;
use crate::platform::linux::wayland::state::Wl;
use crate::ui::layouts::Size;
use crate::ui::render::canvas::Canvas;

/// Error from the Wayland connection or a missing protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenterError(pub String);

impl fmt::Display for PresenterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PresenterError {}

fn error(what: &str, e: impl fmt::Display) -> PresenterError {
    PresenterError(format!("{what}: {e}"))
}

/// Maps a failed bind of a required global to a clear error.
fn missing(name: &'static str) -> impl Fn(BindError) -> PresenterError {
    move |_| PresenterError(format!("the compositor lacks `{name}`"))
}

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

    /// Destroys the popup and frees its buffers. The connection stays.
    pub fn hide(&mut self) {
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

    /// Reads available events and returns what the caller must handle.
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
        Ok(std::mem::take(&mut self.wl.events))
    }
}
