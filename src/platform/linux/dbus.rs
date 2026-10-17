// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The daemon's single session bus connection. Modules register objects
//! and send calls here; none opens its own connection.

use std::cell::RefCell;
use std::ffi::CString;
use std::fmt;
use std::os::fd::BorrowedFd;
use std::rc::Rc;
use std::time::Duration;

use dbus::Message;
use dbus::channel::{BusType, Channel};
use dbus::message::MessageType;
use dbus::strings::ErrorName;

/// Handles a method call; `Err` becomes a D-Bus error reply.
type Handler = Box<dyn FnMut(&Message) -> Result<(), String>>;

/// Receives the reply or error to a call sent with `Bus::call`.
type ReplyFn = Box<dyn FnOnce(Result<&Message, String>)>;

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

/// Non-blocking session bus connection with object handlers. Clones
/// share the connection.
#[derive(Clone)]
pub struct Bus(Rc<Inner>);

struct Inner {
    channel: Channel,
    objects: RefCell<Vec<(String, Handler)>>, // Object path, handler
    pending: RefCell<Vec<(u32, ReplyFn)>>,    // Call serial, callback
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
        Ok(Self(Rc::new(Inner {
            channel,
            objects: RefCell::default(),
            pending: RefCell::default(),
        })))
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

        let reply = self.call_blocking(
            bus_call("RequestName")?.append2(name, DO_NOT_QUEUE),
        )?;

        match reply.read1::<u32>().map_err(failed)? {
            PRIMARY_OWNER | ALREADY_OWNER => Ok(()),
            _ => Err(BusError::AlreadyRunning),
        }
    }

    /// Returns the unique connection name that owns `name`.
    ///
    /// # Errors
    ///
    /// Fails when no connection owns `name`.
    pub fn name_owner(&self, name: &str) -> Result<String, BusError> {
        let reply =
            self.call_blocking(bus_call("GetNameOwner")?.append1(name))?;

        reply.read1::<&str>().map(str::to_owned).map_err(failed)
    }

    /// Returns the connection fd for the poll loop.
    #[must_use]
    pub fn fd(&self) -> BorrowedFd<'_> {
        // SAFETY: the fd belongs to the channel, which outlives the borrow.
        unsafe { BorrowedFd::borrow_raw(self.0.channel.watch().fd) }
    }

    /// Calls `handler` for method calls to object `path`. Must not be
    /// called from a handler.
    pub fn register(
        &self,
        path: &str,
        handler: impl FnMut(&Message) -> Result<(), String> + 'static,
    ) {
        self.0
            .objects
            .borrow_mut()
            .push((path.to_owned(), Box::new(handler)));
    }

    /// Sends `msg` without waiting; `on_reply` gets the reply or error
    /// in a later `dispatch`.
    ///
    /// # Errors
    ///
    /// Fails when the message cannot be queued.
    pub fn call(
        &self,
        msg: Message,
        on_reply: impl FnOnce(Result<&Message, String>) + 'static,
    ) -> Result<(), BusError> {
        let serial = self
            .0
            .channel
            .send(msg)
            .map_err(|()| failed("cannot send a message"))?;

        self.0
            .pending
            .borrow_mut()
            .push((serial, Box::new(on_reply)));
        Ok(())
    }

    /// Sends `msg` and waits up to 2 s for the reply. For startup and
    /// shutdown only, as it blocks the loop.
    ///
    /// # Errors
    ///
    /// Fails on an error reply or a timeout.
    pub fn call_blocking(&self, msg: Message) -> Result<Message, BusError> {
        self.0
            .channel
            .send_with_reply_and_block(msg, Duration::from_secs(2))
            .map_err(failed)
    }

    /// Writes queued outgoing messages, such as calls from `call`.
    pub fn flush(&self) {
        self.0.channel.flush();
    }

    /// Reads available messages, runs handlers and callbacks, and sends
    /// the handlers' replies.
    ///
    /// # Errors
    ///
    /// Fails when the connection to the bus is lost.
    pub fn dispatch(&self) -> Result<(), BusError> {
        let channel = &self.0.channel;

        channel
            .read_write(Some(Duration::ZERO))
            .map_err(|()| failed("connection to the session bus lost"))?;

        while let Some(msg) = channel.pop_message() {
            match msg.msg_type() {
                MessageType::MethodCall => self.handle(&msg),
                MessageType::MethodReturn | MessageType::Error => {
                    self.answer(msg);
                }
                MessageType::Signal => {} // Such as NameAcquired
            }
        }
        channel.flush();
        Ok(())
    }

    /// Runs the handler for a method call and sends its reply.
    fn handle(&self, msg: &Message) {
        let path = msg.path().map(|p| p.to_string()).unwrap_or_default();
        let mut objects = self.0.objects.borrow_mut();
        let result = match objects.iter_mut().find(|(p, _)| *p == path) {
            Some((_, handler)) => handler(msg),
            None => Err(format!("no object at {path}")),
        };

        if !msg.get_no_reply() {
            let _ = self.0.channel.send(reply(msg, result));
        }
    }

    /// Passes a reply to the callback of the call it answers.
    fn answer(&self, mut msg: Message) {
        let serial = msg.get_reply_serial();
        let mut pending = self.0.pending.borrow_mut();
        let Some(i) = pending.iter().position(|(s, _)| Some(*s) == serial)
        else {
            return; // Not a call made with `call`
        };
        let (_, on_reply) = pending.swap_remove(i);

        drop(pending); // The callback may send another call
        on_reply(msg.as_result().map(|m| &*m).map_err(|e| e.to_string()));
    }
}

/// Builds a call to the bus daemon itself.
fn bus_call(member: &str) -> Result<Message, BusError> {
    Message::new_method_call(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
        member,
    )
    .map_err(failed)
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
