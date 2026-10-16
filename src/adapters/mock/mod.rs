// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! In-memory window source for tests and manual runs. No I/O.

mod fixtures;

use crate::core::event::Event;
use crate::core::queue::EventTx;
use crate::core::traits::{SourceError, WindowSource};
use crate::core::window::{WindowId, WindowInfo};

/// Window source with a fixed, in-memory window list.
#[derive(Debug, Default)]
pub struct MockSource {
    windows: Vec<WindowInfo>, // Most recently used first
    activations: Vec<WindowId>,
    tx: Option<EventTx>,
}

impl MockSource {
    /// Creates a source with the built-in fixtures.
    #[must_use]
    pub fn new() -> Self {
        Self::with_windows(fixtures::windows())
    }

    /// Creates a source with the given windows, in MRU order.
    #[must_use]
    pub fn with_windows(windows: Vec<WindowInfo>) -> Self {
        Self {
            windows,
            ..Self::default()
        }
    }

    /// Returns ids passed to `activate`, oldest first.
    #[must_use]
    pub fn activations(&self) -> &[WindowId] {
        &self.activations
    }

    fn send(&self) {
        if let Some(tx) = &self.tx {
            tx.send(Event::Windows(self.windows.clone()));
        }
    }
}

impl WindowSource for MockSource {
    fn start(&mut self, tx: EventTx) -> Result<(), SourceError> {
        self.tx = Some(tx);
        self.send();
        Ok(())
    }

    fn activate(&mut self, id: &WindowId) -> Result<(), SourceError> {
        let pos = self
            .windows
            .iter()
            .position(|w| &w.id == id)
            .ok_or_else(|| SourceError(format!("unknown window `{}`", id.0)))?;

        // Like a real desktop: the activated window becomes most recent.
        let window = self.windows.remove(pos);

        self.windows.insert(0, window);
        self.activations.push(id.clone());
        self.send();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::queue::channel;

    fn ids(event: Option<Event>) -> Vec<String> {
        match event {
            Some(Event::Windows(list)) => {
                list.into_iter().map(|w| w.id.0).collect()
            }
            other => panic!("expected a window list, got {other:?}"),
        }
    }

    #[test]
    fn start_sends_fixtures() {
        let (tx, rx) = channel();
        let mut source = MockSource::new();

        source.start(tx).unwrap();
        assert_eq!(ids(rx.recv()).len(), 10);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn activate_moves_window_to_front_and_records_it() {
        let (tx, rx) = channel();
        let mut source = MockSource::new();

        source.start(tx).unwrap();
        assert_eq!(ids(rx.recv())[0], "mock-01");
        source.activate(&WindowId("mock-03".into())).unwrap();
        assert_eq!(ids(rx.recv())[..3], ["mock-03", "mock-01", "mock-02"]);
        assert_eq!(source.activations(), [WindowId("mock-03".into())]);
    }

    #[test]
    fn unknown_id_is_an_error() {
        let mut source = MockSource::with_windows(Vec::new());
        let err = source.activate(&WindowId("nope".into())).unwrap_err();

        assert_eq!(err.to_string(), "unknown window `nope`");
        assert_eq!(source.activations(), &[] as &[WindowId]);
    }
}
