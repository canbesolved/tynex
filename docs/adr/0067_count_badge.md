---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Window count badge

## Context and Problem Statement

ADR-0062 draws the window count of an app on a line under its icon.
That line makes the strip about 45% taller than the icons, even when
no app has several windows. Where does the count go instead?

## Decision Drivers

- The count stays readable on any icon.
- The strip is no taller than the icons.
- No new crate and no new config key.

## Considered Options

- Badge on the bottom center of the icon
- Badge on the bottom right corner of the icon
- Line under the icon (ADR-0062)

## Decision Outcome

Chosen option: "Badge on the bottom center of the icon", because the
strip loses the count line and the badge stays inside the icon.

- The number is 0.3 of the drawn icon side, in a rounded square of
  the frame border color. The square is 1.25 times the font size
  tall, with corners of a quarter of that. More digits widen it, up
  to one icon side.
- The square sits inside the icon, flush with its bottom edge.
- The strip is one icon side tall. A click below the icons no longer
  hits them.

This amends ADR-0062: the count is no longer under the icon. The body
of ADR-0062 is unchanged.

### Consequences

- Good, because the icon frame is shorter.
- Good, because the badge never spills into the gap or the frame.
- Bad, because the badge hides the bottom of the icon.

### Confirmation

Painter tests check that an app with two windows fills the badge on
the bottom center of its icon, an app with one window does not, the
badge corner is not cut like a circle, and the strip is one icon
side tall.

## Pros and Cons of the Options

### Badge on the bottom center of the icon

- Good, because it lines up with the dropdown, which opens under the
  icon center.
- Bad, because it covers the middle of the icon bottom.

### Badge on the bottom right corner of the icon

- Good, because it is the usual place on desktop docks.
- Bad, because it spills into the gap, or covers a corner detail.

### Line under the icon (ADR-0062)

- Good, because it hides nothing of the icon.
- Bad, because the strip is taller even with no counts to show.

## More Information

Related: [ADR-0062](0062_icon_bar.md), [ADR-0065](0065_taskbar.md).
