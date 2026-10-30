---
status: "accepted, amended by ADR-0073"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Popup fits the screen

## Context and Problem Statement

With `taskbar.mode = "both"`, large icons, `output.max_rows = 19` and
an app with many windows, the popup is taller than a 1536x864 screen.
The compositor centers it, so the input bar and the icon strip are
above the screen and the title list runs off the bottom. How does the
popup stay on screen?

## Decision Drivers

- The input bar and the icon strip are always visible.
- No new crate and no new config key.
- Sizing, drawing and hit testing use the same layout.
- No wakeup while the popup is hidden.

## Considered Options

- Shrink the scrolling lists, then the dropdown room
- Shrink every list by the same factor
- Anchor the popup to the top of the screen when it is too tall

## Decision Outcome

Chosen option: "Shrink the scrolling lists, then the dropdown room",
because the title list and the output bar already scroll, so fewer
rows lose no window.

- The presenter binds every `wl_output` at start and reports the
  shortest current mode height, after rotation, in physical pixels.
- The limit is that height minus 24 logical pixels above and below.
- The title list and the output bar drop one row at a time, down to
  one. Then the dropdown and its spare room do the same.
- The surface size is sent only when it changes, so a configure does
  not lead to another size request and another configure.

### Consequences

- Good, because the popup stays on screen on short or scaled outputs.
- Good, because a popup that fits is drawn as configured.
- Bad, because outputs plugged in after start are not measured.
- Bad, because a shrunk dropdown can hold titles that keys reach but
  that are not drawn, on very short screens only.

### Confirmation

`src/ui/fit_tests.rs`: 31 windows, 12 of one app, on a 1080 pixel tall
output at scale 1.25 shrink the title list and keep the dropdown room;
a popup that fits is unchanged; a tiny screen shrinks both.

## Pros and Cons of the Options

### Shrink the scrolling lists, then the dropdown room

- Good, because the dropdown keeps its rows as long as possible.
- Bad, because the title list may show few rows next to a big
  dropdown room.

### Shrink every list by the same factor

- Good, because the layout keeps its proportions.
- Bad, because the dropdown, which does not scroll, loses titles first.

### Anchor the popup to the top of the screen when it is too tall

- Good, because the input bar stays visible.
- Bad, because the bottom of the popup is still off the screen.

## More Information

Scrolling of the shrunk lists is [ADR-0070](0070_list_scroll.md).
Related: [ADR-0062](0062_icon_bar.md), [ADR-0065](0065_taskbar.md).
