---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Shift+Tab steps back

## Context and Problem Statement

[ADR-0061](0061_prompt_keys.md) and [ADR-0063](0063_tab_icons.md)
map `Shift+Tab` to the same `Key::Tab`, so it steps forward. There
is no key that walks the selection back in the icon strip without
the arrows. How does `Shift+Tab` move back?

## Decision Drivers

- `Shift+Tab` is the reverse of `Tab` everywhere `Tab` moves.
- It wraps the same way as `Tab`.
- The core stays portable: platforms map native keys to `Key`.
- No new crate and no config key.

## Considered Options

- New `Key::BackTab` variant
- `Key::Tab` with a `shift` field
- Keep `Shift+Tab` as `Tab`

## Decision Outcome

Chosen option: "New `Key::BackTab` variant", because it matches
`WordLeft` and `WordRight`: the platform resolves the modifier, and
modes match a plain key.

- The Wayland mapper reports `ISO_Left_Tab`, which `Shift+Tab`
  gives, as `Key::BackTab`. `Tab` stays `Key::Tab`.
- `BackTab` steps one entry back wherever `Tab` steps forward: an
  open dropdown first, then the title list or the output bar, then
  the icons when the title list is off. It wraps from the first
  entry to the last.
- With neither a list nor the strip, `BackTab` does nothing.
- `BackTab` does not repeat, like `Tab`.

This amends ADR-0061 and ADR-0063: `Shift+Tab` no longer steps
forward. Their bodies are unchanged.

### Consequences

- Good, because every `Tab` walk can be undone with one key.
- Bad, because every platform mapper must report `BackTab`.

### Confirmation

Keyboard tests map `ISO_Left_Tab` and a real `Shift+Tab` press to
`BackTab`. Prompt tests walk the title list, matches, icons and the
dropdown back, with wrapping.

## Pros and Cons of the Options

### New `Key::BackTab` variant

- Good, because modes match one key, like `WordLeft`.
- Bad, because `Key` grows by one variant.

### `Key::Tab` with a `shift` field

- Good, because one key covers both directions.
- Bad, because every match on `Tab` must read the field.

### Keep `Shift+Tab` as `Tab`

- Good, because nothing changes.
- Bad, because the selection cannot step back with `Tab`.

## More Information

The item is 0.2.12 in [ROADMAP.md](../../ROADMAP.md). Related:
[ADR-0061](0061_prompt_keys.md), [ADR-0063](0063_tab_icons.md).
