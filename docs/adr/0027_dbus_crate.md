---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# D-Bus with the dbus crate

## Context and Problem Statement

The KDE adapter talks to KWin over D-Bus, and
[ADR-0012](0012_dbus_ipc.md) chose D-Bus for IPC on Linux. The library
must fit the single-thread poll loop
([ADR-0026](0026_poll_loop.md)). Which D-Bus library should `tynex`
use?

## Decision Drivers

- Fewest crates and threads.
- A file descriptor that joins the main poll loop.
- Deferred replies for the KWin `Poll` call.
- Mature and maintained.

## Considered Options

- `dbus` (bindings to the system `libdbus`)
- `zbus` (pure Rust, async-first)
- `rustbus` (pure Rust, sync)

## Decision Outcome

Chosen option: "`dbus`", because it needs 3 crates, wraps the
battle-tested `libdbus` present on every Linux desktop, and its
`Channel` exposes a watch fd, non-blocking `read_write` and `send`
at any time. A stored method call can be answered later with
`method_return()`. It is actively released (0.9.12, 2026-07-03).

### Consequences

- Good, because the tree avoids the 66 crates of `zbus`.
- Good, because D-Bus runs in the main poll loop without a thread.
- Bad, because building needs `libdbus` headers (`dbus-devel`).
- Neutral, because message encoding is slower than `zvariant` in
  benchmarks; `tynex` sends few, small messages.

## Pros and Cons of the Options

### `zbus`

- Good, because it is pure Rust with good documentation.
- Bad, because it pulls 66 crates; the dependency reduction issue
  ([#1649](https://github.com/z-galaxy/zbus/issues/1649)) was closed
  as not planned on 2026-09-04.
- Bad, because its `async-io` backend spawns a reactor thread and its
  blocking API is built on async.

### `rustbus`

- Good, because it is pure Rust and synchronous.
- Bad, because it has had no release since 2023-08.

## More Information

See [ipc.md](../arch/ipc.md) and [kde.md](../arch/adapters/kde.md).
