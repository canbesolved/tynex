# Renderer

Source: `src/ui/render/`. Portable: draws into a plain pixel buffer.

## Files

```text
ui/render/
  canvas.rs     Canvas over a borrowed &mut [u8] pixel buffer
  text.rs       Glyph rasterizing and cache (ab_glyph)
  icons.rs      Icon lookup, decoding and cache
  theme.rs      Colors, paddings, radii
```

## Canvas

- Wraps `tiny_skia::PixmapMut` over the presenter's buffer, so drawing
  goes straight into shared memory.
- Primitives: rounded rect, filled rect, glyph run, image.
- CPU only. A full redraw of a 480 x 600 popup takes well under 1 ms.

## Text

- `ab_glyph` rasterizes glyphs once per (glyph, size) and caches the
  coverage masks; `tiny-skia` blends them.
- Titles are truncated with an ellipsis to fit the row width,
  measured by glyph advances, following the truncation rules in
  [core.md](../core.md#text-and-languages).
- Search matches are drawn with the highlight color using ranges from
  `View`.

## Fonts and scripts

- **Primary font:** from `font` in the config, or `fc-match` for
  `auto` ([ADR-0017](../../adr/0017_fc_match.md)).
- **Fallback:** when the primary font has no glyph for a character,
  `fc-match -f '%{file}:%{index}' 'sans-serif:charset=<hex>'` finds a
  font that has it (for example Noto Sans CJK for Chinese). Results
  are cached per character, so each lookup runs once. Lookups run when
  a `Windows` event brings a new title, never when the popup opens.
- **Memory:** font files are mapped read-only with `rustix` (`mmap`),
  not read into the heap. A CJK font of 20 MB costs RAM only for the
  pages actually used. `ab_glyph` loads font collections (`.ttc`) by
  index.
- **No font at all:** the primary font's missing-glyph box is drawn.
- **Scripts:** Latin, Cyrillic, Greek, Chinese, Japanese and Korean
  render correctly without shaping. Arabic, Hebrew and Indic scripts
  need shaping and right-to-left layout; until then they render
  imperfectly (see [ROADMAP.md](../../../ROADMAP.md)).
- **Emoji:** `ab_glyph` draws outlines only, so color emoji show in
  one color or as a box.

## Icons

1. Take `app_id` (desktop file name) from `WindowInfo`.
2. Read `Icon=` from the matching `.desktop` file in
   `$XDG_DATA_DIRS/applications/`.
3. Look the name up in the current icon theme, then `hicolor`, at the
   row icon size.
4. Decode PNG with `tiny-skia` (`png-format` feature); render SVG
   with `resvg` (default features off: no text, no system fonts, no
   embedded raster images). SVG is required: Breeze ships its icons
   only as SVG.
5. Cache the scaled RGBA image per `app_id`. Misses are cached too.

Icon lookup runs on first sight of a new app, not on every show.

## Theme

One dark theme with fixed colors. Theme values live in one struct, so
other sources can fill it without touching drawing code.
