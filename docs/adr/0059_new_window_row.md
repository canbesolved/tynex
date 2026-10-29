---
status: "accepted"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# New window row text and color

## Context and Problem Statement

A running app gets an extra launch row. That row should be optional,
and its title and color should be configurable. Where do those keys
live, and how does the row carry a color the theme does not have?

## Decision Drivers

- The keys belong with the launcher
  ([ADR-0055](0055_config_sections.md)).
- Only this row is dimmed. Other rows stay on the theme text color.
- An invalid color or an empty title must stop the daemon
  ([ADR-0031](0031_strict_config.md)).
- One font still draws the text ([ADR-0047](0047_one_font.md)). The
  color is just a paint color.

## Considered Options

- Flat keys under `[input_bar]`; the row stores the color
- A `[input_bar.launcher.new_window]` table
- A theme key for muted text, shared by every row

## Decision Outcome

Chosen option: "Flat keys under `[input_bar]`; the row stores the
color", because the launcher keys already use that dotted form
(`launcher.mode`) and the option applies to one row.

- `launcher.mode` defaults to `"gui_only"`.
- `launcher.new_window` defaults to `true`. `false` hides the row
  under a running app. An app that is not running stays one launch
  row titled with its name.
- `launcher.new_window_text` is the title. The default is one
  space, `+`, two spaces, then `New window`. An empty title stops
  the daemon.
- `launcher.new_window_text_color` is `#RRGGBB`. The default
  `#8da787` is a little dimmer than the theme text `#E6EDF3`.
  Anything else stops the daemon.
- The color is `Row.title_color`. `None` means the theme text
  color. The output bar uses it for that row only.

### Consequences

- Good, because the row can be turned off or renamed without a
  theme.
- Good, because missing keys keep the row on.
- Bad, because one row color lives outside the theme.

### Confirmation

Config tests cover the defaults, a custom value, a bad color and an
empty title. A launcher test checks that `new_window = false` drops
the row and that the custom title and color are stored on it.

## Pros and Cons of the Options

### Flat keys under `[input_bar]`; the row stores the color

- Good, because the keys match `launcher.mode`.
- Good, because idle-app rows keep the theme color.
- Bad, because `Row` carries a color.

### A `[input_bar.launcher.new_window]` table

- Good, because the three settings are grouped.
- Bad, because the launcher keys would use two shapes.

### A theme key for muted text, shared by every row

- Good, because colors would stay in one place.
- Bad, because this row is the only one that should be dim, and
  its title is not a theme setting.

## More Information

Related: [ADR-0056](0056_launcher_input.md). The item is not a
roadmap release of its own; it follows the launch-row title from
0.2.4.
