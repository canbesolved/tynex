---
status: "accepted, amended by ADR-0036"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Single-threaded daemon

## Context and Problem Statement

The daemon reacts to Wayland events, key presses and D-Bus calls. How
many threads should it run, and does it need an async runtime?

## Decision Drivers

- No wake-ups while idle.
- No locks or channels around core state.
- No async runtime overhead on the hot path.

## Considered Options

- One thread, no async runtime
- Main thread plus a D-Bus thread
- `tokio` multi-threaded runtime

## Decision Outcome

Chosen option: "One thread, no async runtime", because Wayland and
D-Bus both expose file descriptors that one event loop can wait on.
The loop itself is chosen in [ADR-0026](0026_poll_loop.md).

### Consequences

- Good, because core state lives on one thread with no locks.
- Good, because the process sleeps in the kernel while idle.
- Bad, because every library must offer a non-blocking, fd-based API;
  this rules out async-first crates.

### Confirmation

`powertop` shows no wake-ups from `tynex` while hidden, except the KDE
poll every 20 s (see [ADR-0010](0010_kwin_bridge.md)).

## Pros and Cons of the Options

### Main thread plus a D-Bus thread

- Good, because a blocking D-Bus API can be used.
- Bad, because events cross threads through a channel.

### `tokio` multi-threaded runtime

- Good, because it is the most common Rust runtime.
- Bad, because worker threads and the runtime are pure overhead for
  two file descriptors.
