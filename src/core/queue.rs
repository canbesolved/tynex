// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Single-threaded event queue between sources and the core.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::core::event::Event;

/// Sending end, given to adapters and IPC servers.
#[derive(Debug, Clone)]
pub struct EventTx(Rc<RefCell<VecDeque<Event>>>);

/// Receiving end, drained by the event loop.
#[derive(Debug)]
pub struct EventRx(Rc<RefCell<VecDeque<Event>>>);

impl EventTx {
    /// Queues `event` for the core.
    pub fn send(&self, event: Event) {
        self.0.borrow_mut().push_back(event);
    }
}

impl EventRx {
    /// Takes the oldest queued event, if any.
    #[must_use]
    pub fn recv(&self) -> Option<Event> {
        self.0.borrow_mut().pop_front()
    }
}

/// Creates a connected sender and receiver.
#[must_use]
pub fn channel() -> (EventTx, EventRx) {
    let queue = Rc::new(RefCell::new(VecDeque::new()));

    (EventTx(Rc::clone(&queue)), EventRx(queue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_arrive_in_order() {
        let (tx, rx) = channel();

        tx.send(Event::Hide);
        tx.clone().send(Event::Quit);
        assert_eq!(rx.recv(), Some(Event::Hide));
        assert_eq!(rx.recv(), Some(Event::Quit));
        assert_eq!(rx.recv(), None);
    }
}
