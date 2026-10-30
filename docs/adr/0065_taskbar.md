---
status: "accepted, amended by ADR-0066"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Taskbar module with modes

## Context and Problem Statement

The icon strip and the title list are two modules, each with its own
`enabled` key. They always sit in the same popup and share the rule
that typing hides them. Should they stay two switches, or become one
taskbar with a mode?

## Decision Drivers

- One module owns the strip, the title list and the dropdown.
- The four combinations that exist today stay available, including
  both parts off.
- 0.2.11 moves the remaining keys into `[taskbar]`. This release only
  replaces the two `enabled` keys.
- An old config must fail until it is edited.

## Considered Options

- One `[taskbar] mode` key.
- Keep both `enabled` keys and derive the mode in code.
- Put `mode` inside `[title_bar]`.

## Decision Outcome

Chosen option: "One `[taskbar] mode` key", because the strip and the
title list are one module, and one key names every combination.

- `[taskbar] mode` is `off`, `icons`, `titles` or `both`. The default
  is `icons`, which matches today's defaults (`icon_bar.enabled` on,
  `title_bar.enabled` off).
- **Breaking:** `icon_bar.enabled` and `title_bar.enabled` are
  removed. A file that still sets either key stops the daemon.
- `title_bar.sort_by`, `title_bar.show_icons`,
  `title_bar.strip_app_name`, `icon_bar.icon_size` and
  `icon_bar.border` stay where they are until 0.2.11.
- Drawing, keys and clicks for each combination stay as they are.
  With the title list on, `Enter` and `Space` still confirm that
  list ([ADR-0064](0064_title_priority.md)).

This amends [ADR-0053](0053_ui_module_toggles.md): the strip and the
title list share one switch. It amends the section order in
[ADR-0055](0055_config_sections.md) by adding `[taskbar]`. It amends
[ADR-0062](0062_icon_bar.md): the strip is part of the taskbar, shown
when the mode is `icons` or `both`.

### Consequences

- Good, because the four layouts are one setting.
- Good, because an old file cannot silently drop a part.
- Bad, because existing config files must be edited.
- Bad, because icon size and row order stay in the old sections
  until 0.2.11.

### Confirmation

`taskbar.mode` parses to `TaskbarMode`. The painter and prompt mode
take that mode. `docs/arch/ui/taskbar.md` describes the four modes.

## Pros and Cons of the Options

### One `[taskbar] mode` key

- Good, because `off`, `icons`, `titles` and `both` are named.
- Good, because 0.2.11 can move the other keys without a second
  break of the switch.
- Bad, because two keys that shipped are removed.

### Keep both `enabled` keys and derive the mode in code

- Good, because no config file changes.
- Bad, because the module still looks like two switches.
- Bad, because 0.2.11 would break the same keys anyway.

### Put `mode` inside `[title_bar]`

- Good, because no new section is needed yet.
- Bad, because a title-bar key would also hide the icon strip.
- Bad, because 0.2.11 would move the key again.

## More Information

[ADR-0053](0053_ui_module_toggles.md),
[ADR-0055](0055_config_sections.md),
[ADR-0062](0062_icon_bar.md),
[ADR-0064](0064_title_priority.md).
The section merge is 0.2.11 in [ROADMAP.md](../../ROADMAP.md).
