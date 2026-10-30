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
| `Space` / `Enter`          | Activate or launch, then close |
| `Tab`                      | Walk the list or the icons     |
| `Esc`                      | Clear query; close if empty    |

`Space` is not inserted
([ADR-0061](../../adr/0061_prompt_keys.md), amended by
[ADR-0063](../../adr/0063_tab_icons.md)). `Tab` walks open windows
while the title list is showing, and matches while the output bar is
showing. With the title list off and the icon strip showing, `Tab`
moves to the next icon. With neither a list nor the strip, `Tab`
does nothing. `Shift+Tab` steps forward too.

The popup opens with the previous window selected. `Space` or
`Enter` switches back like Alt+Tab when the title list is on, the
icon strip is off, or that app has one window. With the title list
off, the strip on, and several windows, those keys open its
dropdown (see [taskbar.md](../ui/taskbar.md)). Typing starts at
once; the best match is selected after every change of the query.
The bar on screen is the
[input bar](../ui/input_bar.md). Matches are drawn
in the [output bar](../ui/output_bar.md). Open windows show when
`taskbar.mode` is `titlebar` or `both` and the query is empty.
Typing hides them; clearing the query shows them again (see
[config.md](../config.md#format) and
[taskbar.md](../ui/taskbar.md)).

## Taskbar

While `taskbar.mode` is `iconbar` or `both` and the query is empty,
`Left` and `Right` move along the icons and close an open dropdown.
The pointer does the same to the icon under it, and leaves the
selection when it moves off the icons. `Enter` and `Space` activate
the selected title when that list is on. With it off, they activate
one window, open the dropdown, or launch. An open dropdown still
takes `Enter` and `Space`. `Up` and `Down` move inside an open
dropdown; otherwise they still move the list above. `Tab` steps that
dropdown while it is open. With the title list off, `Tab` moves to
the next icon. `Esc` closes the dropdown first, then clears the
query or closes the popup. A typed character hides the strip, and
`Left` and `Right` move the cursor again.
