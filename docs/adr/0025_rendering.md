---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Render with tiny-skia, ab_glyph and resvg

## Context and Problem Statement

[ADR-0008](0008_cpu_render.md) chose CPU rendering. Which libraries
should draw shapes, text and icons? Breeze, the KDE icon theme, ships
39,864 SVG icons and no PNG, so SVG support is required.

## Decision Drivers

- Icons must work on KDE out of the box.
- Fewest crates; reuse what is already in the tree.
- Widely used, maintained crates.

## Considered Options

- `tiny-skia` + `ab_glyph` + `resvg` (minimal features)
- `tiny-skia` + `fontdue` + `png` + optional `resvg`
- `tiny-skia` + `cosmic-text` + `resvg`

## Decision Outcome

Chosen option: "`tiny-skia` + `ab_glyph` + `resvg`", because SVG
support is mandatory for KDE icons, `resvg` already depends on
`tiny-skia`, PNG decoding comes from `tiny-skia`'s `png-format`
feature, and `ab_glyph` needs 4 crates instead of 8 with about four
times the downloads of `fontdue`.

### Consequences

- Good, because no separate `png` crate is needed.
- Good, because `resvg` with default features off skips text,
  system fonts and embedded raster images that icons do not use.
- Good, because glyphs are rasterized once and cached.
- Neutral, because `ab_glyph` is slower than `fontdue` per glyph;
  glyphs are cached, so it does not matter.
- Bad, because `ab_glyph` does no shaping; complex scripts may render
  imperfectly until `rustybuzz` is added behind a feature.

## Pros and Cons of the Options

### `tiny-skia` + `fontdue` + `png` + optional `resvg`

- Good, because `fontdue` is the fastest glyph rasterizer.
- Bad, because `fontdue` is a solo project with slow issue responses,
  and the extra `png` crate duplicates `tiny-skia`.

### `tiny-skia` + `cosmic-text` + `resvg`

- Good, because it has full shaping and font fallback.
- Bad, because it is much larger and pulls in font discovery.

## More Information

See [render.md](../arch/ui/render.md).
