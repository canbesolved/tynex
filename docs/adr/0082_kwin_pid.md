---
status: "accepted"
date: "2026-11-01"
decision-makers: "CanBeSolved"
---
# KWin window pid

## Context and Problem Statement

The browser extension switches to the chosen tab. On KDE Plasma
Wayland that focus request often only flashes the taskbar, so
`tynex` raises the window itself through KWin. KWin's window id is
not the browser's window id. Which KWin window shows the tab?

## Decision Drivers

- The extension has already switched the tab before the raise.
- A gone tab must not bring the browser forward.
- Several browsers, windows and profiles can be open at once.
- The payload stays a delimited string
  ([ADR-0011](0011_kwin_payload.md)).

## Considered Options

- Match the KWin window by process id, then by title
- Match by title only
- Match by app class and title

## Decision Outcome

Chosen option: "Match the KWin window by process id, then by
title", because the host already knows the browser's pid, and the
title separates that browser's windows. The extension switches the
tab; KWin only raises the window, and only after `activated` says
`ok`.

This amends [ADR-0011](0011_kwin_payload.md). The script adds
`w.pid` as a fifth field. `pid` `0` or not a number is `None`.

Title-only matching can raise a different app that shows the same
title. Class-and-title matching needs a table from browser name to
window class, and those names differ by build. Both were turned
down.

### Consequences

- Good, because only the connected browser's windows are candidates.
- Good, because a failed activation raises nothing.
- Bad, because the payload format changes, so an old script and a
  new daemon do not parse each other until both are reloaded.

### Confirmation

`parse.rs` tests cover five fields, a bad pid and a zero pid.
`browser_window` tests cover each pick branch.

## Pros and Cons of the Options

### Match the KWin window by process id, then by title

- Good, because the pid comes from the process that connected.
- Bad, because the KWin script and the parser both change.

### Match by title only

- Good, because the payload stays at four fields.
- Bad, because another app with the same title can be raised.

### Match by app class and title

- Good, because it needs no new field.
- Bad, because class names differ between distributions and builds.

## More Information

Amends [ADR-0011](0011_kwin_payload.md). The bridge is
[ADR-0081](0081_tab_bridge.md).
