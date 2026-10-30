---
status: "accepted, amended by ADR-0074"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Anchor the popup vertically

## Context and Problem Statement

The popup is a layer-shell surface with no anchors, so the compositor
centers it. When the output bar or a dropdown changes the height, the
input bar jumps. How does the popup stay where it opened, with a
choice of `center` or `top`?

## Decision Drivers

- The input bar does not move while the popup is open.
- No new crate.
- The popup stays on screen.
- `center` remains the default.

## Considered Options

- Anchor the top edge and keep that margin until hide
- Keep a fixed surface height and draw into it
- Leave placement to the compositor

## Decision Outcome

Chosen option: "Anchor the top edge and keep that margin until hide",
because the surface can grow downward without moving the input bar,
and layer-shell already centers a sized surface between the left and
right edges.

- Anchors are `top`, `left` and `right`. The exclusive zone is `-1`,
  so the margin is from the full output, not pushed by a panel.
- `main.position = "center"` (default) sets the top margin from the
  height at open: half the leftover space on the shortest output, and
  at least 24 logical pixels. That margin is not updated when the
  height changes. Hide clears it, so the next open recenters.
- `main.position = "top"` uses a top margin of 24 logical pixels.
- The fit limit keeps 24 logical pixels under the popup. For `top`,
  and for the first frame of `center`, that is the same as 24 above
  and below. After `center` locks a short height, the limit is the
  screen minus the locked top margin minus 24 at the bottom. Lists
  still shrink in the same order
  ([ADR-0069](0069_screen_fit.md)).

### Consequences

- Good, because typing no longer moves the input bar.
- Good, because a missing key stays centered.
- Bad, because a centered popup that grows has less room below, so a
  long list shows fewer rows.
- Bad, because the margin uses the shortest output, so on a taller
  screen the popup is not exactly centered.

### Confirmation

`src/ui/place.rs`: the center margin follows the locked height, the
top margin is 24, and a locked center has a tighter fit limit.
Manual check on KDE Plasma 6: open the popup and type; the input bar
stays put. With `main.position = "top"` it sits near the top of the
screen.

## Pros and Cons of the Options

### Anchor the top edge and keep that margin until hide

- Good, because growth extends downward.
- Good, because horizontal centering stays a layer-shell anchor.
- Bad, because the bottom room depends on the height at open.

### Keep a fixed surface height and draw into it

- Good, because the compositor never recenters.
- Bad, because the empty area would take pointer hits.
- Bad, because a centered box would include that empty area, so the
  input bar would not sit in the middle.

### Leave placement to the compositor

- Good, because no anchor requests are needed.
- Bad, because every height change moves the input bar.

## More Information

The item is 0.2.13 in [ROADMAP.md](../../ROADMAP.md). The screen fit
is [ADR-0069](0069_screen_fit.md). The layer-shell surface is
[ADR-0007](0007_layer_shell.md).
