---
status: "accepted"
date: "2026-10-28"
decision-makers: "CanBeSolved"
---
# Toggle UI modules independently

## Context and Problem Statement

[ADR-0052](0052_ui_modules.md) made the popup a stack of UI modules
instead of a layout. The modules are the input bar, the output bar,
the info bar, the title bar and the icon bar. Which of them can be
turned off, and may one change without the others?

## Decision Drivers

- The input bar is the core of the popup: typing starts there.
- The info bar, the title bar and the icon bar are optional.
- A change to one module should stay inside that module.
- Info bar and icon bar are later work; the rule should hold before
  they exist.

## Considered Options

- Each module has its own switch, except the input bar
- One switch for the whole popup

## Decision Outcome

Chosen option: "Each module has its own switch, except the input
bar", because the modules are independent parts of one popup.

- The input bar is always on. It has no config key to hide it.
- The title bar, the info bar and the icon bar can each be enabled
  or disabled. Turning one off leaves the others as they are.
- Each module can be changed on its own: its look, its config and
  its code stay in that module. The painter only stacks the modules
  that are enabled.
- Today the title bar's switch is `appearance.title_bar`. It shows
  or hides the open windows. Search matches are the output bar
  ([ADR-0054](0054_output_bar.md)).
- The info bar and the icon bar get their own switches when they are
  built. They stay on the roadmap until then.

This adds the enable rule to ADR-0052. ADR-0052 still decides that
the popup is these modules and that `layout` is removed.

### Consequences

- Good, because a bar can be added, removed or redesigned without a
  new layout.
- Good, because the input bar cannot disappear and leave the popup
  with nowhere to type.
- Bad, because a module that is off still has to agree with the
  painter on height, so the frame can shrink.

### Confirmation

`src/ui/input_bar.rs` is drawn on every popup. `appearance.title_bar`
shows or hides the open windows. Architecture pages name the modules
and say the input bar is always on.

## Pros and Cons of the Options

### Each module has its own switch, except the input bar

- Good, because the info bar, the title bar and the icon bar can
  ship and change separately.
- Good, because the input bar stays the stable core.
- Bad, because each new module needs its own enable rule.

### One switch for the whole popup

- Good, because there is a single config key.
- Bad, because hiding the title bar would hide the info bar too.
- Bad, because the input bar could be turned off.

## More Information

The info bar and the icon bar are planned in
[ROADMAP.md](../../ROADMAP.md).
