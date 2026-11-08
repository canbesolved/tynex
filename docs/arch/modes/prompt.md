# Prompt mode

The only mode ([ADR-0050](../../adr/0050_search_only.md)). It is
called prompt mode because it will do more than search
([ADR-0051](../../adr/0051_prompt_mode.md)).

Source: `src/modes/prompt.rs`. The
[input](../modules/input.md) module rebuilds the output list
(search, then the launcher).

## Behavior

| Key                        | Action                         |
| -------------------------- | ------------------------------ |
| Printable char             | Insert at the cursor, refilter |
| `Left` / `Right`           | Move the cursor one character  |
| `Ctrl+Left` / `Ctrl+Right` | Move the cursor one word       |
| `Home` / `End`             | Jump to the start or the end   |
| `Backspace`                | Delete the character before it |
| `Delete`                   | Delete the character after it  |
| `Up` / `Down`              | Move the selection             |
| `hotkeys.select`           | Activate or launch, then close |
| `hotkeys.space`            | Insert a space, refilter       |
| `Tab` / `Shift+Tab`        | Walk the list or the icons     |
| `Esc`                      | Clear query; close if empty    |
| `hotkeys.rename` (`.`)     | Rename the selected window     |
| `hotkeys.pinned` (`Alt+n`) | Open or focus pinned app `n`   |

`hotkeys.select` defaults to `Enter` and `Space`, so `Space` is not
inserted. `hotkeys.space` defaults to a left Shift tap
([ADR-0061](../../adr/0061_prompt_keys.md), amended by
[ADR-0063](../../adr/0063_tab_icons.md) and
[ADR-0094](../../adr/0094_input_keys.md)). Source:
`src/modes/prompt_keys.rs`. `Tab` walks open windows
while the title list is showing, and matches while the output bar is
showing. With the title list off and the icon strip showing, `Tab`
moves to the next icon. With neither a list nor the strip, `Tab`
does nothing. `Shift+Tab` does the same one step back, and wraps
from the first entry to the last
([ADR-0068](../../adr/0068_back_tab.md)).

The popup opens with the previous place selected, a window or a
browser tab ([ADR-0097](../../adr/0097_history.md)). `Space` or
`Enter` switches back like Alt+Tab when the title list is on, the
icon strip is off, that app has one window, or the place is a tab.
With the title list off, the strip on, and several windows, those
keys open its
dropdown (see [taskbar.md](../ui/taskbar.md)). Typing starts at
once. After every change of the query, the first row is selected.
An open window that starts with the query comes before a later
match, and its new-window row stays under that app
([input.md](../modules/input.md)).
The bar on screen is the
[input bar](../ui/input_bar.md). Matches are drawn
in the [output bar](../ui/output_bar.md). Open windows show when
`taskbar.mode` is `titlebar` or `both` and the query is empty.
Typing hides them; clearing the query shows them again (see
[config.md](../config.md#format) and
[taskbar.md](../ui/taskbar.md)).

## Rename

`hotkeys.rename` (default `.`) edits the selected title in place
([ADR-0088](../../adr/0088_rename_windows.md)): the selected title
in an open icon dropdown, else the title list or the output bar.
Source: `src/modes/prompt_rename.rs`.

- The rename key is never typed into the query, also when nothing
  is selected. A dot cannot be searched while `.` is the key.
- While editing, characters, `Backspace`, `Delete`, `Left`, `Right`,
  `Ctrl+Left`, `Ctrl+Right`, `Home` and `End` edit the title.
  `Enter` keeps it, `Esc` cancels and leaves the query alone. Other
  keys, clicks and hover do nothing.
- A kept title replaces the window title in every list and in search
  until the window closes (see [core.md](../core.md#renamed-windows)).

Keys bound in `[hotkeys]` come first. A combination that no binding
takes counts as its plain key, so `Ctrl+Enter` confirms like `Enter`
([ADR-0087](../../adr/0087_hotkey_notation.md)).

## Pinned apps

The n-th key of `hotkeys.pinned` (default `Alt+1` to `Alt+9`) opens
the n-th app of `taskbar.iconbar.pinned` while the query is empty
([ADR-0089](../../adr/0089_pinned_apps.md)). Once something is typed
the keys act like any other key. A
running app's most recent window is activated; an app that is not
running is launched. Either closes the popup. A key past the last
pinned app does nothing. While a title is renamed, the keys wait.
The pinned apps are the first `State::pinned` icons of the strip.

## Taskbar

While `taskbar.mode` is `iconbar` or `both` and the query is empty,
`Left` and `Right` move along the icons and close an open dropdown.
The pointer does the same to the icon under it, and leaves the
selection when it moves off the icons. A left click on the icon
whose dropdown is open closes that menu. Hovering a row of the list
on screen (title list, or output rows while typing) or a dropdown
title selects it, and a click on it activates it. `with_hover`
takes a `Hover` from the `select_on_hover` keys of
`taskbar.titlebar`, `taskbar.iconbar` and `output`; each stops hover
on its rows, and clicks still work
([ADR-0101](../../adr/0101_hover_gate.md),
[ADR-0102](../../adr/0102_output_pointer.md)).
`Enter` and `Space` activate the selected title when that list is
on. With it off, they activate
one window, open the dropdown, or launch. An open dropdown still
takes `Enter` and `Space`. `Up` and `Down` move inside an open
dropdown, scrolling only to keep the title on screen; otherwise
they still move the list above. `Tab` steps that
dropdown while it is open. With the title list off, `Tab` moves to
the next icon. `Shift+Tab` steps back in the same places. `Esc`
closes the dropdown first, then clears the
query or closes the popup. A typed character hides the strip, and
`Left` and `Right` move the cursor again.
