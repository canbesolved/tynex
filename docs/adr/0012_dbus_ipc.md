---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# D-Bus for IPC on Linux

## Context and Problem Statement

The CLI (`tynex show`, `tynex quit`) must reach the daemon (see
[ADR-0004](0004_daemon_cli.md)). Which transport on Linux?

## Decision Drivers

- No extra dependency beyond what the KDE adapter needs.
- Single-instance detection.
- Replaceable on macOS and Windows.

## Considered Options

- D-Bus session bus
- Unix socket in `$XDG_RUNTIME_DIR`

## Decision Outcome

Chosen option: "D-Bus session bus", because the KDE adapter already
talks D-Bus to KWin, and owning the bus name
`io.github.canbesolved.Tynex` gives single-instance detection for
free. The D-Bus library is chosen in [ADR-0027](0027_dbus_crate.md).

### Consequences

- Good, because there is one IPC stack for the adapter and the CLI.
- Good, because other tools can call `tynex` with `busctl` or `qdbus`.
- Bad, because macOS and Windows need another transport behind the
  `IpcServer` / `IpcClient` traits.

## Pros and Cons of the Options

### Unix socket in `$XDG_RUNTIME_DIR`

- Good, because it needs only the standard library.
- Bad, because single-instance detection and stale socket cleanup must
  be written by hand, and the KDE adapter needs D-Bus anyway.

## More Information

See [ipc.md](../arch/ipc.md).
