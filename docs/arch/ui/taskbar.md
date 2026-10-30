# Taskbar

Source: `src/ui/taskbar/`. The icon strip is `icons.rs`, the open
windows are `titles.rs`, and the dropdown of one app's windows is
`menu.rs`. Grouping lives in `src/core/icons.rs`
([ADR-0062](../../adr/0062_icon_bar.md),
[ADR-0065](../../adr/0065_taskbar.md)).

## Modes

`taskbar.mode` picks what shows while the query is empty. Typing
hides it. Matches stay in the output bar.

```text
mode = "off"
╭──────────────────────────────────────╮
│ >                                    │
╰──────────────────────────────────────╯

mode = "iconbar" (default)
╭──────────────────────────────────────╮
│ >                                    │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│   ▣    ▣    ▣                        │
╰──────────────────────────────────────╯

mode = "titlebar", show_icons = true (default)
╭──────────────────────────────────────╮
│ >                                    │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│ ▣  ~/projects/demo                   │
│ ▣  Release notes                     │
│ ▣  Downloads                         │
╰──────────────────────────────────────╯

mode = "both", and some app has several windows
╭──────────────────────────────────────╮
│ >                                    │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│   ▣    ▣                             │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│ ▣  ~/projects/demo                   │
╰──────────────────────────────────────╯
```

With `taskbar.titlebar.show_icons = false` the title list drops
the icon slot. A window without an icon keeps an empty slot
when icons are on, so titles stay aligned.

The blank line between blocks is a gap of 3 logical pixels. It
scales with the display, and the desktop shows through. The input
bar, the icon strip, the title list and the output bar are each
their own block, with the same rounded border and theme colors
([ADR-0075](../../adr/0075_bar_blocks.md)).

The title list sits 3 logical pixels under the block above it: the
icon frame in `both` mode, the input frame in `titlebar` mode. The
dropdown starts at the same place and slides in over the title list
([ADR-0071](../../adr/0071_dropdown_overlay.md)). With
`mode = "iconbar"` there is no list to cover: a spare band under the
icon frame holds the dropdown, and the desktop shows through it until
the dropdown slides in. The gaps scale with the display.

`mode = "off"` leaves the popup as the input bar until you type.

## Icon strip

- Shown when `taskbar.mode` is `iconbar` or `both`, and the query
  is empty.
- One icon per `app_id`, most recently used app first. That order
  does not follow `taskbar.titlebar.sort_by`.
- The selected icon starts as the app of the previous window.
  `taskbar.iconbar.border` outlines it in the selection color,
  outside the icon. The default is 4 logical pixels, from 1 to 32,
  scaled like `taskbar.iconbar.size`.
- An app with several windows draws the count in a badge on the
  bottom center of its icon: the number, 0.3 of the icon side, in a
  rounded square of the frame border color, inside the icon. More
  digits widen the square
  ([ADR-0067](../../adr/0067_count_badge.md)). The strip is one icon
  tall.
- `taskbar.iconbar.size` is the icon side in logical pixels, default
  `24`, from 8 to 128. It is multiplied by the output scale, like
  `main.font_size`. A value outside that range stops the daemon.
- The gap is 0.35 of the drawn side. If the row is wider than the
  inner width, every icon and the gap shrink by the same factor
  until the row fits. A shorter row is centered.
- In `iconbar` mode the spare room is as tall as the dropdown of the
  app with the most windows, capped at `output.max_rows`. It is zero
  when every app has one window. It stays that tall while the strip
  is showing, so opening the dropdown does not resize the popup.
- The dropdown lists titles alphabetically, ignoring case, at most
  `output.max_rows`. In `both` mode it also shows no more rows than
  the title list, so the popup does not grow. A longer dropdown
  keeps the selected title visible, and moves only when that title
  would leave the window. Its width fits the
  longest title, capped at 450 logical pixels times the scale, then
  clamped inside the popup.
  A longer title ends with `…`.
- The dropdown slides in and out over 150 ms. The poll loop wakes
  about every 1/120 s only while a slide is running.

## Title list

The list does not show search matches. Those are the
[output bar](output_bar.md).

- Shown when `taskbar.mode` is `titlebar` or `both`, and the query is
  empty. Typing hides it. `Backspace` that empties the query, or
  `Esc`, shows it again.
- Width from `main.width`. Height is the input frame, the gap under
  it, the output frame when it is shown, and, when this list is
  shown, its own frame of at least one row.
- Sizes follow the font: row height 2 x font size, icon slot 1.5 x,
  gaps 0.6 x; all multiplied by the output scale.
- Row in columns: app icon (with `taskbar.titlebar.show_icons`)
  and the title. Rows have no key letters
  ([ADR-0050](../../adr/0050_search_only.md)) and no search
  highlights. A window without an icon keeps an empty slot when
  icons are on, so titles stay aligned.
- The title uses the rest of `main.width` after the icon slot. There
  is no app name column
  ([ADR-0058](../../adr/0058_title_bar_row.md)).
- Titles are truncated with an ellipsis (see [render.md](render.md)).
- At most `output.max_rows` rows, fewer on a short screen
  ([fit.md](fit.md)). A longer list scrolls: with the wheel or the
  touchpad, and to keep a newly selected window visible. A scrollbar
  shows while it scrolls ([scroll.md](scroll.md)).
- The selected row is highlighted. `Tab` moves that highlight one
  row forward and wraps. `Up` and `Down` move it too
  ([ADR-0061](../../adr/0061_prompt_keys.md)).

## Keys and clicks

While the strip is showing, `Left` and `Right` move the selection
and close an open dropdown. With the title list off, `Enter` and
`Space` match a left click on the selected icon: one window
activates, several open the dropdown, and none launches that app.
With the title list on, those keys activate the selected title. An
open dropdown still takes them. `Up` and `Down` move inside an open
dropdown, and the window moves only to keep that title on screen.
Otherwise they still move the title or output list.
`Tab` steps the dropdown while it is open. With the title list off,
`Tab` moves to the next icon. `Shift+Tab` steps back in the same
places, in the title list too, and wraps the same way. `Esc` closes
the dropdown first.

A left click on an icon selects it and opens the dropdown, or
activates its one window. A left click on that same icon while its
dropdown is open closes the menu and leaves the icon selected.
`Enter` and `Space` still activate the highlighted title. A left
click on a dropdown row activates that window. Hovering a dropdown
title selects it and does not activate. A left click elsewhere in
the popup closes the dropdown. The right button is ignored. Moving
the pointer onto an icon selects it and closes an open dropdown.
Moving off the icons leaves the selection where it is.

Hovering a title-list row selects that window. A click activates
it. Moving off the rows leaves the selection. The scrollbar is not
a row. The output bar has no pointer selection.

## Contract

- `icons::shows`: `taskbar.mode` is `iconbar` or `both`, and the
  query is empty.
- `titles::shows`: `taskbar.mode` is `titlebar` or `both`, and the
  query is empty.
- Strip height is one icon side, or zero when hidden.
- Title body is the window rows, or zero when hidden. The frame
  around that body is padding above and below. Pure.
- `frames::overlays`: the dropdown covers the title list.
- `menu::Place` keeps the dropdown's first row and moves it only
  when the selection would leave the window. A click on shown row
  `n` picks `first + n`.
- Drawing reads only `View` (`icons`, `windows`).

Shared row drawing lives in `src/ui/draw.rs`. See
[input_bar.md](input_bar.md).
