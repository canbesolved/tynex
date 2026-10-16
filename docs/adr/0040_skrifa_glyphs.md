---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Glyphs with skrifa

## Context and Problem Statement

[ADR-0025](0025_rendering.md) chose `ab_glyph` for text. Its font
parser, `ttf-parser`, was declared unmaintained (RUSTSEC-2026-0192),
with no fixed version, and `cargo deny` rejects it under
`unmaintained = "all"`. What should parse fonts and draw glyphs?

## Decision Drivers

- Maintained font parsing; font files come from the system.
- No new rasterizer: `tiny-skia` already fills paths.
- Font collections (`.ttc`) for CJK fallback fonts.
- Keep the `unmaintained = "all"` rule without exceptions.

## Considered Options

- `skrifa` for outlines and metrics, rasterized with `tiny-skia`
- Keep `ab_glyph`, ignore the advisory in `deny.toml`
- `fontdue`

## Decision Outcome

Chosen option: "`skrifa` for outlines and metrics, rasterized with
`tiny-skia`", because it is the maintained parser of the Google Fonts
`fontations` project, recommended by the advisory, and its outlines
become `tiny-skia` paths, so no separate rasterizer is needed.

- `glyphs.rs` wraps `skrifa::FontRef` in `Face` (`from_index` for
  collections), maps characters, reads advances and metrics, and
  rasterizes outlines into a `tiny-skia` mask.
- Font data must live for the whole program (`&'static [u8]`), for
  example a memory-mapped file that is never unmapped.
- No kerning: without shaping, glyphs follow their advances.

This amends ADR-0025 (`ab_glyph` becomes `skrifa`), ADR-0017 and
ADR-0033, which name `ab_glyph` as the font user; `tiny-skia` and
`resvg` stay.

### Consequences

- Good, because `cargo deny` passes without exceptions.
- Good, because `skrifa` reads variable fonts and color glyph tables,
  for later emoji work.
- Neutral, because the tree grows by about 8 crates (37 in total now).
- Bad, because pair kerning is dropped until shaping arrives.

### Confirmation

Text tests (widths, fallback, truncation at safe cut points, colors)
pass with the bundled DejaVu Sans; `cargo deny check` passes.

## Pros and Cons of the Options

### Keep `ab_glyph`, ignore the advisory

- Good, because no code changes.
- Bad, because an unmaintained parser would read every font file on
  the system, and the exception weakens the policy.

### `fontdue`

- Good, because it is a fast rasterizer.
- Bad, because it also parses fonts with `ttf-parser`.

## More Information

See [render.md](../arch/ui/render.md) and
[RUSTSEC-2026-0192](https://rustsec.org/advisories/RUSTSEC-2026-0192).
