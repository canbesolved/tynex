---
status: "accepted"
date: "2026-10-28"
decision-makers: "CanBeSolved"
---
# Popup UI modules

## Context and Problem Statement

[ADR-0051](0051_prompt_mode.md) left one mode. The popup was still a
layout: `layout = "vertical_list"` and a `Layout` trait, with the
input bar and the window list drawn as one unit. Later plans add an
info bar and an icon bar to that same popup. A layout you pick no
longer describes it. How should the popup be structured?

## Decision Drivers

- One mode, so there is no second arrangement to select.
- The prompt, the info bar, the icon bar and the title bar are
  separate parts of one popup.
- Info bar and icon bar are later work; the structure should name
  them without building them yet.
- An old config key must fail loudly, as in ADR-0031.

## Considered Options

- UI modules stacked by the painter
- Keep the layout trait and the `layout` key

## Decision Outcome

Chosen option: "UI modules stacked by the painter", because the popup
is those parts, not a layout chosen in config.

- The input bar module (`src/ui/input_bar.rs`) draws the input bar.
- The title bar module (`src/ui/title_bar.rs`) draws the window list.
- The painter (`src/ui/painter.rs`) sizes the surface and draws the
  frame, then the input bar, then the title bar.
- The info bar and the icon bar are the same kind of module when they
  are built. They stay on the roadmap until then.
- **Breaking:** the `layout` config key is removed. A file that still
  sets it stops the daemon until the key is removed.

Prompt mode stays the mode in `src/modes/prompt.rs`. It handles keys.
The input bar module only draws the bar.

This replaces the layout seam described in
[ADR-0005](0005_trait_modules.md). ADR-0005 still decides trait
modules, one adapter and one presenter. It does not decide a layout.

### Consequences

- Good, because a new bar is a module the painter stacks, not a new
  layout and a config value.
- Good, because prompt mode and the input bar stay different things.
- Bad, because existing config files that set `layout` stop the
  daemon until the key is removed.

### Confirmation

`src/ui/` has `input_bar.rs` and `title_bar.rs` and no `layouts/`
module.
`layout` is absent from `src/config/default.toml`. A config test
rejects `layout` as an unknown key.

## Pros and Cons of the Options

### UI modules stacked by the painter

- Good, because each bar can change without a layout enum.
- Good, because the config no longer offers a choice of one value.
- Bad, because docs and config files that say `layout` must change.

### Keep the layout trait and the `layout` key

- Good, because current configs keep working.
- Bad, because the only value is `vertical_list`, so the key chooses
  nothing.
- Bad, because an info bar and an icon bar are not layouts.

## More Information

Planned bars are the info bar and the icon bar in
[ROADMAP.md](../../ROADMAP.md).
