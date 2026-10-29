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
After typing
╭──────────────────────────────────────╮
│ / firef                              │
├──────────────────────────────────────┤
│ ▣  Firefox   Release notes           │
│ ▣  Firefox   Café                    │
╰──────────────────────────────────────╯
```

The list under the input is the output bar. The title bar is hidden
while the query is non-empty. Clearing the query shows the open
windows again when `title_bar.enabled` is set. See
[title_bar.md](title_bar.md).

## Rules

- Shown only when the query is non-empty. There is no config key to
  hide it.
- An empty match list still keeps one row of height, so the popup
  does not collapse.
- At most `output_bar.max_rows` rows. The list scrolls so the
  selected match stays visible.
- Rows are icon and title. The icon slot is always reserved here.
  Matched letters use the highlight color (see [render.md](render.md)).
  A new-window row uses its configured title color
  ([ADR-0059](../../adr/0059_new_window_row.md)).
- `Enter` activates a window row or launches an app row. Up and down
  move that selection. Prompt mode owns the keys; this module only
  draws.

## Contract

- `shows`: the query is non-empty.
- `height`: the separator and the result rows, or zero when hidden.
  Pure.
- `draw`: draws the separator, the selection and the columns. Reads
  only `View` (`rows`, the output list).
