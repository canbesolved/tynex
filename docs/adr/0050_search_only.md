---
status: "proposed"
date: "2026-10-27"
decision-makers: "CanBeSolved"
---
# Search mode only, window list as title_bar

## Context and Problem Statement

The popup has two modes: hotkey mode (default) gives each window a
letter, search mode filters windows by a typed query. Letters are
pinned ([ADR-0041](0041_pinned_keys.md)) and rows are kept in letter
order ([ADR-0048](0048_row_order.md)), but in practice hotkey mode
does not work out: the user still reads the popup before each key
press. Planned features, such as the task list (`icon_bar`), build on
search. Should hotkey mode stay?

## Decision Drivers

- One way to switch windows, with fewer keys and config options.
- Alt+Tab habit: `Enter` right after opening goes back to the
  previous window.
- The window list stays available for users who want to see it.
- Less code in the core and the modes.

## Considered Options

- Search mode only, window list as the `title_bar` option
- Keep both modes
- Search mode only, without a window list

## Decision Outcome

Chosen option: "Search mode only, window list as the `title_bar`
option", because it drops the part that does not work and keeps the
list for users who want it.

- Search mode is the only mode. `src/modes/hotkey.rs` and
  `src/core/keys.rs` are removed; `Switcher` drops its hotkey mode
  parameter.
- `title_bar = false` (default) hides the window list; `true` shows it
  under the prompt, or under the icon strip when `icon_bar` is on.
- Rows show the icon, the app name with `show_app`, and the title; no
  key letters. `strip_app_name` now defaults to `false`.
- The popup opens with the previous window selected, so `Enter` keeps
  its Alt+Tab meaning (as in ADR-0048).
- Removed config: `mode`, `keys` and `sort_by = "key"`. `sort_by`
  defaults to `recently`.
- Removed CLI: `tynex show --mode`. The D-Bus `Show` call keeps its
  string argument and accepts only the empty string.
- Breaking: a config file with `mode`, `keys` or `sort_by = "key"`
  stops the daemon until edited ([ADR-0031](0031_strict_config.md)).

This supersedes ADR-0041 and amends ADR-0048, which loses `key`.

### Consequences

- Good, because there is one mode to learn, document and test.
- Good, because key assignment, pinning and letter order are gone.
- Good, because the task list and launcher build on the same mode.
- Bad, because a window can no longer be picked with one key press
  without typing.
- Bad, because users with `mode`, `keys` or `sort_by = "key"` must
  edit their config, and scripts with `tynex show --mode` break.

### Confirmation

Config tests check that `mode` and `keys` are unknown keys and that
`sort_by = "key"` is rejected. Layout tests check that rows are drawn
only with `title_bar = true` and have no key column. A CLI test checks
that `--mode` is rejected.

## Pros and Cons of the Options

### Keep both modes

- Good, because no config breaks.
- Bad, because hotkey mode does not work out in practice, and both
  modes must be kept in sync with new features.

### Search mode only, without a window list

- Good, because it is the smallest change in code.
- Bad, because users who want to see their windows lose the list.

## More Information

Planned in [ROADMAP.md](../../ROADMAP.md) for 0.1.6. On release, the
removed keys get a **Breaking:** entry in `CHANGELOG.md` and an entry
in `DEPRECATIONS.md`. Related: [ADR-0015](0015_key_order.md),
[ADR-0049](0049_popup_columns.md).
