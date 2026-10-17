---
status: "superseded by ADR-0041"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Home row keys in most recently used order

## Context and Problem Statement

Each listed window gets a letter key. Windows come in most recently
used (MRU) order: the window focused last comes first, as in Alt+Tab.
Which letters, and which window gets the first one?

## Decision Drivers

- Fastest keys first, without moving the hands.
- The most common switch (to the previous window) must be quickest.
- Works with any keyboard layout.

## Considered Options

- Home row first (`asdfhjkl`, then `g` and the top row), MRU order with
  the active window last
- Numbers `1` to `9`
- Alphabetical by title

## Decision Outcome

Chosen option: "Home row first, MRU order with the active window
last", because the previous window always lands on `a`, so hotkey then
`a` acts like Alt+Tab, and the next most recent windows follow on the
strongest fingers.

### Consequences

- Good, because the order is configurable with `keys` in the config.
- Good, because keys match the produced character, so non-QWERTY
  layouts work.
- Bad, because letters move when MRU order changes; per-app pinned
  keys can be added later.

## More Information

See [core.md](../arch/core.md) and [hotkey.md](../arch/modes/hotkey.md).
