# Title bar

Source: `src/ui/title_bar.rs`. The open windows. It does not show
search matches. Those are the [output bar](output_bar.md).

## Look

```text
title_bar.enabled = false (default)
╭──────────────────────────────────────╮
│ /                                    │
╰──────────────────────────────────────╯

title_bar.enabled = true, show_icons = true (default)
╭──────────────────────────────────────╮
│ /                                    │
├──────────────────────────────────────┤
│ ▣  ~/projects/demo                   │
│ ▣  Release notes                     │
│ ▣  Downloads                         │
╰──────────────────────────────────────╯

show_icons = false
╭──────────────────────────────────────╮
│ /                                    │
├──────────────────────────────────────┤
│ ~/projects/demo                      │
│ Release notes                        │
│ Downloads                            │
╰──────────────────────────────────────╯
```

With `show_icons = true`, a window without an icon keeps an empty
slot so titles stay aligned. `false` shows titles only.

The list hides as soon as you type. Matches appear in the output
bar. Clearing the query with `Backspace` or `Esc` shows this list
again when it is enabled. The selected row is highlighted.
`Tab` moves that highlight one row forward and wraps. `Up` and
`Down` move it too
([ADR-0061](../../adr/0061_prompt_keys.md)).

## Rules

- Shown when `title_bar.enabled` is set and the query is empty.
  Typing hides it. `Backspace` that empties the query, or `Esc`,
  shows it again.
- Width from `main.width`. Height is the input bar plus padding, the
  output bar when it is shown, and, when this module is shown, a
  separator and at least one row.
- Sizes follow the font: row height 2 x font size, icon slot 1.5 x,
  gaps 0.6 x; all multiplied by the output scale.
- Row in columns: app icon (with `show_icons`), title. Rows have no
  key letters
  ([ADR-0050](../../adr/0050_search_only.md)) and no search
  highlights. A window without an icon keeps an empty slot when
  icons are on, so titles stay aligned.
- The title uses the rest of `main.width` after the icon slot. There
  is no app name column
  ([ADR-0058](../../adr/0058_title_bar_row.md)).
- Titles are truncated with an ellipsis (see [render.md](render.md)).
- At most `output_bar.max_rows` rows. The list scrolls so the
  selected window stays visible.
- With the icon strip showing, this list is its own rounded frame,
  3 logical pixels under the spare room, with no separator. Without
  the strip, a separator divides it from the bar above
  ([icon_bar.md](icon_bar.md)).

The painter draws this module under the icon bar when that strip is
showing, and under the output bar while a query is typed. A query
hides it, so the two lists are not on screen together
([ADR-0057](../../adr/0057_title_bar_query.md)). Its code and config
are separate from the input bar and the output bar
([ADR-0053](../../adr/0053_ui_module_toggles.md),
[ADR-0054](../../adr/0054_output_bar.md)).

## Contract

- `shows`: `title_bar.enabled` is set and the query is empty.
- `height`: the separator and the windows, or zero when hidden. Pure.
- `draw`: draws the separator, the selection and the columns. Reads
  only `View` (`windows`, every open window).

Shared row drawing lives in `src/ui/draw.rs`.
