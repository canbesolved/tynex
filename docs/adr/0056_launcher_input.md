---
status: "accepted"
date: "2026-10-28"
decision-makers: "CanBeSolved"
---
# Launcher under the input bar

## Context and Problem Statement

The popup has five UI modules. Installed apps must appear in search
results and start from the same popup. Should the launcher be its
own bar, or a sub-module of an existing one?

## Decision Drivers

- The output bar already draws the result of the input
  ([ADR-0054](0054_output_bar.md)).
- `ui/` stays free of OS types; desktop files and `Exec` stay in the
  platform layer.
- No new crate for desktop entries ([ADR-0045](0045_app_names.md)).
- Config keys sit with their module
  ([ADR-0055](0055_config_sections.md)).

## Considered Options

- Launcher as an input-bar sub-module; output bar draws the rows
- Launcher as its own UI bar
- Use a desktop-entry crate for `Exec`

## Decision Outcome

Chosen option: "Launcher as an input-bar sub-module; output bar
draws the rows", because the input owns the query and the output
bar already shows its result.

- Config: `launcher.mode` under `[input_bar]`: `"off"` (default),
  `"all"`, or `"gui_only"`.
- `src/modules/input/` runs search, then the launcher, into one
  output list. Prompt mode keeps the keys and asks that module to
  refresh.
- Core `AppInfo` is `id`, `name`, `icon`. A row target is a window
  or a launch. `Action::Launch` starts the app.
- Linux: own `.desktop` scan and `Exec` parser in
  `platform/linux/apps.rs`. No new crate.

### Consequences

- Good, because there is no sixth bar and the output bar stays the
  single result list.
- Good, because the core stays pure and `Exec` stays on the
  platform.
- Bad, because the desktop-entry id and `Exec` rules are maintained
  here.

### Confirmation

Unit tests cover config values, grouping, `Exec` splitting, and
`Enter` returning `Launch`. Architecture pages name the input
sub-module.

## Pros and Cons of the Options

### Launcher as an input-bar sub-module; output bar draws the rows

- Good, because the ownership matches the query and the result.
- Good, because later input results share the same output bar.
- Bad, because prompt mode must distinguish activate and launch.

### Launcher as its own UI bar

- Good, because the strip of apps is separate from search.
- Bad, because that role is the icon bar (0.2.2), not the launcher.
- Bad, because matching apps would still need a result list.

### Use a desktop-entry crate for `Exec`

- Good, because field codes are already handled.
- Bad, because of weight and license, as in ADR-0045.

## More Information

Related: [ADR-0045](0045_app_names.md), [ADR-0054](0054_output_bar.md),
[ADR-0055](0055_config_sections.md), [ROADMAP.md](../../ROADMAP.md).
