---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Dropdown over title list

## Context and Problem Statement

[ADR-0062](0062_icon_bar.md) keeps a spare band under the icon strip,
as tall as the largest dropdown, so opening it does not resize the
popup. With `taskbar.mode = "both"` and one app with many windows,
that band leaves a large empty gap between the icons and the title
list. How does the dropdown get room without that gap?

## Decision Drivers

- No empty band between the icon strip and the title list.
- Opening the dropdown must not resize the popup.
- The selected dropdown title stays visible.

## Considered Options

- Keep the spare band.
- Open the dropdown over the title list, scrolling when longer.
- Grow the popup when the dropdown opens.

## Decision Outcome

Chosen option: "Open the dropdown over the title list, scrolling when
longer", because it removes the gap and keeps the popup size.

- In `both` mode the title list sits 3 logical pixels under the icon
  frame, with no spare room.
- The dropdown starts at the same place, over the title list. It
  shows at most as many rows as the title list, still capped by
  `output.max_rows`, and scrolls so the selected title stays visible.
- `iconbar` mode keeps the spare band: there is no list to cover.

### Consequences

- Good, because the popup is as short as its content.
- Good, because the popup does not resize when the dropdown opens.
- Bad, because the dropdown hides the title list while it is open.
- Bad, because with few windows overall the dropdown shows fewer rows
  and scrolls sooner.

### Confirmation

Tests in `src/ui/taskbar/menu_tests.rs` check the title list position,
the scrolled dropdown and its click mapping.

## More Information

Amends [ADR-0062](0062_icon_bar.md). Scrolling follows
[ADR-0070](0070_list_scroll.md), without wheel support in the
dropdown.
