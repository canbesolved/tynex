---
status: "accepted"
date: "2026-11-05"
decision-makers: "CanBeSolved"
---
# Window and tab history

## Context and Problem Statement

ADR-0096 keeps the current and the previous place, a window or a
browser tab, for the hotkey pressed again. The rest of the popup
still knows only windows: the title list shows a browser window, not
its tab, the popup opens on the previous window, and the strip lists
apps only. Tabs should be "windows" like any other everywhere a
window is switched to (RM-0069). How far does the history go, and
how do the title list and the strip use it?

## Decision Drivers

- A tab and a window behave the same when switching.
- The popup never shows a tab late or in the wrong place.
- The default order of today stays when there is no history.
- No new protocol, no new crate, no timers while idle.

## Considered Options

- A bounded history of places, with the popup waiting for tab lists
- Tabs in the title list only, without a history

## Decision Outcome

Chosen option: "A bounded history of places, with the popup waiting
for tab lists", because it gives one order for every place the user
switched to.

- `Places` keeps up to `main.history` places (default 10, 2 to 50),
  newest first, in memory only. A window that closes leaves; a tab
  leaves when its browser's list lacks it or the browser disconnects.
- A browser window counts as its active tab: when a list arrives, a
  window of that browser whose title shows an active tab becomes that
  tab. Focusing a window known by its tab moves the tab to the front.
- The switch back uses the first two places (ADR-0096).
- The title list, with `sort_by = "recently"`, lists the places from
  the previous one on, windows and tabs, then the other windows; the
  current place stays last like the active window. Browser windows
  show as their active tab. `sort_by = "app"` keeps its order.
- The popup opens on the previous place: its row in the title list,
  and in the strip the icon after the pinned apps. A previous tab gets
  an icon with its favicon; the browser's own icon stays. A previous
  window's app icon moves there; a pinned app's icon is selected in
  place.
- `Enter` on a tab row or tab icon opens the tab (`ActivateTab`).
- While a list is due, the first draw waits for every asked browser,
  at most `browser_tabs.wait` ms (default 150, 0 to 1000; 0 draws at
  once). A list that arrives later rebuilds the popup; the selection
  moves to the previous place only until the user acts.

### Consequences

- Good, because GitHub after Kate and Cursor is the first title and
  the second strip icon, and `Enter` opens it.
- Good, because the wait is bounded and only happens with tab search
  on and a browser connected.
- Bad, because the popup can show up to `browser_tabs.wait` ms later
  when a browser is slow.
- Bad, because the history starts empty after a restart.

### Confirmation

Tests in `src/core/places_tests.rs`,
`src/modules/input/titles_tests.rs`,
`src/modes/prompt_history_tests.rs`,
`src/config/history_tests.rs` and
`src/adapters/browser_tabs/socket_tests.rs`.

## Pros and Cons of the Options

### Tabs in the title list only, without a history

- Good, because it is smaller.
- Bad, because the order and the opening place would still ignore
  tabs, so switching to a tab and back would not work like windows.
