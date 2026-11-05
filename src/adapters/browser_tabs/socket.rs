// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Daemon side of the tabs socket. Hosts connect; this only listens.

use std::fs;
use std::io::ErrorKind;
use std::os::fd::{AsFd, BorrowedFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

use super::bind;
use super::conn::Conn;
use super::msg;
use crate::core::browser_tabs::{SourceId, TabRef, TabsUpdate};

/// Most hosts at once.
const MAX_CONN: usize = 8;

/// One favicon, still encoded as PNG.
pub struct IconPng {
    pub source: SourceId,
    pub key: String,
    pub png: Vec<u8>,
}

/// What one `drive` call produced.
#[derive(Default)]
pub struct Batch {
    pub updates: Vec<TabsUpdate>,
    pub icons: Vec<IconPng>,
}

/// Listening socket and its connections.
pub struct Socket {
    path: PathBuf,
    listener: UnixListener,
    conns: Vec<Conn>,
    next_id: u32,
    req: u64,                         // Last request id sent
    list_req: Option<u64>,            // Some while the popup is open
    asked: Vec<SourceId>,             // Hosts whose list has not come
    pending: Option<(u64, SourceId)>, // Activation awaiting its reply
}

impl Socket {
    /// Listens on `dir/tabs.sock`. `None` means tabs stay off.
    #[must_use]
    pub fn bind(dir: &Path) -> Option<Self> {
        let (path, listener) = bind::listen(dir)?;

        Some(Self {
            path,
            listener,
            conns: Vec::new(),
            next_id: 1,
            req: 0,
            list_req: None,
            asked: Vec::new(),
            pending: None,
        })
    }

    /// Descriptors to poll, and whether each one wants a write.
    #[must_use]
    pub fn fds(&self) -> Vec<(BorrowedFd<'_>, bool)> {
        std::iter::once((self.listener.as_fd(), false))
            .chain(
                self.conns
                    .iter()
                    .map(|c| (c.stream().as_fd(), c.wants_write())),
            )
            .collect()
    }

    /// Popup opened: asks every host that has said hello.
    pub fn list(&mut self) {
        let req = self.next_req();
        let frame = msg::list(req);
        let mut asked = 0;

        self.list_req = Some(req);
        self.asked.clear();
        for conn in self.conns.iter_mut().filter(|c| c.pid.is_some()) {
            conn.send(&frame);
            self.asked.push(conn.id);
            asked += 1;
        }
        if asked == 0 {
            log::debug!("browser tabs: popup open, no browser connected");
        } else {
            log::debug!("browser tabs: list {req} sent to {asked} browser(s)");
        }
    }

    /// Returns true while a host asked by `list` has not answered.
    #[must_use]
    pub fn listing(&self) -> bool {
        !self.asked.is_empty()
    }

    /// Popup closed. Late `tabs` replies are dropped.
    pub fn hide(&mut self) {
        self.list_req = None;
        self.asked.clear();
        log::debug!("browser tabs: popup closed, idle");
    }

    /// Asks the browser of `tab` to switch to it.
    pub fn activate(&mut self, tab: &TabRef) {
        let req = self.next_req();
        let Some(conn) = self
            .conns
            .iter_mut()
            .find(|c| c.id == tab.source && c.pid.is_some() && c.open())
        else {
            log::debug!(
                "browser tabs: cannot activate, source {} is gone",
                tab.source.0
            );
            return;
        };

        log::debug!("browser tabs: activate {req} sent to {}", conn.label());
        conn.send(&msg::activate(req, tab.id));
        self.pending = Some((req, tab.source));
    }

    /// Accepts, reads and writes without blocking. Closed hosts come
    /// back as `TabsUpdate::Gone`.
    pub fn drive(&mut self) -> Batch {
        let mut batch = Batch::default();

        self.accept();
        for i in 0..self.conns.len() {
            for frame in self.conns[i].read() {
                self.on_frame(i, &frame, &mut batch);
            }
            self.conns[i].flush();
        }
        if !batch.icons.is_empty() {
            batch.updates.push(TabsUpdate::Icons);
        }
        self.reap(&mut batch);
        for update in &batch.updates {
            if let TabsUpdate::List { source, .. } | TabsUpdate::Gone(source) =
                update
            {
                self.asked.retain(|asked| asked != source);
            }
        }
        batch
    }

    fn next_req(&mut self) -> u64 {
        self.req += 1;
        self.req
    }

    fn accept(&mut self) {
        loop {
            match self.listener.accept() {
                Ok((stream, _)) => self.take(stream),
                Err(e) if e.kind() == ErrorKind::WouldBlock => return,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => {
                    log::warn!("browser tabs: accept: {e}");
                    return;
                }
            }
        }
    }

    fn take(&mut self, stream: UnixStream) {
        if self.conns.len() >= MAX_CONN {
            log::warn!(
                "browser tabs: {MAX_CONN} hosts connected, refusing one"
            );
            return; // Dropping the stream closes it
        }
        let id = SourceId(self.next_id);

        self.next_id = self.next_id.checked_add(1).unwrap_or(1);
        log::debug!("browser tabs: host accepted as source {}", id.0);
        self.conns.push(Conn::new(id, stream));
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[path = "recv.rs"]
mod recv;

#[cfg(test)]
#[path = "socket_tests.rs"]
mod tests;
