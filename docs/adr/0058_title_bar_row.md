---
status: "accepted"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Title bar icon and title

## Context and Problem Statement

[ADR-0049](0049_popup_columns.md) gave the title bar an optional app
name column, switched by `show_app_icon`. The row the user reads is
the window title, with the app icon beside it. The app name is
already used for search and for `strip_app_name`. Should the column
stay, and can the icon be turned off?

## Decision Drivers

- A title-bar row is the app icon and the window title.
- Titles stay aligned when a window has no icon.
- The output bar keeps its icon, whatever the title bar does.
- An old config key must fail loudly, as in ADR-0031.

## Considered Options

- Icon and title, with `show_icons`; remove the app name column
- Keep `show_app_icon` and add `show_icons` beside it

## Decision Outcome

Chosen option: "Icon and title, with `show_icons`; remove the app
name column", because the app name is not a column the row needs.

- `title_bar.show_icons` defaults to `true`. Each title-bar row is
  the app icon and the title. A window without an icon keeps an empty
  slot, so titles stay aligned. `false` shows titles only, with no
  icon slot.
- The output bar always draws the icon slot and the title.
- **Breaking:** `title_bar.show_app_icon` is removed. A config that
  still sets it stops the daemon with an unknown key error.
- App names stay on `Row`. Search still matches them, and
  `strip_app_name` still drops them from titles. They are not drawn.

This replaces the app-name column in ADR-0049. ADR-0049 still decides
that rows are columns sized from `main.width`, and that `title_format`
is gone.

### Consequences

- Good, because the title bar matches the row the user came for.
- Good, because `show_icons = false` is a new key, so files that
  never set `show_app_icon` keep working.
- Bad, because a file that sets `show_app_icon` stops the daemon
  until the key is removed.

### Confirmation

`src/ui/draw.rs` draws the icon slot only when asked, then the title.
A config test rejects `show_app_icon`. A painter test checks that
`show_icons = false` starts the title where the icon slot was, and
that the output bar still reserves the slot.

## Pros and Cons of the Options

### Icon and title, with `show_icons`; remove the app name column

- Good, because one switch describes the icon, not a second name.
- Good, because search still sees the app name.
- Bad, because the app name is no longer on screen.

### Keep `show_app_icon` and add `show_icons` beside it

- Good, because current configs keep the column.
- Bad, because the key name says icon and the column is the app name.
- Bad, because two switches describe one row.

## More Information

The item is 0.2.3 in [ROADMAP.md](../../ROADMAP.md), including the
former Later item for title bar app icons.
