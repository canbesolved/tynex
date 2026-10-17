// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The daemon's single session bus connection. Modules register objects
//! here; none opens its own connection.

use std::ffi::CString;
use std::fmt;
use std::os::fd::BorrowedFd;
use std::time::Duration;

use dbus::Message;
use dbus::channel::{BusType, Channel};
use dbus::message::MessageType;
use dbus::strings::ErrorName;

/// Handles a method call; `Err` becomes a D-Bus error reply.
type Handler = Box<dyn FnMut(&Message) -> Result<(), String>>;

/// Error from the session bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusError {
    AlreadyRunning, // Another daemon owns the bus name
    Failed(String),
}

impl fmt::Display for BusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRunning => {
                f.write_str("tynex daemon is already running")
            }
            Self::Failed(why) => write!(f, "D-Bus: {why}"),
        }
    }
}

impl std::error::Error for BusError {}

fn failed(e: impl fmt::Display) -> BusError {
    BusError::Failed(e.to_string())
}

/// Non-blocking session bus connection with object handlers.
pub struct Bus {
    channel: Channel,
    objects: Vec<(String, Handler)>, // Object path, handler
}

impl Bus {
    /// Connects to the session bus.
    ///
    /// # Errors
    ///
    /// Fails without a session bus.
    pub fn session() -> Result<Self, BusError> {
        let mut channel =
            Channel::get_private(BusType::Session).map_err(failed)?;

        channel.set_watch_enabled(true);
        Ok(Self {
            channel,
            objects: Vec::new(),
        })
    }

    /// Takes the well-known `name`, failing if another process has it.
    ///
    /// # Errors
    ///
    /// `AlreadyRunning` when the name is taken.
    pub fn own_name(&self, name: &str) -> Result<(), BusError> {
        const DO_NOT_QUEUE: u32 = 4;
        const PRIMARY_OWNER: u32 = 1;
        const ALREADY_OWNER: u32 = 4;

        let request = Message::new_method_call(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
            "RequestName",
        )
        .map_err(failed)?
        .append2(name, DO_NOT_QUEUE);
        let reply = self
            .channel
            .send_with_reply_and_block(request, Duration::from_secs(5))
            .map_err(failed)?;

        match reply.read1::<u32>().map_err(failed)? {
            PRIMARY_OWNER | ALREADY_OWNER => Ok(()),
            _ => Err(BusError::AlreadyRunning),
        }
    }

    /// Returns the connection fd for the poll loop.
    #[must_use]
    pub fn fd(&self) -> BorrowedFd<'_> {
        // SAFETY: the fd belongs to the channel, which outlives the borrow.
        unsafe { BorrowedFd::borrow_raw(self.channel.watch().fd) }
    }

    /// Calls `handler` for method calls to object `path`.
    pub fn register(
        &mut self,
        path: &str,
        handler: impl FnMut(&Message) -> Result<(), String> + 'static,
    ) {
        self.objects.push((path.to_owned(), Box::new(handler)));
    }

    /// Reads available messages, runs handlers and sends their replies.
    ///
    /// # Errors
    ///
    /// Fails when the connection to the bus is lost.
    pub fn dispatch(&mut self) -> Result<(), BusError> {
        self.channel
            .read_write(Some(Duration::ZERO))
            .map_err(|()| failed("connection to the session bus lost"))?;

        while let Some(msg) = self.channel.pop_message() {
            if msg.msg_type() != MessageType::MethodCall {
                continue; // Signals such as NameAcquired
            }

            let path = msg.path().map(|p| p.to_string()).unwrap_or_default();
            let result = self
                .objects
                .iter_mut()
                .find(|(p, _)| *p == path)
                .map_or_else(
                    || Err(format!("no object at {path}")),
                    |(_, h)| h(&msg),
                );

            if !msg.get_no_reply() {
                let _ = self.channel.send(reply(&msg, result));
            }
        }
        self.channel.flush();
        Ok(())
    }
}

/// Builds the method return or error reply for `msg`.
fn reply(msg: &Message, result: Result<(), String>) -> Message {
    match result {
        Ok(()) => msg.method_return(),
        Err(why) => {
            let name = ErrorName::new("io.github.canbesolved.Tynex.Error")
                .expect("valid error name");
            let text = CString::new(why).unwrap_or_default();

            msg.error(&name, &text)
        }
    }
}
