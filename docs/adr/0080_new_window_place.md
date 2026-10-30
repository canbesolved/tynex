---
status: "accepted"
date: "2026-10-31"
decision-makers: "CanBeSolved"
---
# New window row under its app

## Context and Problem Statement

ADR-0079 lifts an app that starts with the query above a later
match. Putting every launch row after every window stacks the
new-window rows together, away from the app they open. Where does
that row belong?

## Decision Drivers

- The new-window row opens another window of one app (ADR-0059).
- A leading app still stays above a later match (ADR-0079).
- An app that is not running still follows the open windows
  (ADR-0078).
- No new config key.

## Considered Options

- The new-window row stays directly under that app's windows
- Every launch row follows every window
- The new-window row stays under the app only when it is the only
  match

## Decision Outcome

Chosen option: "The new-window row stays directly under that app's
windows", because the row belongs to the app above it.

- A running app lists its windows, then its new-window row when
  that row is on. The next app starts after that row.
- An app whose title or name starts with the query, including its
  new-window row, comes before an app that matches later in the
  text. `c` lists Cursor, its new-window row, then Google Chrome
  and its own new-window row.
- An app that is not running stays one launch row after the open
  apps.

This amends ADR-0079. The body of that record is unchanged.

### Consequences

- Good, because each new-window row sits under its app.
- Good, because `c` still lists Cursor before Google Chrome.
- Bad, because a new-window row can sit between two windows of
  different apps.

### Confirmation

A launcher test for `c` expects Cursor, its new-window row, Google
Chrome, then Chrome's new-window row.

## Pros and Cons of the Options

### The new-window row stays directly under that app's windows

- Good, because the row is the next action for the app above it.
- Good, because a leading app and its row stay together.
- Bad, because the row separates that app from the next window.

### Every launch row follows every window

- Good, because the window list is one block.
- Bad, because every new-window row stacks at the bottom.

### The new-window row stays under the app only when it is the only match

- Good, because a single app looks the same as before.
- Bad, because several matches would still stack the rows.

## More Information

Ships in 0.2.17. Amends [ADR-0079](0079_leading_match.md). Related:
[ADR-0059](0059_new_window_row.md),
[ADR-0078](0078_windows_before_apps.md).
