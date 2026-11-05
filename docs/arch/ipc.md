# IPC

Source: `src/ipc/` (traits, commands) and `src/platform/*/` (transport).

## Purpose

Lets the short-lived CLI (`tynex show`) control the resident daemon.
Any desktop can bind the CLI to a hotkey, even without an adapter
hotkey.

## Commands

| CLI          | D-Bus call      | Effect                  |
| ------------ | --------------- | ----------------------- |
| `tynex show` | `Show("")`      | Open the popup          |
| `tynex hide` | `Hide()`        | Close the popup         |
| `tynex qs`   | `QuickSwitch()` | Switch back, no popup   |
| `tynex quit` | `Quit()`        | Stop the daemon         |

While the popup is open, `show` acts as a second hotkey press and
follows `hotkeys.action` ([config.md](config.md)). `Show` keeps its
string argument from when it named a mode; any value other than the
empty string is refused ([ADR-0050](../adr/0050_search_only.md)).

`qs` (quick switch) goes back to the previous window or browser tab
without opening the popup, like a second press with
`hotkeys.action = "last"`. While the popup is open it switches back
whatever `hotkeys.action` says
([ADR-0098](../adr/0098_quick_switch.md)).

## Interface

`src/ipc/` is portable: `Request` (`Show`, `Hide`, `QuickSwitch`,
`Quit`) and
`IpcError` (`NotRunning` or `Failed`). Each platform module provides
two functions:

- `serve(bus, tx)`: registers the daemon's object; each call becomes
  an `Event` on `tx`. Unknown methods or modes get a D-Bus error
  reply.
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
