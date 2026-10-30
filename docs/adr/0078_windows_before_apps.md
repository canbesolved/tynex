---
status: "accepted"
date: "2026-10-31"
decision-makers: "CanBeSolved"
---
# Windows before launch rows

## Context and Problem Statement

A search with the launcher on groups each app's windows with its
launch row, then sorts those groups by match score. A stronger idle
app can sit above a weaker open window. Should open windows stay
first?

## Decision Drivers

- Typing is how you switch to a window that is already open.
- An installed app that is not running is a launch, not a switch.
- The new-window row stays under the app it opens (ADR-0059).
- No new config key.

## Considered Options

- Window groups first, then launch-only groups
- One list sorted by score, running apps only on a tie
- Every window row, then every launch row, including new-window

## Decision Outcome

Chosen option: "Window groups first, then launch-only groups",
because a search should switch to an open window before it offers
to start an app.

- A group with at least one open window comes first. Inside that
  band, a better score comes first, then app id.
- A launch-only group (installed, not running) comes after, with
  the same score order.
- A running app still lists its windows, then the new-window row
  when that row is on. That row stays in the window band.
- With the launcher off, the list is still only the ranked windows.

### Consequences

- Good, because an open window stays above a better-scoring idle
  app.
- Good, because the new-window row stays next to its windows.
- Bad, because the best-scoring idle app is no longer the first
  row when any window matches.

### Confirmation

`src/modules/input/launcher_tests.rs`: an idle app whose name is
the query still follows the matching window and its new-window
row. Equal scores still list the running app first.

## Pros and Cons of the Options

### Window groups first, then launch-only groups

- Good, because switching to an open window stays the first result.
- Good, because the new-window row stays under its app.
- Bad, because score no longer decides which band a group is in.

### One list sorted by score, running apps only on a tie

- Good, because the closest name stays first.
- Bad, because an idle app can hide the window you meant to switch
  to.

### Every window row, then every launch row, including new-window

- Good, because the two kinds of rows are two blocks.
- Bad, because the new-window row leaves the app it belongs to.

## More Information

Ships in 0.2.17. Related: [ADR-0056](0056_launcher_input.md),
[ADR-0059](0059_new_window_row.md). Behavior is in
[input.md](../arch/modules/input.md).
