---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Built-in light theme

## Context and Problem Statement

The popup has one dark theme with fixed colors. How does a light
theme sit next to it, chosen in the config, without theme files or
desktop color schemes?

## Decision Drivers

- The default stays the current dark theme.
- A missing key does not change an existing config.
- No new crate, and drawing code keeps reading one struct.
- Custom theme files stay a later item.

## Considered Options

- `main.theme` selects a built-in dark or light palette
- A new `[theme]` section for the one key
- Per-color keys in the config

## Decision Outcome

Chosen option: "`main.theme` selects a built-in dark or light
palette", because it matches `main.position`: one key, a small set
of values, and the dark look remains the default.

- `main.theme` is `dark` (default) or `light`. A missing key stays
  dark. Any other value stops the daemon.
- Both themes share the radius (`12`), the padding (`8`), the
  background alpha (`0xf2`) and the selection color (`#1f6feb` at
  `0x66`).
- `dark` keeps the current colors: background `#0d1117`, border
  `#30363d`, text `#e6edf3`, highlight `#58a6ff`.
- `light` is the light pair: background `#ffffff`, border
  `#d0d7de`, text `#1f2328`, highlight `#0969da`.
- Hint and new-window colors stay their own keys.

### Consequences

- Good, because an existing file keeps the dark popup.
- Good, because later `dark_theme` and `light_theme` file names do
  not collide with this key.
- Bad, because the hint green `#8da787` is dimmer on white than on
  the dark background. It stays configurable.

### Confirmation

`main.theme = "light"` parses, and any other value is rejected.
`Theme::light` changes background, border, text and highlight, and
shares radius, padding and the selection color with `Theme::dark`.

## Pros and Cons of the Options

### `main.theme` selects a built-in dark or light palette

- Good, because one key matches the other `[main]` choices.
- Good, because drawing stays on one struct.
- Bad, because the two palettes are fixed until custom themes.

### A new `[theme]` section for the one key

- Good, because later theme files would have a home.
- Bad, because a section for one key changes the section order.

### Per-color keys in the config

- Good, because every color would be editable.
- Bad, because that is the later theme-options item, not this one.

## More Information

The item is 0.2.18 in [ROADMAP.md](../../ROADMAP.md). Custom theme
files, per-color options and following the desktop scheme stay
later. Colors live in [render.md](../arch/ui/render.md).
