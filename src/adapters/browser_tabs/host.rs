// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Native messaging host: relays frames between the browser (stdio)
//! and the daemon (the tabs socket). It adds only `hello` and `status`.

use std::ffi::OsString;
use std::io::{ErrorKind, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use crate::adapters::browser_tabs::bind::NAME;
use crate::adapters::browser_tabs::frame::{self, BadFrame, FrameReader};

/// Largest frame from the browser.
const TO_DAEMON: usize = 32 * 1024 * 1024;

/// Largest frame toward the browser, Chrome's own limit.
const TO_BROWSER: usize = 1024 * 1024;

/// Most bytes queued toward the daemon. Frames beyond are dropped.
const MAX_OUT: usize = 8 * 1024 * 1024;

/// True for the single argument Chrome passes to a native host:
/// `chrome-extension://<32 letters a to p>/`.
#[must_use]
pub fn is_host<I: IntoIterator<Item = OsString>>(args: I) -> bool {
    let mut args = args.into_iter();
    let (Some(arg), None) = (args.next(), args.next()) else {
        return false;
    };

    arg.to_str()
        .and_then(|text| text.strip_prefix("chrome-extension://"))
        .and_then(|rest| rest.strip_suffix('/'))
        .is_some_and(|id| {
            id.len() == 32 && id.bytes().all(|b| (b'a'..=b'p').contains(&b))
        })
}

/// Relay state. The poll loop owns the descriptors' readiness.
pub struct Relay {
    path: PathBuf,
    sock: Option<UnixStream>,
    to_daemon: Vec<u8>,
    pub to_browser: Vec<u8>, // Written to stdout by the loop
    from_browser: FrameReader,
    from_daemon: FrameReader,
}

impl Relay {
    /// `dir` holds the tabs socket.
    #[must_use]
    pub fn new(dir: &Path) -> Self {
        Self {
            path: dir.join(NAME),
            sock: None,
            to_daemon: Vec::new(),
            to_browser: Vec::new(),
            from_browser: FrameReader::new(TO_DAEMON),
            from_daemon: FrameReader::new(TO_BROWSER),
        }
    }

    /// Queues a `status` frame for the extension's badge.
    pub fn status(&mut self, up: bool) {
        let body = format!(r#"{{"v":1,"type":"status","daemon":{up}}}"#);

        self.to_browser.extend(frame::encode(body.as_bytes()));
    }

    /// Connects if the socket accepts. Sends `hello` with the browser
    /// pid, then tells the extension. True when now connected.
    pub fn try_connect(&mut self) -> bool {
        if self.sock.is_some() {
            return true;
        }
        let stream = match UnixStream::connect(&self.path) {
            Ok(stream) => stream,
            Err(e) => {
                if !matches!(
                    e.kind(),
                    ErrorKind::NotFound | ErrorKind::ConnectionRefused
                ) {
                    log::warn!("browser tabs: host cannot connect: {e}");
                }
                return false;
            }
        };
        if let Err(e) = stream.set_nonblocking(true) {
            log::warn!("browser tabs: host socket: {e}");
            return false;
        }
        let pid = rustix::process::Pid::as_raw(rustix::process::getppid());
        let hello = format!(r#"{{"v":1,"type":"hello","pid":{pid}}}"#);

        self.to_daemon = frame::encode(hello.as_bytes());
        self.from_daemon = FrameReader::new(TO_BROWSER);
        self.sock = Some(stream);
        self.status(true);
        log::info!("browser tabs: host connected to the daemon");
        true
    }

    /// Forwards browser frames. Dropped while the daemon is away.
    ///
    /// # Errors
    ///
    /// Returns an error when a frame length is bad; the host exits.
    pub fn on_stdin(&mut self, bytes: &[u8]) -> Result<(), BadFrame> {
        for frame in self.from_browser.push(bytes)? {
            if self.sock.is_none() {
                log::debug!("browser tabs: host dropped a frame, no daemon");
            } else if self.to_daemon.len() + 4 + frame.len() > MAX_OUT {
                log::warn!("browser tabs: host dropped a frame, daemon slow");
            } else {
                self.to_daemon.extend(frame::encode(&frame));
            }
        }
        Ok(())
    }

    /// Reads daemon frames toward the browser. Closes on EOF, an error
    /// or a bad length.
    pub fn on_socket(&mut self) {
        let mut buf = [0u8; 8192];

        while let Some(sock) = &mut self.sock {
            match sock.read(&mut buf) {
                Ok(0) => return self.disconnect("daemon closed"),
                Ok(n) => match self.from_daemon.push(&buf[..n]) {
                    Ok(frames) => {
                        for frame in frames {
                            self.to_browser.extend(frame::encode(&frame));
                        }
                    }
                    Err(_) => return self.disconnect("bad frame length"),
                },
                Err(e) if e.kind() == ErrorKind::WouldBlock => return,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(_) => return self.disconnect("read error"),
            }
        }
    }

    /// Writes queued frames to the daemon.
    pub fn flush(&mut self) {
        while let Some(sock) = &mut self.sock {
            if self.to_daemon.is_empty() {
                return;
            }
            match sock.write(&self.to_daemon) {
                Ok(0) => return self.disconnect("write returned 0"),
                Ok(n) => drop(self.to_daemon.drain(..n)),
                Err(e) if e.kind() == ErrorKind::WouldBlock => return,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(_) => return self.disconnect("write error"),
            }
        }
    }

    /// Drops the daemon connection for `reason` and tells the
    /// extension.
    pub fn disconnect(&mut self, reason: &str) {
        log::info!("browser tabs: host lost the daemon ({reason}), waiting");
        self.sock = None;
        self.to_daemon = Vec::new();
        self.status(false);
    }

    /// The daemon connection, when one is open.
    #[must_use]
    pub fn socket(&self) -> Option<&UnixStream> {
        self.sock.as_ref()
    }

    /// True when bytes wait to be written to the daemon.
    #[must_use]
    pub fn wants_write(&self) -> bool {
        !self.to_daemon.is_empty()
    }
}

#[cfg(test)]
#[path = "host_tests.rs"]
mod tests;
