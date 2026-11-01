// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Incoming frames and closed connections of the tabs socket.

use super::{Batch, Conn, IconPng, Socket};
use crate::adapters::browser_tabs::base64;
use crate::adapters::browser_tabs::msg::{self, Icon, Incoming};
use crate::core::browser_tabs::{SourceId, TabInfo, TabsUpdate};

impl Socket {
    /// Handles one frame of connection `i`.
    pub(super) fn on_frame(
        &mut self,
        i: usize,
        frame: &[u8],
        batch: &mut Batch,
    ) {
        let parsed = msg::parse(frame);
        let list_req = self.list_req;
        let conn = &mut self.conns[i];

        let Some(pid) = conn.pid else {
            return hello(conn, &parsed, list_req);
        };

        match parsed {
            Ok(Incoming::Tabs { req, browser, tabs }) => {
                tabs_reply(conn, pid, (req, list_req), browser, tabs, batch);
            }
            Ok(Incoming::Icons(icons)) => icons_reply(conn, icons, batch),
            Ok(Incoming::Activated { req, ok, window }) => {
                if self.pending != Some((req, conn.id)) {
                    log::debug!(
                        "browser tabs: stale reply {req} from {} ignored",
                        conn.label()
                    );
                    return;
                }
                log::debug!(
                    "browser tabs: activate {req} {} in {}",
                    if ok { "done" } else { "failed, tab is gone" },
                    conn.label()
                );
                self.pending = None;
                batch.updates.push(TabsUpdate::Activated {
                    source: conn.id,
                    ok,
                    window,
                });
            }
            Ok(Incoming::Hello { .. } | Incoming::Skip) => {
                log::debug!(
                    "browser tabs: unknown message from {}",
                    conn.label()
                );
            }
            Err(()) if !conn.warned => {
                log::warn!("browser tabs: bad JSON from {}", conn.label());
                conn.warned = true;
            }
            Err(()) => {}
        }
    }

    /// Drops closed connections and reports each one.
    pub(super) fn reap(&mut self, batch: &mut Batch) {
        for conn in &self.conns {
            let Some(reason) = conn.closed else {
                continue;
            };

            log::info!("browser tabs: {} disconnected: {reason}", conn.label());
            batch.updates.push(TabsUpdate::Gone(conn.id));
            if self.pending.is_some_and(|(_, source)| source == conn.id) {
                self.pending = None;
            }
        }
        self.conns.retain(Conn::open);
    }
}

/// The first frame: it must be a version 1 `hello`.
fn hello(conn: &mut Conn, parsed: &Result<Incoming, ()>, list: Option<u64>) {
    let Ok(Incoming::Hello { pid }) = *parsed else {
        log::warn!("browser tabs: source {} sent no hello", conn.id.0);
        return conn.kill("no hello");
    };

    conn.pid = Some(pid);
    log::info!("browser tabs: {} connected", conn.label());
    if let Some(req) = list {
        conn.send(&msg::list(req)); // The popup is already open
    }
}

/// A `tabs` reply. `req` is `(reply, wanted)`.
fn tabs_reply(
    conn: &mut Conn,
    pid: u32,
    req: (u64, Option<u64>),
    browser: String,
    tabs: Vec<TabInfo>,
    batch: &mut Batch,
) {
    if conn.browser.as_deref() != Some(browser.as_str()) {
        conn.browser = Some(browser.clone());
    }
    if Some(req.0) != req.1 {
        log::debug!(
            "browser tabs: late tabs {} from {} ignored",
            req.0,
            conn.label()
        );
        return;
    }
    log::debug!("browser tabs: {} tabs from {}", tabs.len(), conn.label());
    batch.updates.push(TabsUpdate::List {
        source: conn.id,
        pid,
        browser,
        tabs,
    });
}

/// An `icons` message: decodes the base64 of each favicon.
fn icons_reply(conn: &Conn, icons: Vec<Icon>, batch: &mut Batch) {
    let sent = icons.len();
    let source: SourceId = conn.id;
    let before = batch.icons.len();

    batch.icons.extend(icons.into_iter().filter_map(|icon| {
        Some(IconPng {
            source,
            png: base64::decode(&icon.png)?,
            key: icon.key,
        })
    }));
    log::debug!(
        "browser tabs: {sent} favicons from {}, {} bad",
        conn.label(),
        sent - (batch.icons.len() - before)
    );
}
