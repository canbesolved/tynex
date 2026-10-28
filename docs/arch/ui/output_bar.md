# Output bar

Source: `src/ui/output_bar.rs`. The result of what was typed in the
[input bar](input_bar.md).

The result depends on the request. A query shows the windows it
matched, and with the launcher on, launch rows for installed apps
([ADR-0054](../../adr/0054_output_bar.md),
[ADR-0056](../../adr/0056_launcher_input.md),
[modules/input.md](../modules/input.md)). Later requests can show a
model answer in this same module.

## Look

```text
After typing, title_bar.enabled = false
╭──────────────────────────────────────╮
│ / firef                              │
├──────────────────────────────────────┤
│ ▣  Firefox   Release notes           │
│ ▣  Firefox   Café                    │
╰──────────────────────────────────────╯

After typing, title_bar.enabled = true
╭──────────────────────────────────────╮
│ / firef                              │
├──────────────────────────────────────┤
│ ▣  Firefox   Release notes           │
├──────────────────────────────────────┤
│ ▣  Konsole   ~/projects/demo         │
│ ▣  Firefox   Release notes           │
│ ▣  Dolphin   Downloads               │
╰──────────────────────────────────────╯
```

The first list under the input is the output bar. With
`title_bar.enabled` the open windows stay below it, unfiltered. See
[title_bar.md](title_bar.md).

## Rules

- Shown only when the query is non-empty. There is no config key to
  hide it.
- An empty match list still keeps one row of height, so the popup
  does not collapse.
- At most `output_bar.max_rows` rows. The list scrolls so the
  selected match stays visible.
- Rows are icon and title only. The app name column is title-bar
  only (`title_bar.show_app_icon`). Matched letters use the highlight
  color (see [render.md](render.md)).
- `Enter` activates a window row or launches an app row. Up and down
  move that selection. Prompt mode owns the keys; this module only
  draws.

## Contract

- `shows`: the query is non-empty.
- `height`: the separator and the result rows, or zero when hidden.
  Pure.
- `draw`: draws the separator, the selection and the columns. Reads
  only `View` (`rows`, the output list).
