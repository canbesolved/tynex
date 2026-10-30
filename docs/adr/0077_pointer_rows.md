---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Pointer selects rows

## Context and Problem Statement

The pointer already selects an icon. The title list and the dropdown
move only by keys, and a second click on the icon whose dropdown is
open activates the highlighted title. How does the pointer select and
activate those rows, and close that menu?

## Decision Drivers

- Hover moves the selection, the same as on an icon.
- A click activates the row under the pointer.
- A title under the pointer must not scroll away.
- `Enter` on an open dropdown still activates its highlighted title.

## Considered Options

- Hover selects, a click activates, and a second icon click closes
- Hover highlights without changing the selection
- A second click on the open icon activates, like `Enter`

## Decision Outcome

Chosen option: "Hover selects, a click activates, and a second icon
click closes", because that matches the icon strip and keeps the row
under the pointer.

- Hovering a title-list row sets the selection. A click activates
  that window. Moving off the rows leaves the selection. The
  scrollbar is not a row.
- Hovering a dropdown title sets that selection and does not
  activate. A click on the title still activates it.
- A left click on the icon whose dropdown is already open closes
  the menu and leaves the icon selected. `Enter` and `Space` still
  activate the highlighted title. This amends the "a click matches
  Enter" rule in [ADR-0062](0062_icon_bar.md) for that one case.
- The dropdown window moves only when the selected title would leave
  it, the same rule as the title list
  ([ADR-0070](0070_list_scroll.md)). `Up` and `Down` share that rule.

### Consequences

- Good, because the pointer and the keys share one selection.
- Good, because a hovered title stays under the pointer.
- Bad, because a click on the open icon no longer matches `Enter`.
- Bad, because after the first page, `Up` moves the highlight
  instead of keeping it on the last visible row.

### Confirmation

Prompt tests cover title hover and click, dropdown hover, and a
second click that closes the menu. A scrolled title list maps the
pointer to `first + row`. A scrolled dropdown keeps its first row
when the selection stays inside the window.

## Pros and Cons of the Options

### Hover selects, a click activates, and a second icon click closes

- Good, because one gesture selects and the next confirms.
- Good, because the open menu can be dismissed on its own icon.
- Bad, because the icon click and `Enter` differ while the menu is
  open.

### Hover highlights without changing the selection

- Good, because the keyboard selection would stay put.
- Bad, because the roadmap asks the pointer to move the selection.

### A second click on the open icon activates, like `Enter`

- Good, because the click would keep matching `Enter`.
- Bad, because the menu could not be closed from its icon.

## More Information

Ships in 0.2.16. The output bar is unchanged. Behavior is in
[taskbar.md](../arch/ui/taskbar.md).
