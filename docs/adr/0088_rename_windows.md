---
status: "accepted"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# Rename windows

## Context and Problem Statement

Several windows of one app often share a title, or have titles that
say little, for example two terminals named `~`. Users want to give
a window their own name from the popup (RM-0001). Where does the new
title live, how is it edited, and which key starts it?

## Decision Drivers

- Works on every desktop, without a protocol to set titles.
- No idle work, no files: footprint first.
- One key, no mouse needed; consistent with the query editing.
- The new name is what search finds.

## Considered Options

- tynex's own title, in memory until the window closes
- Set the real window title through the window manager
- Keep renames in a file across restarts

## Decision Outcome

Chosen option: "tynex's own title, in memory until the window
closes", because it needs no window manager support and no storage,
and it lasts exactly as long as the window it names.

- `hotkeys.rename` (default `"."`, notation from
  [ADR-0087](0087_hotkey_notation.md)) starts editing the selected
  title: in an open icon dropdown, in the title list, or in the
  output bar. With nothing selected it does nothing.
- The rename key never reaches the query, so with the default `.`
  a dot cannot be typed into a search. A user who needs dots sets
  another key, such as `"F2"`.
- The title is edited in place with the query's editing keys:
  characters, `Backspace`, `Delete`, `Left`, `Right`, `Ctrl+Left`,
  `Ctrl+Right`, `Home`, `End`. `Enter` keeps it, `Esc` cancels.
  Other keys and the pointer do nothing until then. Closing the
  popup cancels.
- The rows keep their titles while editing; the view draws the
  edited text with a beam in their place. The dropdown order does
  not move while typing.
- The switcher keeps a map from window id to the new title and the
  window's own title. Each `Windows` event replaces the title of a
  renamed window and drops the windows that closed. A blank title or
  the window's own title removes the rename.
- Search, the title list, the output bar and the dropdown use the
  new title. The window's own title is not matched.

### Consequences

- Good, because it works with any window source, now and on later
  ports.
- Good, because it costs one map entry per renamed window.
- Bad, because renames are lost when the daemon restarts.
- Bad, because with the default key, dots cannot be typed into a
  search.

### Confirmation

Unit tests in `core/rename_tests.rs` and `modes/prompt_rename_tests.rs`
cover editing, keeping, cancelling, the window closing, and the
switcher end to end.

## Pros and Cons of the Options

### tynex's own title, in memory until the window closes

- Good, because no window manager support is needed.
- Bad, because other programs still show the old title.

### Set the real window title through the window manager

- Good, because every program would show the new title.
- Bad, because KWin scripts cannot set a window caption, and other
  window managers differ or do not allow it.

### Keep renames in a file across restarts

- Good, because renames survive a restart.
- Bad, because window ids change between sessions, so a stored name
  cannot be matched back to its window reliably.

## More Information

Roadmap item RM-0001. Keys are bound with the notation of
[ADR-0087](0087_hotkey_notation.md).
