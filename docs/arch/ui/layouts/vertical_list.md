# Vertical list layout

Source: `src/ui/layouts/vertical_list.rs`. The default layout.

## Look

```text
Hotkey mode, show_app = true
╭──────────────────────────────────────╮
│ [a] ▣  Konsole   ~/projects/demo     │
│ [s] ▣  Firefox   Release notes       │
│ [d] ▣  Dolphin   Downloads           │
│ [f] ▣  Kate      overview.md         │
╰──────────────────────────────────────╯

Search mode, show_app = true
╭──────────────────────────────────────╮
│ / firef                              │
├──────────────────────────────────────┤
│ ▣  Firefox   Release notes           │
│ ▣  Firefox   Café                    │
╰──────────────────────────────────────╯
```

With `show_app = false` (default) the app name column is left out.

The selected row is highlighted. The search bar shows only in search
mode.

## Rules

- Width from `appearance.width`; height = rows x row height + padding,
  at least one row, plus the search bar in search mode.
- Sizes follow the font: row height 2 x font size, key badge 1.4 x,
  icon slot 1.5 x, gaps 0.6 x; all multiplied by the output scale.
- Row in columns: key badge (hotkey mode), app icon, app name (with
  `show_app`), title. A window without an icon keeps an empty slot,
  so titles stay aligned.
- Column widths follow `appearance.width`: the app name column takes
  30 % of the text width after the icon, the title the rest; no extra
  config keys
  ([ADR-0049](../../../adr/0049_popup_columns.md)).
- App names are drawn in a dimmer color (`app_text` in the theme) and
  cut with an ellipsis when too long.
- Titles are truncated with an ellipsis and drawn with search
  highlights (see [render.md](../render.md)).
- At most `appearance.max_rows` rows; the list scrolls so the selected
  row stays visible.
- In search mode the key badge column is hidden; the search bar shows
  `/ query` above a separator line.

## Contract

Implements the `Layout` trait from `src/ui/layouts/mod.rs`:

- `size(view, metrics, theme)`: pure, used by the presenter to size
  the surface;
- `icon_size(metrics, theme)`: pure, the icon side in pixels, used to
  load icons before a show;
- `draw(view, metrics, paint)`: draws the whole popup with the canvas,
  text renderer, icons and theme in `Paint`; reads only `View`.

`Metrics` holds the font size, width, maximum rows and output scale.

New layouts (grid, horizontal strip) are new files in `ui/layouts/`
plus one entry in the layout registry, selected by `layout =` in the
config.
