---
status: "accepted"
date: "2026-10-18"
decision-makers: "CanBeSolved"
---
# Row order

## Context and Problem Statement

[ADR-0041](0041_pinned_keys.md) pins a letter to each window, but rows
stay in most recently used order, so every switch moves the rows and
the letters jump around the list. Users find it hard to follow. In
which order should rows be shown?

## Decision Drivers

- A window should be found at the same place in the popup.
- Alt+Tab habits: `Enter` right after opening goes back to the
  previous window.
- Simple, pure logic in the core.

## Considered Options

- `sort_by` with `key` (default), `app` and `recently`
- Keep most recently used order only

## Decision Outcome

Chosen option: "`sort_by` with `key` (default), `app` and
`recently`", because letter order keeps every row in place, and the
other two orders stay available for users who prefer them.

- `key`: rows follow the position of their letter in `keys`. A new
  window takes the first free letter and shows at that letter's
  place, also in the middle of the list.
- `app`: rows grouped by app name, ignoring case, then by letter.
- `recently`: most recently used first, active window last, the order
  before this decision.
- All orders start from the most recently used order with the active
  window last and sort stably, so windows without a letter come last
  in that order.
- Whatever the order, the popup opens with the previous window
  selected, so `Enter` keeps its Alt+Tab meaning.
- `core/order.rs` holds `SortBy` and `order`; the switcher builds rows
  with it.

This amends ADR-0041: rows no longer stay in most recently used order
by default.

### Consequences

- Good, because rows and letters stay in place while switching.
- Neutral, because the selection may start in the middle of the list.
- Bad, because the default order changes for existing users; they get
  the old one back with `sort_by = "recently"`.

### Confirmation

Unit tests in `core/order.rs` cover the three orders, and
`letter_order_tests.rs` checks that rows stay in place and the
previous window is selected. The end-to-end test switches windows and
expects the same rows.

## Pros and Cons of the Options

### Keep most recently used order only

- Good, because the most likely targets are always at the top.
- Bad, because rows move after every switch.
