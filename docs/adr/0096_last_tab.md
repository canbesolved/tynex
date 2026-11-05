---
status: "accepted"
date: "2026-11-05"
decision-makers: "CanBeSolved"
---
# Back to the last tab

## Context and Problem Statement

`hotkeys.action = "last"` switches to the previous window
(`windows[1]` in most recently used order). A browser tab is not a
window: after Kate, a GitHub tab opened from the popup and then
Cursor, the hotkey raises the browser, not that tab; after GitHub
and then LinkedIn in one browser window, it leaves the browser.
How does the switch back reach the previous tab (RM-0068)?

## Decision Drivers

- The previous place is a window or a tab, whichever came last.
- No change to the KWin script or the browser extension.
- Pure core logic, no timers.

## Considered Options

- Keep the current and the previous place, a window or a tab
- Use the browser's last-access times against window focus times

## Decision Outcome

Chosen option: "Keep the current and the previous place, a window
or a tab", because it follows what the user did, and every event it
needs already reaches the core.

`Places` in `src/core/places.rs` holds two places, newest first. A
new place moves the current one to previous:

- The window in front changes: that window, unless it belongs to
  the current tab's browser process, so raising a tab's window
  changes nothing.
- A tab is opened from the popup: that tab.
- A browser sends its tab list while its window is in front: its
  newest active tab. A tab switched in the browser is seen this way
  on the next popup. If the current place is that same window, the
  tab replaces it without a move.

A second hotkey press with `last` opens the previous place: its
window, or its tab with `ActivateTab`. A tab whose list has not
arrived yet raises a window of its browser. With no previous place,
or one that is gone, it falls back to the previous window.

### Consequences

- Good, because both stories above end on the GitHub tab.
- Good, because the same `back` serves a future `tynex qs` command.
- Bad, because only two places are kept: a third switch forgets the
  oldest.
- Bad, because a tab switched in the browser is seen only when the
  popup opens and the list arrives.

### Confirmation

Tests in `src/core/places_tests.rs` and
`src/core/switcher_usage_tests.rs`.

## Pros and Cons of the Options

### Use the browser's last-access times against window focus times

- Good, because tabs switched in the browser count at once.
- Bad, because the core has no clock and `KWin` sends no focus
  times; both would be new.
