---
status: "accepted"
date: "2026-10-28"
decision-makers: "CanBeSolved"
---
# Config sections per module

## Context and Problem Statement

The config mixed process settings, the popup frame, row columns and
module switches. Top-level keys sat next to `[appearance]`, which held
both the frame and the title-bar switch. Later modules
(`launcher`, `icon_bar`, `status_bar`) would keep landing in that one
table. How should the file be split?

## Decision Drivers

- One section per part or module, so a new bar adds a table.
- An old key must fail loudly, as in ADR-0031.
- The painter and each module receive only the sub-struct they need.
- The section order stays stable: main, hotkeys, input bar, output
  bar, then the other modules.

## Considered Options

- One section per part and module
- Keep `[appearance]` and add new tables beside it

## Decision Outcome

Chosen option: "One section per part and module", because the file
mirrors the popup modules and process settings.

- `[main]`: `desktop`, `font`, `font_size`, `width`.
- `[hotkeys]`: `action` (was `hotkey_action`).
- `[input_bar]`: empty in 0.2.0; `launcher.mode` comes in 0.2.1.
- `[output_bar]`: `max_rows` (shared cap with the title bar).
- `[title_bar]`: `enabled`, `sort_by`, `show_app_icon`,
  `strip_app_name`.
- **Breaking:** `[appearance]` and the old top-level keys are
  removed. A file that still has them stops the daemon until edited.
  `appearance.show_app` becomes `title_bar.show_app_icon`.

Sections stay in this order: `[main]`, `[hotkeys]`, `[input_bar]`,
`[output_bar]`, then `[title_bar]`, `[icon_bar]`, `[status_bar]`.
Per-app tables come last.

### Consequences

- Good, because a new module gets its own table.
- Good, because the title-bar switch is no longer under appearance.
- Bad, because every existing config file must be rewritten by hand.

### Confirmation

`src/config/default.toml` uses the new sections. Config tests reject
each old key and `[appearance]`. Architecture pages name the
sections.

## Pros and Cons of the Options

### One section per part and module

- Good, because the file follows the UI modules.
- Good, because `show_app_icon` and `strip_app_name` sit with the
  title bar that owns the row look.
- Bad, because every config file breaks once.

### Keep `[appearance]` and add new tables beside it

- Good, because font and width keys stay where they are.
- Bad, because the title-bar switch and row columns stay mixed with
  the frame.
- Bad, because later modules still have no clear home for shared
  look keys.

## More Information

Related: [ADR-0031](0031_strict_config.md),
[ADR-0053](0053_ui_module_toggles.md),
[config.md](../arch/config.md).
