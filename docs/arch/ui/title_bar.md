# Title bar

Source: `src/ui/title_bar.rs`. The open windows. It does not show
search matches. Those are the [output bar](output_bar.md).

## Look

```text
title_bar.enabled = false (default)
╭──────────────────────────────────────╮
│ /                                    │
╰──────────────────────────────────────╯

title_bar.enabled = true, show_app_icon = true
╭──────────────────────────────────────╮
│ /                                    │
├──────────────────────────────────────┤
│ ▣  Konsole   ~/projects/demo         │
│ ▣  Firefox   Release notes           │
│ ▣  Dolphin   Downloads               │
╰──────────────────────────────────────╯
```

With `show_app_icon = false` (default) the app name column is left
out.

The list does not change when you type. Matches appear in the output
bar above it. The selected row is highlighted here only while the
query is empty, because a query selects a match in the output bar.

## Rules

- Shown only when `title_bar.enabled` is set. The query does not
  turn it on or off.
- Width from `main.width`. Height is the input bar plus padding, the
  output bar when it is shown, and, when this module is shown, a
  separator and at least one row.
- Sizes follow the font: row height 2 x font size, icon slot 1.5 x,
  gaps 0.6 x; all multiplied by the output scale.
- Row in columns: app icon, app name (with `show_app_icon`), title.
  Rows have no key letters
  ([ADR-0050](../../adr/0050_search_only.md)) and no search
  highlights. A window without an icon keeps an empty slot, so
  titles stay aligned.
- Column widths follow `main.width`: the app name column takes 30 %
  of the text width after the icon, the title the rest; no extra
  config keys
  ([ADR-0049](../../adr/0049_popup_columns.md)).
- App names are drawn in a dimmer color (`app_text` in the theme) and
  cut with an ellipsis when too long.
- Titles are truncated with an ellipsis (see [render.md](render.md)).
- At most `output_bar.max_rows` rows. While the query is empty the
  list scrolls so the selected window stays visible.
- A separator line divides this list from whatever is above it.

The painter stacks this module under the output bar. Its code and
config are separate from the input bar and the output bar
([ADR-0053](../../adr/0053_ui_module_toggles.md),
[ADR-0054](../../adr/0054_output_bar.md)).

## Contract

- `shows`: `title_bar.enabled` is set.
- `height`: the separator and the windows, or zero when hidden. Pure.
- `draw`: draws the separator, the selection and the columns. Reads
  only `View` (`windows`, every open window).

Shared row drawing lives in `src/ui/draw.rs`.
