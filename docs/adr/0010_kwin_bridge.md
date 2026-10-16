---
status: "superseded by ADR-0034"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# KWin script bridge for KDE

## Context and Problem Statement

On Plasma 6 Wayland, clients cannot list or activate other windows, and
KWin has no public D-Bus API for it. How does `tynex` get the window
list and switch windows?

## Decision Drivers

- Full window list in MRU order, kept up to date.
- Activation latency of a few ms.
- No temp files or script loads per switch.

## Considered Options

- Persistent KWin script with push updates and a pending `Poll` call
- Load a one-shot KWin script for every activation
- `plasma-window-management` Wayland protocol

## Decision Outcome

Chosen option: "Persistent KWin script with push updates and a pending
`Poll` call", because the list is pushed on change and cached, and the
daemon activates a window by answering a call the script already made.

The script also registers the global shortcut (`Meta+Tab` by default),
so no global key grab is needed.

### Consequences

- Good, because show needs no round trip and activation takes ~1 ms.
- Good, because the shortcut is rebindable in System Settings.
- Bad, because the daemon answers the `Poll` with a no-op every 20 s
  to stay under the D-Bus timeout.
- Bad, because KWin scripting API changes can break the adapter.

## Pros and Cons of the Options

### One-shot KWin script per activation

- Good, because it is simple and stateless.
- Bad, because each switch writes and loads a script, tens of ms.

### `plasma-window-management` protocol

- Good, because it is a native Wayland protocol.
- Bad, because it is restricted to privileged Plasma clients.

## More Information

See [kde.md](../arch/adapters/kde.md).
