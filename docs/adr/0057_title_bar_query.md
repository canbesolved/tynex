---
status: "accepted"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Hide title bar while typing

## Context and Problem Statement

[ADR-0054](0054_output_bar.md) draws the open windows under the
matches whenever the title bar is enabled. A query then makes the
popup taller, because both lists are on screen. The title bar's job
is the open windows before you type. Should it stay up while a query
is showing matches?

## Decision Drivers

- The output bar already shows the matches.
- Typing should not add a second list.
- Clearing the query should bring the open windows back.
- `title_bar.enabled` still turns the list off entirely.

## Considered Options

- Hide the title bar while the query is non-empty
- Keep both lists on screen

## Decision Outcome

Chosen option: "Hide the title bar while the query is non-empty",
because the matches already have a module and the open windows are
what you see before you type.

- `shows` is `title_bar.enabled` and an empty query.
- Typing hides the title bar. The output bar shows the matches.
- `Backspace` that empties the query, or `Esc`, shows the title bar
  again when it is enabled. `Esc` on an empty query still closes.
- With `title_bar.enabled = false` the title bar stays hidden.
- This replaces the part of ADR-0054 that kept the title bar up
  during a query and stacked both lists. ADR-0054 still decides that
  matches belong to the output bar.

### Consequences

- Good, because typing does not make the popup a second list taller.
- Good, because clearing the query restores the open windows.
- Bad, because the full window list and the matches are not on
  screen together.

### Confirmation

`src/ui/title_bar.rs` shows the list only when it is enabled and the
query is empty. A painter test checks that a query shrinks the popup
to the output bar and that clearing the query restores the title-bar
height.

## Pros and Cons of the Options

### Hide the title bar while the query is non-empty

- Good, because one list is on screen: windows, or matches.
- Good, because clearing the query returns to the open windows.
- Bad, because you cannot compare the full list with the matches.

### Keep both lists on screen

- Good, because the open windows stay visible while you search.
- Bad, because the popup grows by a whole list when you type.
- Bad, because the title bar ignores the query, which is what this
  release changes.

## More Information

Related: [ADR-0054](0054_output_bar.md). The item is 0.2.2 in
[ROADMAP.md](../../ROADMAP.md).
