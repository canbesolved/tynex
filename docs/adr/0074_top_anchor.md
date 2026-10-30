---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Center on the top anchor

## Context and Problem Statement

[ADR-0073](0073_popup_place.md) anchors the popup to the top, left and
right edges, expecting a sized surface to sit in the middle. On KWin
it draws on the left. Which anchor keeps the top edge fixed and the
popup centered horizontally?

## Decision Drivers

- KDE Plasma 6 is the main target.
- The top margin from ADR-0073 stays in charge of the vertical place.
- No new crate.

## Considered Options

- Anchor the top edge only
- Anchor the left edge and set a left margin
- Keep the left and right anchors

## Decision Outcome

Chosen option: "Anchor the top edge only", because KWin moves a
surface to the left edge whenever the left anchor is set, even when
the right anchor is set and the width is explicit. With only the top
anchor, KWin centers the surface on that edge, then applies the top
margin. The exclusive zone stays `-1`, so the center is the full
output.

### Consequences

- Good, because the popup is centered on KWin and still grows
  downward.
- Good, because wlroots also centers a surface that has no left or
  right anchor.
- Bad, because a compositor that does not center a top-only anchor
  would still pin the popup to one side.

### Confirmation

`src/platform/linux/wayland/surface.rs`: the anchor is the top edge
and does not include left or right. Manual check on KDE Plasma 6: the
popup opens in the horizontal center.

## Pros and Cons of the Options

### Anchor the top edge only

- Good, because the compositor knows which output it placed the popup
  on, so the center matches that output.
- Bad, because it depends on the compositor centering an unanchored
  axis.

### Anchor the left edge and set a left margin

- Good, because the horizontal place is computed by `tynex`.
- Bad, because the margin needs the width of the output the
  compositor chose, which the client does not know.

### Keep the left and right anchors

- Good, because some compositors center a sized surface between
  opposite edges.
- Bad, because KWin places that surface on the left.

## More Information

Amends [ADR-0073](0073_popup_place.md). KWin's placement is
`rearrangeLayer` in `layershellv1integration.cpp`: a left anchor calls
`moveLeft` when the width is set.
