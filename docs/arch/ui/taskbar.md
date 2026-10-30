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

mode = "icons" (default)
╭──────────────────────────────────────╮
│ >                                    │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│   ▣    ▣    ▣                        │
│        2                             │
╰──────────────────────────────────────╯

mode = "titles", show_icons = true (default)
╭──────────────────────────────────────╮
│ >                                    │
├──────────────────────────────────────┤
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
│        2                             │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│ ▣  ~/projects/demo                   │
╰──────────────────────────────────────╯
```

With `title_bar.show_icons = false` the title list drops the icon
slot. A window without an icon keeps an empty slot when icons are
on, so titles stay aligned.

The blank line between the input frame and the icon frame is a gap
of 3 logical pixels. It scales with the display, and the desktop
shows through.

The spare band between the icon frame and the title frame is empty.
The desktop shows through it until the dropdown slides in. The
dropdown starts 3 logical pixels under the icon frame. The title
list is its own frame, another 3 logical pixels below that room.
Both gaps scale with the display. When every running app has one
window, the spare room is zero and only that 3 pixel gap remains.

`mode = "titles"` keeps the title list in the input frame, divided
by a separator. `mode = "off"` leaves the popup as the input bar
until you type.

## Icon strip

- Shown when `taskbar.mode` is `icons` or `both`, and the query is
  empty.
- One icon per `app_id`, most recently used app first. That order
  does not follow `title_bar.sort_by`.
- The selected icon starts as the app of the previous window.
  `icon_bar.border` outlines it in the selection color, outside the
  icon. The default is 4 logical pixels, from 1 to 32, scaled like
  `icon_size`.
- An app with several windows draws the count under its icon. The
  strip is always tall enough for that line.
- `icon_bar.icon_size` is the icon side in logical pixels, default
  `24`, from 8 to 128. It is multiplied by the output scale, like
  `main.font_size`. A value outside that range stops the daemon.
- The gap is 0.35 of the drawn side. If the row is wider than the
  inner width, every icon and the gap shrink by the same factor
  until the row fits. A shorter row is centered.
- The spare room is as tall as the dropdown of the app with the most
  windows, capped at `output_bar.max_rows`. It is zero when every
  app has one window. It stays that tall while the strip is showing,
  so opening the dropdown does not resize the popup.
- The dropdown lists titles alphabetically, ignoring case, at most
  `output_bar.max_rows`. Its width fits the longest title, capped at
  450 logical pixels times the scale, then clamped inside the popup.
  A longer title ends with `…`.
- The dropdown slides in and out over 150 ms. The poll loop wakes
  about every 1/120 s only while a slide is running.

## Title list

The list does not show search matches. Those are the
[output bar](output_bar.md).

- Shown when `taskbar.mode` is `titles` or `both`, and the query is
  empty. Typing hides it. `Backspace` that empties the query, or
  `Esc`, shows it again.
- Width from `main.width`. Height is the input bar plus padding, the
  output bar when it is shown, and, when this list is shown, a
  separator or its own frame, and at least one row.
- Sizes follow the font: row height 2 x font size, icon slot 1.5 x,
  gaps 0.6 x; all multiplied by the output scale.
- Row in columns: app icon (with `title_bar.show_icons`), title.
  Rows have no key letters
  ([ADR-0050](../../adr/0050_search_only.md)) and no search
  highlights. A window without an icon keeps an empty slot when
  icons are on, so titles stay aligned.
- The title uses the rest of `main.width` after the icon slot. There
  is no app name column
  ([ADR-0058](../../adr/0058_title_bar_row.md)).
- Titles are truncated with an ellipsis (see [render.md](render.md)).
- At most `output_bar.max_rows` rows. The list scrolls so the
  selected window stays visible.
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
dropdown; otherwise they still move the title or output list.
`Tab` steps the dropdown while it is open. With the title list off,
`Tab` moves to the next icon. `Esc` closes the dropdown first.

A left click on an icon selects it and opens the dropdown, or
activates its one window. A left click on a dropdown row activates
that window. A left click elsewhere in the popup closes the
dropdown. The right button is ignored. Moving the pointer onto an
icon selects it and closes an open dropdown. Moving off the icons
leaves the selection where it is.

## Contract

- `icons::shows`: `taskbar.mode` is `icons` or `both`, and the query
  is empty.
- `titles::shows`: `taskbar.mode` is `titles` or `both`, and the
  query is empty.
- Strip height is the icons and the count line, or zero when hidden.
- Title height is the separator and the windows, or the framed list
  under the spare room, or zero when hidden. Pure.
- Drawing reads only `View` (`icons`, `windows`).

Shared row drawing lives in `src/ui/draw.rs`. See
[input_bar.md](input_bar.md).
