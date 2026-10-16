// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Window source adapters and the registry that selects one.

pub mod mock;

use crate::adapters::mock::MockSource;
use crate::config::Desktop;
use crate::core::queue::EventTx;
use crate::core::traits::{SourceError, WindowSource};
use crate::core::window::WindowId;

/// The selected window source: one variant per adapter, static dispatch.
#[derive(Debug)]
pub enum Source {
    Mock(MockSource),
}

impl WindowSource for Source {
    fn start(&mut self, tx: EventTx) -> Result<(), SourceError> {
        match self {
            Self::Mock(s) => s.start(tx),
        }
    }

    fn activate(&mut self, id: &WindowId) -> Result<(), SourceError> {
        match self {
            Self::Mock(s) => s.activate(id),
        }
    }
}

/// Creates the adapter for `desktop`; only the selected one is built.
///
/// `current_desktop` is `XDG_CURRENT_DESKTOP`, read by the caller.
/// `Desktop::Auto` never selects the mock adapter.
///
/// # Errors
///
/// Returns an error when no adapter supports the desktop.
pub fn select(
    desktop: Desktop,
    current_desktop: Option<&str>,
) -> Result<Source, SourceError> {
    match desktop {
        Desktop::Mock => Ok(Source::Mock(MockSource::new())),
        Desktop::Kde | Desktop::Auto => Err(SourceError(format!(
            "no window source for desktop `{}` yet; set `desktop = \"mock\"` \
             to try tynex",
            current_desktop.unwrap_or("unknown")
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_only_when_asked() {
        assert!(matches!(select(Desktop::Mock, None), Ok(Source::Mock(_))));
        assert!(select(Desktop::Auto, Some("KDE")).is_err());
    }
}
