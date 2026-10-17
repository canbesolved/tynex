// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! The D-Bus object the `KWin` script calls.

use dbus::Message;

use crate::adapters::kde::parse::parse;
use crate::core::event::{Event, ModeKind};
use crate::core::queue::EventTx;
use crate::platform::linux::dbus::Bus;
use crate::platform::linux::desktop_files::DesktopFiles;

/// Object path the script calls.
pub const PATH: &str = "/io/github/canbesolved/Tynex/Kde";

/// Interface of the object.
pub const INTERFACE: &str = "io.github.canbesolved.Tynex.Kde";

/// Registers the object; calls become events on `tx`, `Show` opens in
/// `mode`. Only `kwin`, `KWin`'s unique bus name, may call it, so other
/// programs cannot inject a fake window list. Windows get app names and
/// icons from desktop files.
pub fn serve(bus: &Bus, tx: EventTx, kwin: String, mode: ModeKind) {
    let mut apps = DesktopFiles::default();

    bus.register(PATH, move |msg| {
        let mut event = request(msg, &kwin, mode)
            .inspect_err(|e| log::warn!("refused a call to {PATH}: {e}"))?;

        if let Event::Windows(list) = &mut event {
            apps.fill(list);
        }
        tx.send(event);
        Ok(())
    });
}

/// Turns a method call from `kwin` into an event.
fn request(msg: &Message, kwin: &str, mode: ModeKind) -> Result<Event, String> {
    if msg.sender().as_deref() != Some(kwin) {
        return Err("only KWin may call this object".into());
    }
    if msg.interface().as_deref() != Some(INTERFACE) {
        return Err("unknown interface".into());
    }

    match msg.member().as_deref() {
        Some("Update") => msg
            .read1::<&str>()
            .map(|list| Event::Windows(parse(list)))
            .map_err(|e| e.to_string()),
        Some("Show") => Ok(Event::Show(mode)),
        _ => Err("unknown method".into()),
    }
}

#[cfg(test)]
mod tests {
    use dbus::strings::BusName;

    use super::*;

    const KWIN: &str = ":1.7"; // Made-up unique name

    fn call(sender: &str, member: &str, arg: Option<&str>) -> Message {
        let mut msg = Message::new_method_call(
            "io.github.canbesolved.Tynex",
            PATH,
            INTERFACE,
            member,
        )
        .unwrap();

        msg.set_sender(Some(BusName::new(sender).unwrap()));
        match arg {
            Some(a) => msg.append1(a),
            None => msg,
        }
    }

    #[test]
    fn kwin_can_update_and_show() {
        let update = call(KWIN, "Update", Some(""));
        let show = call(KWIN, "Show", None);

        assert_eq!(
            request(&update, KWIN, ModeKind::Hotkey),
            Ok(Event::Windows(Vec::new()))
        );
        assert_eq!(
            request(&show, KWIN, ModeKind::Search),
            Ok(Event::Show(ModeKind::Search))
        );
    }

    #[test]
    fn other_connections_are_refused() {
        let fake = call(":1.99", "Update", Some(""));
        let mut anonymous = call(KWIN, "Show", None);

        anonymous.set_sender(None);
        assert!(request(&fake, KWIN, ModeKind::Hotkey).is_err());
        assert!(request(&anonymous, KWIN, ModeKind::Hotkey).is_err());
    }

    #[test]
    fn bad_calls_are_errors() {
        let get = |msg| request(&msg, KWIN, ModeKind::Hotkey);

        assert!(get(call(KWIN, "Update", None)).is_err()); // No list
        assert!(get(call(KWIN, "Poll", None)).is_err());
    }
}
