---
status: "accepted"
date: "2026-11-04"
decision-makers: "CanBeSolved"
---
# Window and tab dedupe

## Context and Problem Statement

A browser window and its active tab share a title, so a search for
`My website` lists two rows: the window with the browser icon and
the tab with the site favicon (RM-0011). Both raise the same window.
How does the output bar list them once?

## Decision Drivers

- One row per page the user sees.
- A wrong match must never hide a window that has no other row.
- Pure logic, no new data from the browser or `KWin`.

## Considered Options

- Drop the window row when its active tab row is listed
- Drop the active tab row and keep the window row
- Merge both into one row with the window target

## Decision Outcome

Chosen option: "Drop the window row when its active tab row is
listed", because the tab row carries the favicon and the host, and
activating it already raises its window
([ADR-0082](0082_kwin_pid.md)).

A window row is dropped when all of these hold:

1. A tab row in the results is an active tab.
2. The window belongs to that tab's browser: its pid equals the
   browser pid when the tab list came in.
3. The window title is the tab title, alone or followed by a title
   separator (` - `, ` – `, ` — `), as in `Notes - Brave`. An empty
   tab title matches nothing.

A tab row that did not match the query hides nothing, so a search
for the browser name still lists its windows. An empty query shows
no tab rows, so the title list is unchanged.

### Consequences

- Good, because a browser page is listed once, with its favicon.
- Good, because a renamed window or a window of another app keeps
  its row: its title or pid does not match.
- Bad, because a window opened while the popup is open is not in the
  tab list's window set until the list is sent again, so it can be
  listed twice for that popup.

### Confirmation

Tests in `src/modules/input/browser_tabs_tests.rs` and
`src/modules/browser_tabs/tests.rs`.

## Pros and Cons of the Options

### Drop the active tab row and keep the window row

- Good, because the window list stays as before.
- Bad, because the favicon and host, which tell pages apart, are
  lost.

### Merge both into one row with the window target

- Good, because the row could show both icons.
- Bad, because it needs a new row kind and new drawing for one case.
