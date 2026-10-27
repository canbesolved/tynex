// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! IPC over D-Bus: the daemon's object and the CLI client.

use std::time::Duration;

use dbus::Message;
use dbus::channel::{BusType, Channel};

use crate::core::event::Event;
use crate::core::queue::EventTx;
use crate::ipc::{IpcError, Request};
use crate::platform::linux::dbus::Bus;

/// Bus name owned by the daemon; also the interface name.
pub const NAME: &str = "io.github.canbesolved.Tynex";

/// Object path of the IPC object.
pub const PATH: &str = "/io/github/canbesolved/Tynex";

/// Registers the IPC object; requests become events on `tx`.
pub fn serve(bus: &Bus, tx: EventTx) {
    bus.register(PATH, move |msg| {
        let event = request(msg)?;

        tx.send(event);
        Ok(())
    });
}

/// Turns a method call into an event. `Show` keeps its string argument
/// from when it named a mode; only the empty string is accepted.
fn request(msg: &Message) -> Result<Event, String> {
    if msg.interface().as_deref() != Some(NAME) {
        return Err("unknown interface".into());
    }

    match msg.member().as_deref() {
        Some("Show") => match msg.read1::<&str>().unwrap_or("") {
            "" => Ok(Event::Show),
            other => Err(format!("unknown mode `{other}`: modes were removed")),
        },
        Some("Hide") => Ok(Event::Hide),
        Some("Quit") => Ok(Event::Quit),
        _ => Err("unknown method".into()),
    }
}

/// Sends `req` to the running daemon and waits for its reply.
///
/// # Errors
///
/// `NotRunning` when no daemon owns the bus name.
pub fn send(req: Request) -> Result<(), IpcError> {
    let failed = |e: &dyn std::fmt::Display| IpcError::Failed(e.to_string());
    let channel =
        Channel::get_private(BusType::Session).map_err(|e| failed(&e))?;
    let member = match req {
        Request::Show => "Show",
        Request::Hide => "Hide",
        Request::Quit => "Quit",
    };
    let mut msg = Message::new_method_call(NAME, PATH, NAME, member)
        .map_err(|e| failed(&e))?;

    if member == "Show" {
        msg = msg.append1("");
    }

    match channel.send_with_reply_and_block(msg, Duration::from_secs(2)) {
        Ok(_) => Ok(()),
        Err(e) if is_not_running(e.name()) => Err(IpcError::NotRunning),
        Err(e) => Err(failed(&e)),
    }
}

fn is_not_running(name: Option<&str>) -> bool {
    matches!(
        name,
        Some(
            "org.freedesktop.DBus.Error.ServiceUnknown"
                | "org.freedesktop.DBus.Error.NameHasNoOwner"
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(member: &str, arg: Option<&str>) -> Message {
        let msg = Message::new_method_call(NAME, PATH, NAME, member).unwrap();

        match arg {
            Some(a) => msg.append1(a),
            None => msg,
        }
    }

    #[test]
    fn show_accepts_only_an_empty_mode() {
        let show = |arg| request(&call("Show", arg));

        assert_eq!(show(Some("")), Ok(Event::Show));
        assert_eq!(show(None), Ok(Event::Show));
        assert!(show(Some("hotkey")).is_err());
        assert!(show(Some("search")).is_err());
    }

    #[test]
    fn hide_quit_and_unknown_methods() {
        let get = |m| request(&call(m, None));

        assert_eq!(get("Hide"), Ok(Event::Hide));
        assert_eq!(get("Quit"), Ok(Event::Quit));
        assert!(get("Reload").is_err());

        let other =
            Message::new_method_call(NAME, PATH, "org.example.Other", "Hide")
                .unwrap();

        assert!(request(&other).is_err());
    }

    #[test]
    fn missing_daemon_errors_are_recognized() {
        assert!(is_not_running(Some(
            "org.freedesktop.DBus.Error.ServiceUnknown"
        )));
        assert!(!is_not_running(Some("org.freedesktop.DBus.Error.NoReply")));
        assert!(!is_not_running(None));
    }
}
