---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBeSolved"
---
# Shared D-Bus connection

## Context and Problem Statement

D-Bus is used by the IPC server, the KDE adapter, and later by other
desktop adapters and portals. Each opening its own connection would
add file descriptors to the poll loop and duplicate the dispatch and
call code. Where should D-Bus code live?

## Decision Drivers

- No duplicated D-Bus code across modules.
- One file descriptor in the poll loop.
- Desktop-specific code stays in its adapter.

## Considered Options

- One shared connection in `platform/linux/dbus.rs`, users register
  objects and make calls through it
- One connection per module

## Decision Outcome

Chosen option: "One shared connection in `platform/linux/dbus.rs`",
because connection setup, bus name ownership, dispatch and
non-blocking calls are written once, and modules keep only their own
objects and calls.

- `dbus.rs` owns the session bus connection and the bus name
  `io.github.canbesolved.Tynex`.
- Modules register an object path with a handler, and send calls with
  an optional reply callback that runs in the main loop.
- Desktop-specific logic (KWin calls, payload parsing) stays in
  `src/adapters/<desktop>/`.

### Consequences

- Good, because adding a D-Bus based adapter means registering one
  object and writing its calls.
- Good, because the poll loop watches one D-Bus fd.
- Neutral, because the CLI client still opens its own short-lived
  connection, as it runs in a separate process.

### Confirmation

Code review: only `dbus.rs` creates a daemon connection.

## More Information

See [linux.md](../arch/platforms/linux.md#d-bus).
