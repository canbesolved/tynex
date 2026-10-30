---
status: "accepted, amended by ADR-0064, ADR-0065 and ADR-0067"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Icon bar of running apps

## Context and Problem Statement

The popup lists windows as rows. A task list of one icon per running
app, with a dropdown of that app's windows, is still only a roadmap
item. Where does the strip sit, and which keys move it?

## Decision Drivers

- The input bar stays at the top of the popup.
- The strip hides while a query is typed, like the title bar.
- With the title bar on, its rows stay under the strip.
- Opening the dropdown does not move the strip.
- `Left` and `Right` still move the text cursor while typing.
- Launching uses the existing `Action::Launch` path.
- No new crate, and no wakeup while the popup is hidden.

## Considered Options

- Strip under the input bar, hidden while typing
- Strip always visible, including while typing
- Horizontal scroll when the icons do not fit

## Decision Outcome

Chosen option: "Strip under the input bar, hidden while typing",
because the input stays where it is and the title list stays under
the icons.

- Config `[icon_bar]`: `enabled` (default `true`) and `icon_size`
  (default `24`, range 8 to 128). `icon_size` is logical pixels,
  multiplied by the output scale like `main.font_size`.
- One icon per `app_id`, most recently used app first. The selected
  icon starts as the app of the previous window.
- An app with several windows shows the count under its icon.
- The row is centered. If it is wider than the inner popup width,
  every icon and the gap shrink by the same factor until it fits.
- `Left` and `Right`, while the query is empty, move along the
  strip and close an open dropdown. While typing, they move the
  cursor.
- `Enter` and `Space` match a click: one window activates, several
  open the dropdown, none launches. `Up`, `Down` and `Tab` move
  inside an open dropdown. `Esc` closes it first.
- The dropdown lists titles alphabetically, at most
  `output_bar.max_rows`, up to 450 logical pixels wide. It slides
  for 150 ms. The poll loop wakes about every 1/120 s only while
  that slide runs.
- The surface keeps a transparent band tall enough for the largest
  dropdown, between the strip and the title bar, so the strip does
  not move when the dropdown opens.
- A left click is the same as `Enter` on an icon, activates a
  dropdown row, or closes the dropdown. The right button is ignored.

This amends ADR-0036: a slide in progress is another poll deadline,
next to key repeat and the hint timer. An idle daemon still has no
timeout. The body of ADR-0036 is unchanged.

### Consequences

- Good, because the strip, the dropdown and the keys stay in the
  modules that already own input and drawing.
- Good, because a hidden popup still does not wake to animate.
- Bad, because `Enter` on an app with several windows opens the
  dropdown instead of switching straight away.
- Bad, because many apps make the icons small rather than scrolling.

### Confirmation

Config tests accept the default and reject an `icon_size` outside
8 to 128. Core tests group apps and sort dropdown titles. Prompt
tests cover the keys. Painter tests cover centering, shrinking,
the spare height and a click on a shrunk icon.

## Pros and Cons of the Options

### Strip under the input bar, hidden while typing

- Good, because typing keeps the cursor keys and one result list.
- Good, because the title bar stays under the icons.
- Bad, because the icons are gone while a query is showing.

### Strip always visible, including while typing

- Good, because the task list stays on screen.
- Bad, because `Left` and `Right` cannot both move the cursor and
  the strip.

### Horizontal scroll when the icons do not fit

- Good, because icons keep the configured size.
- Bad, because a click then depends on a scroll offset.

## More Information

The item is 0.2.9 in [ROADMAP.md](../../ROADMAP.md). Pinned apps
stay later. Related: [ADR-0053](0053_ui_module_toggles.md),
[ADR-0061](0061_prompt_keys.md).
