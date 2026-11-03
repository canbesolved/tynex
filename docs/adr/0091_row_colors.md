---
status: "accepted"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# Output row colors

## Context and Problem Statement

The output bar mixes open windows, browser tabs and apps that are
not running, and only the icon tells them apart. RM-0064 asks for a
color per result type, set in the config. What should the color
paint, and how is it written?

## Decision Drivers

- Readable on the dark and the light theme.
- No visible change for users who do not opt in.
- No new crate; drawing stays in the shared renderer.

## Considered Options

- Title text color per type
- Row background band per type

## Decision Outcome

Chosen option: "Row background band per type", because the title
keeps the theme text color and its match highlights, so it stays
readable on both themes.

- `[output]` gets `colors.window`, `colors.tab` and `colors.app`
  (apps that are not running). Each is `""` (default, no band),
  `#RRGGBB` (opaque) or `#RRGGBBAA`, where `AA` is the opacity, so a
  pastel color can be a faint tint on any theme.
- The band is a rounded rectangle behind the row, the shape of the
  selection. The selection, which is translucent, is drawn over it.
- The new-window row of a running app gets no band; it keeps its
  own title color (`input.launcher.new_window_text_color`).
- Only the output bar draws bands. The title list and the icon
  dropdown hold windows only.
- `core::row_color` holds the types (`Tint`, `RowColors`) and picks
  the band of a row, so the rule is shared by every presenter.

### Consequences

- Good, because titles and highlights keep their contrast.
- Good, because the default config draws exactly what 0.3.3 drew.
- Bad, because an opaque bright color hides the popup background;
  the docs suggest colors with opacity, such as `#a3d9a540`.

### Confirmation

Tests in `src/core/row_color_tests.rs`, `src/config/output_tests.rs`
and `src/ui/output_bar_tests.rs`.

## Pros and Cons of the Options

### Title text color per type

- Good, because it reuses the per-row title color of the new-window
  row.
- Bad, because pastel text is faint on the light theme, so each
  theme would need its own values.
- Bad, because colored text competes with the match highlight color.
