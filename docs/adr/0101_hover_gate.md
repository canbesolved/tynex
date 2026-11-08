---
status: "accepted"
date: "2026-11-07"
decision-makers: "CanBeSolved"
---
# Hover after a pointer move

## Context and Problem Statement

Hovering a title-list row selects it ([ADR-0077](0077_pointer_rows.md)).
A popup that opens under a resting cursor, for example in the middle
of the screen, gets a pointer `enter` at once, so the row under the
cursor is selected instead of the previous window. The user did not
point at it. How does hover stop doing that, and can it be turned
off for the title list and the icon strip (RM-0070)?

## Decision Drivers

- The popup opens on the previous window or tab (ADR-0097), whatever
  the cursor rests on.
- Hover still selects as soon as the user moves the pointer.
- A click always works.
- No timers and no work while the pointer rests.

## Considered Options

- Hover waits for the first pointer move, plus options to turn it
  off
- Only an option to turn hover off
- Ignore hover for a short time after the popup opens

## Decision Outcome

Chosen option: "Hover waits for the first pointer move, plus options
to turn it off", because it fixes the default for everyone
and still lets users who never want hover turn it off.

- The presenter reports hover only after the pointer moved in the
  open popup. An `enter` on a new popup and a `motion` that moves
  less than half a surface pixel are not moves. A `leave` is: the
  pointer went out, so the next `enter` hovers. Opening the popup
  resets it. Clicks and scrolling are never held back.
- This holds for every hover target: title-list rows, icons and
  dropdown titles.
- `taskbar.titlebar.select_on_hover` (default `true`): `false` makes
  hover never select a title-list row. A click still opens the row.
- `taskbar.iconbar.select_on_hover` (default `true`): `false` makes
  hover never select an icon or a title of its dropdown. A click
  still selects the icon or opens the title. The two keys are
  separate, so the strip and the title list can differ.
- ADR-0077 stays in force otherwise; its status notes this amendment.

### Consequences

- Good, because the popup opens on the previous place under a
  resting cursor, so `Enter` switches back as expected.
- Good, because the first real move selects at once.
- Bad, because a popup that moves or grows under a resting pointer
  could still report a `motion`; the popup keeps its top edge while
  it grows, so rows do not move under the pointer.

### Confirmation

Tests in `src/platform/linux/wayland/pointer_tests.rs` for the move
rule, `src/modes/prompt_pointer_tests.rs` for the options and
`src/config/taskbar_tests.rs` for the keys.

## Pros and Cons of the Options

### Only an option to turn hover off

- Good, because it is the smallest change.
- Bad, because the default stays annoying, and users who like hover
  keep the problem.

### Ignore hover for a short time after the popup opens

- Bad, because it needs a timer and a guessed delay, and a quick real
  move inside it is lost.
