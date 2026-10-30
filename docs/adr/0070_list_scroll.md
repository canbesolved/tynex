---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Scrolling lists

## Context and Problem Statement

[ADR-0069](0069_screen_fit.md) shows fewer title rows on a short
screen. The hidden rows are reachable only with the keys, which move
the selection. How does the user see the rest of a long title list or
output bar?

## Decision Drivers

- The title list always fits the screen.
- `output.max_rows` stays the row limit; scrolling turns on only when
  a list is longer.
- The mouse wheel and the touchpad work like in other lists.
- No new crate, no config key, no wakeup while hidden.

## Considered Options

- Wheel moves the rows, with a scrollbar indicator
- Wheel moves the selection, like `Up` and `Down`
- Draggable scrollbar with a track

## Decision Outcome

Chosen option: "Wheel moves the rows, with a scrollbar indicator",
because it matches common lists and keeps the selection where the
user put it.

- Rows shown: `output.max_rows`, or fewer to fit the screen.
- One notch moves 3 rows; touchpad pixels add up to whole rows. The
  list stops at the first and last page.
- A new selection moves the rows the least that shows it.
- A thumb at the right edge shows the position. No dragging yet.
- The position lives in the painter, like the dropdown slide, and is
  forgotten when the popup closes. The core is unchanged.
- The presenter reports `wl_pointer` vertical axis events as one
  `Scroll` per pointer frame.

### Consequences

- Good, because a list of any length fits and can be browsed.
- Good, because a wheel event redraws only when the rows moved.
- Bad, because the selected row can scroll out of sight.
- Bad, because the thumb cannot be dragged.

### Confirmation

`src/ui/scroll_tests.rs`: the wheel moves the rows and not the
selection and stops at the last page, touchpad rows add up, a new
selection is followed, and a long list draws a scrollbar.

## Pros and Cons of the Options

### Wheel moves the rows, with a scrollbar indicator

- Good, because it behaves like other desktop lists.
- Bad, because it adds a scroll position next to the selection.

### Wheel moves the selection, like `Up` and `Down`

- Good, because there is one position to track.
- Bad, because browsing changes what `Enter` activates.

### Draggable scrollbar with a track

- Good, because long lists are quick to cross.
- Bad, because it needs press, drag and release handling.

## More Information

Mouse selection in the title list is 0.2.16 in
[ROADMAP.md](../../ROADMAP.md). Related:
[ADR-0069](0069_screen_fit.md).
