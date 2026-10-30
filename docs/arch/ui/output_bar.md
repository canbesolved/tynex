# Output bar

Source: `src/ui/output_bar.rs`. The result of what was typed in the
[input bar](input_bar.md).

The result depends on the request. A query shows the windows it
matched, then, with the launcher on, launch rows for installed apps
([ADR-0054](../../adr/0054_output_bar.md),
[ADR-0056](../../adr/0056_launcher_input.md),
[ADR-0078](../../adr/0078_windows_before_apps.md),
[ADR-0079](../../adr/0079_leading_match.md),
[modules/input.md](../modules/input.md)). A window whose title or
app name starts with the query comes before the other windows, and
its new-window row stays under that app. Later requests can show a
model answer in this same module.

## Look

```text
After typing
╭──────────────────────────────────────╮
│ > firef                              │
╰──────────────────────────────────────╯

╭──────────────────────────────────────╮
│ ▣  Firefox   Release notes           │
│ ▣  Firefox   Café                    │
╰──────────────────────────────────────╯
```

The list under the input is the output bar, in its own frame. The
title list is hidden while the query is non-empty. Clearing the
query shows the open windows again when `taskbar.mode` is
`titlebar` or `both`. See [taskbar.md](taskbar.md).

## Rules

- Shown only when the query is non-empty. There is no config key to
  hide it.
- An empty match list still keeps one row of height, so the popup
  does not collapse.
- At most `output.max_rows` rows, fewer on a short screen
  ([fit.md](fit.md)). More matches scroll: with the wheel or the
  touchpad, and to keep a newly selected match visible. A scrollbar
  shows while it scrolls ([scroll.md](scroll.md)).
- Rows are icon and title. The icon slot is always reserved here.
  Matched letters use the highlight color (see [render.md](render.md)).
  A new-window row uses its configured title color
  ([ADR-0059](../../adr/0059_new_window_row.md)).
- `Enter` and `Space` activate a window row or launch an app row.
  `Tab`, `Shift+Tab`, up and down move that selection
  ([ADR-0061](../../adr/0061_prompt_keys.md)). Prompt mode owns the
  keys; this module only draws.

## Contract

- `shows`: the query is non-empty.
- `body`: the result rows, or zero when hidden. The frame around
  them is drawn with the other blocks. Pure.
- `draw`: draws the selection and the columns. Reads only `View`
  (`rows`, the output list).
