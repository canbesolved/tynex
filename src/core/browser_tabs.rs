// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Browser tab identity. The module and the adapter share these types.
//! Neither this file nor its callers perform I/O.

/// Daemon-assigned id of one host connection. Never reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(pub u32);

/// One browser tab of the current popup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabInfo {
    pub id: i32,
    pub window: i32, // Browser window id
    pub active: bool,
    pub last: u64, // Last access, ms since the epoch; 0 if missing
    pub title: String, // Sanitized, like a window title
    pub url: String,
    pub icon: String, // Icon key: origin, or the scheme
}

/// What selecting a tab row activates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabRef {
    pub source: SourceId,
    pub id: i32,
}

/// Tabs of one connected browser, for the open popup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabList {
    pub source: SourceId,
    pub pid: u32,
    pub browser: String, // Display name, when no window matches
    pub icon: String,    // Fallback icon name or path
    pub tabs: Vec<TabInfo>,
}

/// A tab-list change the mode applies to the open popup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsUpdate {
    List {
        source: SourceId,
        pid: u32,
        browser: String,
        tabs: Vec<TabInfo>,
    },
    Icons,
    Gone(SourceId),
    Activated {
        source: SourceId,
        ok: bool,
        window: i32,
    },
}
