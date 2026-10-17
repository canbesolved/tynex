// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Window source adapters and the registry that selects one.

#[cfg(target_os = "linux")]
pub mod kde;
pub mod mock;

#[cfg(target_os = "linux")]
use crate::adapters::kde::KdeSource;
use crate::adapters::mock::MockSource;
use crate::config::Desktop;
#[cfg(target_os = "linux")]
use crate::core::event::ModeKind;
use crate::core::queue::EventTx;
use crate::core::traits::{SourceError, WindowSource};
use crate::core::window::WindowId;
#[cfg(target_os = "linux")]
use crate::platform::linux::dbus::Bus;

/// Adapter picked for the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Mock,
    Kde,
}

/// The selected window source: one variant per adapter, static dispatch.
pub enum Source {
    Mock(MockSource),
    #[cfg(target_os = "linux")]
    Kde(KdeSource),
}

impl WindowSource for Source {
    fn start(&mut self, tx: EventTx) -> Result<(), SourceError> {
        match self {
            Self::Mock(s) => s.start(tx),
            #[cfg(target_os = "linux")]
            Self::Kde(s) => s.start(tx),
        }
    }

    fn activate(&mut self, id: &WindowId) -> Result<(), SourceError> {
        match self {
            Self::Mock(s) => s.activate(id),
            #[cfg(target_os = "linux")]
            Self::Kde(s) => s.activate(id),
        }
    }

    fn stop(&mut self) {
        match self {
            Self::Mock(s) => s.stop(),
            #[cfg(target_os = "linux")]
            Self::Kde(s) => s.stop(),
        }
    }
}

/// Picks the adapter for `desktop`.
///
/// `current_desktop` is `XDG_CURRENT_DESKTOP`, read by the caller.
/// `Desktop::Auto` picks KDE when the list names it, and never picks the
/// mock adapter.
///
/// # Errors
///
/// Returns an error when no adapter supports the desktop.
pub fn pick(
    desktop: Desktop,
    current_desktop: Option<&str>,
) -> Result<Kind, SourceError> {
    let is_kde = |list: &str| {
        list.split(':').any(|name| name.eq_ignore_ascii_case("KDE"))
    };

    match desktop {
        Desktop::Mock => Ok(Kind::Mock),
        Desktop::Kde => Ok(Kind::Kde),
        Desktop::Auto if current_desktop.is_some_and(is_kde) => Ok(Kind::Kde),
        Desktop::Auto => Err(SourceError(format!(
            "no window source for desktop `{}`; set `desktop = \"kde\"` on \
             KDE Plasma, or `desktop = \"mock\"` to try tynex",
            current_desktop.unwrap_or("unknown")
        ))),
    }
}

/// Creates the adapter for `desktop`; only the picked one is built.
///
/// # Errors
///
/// Returns an error when no adapter supports the desktop.
#[cfg(target_os = "linux")]
pub fn select(
    desktop: Desktop,
    current_desktop: Option<&str>,
    bus: &Bus,
    mode: ModeKind,
) -> Result<Source, SourceError> {
    Ok(match pick(desktop, current_desktop)? {
        Kind::Mock => Source::Mock(MockSource::new()),
        Kind::Kde => Source::Kde(KdeSource::new(bus.clone(), mode)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_only_when_asked() {
        assert_eq!(pick(Desktop::Mock, None), Ok(Kind::Mock));
        assert!(pick(Desktop::Auto, None).is_err());
        assert!(pick(Desktop::Auto, Some("GNOME")).is_err());
    }

    #[test]
    fn kde_when_asked_or_detected() {
        assert_eq!(pick(Desktop::Kde, Some("GNOME")), Ok(Kind::Kde));
        assert_eq!(pick(Desktop::Auto, Some("KDE")), Ok(Kind::Kde));
        assert_eq!(pick(Desktop::Auto, Some("ubuntu:KDE")), Ok(Kind::Kde));
        assert!(pick(Desktop::Auto, Some("KDEx")).is_err());
    }
}
