# IPC

Source: `src/ipc/` (traits, commands) and `src/platform/*/` (transport).

## Purpose

Lets the short-lived CLI (`tynex show`) control the resident daemon.
Any desktop can bind the CLI to a hotkey, even without an adapter
hotkey.

## Commands

| CLI                        | D-Bus call       | Effect                    |
| -------------------------- | ---------------- | ------------------------- |
| `tynex show`               | `Show("")`       | Open in the config's mode |
| `tynex show --mode search` | `Show("search")` | Open in search mode       |
| `tynex hide`               | `Hide()`         | Close the popup           |
| `tynex quit`               | `Quit()`         | Stop the daemon           |

While the popup is open, both `show` forms act as a second hotkey
press and follow `hotkey_action` ([config.md](config.md)).

## Interface

`src/ipc/` is portable: `Request` (`Show(Option<ModeKind>)`, `Hide`,
`Quit`) and `IpcError` (`NotRunning` or `Failed`). Each platform
module provides two functions:

- `serve(bus, tx, default_mode)`: registers the daemon's object; each
  call becomes an `Event` on `tx`. Unknown methods or modes get a
  D-Bus error reply.
- `send(request)`: used by the CLI; opens a short-lived connection,
  sends one call and waits up to 2 s for the reply. A missing daemon
  (`ServiceUnknown`, `NameHasNoOwner`) is reported as `NotRunning`.

Object `/io/github/canbesolved/Tynex`, interface
`io.github.canbesolved.Tynex`.

## Transports

| Platform | Transport                                              |
| -------- | ------------------------------------------------------ |
| Linux    | D-Bus session bus, name `io.github.canbesolved.Tynex`  |
| macOS    | Unix socket (later)                                    |
| Windows  | Named pipe (later)                                     |

On Linux D-Bus is used because the KDE adapter needs it anyway, so it
adds no extra dependency. The server registers its object on the
daemon's shared connection (see
[linux.md](platforms/linux.md#d-bus)), next to the KDE adapter. A
second `tynex daemon` exits early when the bus name is already owned.
The CLI sends one method call and exits.
