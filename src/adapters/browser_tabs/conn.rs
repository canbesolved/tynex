// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! One host connection of the tabs socket: buffers and liveness.

use std::io::{ErrorKind, Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

use crate::adapters::browser_tabs::frame::FrameReader;
use crate::core::browser_tabs::SourceId;

/// Largest frame toward the daemon. A `tabs` reply of 10 000 rows fits.
const FRAME: usize = 32 * 1024 * 1024;

/// Most bytes queued toward one host.
const MAX_OUT: usize = 64 * 1024;

/// Most bytes read from one host per call; poll wakes again for more.
const MAX_READ: usize = 1024 * 1024;

/// One connected host. `pid` is set by its `hello`.
pub(super) struct Conn {
    pub id: SourceId,
    pub pid: Option<u32>,
    pub browser: Option<String>, // From its first `tabs` reply, for logs
    pub closed: Option<&'static str>, // Why it closed; dropped next pass
    pub warned: bool,            // Bad JSON was logged once
    stream: UnixStream,
    reader: FrameReader,
    out: Vec<u8>,
}

impl Conn {
    pub fn new(id: SourceId, stream: UnixStream) -> Self {
        let closed = stream
            .set_nonblocking(true)
            .is_err()
            .then_some("cannot set non-blocking");

        Self {
            id,
            pid: None,
            browser: None,
            closed,
            warned: false,
            stream,
            reader: FrameReader::new(FRAME),
            out: Vec::new(),
        }
    }

    /// `source 1 (Brave, pid 42)`, for logs.
    pub fn label(&self) -> String {
        let browser = self.browser.as_deref().unwrap_or("browser");

        match self.pid {
            Some(pid) => format!("source {} ({browser}, pid {pid})", self.id.0),
            None => format!("source {} (no hello yet)", self.id.0),
        }
    }

    /// True until the connection is closed.
    pub fn open(&self) -> bool {
        self.closed.is_none()
    }

    pub fn stream(&self) -> &UnixStream {
        &self.stream
    }

    /// True when queued bytes wait for the socket to drain.
    pub fn wants_write(&self) -> bool {
        !self.out.is_empty()
    }

    /// Queues one frame. A full buffer closes the connection.
    pub fn send(&mut self, frame: &[u8]) {
        if self.out.len() + frame.len() > MAX_OUT {
            log::warn!("browser tabs: {} is not reading", self.label());
            self.kill("output buffer full");
            return;
        }
        self.out.extend_from_slice(frame);
    }

    /// Writes what the socket takes now.
    pub fn flush(&mut self) {
        while self.open() && !self.out.is_empty() {
            match self.stream.write(&self.out) {
                Ok(0) => self.kill("write returned 0"),
                Ok(n) => drop(self.out.drain(..n)),
                Err(e) if e.kind() == ErrorKind::WouldBlock => return,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => {
                    log::debug!(
                        "browser tabs: write to source {}: {e}",
                        self.id.0
                    );
                    self.kill("write error");
                }
            }
        }
    }

    /// Reads whole frames. EOF, an error or a bad length closes.
    pub fn read(&mut self) -> Vec<Vec<u8>> {
        let mut frames = Vec::new();
        let mut buf = [0u8; 8192];
        let mut total = 0;

        while self.open() && total < MAX_READ {
            match self.stream.read(&mut buf) {
                Ok(0) => self.kill("closed by the host"),
                Ok(n) => {
                    total += n;
                    match self.reader.push(&buf[..n]) {
                        Ok(more) => frames.extend(more),
                        Err(_) => self.kill("bad frame length"),
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => {
                    log::debug!(
                        "browser tabs: read from source {}: {e}",
                        self.id.0
                    );
                    self.kill("read error");
                }
            }
        }
        frames
    }

    /// Marks the connection closed for `reason`. The shutdown wakes the
    /// poll loop, so the owner drops it on its next pass.
    pub fn kill(&mut self, reason: &'static str) {
        if self.closed.is_some() {
            return; // Keep the first reason
        }
        self.closed = Some(reason);
        self.out = Vec::new();
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}
