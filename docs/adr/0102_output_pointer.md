---
status: "accepted"
date: "2026-11-08"
decision-makers: "CanBeSolved"
---
# Pointer in the output list

## Context and Problem Statement

The pointer selects and activates title-list rows
([ADR-0077](0077_pointer_rows.md)), but the output list, the results
of a typed query, moves only by keys. How do hover and click reach
the output rows, and can hover there be turned off on its own
(RM-0065)?

## Decision Drivers

- The same gestures as in the title list: hover selects, a click
  activates.
- The hover rules of [ADR-0101](0101_hover_gate.md): wait for the
  first pointer move, and an option to turn hover off.
- A row under the pointer must not scroll away.
- No new kind of click, so modes keep one row target.

## Considered Options

- One hit test for the list on screen, `Click::Row` for both lists
- A new `Click::Output` for output rows

## Decision Outcome

Chosen option: "One hit test for the list on screen, `Click::Row`
for both lists", because the two lists never show at once (the title
list needs an empty query, the output list a typed one) and already
share their place, their rows in `State::output`, the selection and
the scroll position.

- `scroll::hit` returns the row under the pointer in the list on
  screen, the same way for both lists: the scrollbar is not a row,
  and `first_row` maps a scrolled list.
- `Click::Row(index)` is an index into the list on screen. Prompt
  mode reads the query to know which one it is.
- Hovering an output row selects it; a click activates, launches or
  switches to it, like `Enter`. Hover waits for the first pointer
  move in the open popup (ADR-0101).
- `output.select_on_hover` (default `true`): `false` makes hover
  never select an output row. A click still activates. It is
  separate from the title list and icon bar keys.
- Prompt mode takes the three hover switches as one `Hover` value
  (`with_hover`).

### Consequences

- Good, because the output list behaves like the title list.
- Good, because one hit test serves both lists.
- Bad, because `Click::Row` means a different list depending on the
  query; a mode must check it.

### Confirmation

Tests in `src/ui/scroll_hit_tests.rs` for the hit test,
`src/modes/prompt_pointer_tests.rs` for hover and click, and
`src/config/output_tests.rs` for the key.

## Pros and Cons of the Options

### A new `Click::Output` for output rows

- Good, because a click names its list.
- Bad, because every match on `Click` gains a case, and the hit
  test is duplicated for two lists that never show together.
