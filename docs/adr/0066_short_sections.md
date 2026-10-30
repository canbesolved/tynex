---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Short config sections

## Context and Problem Statement

The file has seven sections. `[input_bar]` and `[output_bar]` name
the same bars as `[input]` and `[output]` would. `[title_bar]` and
`[icon_bar]` hold keys of the taskbar, which already has `[taskbar]`
for `mode`. Should the file shrink to five sections, and should the
mode values use the same words as the groups inside `[taskbar]`?

## Decision Drivers

- Five sections: `[main]`, `[hotkeys]`, `[input]`, `[output]`,
  `[taskbar]`.
- Title and icon settings stay grouped, the way `launcher` and
  `hint` are grouped under `[input]`.
- `mode` uses the same words as those groups.
- An old section or an old mode value must fail until the file is
  edited.

## Considered Options

- Five sections, `titlebar` and `iconbar` groups, matching mode
  values.
- Shorten only `[input_bar]` and `[output_bar]`.
- Flat keys in `[taskbar]`, and keep `icons` and `titles`.

## Decision Outcome

Chosen option: "Five sections, `titlebar` and `iconbar` groups,
matching mode values", because the file then has one section per
part, and the mode names the same groups the keys use.

- `[input_bar]` becomes `[input]`. `[output_bar]` becomes
  `[output]`.
- `title_bar.sort_by`, `title_bar.show_icons` and
  `title_bar.strip_app_name` become `taskbar.titlebar.sort_by`,
  `taskbar.titlebar.show_icons` and
  `taskbar.titlebar.strip_app_name`.
- `icon_bar.icon_size` becomes `taskbar.iconbar.size`.
  `icon_bar.border` becomes `taskbar.iconbar.border`.
- **Breaking:** `taskbar.mode` values `icons` and `titles` become
  `iconbar` and `titlebar`. `off` and `both` stay. The default is
  `iconbar`.
- **Breaking:** a file that still has `[input_bar]`, `[output_bar]`,
  `[title_bar]` or `[icon_bar]`, or a mode value `icons` or
  `titles`, stops the daemon.

This amends the section order in
[ADR-0055](0055_config_sections.md). It amends
[ADR-0065](0065_taskbar.md): the keys that stayed in the old
sections move into `[taskbar]`, and the mode values are renamed.

### Consequences

- Good, because the section list matches the parts of the popup.
- Good, because `mode` and the groups use one pair of names.
- Bad, because every existing config file must be edited.

### Confirmation

`src/config/default.toml` uses the five sections. Config tests
reject each old section and the mode values `icons` and `titles`.
`docs/arch/config.md` names the sections and the keys.

## Pros and Cons of the Options

### Five sections, `titlebar` and `iconbar` groups, matching mode values

- Good, because title keys and icon keys stay apart.
- Good, because `iconbar` means both the mode and the size keys.
- Bad, because `mode` values that shipped in 0.2.10 are renamed.

### Shorten only `[input_bar]` and `[output_bar]`

- Good, because title and icon keys stay where 0.2.10 left them.
- Bad, because the file still has two sections for one module.
- Bad, because 0.2.11 on the roadmap is the merge into `[taskbar]`.

### Flat keys in `[taskbar]`, and keep `icons` and `titles`

- Good, because no nested names.
- Bad, because `show_icons` and `icon_size` sit side by side.
- Bad, because `mode = "icons"` and an `icons` key would differ
  only by being a value or a table.

## More Information

[ADR-0055](0055_config_sections.md),
[ADR-0065](0065_taskbar.md). The section list is 0.2.11 in
[ROADMAP.md](../../ROADMAP.md).
