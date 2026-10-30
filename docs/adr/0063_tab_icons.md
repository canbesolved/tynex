---
status: "accepted, amended by ADR-0068"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Tab moves icons without a title bar

## Context and Problem Statement

[ADR-0061](0061_prompt_keys.md) makes `Tab` walk the list on screen,
and do nothing when neither the title bar nor the output bar is
showing. With the icon strip up and the title bar off, that leaves
no way to move the selected icon except `Left` and `Right`. What
should `Tab` do then?

## Decision Drivers

- `Tab` moves the selection the user can see.
- An open dropdown already takes `Tab`, `Up` and `Down`.
- With the title bar on, `Tab` still walks that list.
- `Shift+Tab` stays the same key, so it still steps forward.
- No new `Key` variant and no config key.

## Considered Options

- `Tab` moves to the next icon when the title bar is off
- `Tab` does nothing when the title bar is off
- `Tab` always moves icons, including when the title bar is on

## Decision Outcome

Chosen option: "`Tab` moves to the next icon when the title bar is
off", because the icon is the selection on screen, and the title
list keeps `Tab` when that list is drawn.

- While the strip is showing, the query is empty, and
  `title_bar.enabled` is off, `Tab` calls the same step as `Right`:
  the next icon, wrapping. It does not move the hidden title list.
- An open dropdown still takes `Tab` first, one title forward.
- With the title bar on, `Tab` still walks that list. A non-empty
  query still walks the output bar.
- With neither a list nor the strip, `Tab` does nothing.
- `Shift+Tab` is the same `Key::Tab`, so it steps forward too.

This amends ADR-0061. The body of that record is unchanged.
[ADR-0062](0062_icon_bar.md) still gives an open dropdown `Tab`
before anything else.

### Consequences

- Good, because `Tab` moves a selection whenever the strip or a
  list is on screen.
- Bad, because `Tab` and `Right` do the same thing when the title
  bar is off and the dropdown is closed.

### Confirmation

`src/modes/prompt_icon_tests.rs`: `Tab` moves the next icon and
wraps when the title bar is off, steps an open dropdown first, and
walks the title list when that bar is on.

## Pros and Cons of the Options

### `Tab` moves to the next icon when the title bar is off

- Good, because the visible selection always has a key.
- Good, because the title list keeps `Tab` when it is drawn.
- Bad, because `Tab` and `Right` overlap in this one case.

### `Tab` does nothing when the title bar is off

- Good, because ADR-0061 stays word for word.
- Bad, because the selected icon does not move from `Tab`.

### `Tab` always moves icons, including when the title bar is on

- Good, because one key always means the strip.
- Bad, because the title list loses `Tab` while both bars show.

## More Information

The icon strip is 0.2.9 in [ROADMAP.md](../../ROADMAP.md). Related:
[ADR-0061](0061_prompt_keys.md), [ADR-0062](0062_icon_bar.md).
