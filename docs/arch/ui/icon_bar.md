# Icon bar

Source: `src/ui/icon_bar.rs`. One icon per running app, under the
input bar. The dropdown of one app's windows is `src/ui/icon_menu.rs`.
Grouping lives in `src/core/icons.rs`
([ADR-0062](../../adr/0062_icon_bar.md)).

## Look

```text
icon_bar.enabled = true (default), query empty
╭──────────────────────────────────────╮
│ /                                    │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│   ▣    ▣    ▣                        │
│        2                             │
╰──────────────────────────────────────╯

title bar on, and some app has several windows
╭──────────────────────────────────────╮
│ /                                    │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│   ▣    ▣                             │
│        2                             │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│ ▣  ~/projects/demo                   │
╰──────────────────────────────────────╯
```

The blank line between the input frame and the icon frame is a gap
of 5 logical pixels. It scales with the display, and the desktop
shows through.

The spare band between the icon frame and the title frame is empty.
The desktop shows through it until the dropdown slides in under the
selected icon. When every running app has one window, that band is
zero and the title list stays in the icon frame, under a separator.

Typing hides the strip, the band and the dropdown. Matches stay in
the output bar. `icon_bar.enabled = false` leaves the popup as it is
without this module.

## Rules

- Shown when `icon_bar.enabled` is set and the query is empty.
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

## Keys and clicks

While the strip is showing, `Left` and `Right` move the selection
and close an open dropdown. `Enter` and `Space` match a left click
on the selected icon: one window activates, several open the
dropdown, and none launches that app. `Up`, `Down` and `Tab` move
inside an open dropdown. Otherwise they still move the title or
output list. `Esc` closes the dropdown first.

A left click on an icon selects it and does the same as `Enter`. A
left click on a dropdown row activates that window. A left click
elsewhere in the popup closes the dropdown. The right button is
ignored.

See [input_bar.md](input_bar.md) and [title_bar.md](title_bar.md).
