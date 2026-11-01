// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Supported release channels and their folders.

use crate::config::Browser;

/// System-wide manifest folders, under `root`.
const BRAVE_SYSTEM: &str = "etc/opt/brave/native-messaging-hosts";
const CHROME_SYSTEM: &str = "etc/opt/chrome/native-messaging-hosts";
const CHROMIUM_SYSTEM: &str = "etc/chromium/native-messaging-hosts";

/// One release channel of a supported browser.
pub(super) struct Spec {
    pub browser: Browser,
    pub name: &'static str,   // For logs
    pub folder: &'static str, // Under `~/.config/`
    pub system: &'static str, // Under `root`
}

/// State of a selected channel.
pub(super) enum Channel {
    Missing, // No `Local State`
    System,  // Installed; a system-wide manifest serves it
    User,    // Installed; needs the per-user manifest
}

pub(super) const SPECS: &[Spec] = &[
    Spec {
        browser: Browser::Brave,
        name: "Brave",
        folder: "BraveSoftware/Brave-Browser",
        system: BRAVE_SYSTEM,
    },
    Spec {
        browser: Browser::Brave,
        name: "Brave Beta",
        folder: "BraveSoftware/Brave-Browser-Beta",
        system: BRAVE_SYSTEM,
    },
    Spec {
        browser: Browser::Brave,
        name: "Brave Nightly",
        folder: "BraveSoftware/Brave-Browser-Nightly",
        system: BRAVE_SYSTEM,
    },
    Spec {
        browser: Browser::Brave,
        name: "Brave Origin",
        folder: "BraveSoftware/Brave-Origin",
        system: BRAVE_SYSTEM,
    },
    Spec {
        browser: Browser::Chrome,
        name: "Chrome",
        folder: "google-chrome",
        system: CHROME_SYSTEM,
    },
    Spec {
        browser: Browser::Chrome,
        name: "Chrome Beta",
        folder: "google-chrome-beta",
        system: CHROME_SYSTEM,
    },
    Spec {
        browser: Browser::Chrome,
        name: "Chrome Dev",
        folder: "google-chrome-unstable",
        system: CHROME_SYSTEM,
    },
    Spec {
        browser: Browser::Chromium,
        name: "Chromium",
        folder: "chromium",
        system: CHROMIUM_SYSTEM,
    },
];
