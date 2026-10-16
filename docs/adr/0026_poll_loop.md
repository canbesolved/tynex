---
status: "accepted, amended by ADR-0036"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Event loop with rustix poll

## Context and Problem Statement

The daemon runs on one thread ([ADR-0009](0009_single_thread.md)).
With the `dbus` crate ([ADR-0027](0027_dbus_crate.md)) the D-Bus
connection is a plain file descriptor, like the Wayland connection.
Which event loop should wait on them?

## Decision Drivers

- Simplest possible loop.
- No wake-ups while idle.
- Fewest crates.

## Considered Options

- `rustix::event::poll` over the Wayland and D-Bus fds
- `calloop`

## Decision Outcome

Chosen option: "`rustix::event::poll`", because there are only two
file descriptors and a few deadlines (key repeat, the 20 s KDE
refresh). A plain `poll` with the nearest deadline as timeout covers
them, and `rustix` is already in the tree through `wayland-backend`.

### Consequences

- Good, because there are no threads, locks or channels.
- Good, because `calloop` and its 10 crates are not needed.
- Bad, because timers are computed by hand in the loop.

### Confirmation

`powertop` shows no wake-ups from `tynex` while hidden, except the KDE
refresh every 20 s.

## Pros and Cons of the Options

### `calloop`

- Good, because it is the loop `smithay-client-toolkit` is built for,
  with timers and channels included.
- Bad, because it adds 10 crates for features two fds do not need.

## More Information

See [linux.md](../arch/platforms/linux.md).
