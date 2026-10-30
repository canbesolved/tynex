---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Cursor shape

## Context and Problem Statement

A Wayland client sets the cursor image when the pointer enters its
surface. The popup never did, so when it opened under the pointer the
compositor showed no cursor until the pointer left. How does the popup
show a cursor without a toolkit?

## Decision Drivers

- Footprint: no cursor theme files loaded, no new crate.
- Works on KWin, the main target.

## Considered Options

- `cursor-shape-v1`: name the shape, the compositor draws it.
- `wayland-cursor`: load an Xcursor theme and attach its image.
- Set no cursor.

## Decision Outcome

Chosen option: "`cursor-shape-v1`", because it is one request on each
pointer `enter`, uses the user's own cursor theme and size, and needs
only the `unstable` feature of `wayland-protocols`, which it already
uses (the protocol refers to the tablet interfaces).

The popup sets the `default` shape. Without the global the popup
keeps its old behavior.

### Consequences

- Good, because the cursor matches the desktop theme and scale.
- Good, because no theme files are read and no image is uploaded.
- Bad, because compositors without `cursor-shape-v1` still show no
  cursor over a popup opened under the pointer.

### Confirmation

Manual check on KDE Plasma 6: open the popup with the pointer over
where it appears; the arrow stays visible.

## More Information

KWin, wlroots compositors, Hyprland and niri support
`cursor-shape-v1`. Revisit with `wayland-cursor` if a target
compositor lacks it.
