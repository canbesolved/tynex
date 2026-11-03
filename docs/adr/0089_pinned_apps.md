---
status: "accepted"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# Pinned apps

## Context and Problem Statement

The icon strip shows running apps only, so an app that is closed
cannot be started from it, and the strip changes order with use
(RM-0002). Users want a few apps to stay in place, and a key that
opens or focuses each one. How are they configured, where do they
sit, and when do their keys work?

## Decision Drivers

- Keys stay in `[hotkeys]`, in the notation of
  [ADR-0087](0087_hotkey_notation.md).
- Works on every desktop, without global shortcut support.
- No idle work: the catalog is scanned once at start, as for the
  launcher.

## Considered Options

- A list of ids plus a positional key list in `[hotkeys]`
- One list of inline tables with an app and its key
- Global shortcuts registered with the window manager

## Decision Outcome

Chosen option: "A list of ids plus a positional key list in
`[hotkeys]`", because it keeps every key in `[hotkeys]` and works
the same on every desktop.

- `taskbar.iconbar.pinned` lists desktop file ids, such as
  `"firefox"` or `"org.kde.konsole"`. An empty id, or an id listed
  twice, stops the daemon. Default: none.
- The strip shows the pinned apps first, in that order, running or
  not, then the other running apps by recent use. A pinned app that
  is not running is drawn at half opacity; `Enter`, `Space` or a
  click on it launches it.
- `hotkeys.pinned` is positional: the n-th entry opens the n-th
  pinned app. Each entry is a binding, a list of bindings, `""` or
  `[]`, as in ADR-0087. Unlike other `[hotkeys]` keys, the outer list
  is not "any of these keys". Default: `Alt+1` to `Alt+9`. `Meta+1`
  to `Meta+9` are not used because Plasma's task manager takes them
  globally.
- The keys work only while the popup is open, also while typing. A
  running app's most recent window is activated; an app that is not
  running is launched. A key past the last pinned app does nothing.
- The app catalog is scanned when the launcher is on or any app is
  pinned. It keeps terminal apps, so they can be pinned; the
  launcher list leaves them out with `gui_only`. A pinned id without
  a desktop file is kept with no icon and logged.

### Consequences

- Good, because pinned apps behave the same on every desktop and
  port.
- Good, because a pinned key is one press after the popup opens.
- Bad, because the keys need the popup open first; global keys are
  left to adapters later.
- Bad, because the n-th key and the n-th app must be kept in the
  same order by hand.

### Confirmation

Tests in `core/icons.rs`, `modes/prompt_pinned_tests.rs`,
`config/hotkeys_tests.rs`, `config/taskbar_tests.rs` and
`platform/linux/apps/tests.rs` cover the order, the keys, the config
checks and the catalog.

## Pros and Cons of the Options

### A list of ids plus a positional key list in `[hotkeys]`

- Good, because all keys are in one section.
- Bad, because the pairing is by position.

### One list of inline tables with an app and its key

- Good, because each key sits next to its app.
- Bad, because a key would live outside `[hotkeys]`.

### Global shortcuts registered with the window manager

- Good, because the key works without opening the popup.
- Bad, because each desktop needs its own registration, and a
  Wayland client cannot grab keys on its own.

## More Information

Roadmap item RM-0002. The strip is described in
[ADR-0062](0062_icon_bar.md) and the launcher catalog in
[ADR-0056](0056_launcher_input.md).
