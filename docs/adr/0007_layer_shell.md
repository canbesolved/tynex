---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Layer-shell popup without UI toolkit

## Context and Problem Statement

The popup is a centered list of rows that must appear instantly, take
all keyboard input and use little memory. On Wayland a normal window
cannot position itself or reliably grab focus.

## Decision Drivers

- RAM and start-up cost.
- Centered, focused overlay on Wayland.
- No dependency on one desktop's toolkit.

## Considered Options

- Raw `wlr-layer-shell` surface, no toolkit
- Qt6 / QML
- GTK4

## Decision Outcome

Chosen option: "Raw `wlr-layer-shell` surface, no toolkit", because a
list of rows does not need a toolkit, and layer-shell gives a centered
overlay with exclusive keyboard focus.

### Consequences

- Good, because RAM stays in single-digit MB and show is instant.
- Good, because KWin, wlroots compositors, Hyprland and niri support
  layer-shell.
- Bad, because widgets, text layout and accessibility are hand-made.
- Bad, because GNOME Mutter lacks layer-shell and needs another
  presenter later.

## Pros and Cons of the Options

### Qt6 / QML

- Good, because it matches the native KDE look.
- Bad, because it costs 40 to 60 MB of RAM and a slower cold start.

### GTK4

- Good, because it is a widespread cross-desktop toolkit.
- Bad, because of similar memory cost and a GPU renderer by default.

## More Information

See [linux.md](../arch/platforms/linux.md).
