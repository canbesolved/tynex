---
status: "accepted"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Space confirms, Tab walks the list

## Context and Problem Statement

Typing a space inserts it into the query, and `Tab` does nothing.
A search is one token, and the row on screen should move from the
keyboard. What should `Space` and `Tab` do?

## Decision Drivers

- A query does not contain spaces.
- `Space` and `Enter` pick the same row, including when none matches.
- `Tab` moves the list the user can see.
- The title bar and the output bar share one selection index and are
  never on screen together
  ([ADR-0057](0057_title_bar_query.md)).
- No new `Key` variant and no config key.

## Considered Options

- `Space` confirms; `Tab` walks the visible list
- `Space` inserts a character; `Tab` stays ignored
- Map the space bar to `Enter` in the Wayland keyboard

## Decision Outcome

Chosen option: "`Space` confirms; `Tab` walks the visible list",
because both keys act on the row that is already selected, and the
query stays a single token.

- `Space` (U+0020) has the same result as `Enter`: activate a window,
  launch an app, or do nothing when no row is selected. The character
  is not inserted. Other whitespace is unchanged.
- `Tab` calls the same step as `Down`: one row forward, wrapping.
  It runs when the query is non-empty (the output bar) or when
  `title_bar.enabled` is set and the query is empty (the title bar).
  With neither list showing, `Tab` does nothing.
- `Shift+Tab` is the same `Key::Tab`, so it steps forward too.
- `Space` and `Tab` do not repeat.
- The Wayland mapper still reports a space as `Key::Char`. Prompt
  mode owns the rule, so later platforms get it without a new key.

### Consequences

- Good, because a query cannot pick up a space from the space bar.
- Good, because `Tab` moves whichever list is drawn.
- Bad, because a multi-word query cannot be typed.
- Bad, because `Shift+Tab` does not move backward.

### Confirmation

`src/modes/prompt_key_tests.rs`: `Space` activates, launches, and
leaves a query with no matches; `Tab` walks the title bar, walks
output matches with the title bar on or off, and does nothing when
no list is showing.

## Pros and Cons of the Options

### `Space` confirms; `Tab` walks the visible list

- Good, because confirm and movement stay in prompt mode.
- Good, because one selection index already serves both lists.
- Bad, because holding `Space` does not insert a space.

### `Space` inserts a character; `Tab` stays ignored

- Good, because multi-word queries can be typed.
- Bad, because the space bar does not pick the row on screen.
- Bad, because `Tab` never moves a list.

### Map the space bar to `Enter` in the Wayland keyboard

- Good, because prompt mode stays unchanged for `Space`.
- Bad, because each later platform must repeat the mapping.
- Bad, because it does not decide what `Tab` does.

## More Information

The item is 0.2.8 in [ROADMAP.md](../../ROADMAP.md), including the
former 0.2.13 title-bar `Tab` and the output-bar `Tab` that was
listed under 0.2.9. `Left` and `Right` on the icon strip stay in
0.2.9.
