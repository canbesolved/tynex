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

- Width from `appearance.width`; height = rows x row height + padding.
- Row: key badge, icon, title.
- At most `appearance.max_rows` rows; the list scrolls with the
  selection beyond that.
- In search mode the key badge column is hidden, freeing space for the
  title.

## Contract

Implements `core::Layout`:

- `size(view, metrics)`: pure, used by the presenter to size the
  surface;
- `draw(view, canvas)`: draws the whole popup; reads only `View`.

New layouts (grid, horizontal strip) are new files in `ui/layouts/`
plus one entry in the layout registry, selected by `layout =` in the
config.
