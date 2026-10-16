---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Resident daemon with CLI client

## Context and Problem Statement

The popup must appear within one frame after the hotkey. Starting a
process, connecting to the compositor and loading fonts and icons on
every press takes too long.

## Decision Drivers

- Show latency under 16 ms.
- Zero CPU while idle.
- Any desktop can trigger it with a plain command.

## Considered Options

- Resident daemon plus thin CLI client, one binary
- Resident daemon and CLI client as two binaries
- New process per hotkey press

## Decision Outcome

Chosen option: "Resident daemon plus thin CLI client, one binary",
because connections, fonts, glyphs and icons stay warm, while the idle
daemon is blocked on its event loop and uses no CPU.

Daemon and client are the same `tynex` binary. `tynex daemon` starts
the resident daemon. Every other command (`show`, `hide`, `quit`) runs
as the client: it sends one IPC call to the daemon and exits, without
touching Wayland, fonts or the config.

### Consequences

- Good, because show is a cached redraw, not a cold start.
- Good, because `tynex show` works as a hotkey command anywhere.
- Good, because one binary is built, installed and packaged, and
  daemon and client can never be out of sync.
- Bad, because a few MB of RAM stay resident.
- Bad, because it needs IPC (see [ADR-0012](0012_dbus_ipc.md)) and
  autostart.

### Confirmation

Measure RSS with `ps -o rss` (target under 10 MB) and idle wakeups with
`powertop`.
