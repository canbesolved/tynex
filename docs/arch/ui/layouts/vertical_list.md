# Vertical list layout

Source: `src/ui/layouts/vertical_list.rs`. The default layout.

## Look

```text
Hotkey mode
╭──────────────────────────────────────╮
│ [a] ▣  Konsole: ~/works/tynex        │
│ [s] ▣  Firefox: GitHub               │
│ [d] ▣  Dolphin: Downloads            │
│ [f] ▣  Kate: overview.md             │
╰──────────────────────────────────────╯

Search mode
╭──────────────────────────────────────╮
│ / firef                              │
├──────────────────────────────────────┤
│ ▣  Firefox: GitHub                   │
│ ▣  Firefox: Rust docs                │
╰──────────────────────────────────────╯
```

The selected row is highlighted. The search bar shows only in search
mode.

## Rules

- Width from `appearance.width`; height = rows x row height + padding,
  at least one row, plus the search bar in search mode.
- Sizes follow the font: row height 2 x font size, key badge 1.4 x,
  icon slot 1.5 x, gaps 0.6 x; all multiplied by the output scale.
- Row: key badge, icon slot, title. The icon slot stays empty until
  icons land (build order step 5).
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
- `draw(view, metrics, paint)`: draws the whole popup with the canvas,
  text renderer and theme in `Paint`; reads only `View`.

`Metrics` holds the font size, width, maximum rows and output scale.

New layouts (grid, horizontal strip) are new files in `ui/layouts/`
plus one entry in the layout registry, selected by `layout =` in the
config.
