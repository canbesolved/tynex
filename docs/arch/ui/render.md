# Renderer

Source: `src/ui/render/`. Portable: draws into a plain pixel buffer.

## Files

```text
ui/render/
  canvas.rs     Canvas over a borrowed &mut [u8] pixel buffer
  glyphs.rs     Font faces and glyph rasterizing (skrifa)
  text.rs       Measuring, truncation, highlights, glyph cache
  icons.rs      Icon lookup, decoding and cache
  theme.rs      Colors, paddings, radii
```

## Canvas

- Wraps `tiny_skia::PixmapMut` over the presenter's buffer, so drawing
  goes straight into shared memory.
- Primitives: rounded rect, filled rect, glyph coverage, image.
- CPU only. A full redraw of a 480 x 600 popup takes well under 1 ms.

## Text

- `skrifa` reads glyph outlines and metrics; each outline becomes a
  `tiny-skia` path and is rasterized into a coverage mask once per
  (face, glyph, size). The canvas blends cached masks.
- No kerning and no shaping: glyphs follow their advances.
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
  are cached per character, so each lookup runs once. `Text::prepare`
  resolves a title's fonts when a `Windows` event brings it, so lookups
  never run when the popup opens.
- **Memory:** font files are mapped read-only with `rustix` (`mmap`),
  not read into the heap. A CJK font of 20 MB costs RAM only for the
  pages actually used. `skrifa` loads font collections (`.ttc`) by
  index. A `Face` needs its data for the whole run (`&'static [u8]`).
- **No font at all:** the primary font's missing-glyph box is drawn.
- **Scripts:** Latin, Cyrillic, Greek, Chinese, Japanese and Korean
  render correctly without shaping. Arabic, Hebrew and Indic scripts
  need shaping and right-to-left layout; until then they render
  imperfectly (see [ROADMAP.md](../../../ROADMAP.md)).
- **Emoji:** only outlines are drawn, so color emoji show in one color
  or as a box.

## Icons

`icons.rs` turns an icon name into an image of the row icon size
([ADR-0046](../../adr/0046_icon_lookup.md)):

1. The adapter sets `icon` on each window: `Icon=` from the app's
   desktop file, else the app id (see
   [linux.md](../platforms/linux.md#desktop-files)).
2. An absolute path is used as is. Other names go to the platform's
   `IconLookup` (`IconTheme` on Linux, see
   [linux.md](../platforms/linux.md#icon-themes)), which returns the
   file closest to the size.
3. PNG is decoded with `tiny-skia` (`png-format`) and scaled with a
   bicubic filter; SVG is rendered with `resvg` (default features
   off: no text, no system fonts, no embedded raster images). SVG is
   required: Breeze ships its icons only as SVG. Other formats (XPM)
   are skipped.
4. The image is fitted into a square of the icon size, centered, with
   its aspect kept, and cached per (name, size). Misses are cached
   too.

The icon size is the layout's icon slot at the output scale
(`Layout::icon_size`). `Painter::prepare` loads the icons of a new
window list, so files are read when windows change, not when the
popup opens; a scale change loads the new size on the next draw. A
missing icon leaves its slot empty.

## Theme

One dark theme with fixed colors. Theme values live in one struct, so
other sources can fill it without touching drawing code.
