---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# One font for all text

## Context and Problem Statement

[ADR-0033](0033_unicode_text.md) finds a fallback font for every
character the main font lacks: one `fc-match` process per new
character, a font file mapped per match, and a font index kept per
character. It adds code, processes and memory for titles that are
mostly in the main font's script. Should `tynex` keep the fallback?

## Decision Drivers

- Footprint: no processes spawned while windows change, less memory.
- Simple, predictable text code.
- The user can still pick a font that covers their scripts.

## Considered Options

- One font: the system font or `font` from the config
- Keep the per-character `fc-match` fallback

## Decision Outcome

Chosen option: "One font: the system font or `font` from the config",
because it removes the fallback machinery, and the config already lets
users choose a font with the scripts they need.

- `font = "auto"` runs `fc-match sans-serif` once at start
  ([ADR-0017](0017_fc_match.md)); a path loads that file.
- All text is drawn with that one face. A character it lacks shows as
  its missing-glyph box (`.notdef`).
- `Text` keeps one glyph cache; the `FontLookup` trait, `FcFallback`
  and `Text::prepare` are removed.
- Invisible characters that apps put in titles (bidi marks such as
  `U+200E`, zero width space, word joiner, byte order mark) are
  removed when titles are sanitized, so they never show as boxes.

This amends ADR-0033: its "missing glyphs are found with `fc-match` by
character" part no longer applies.

### Consequences

- Good, because no `fc-match` runs after start and no fallback font
  files are mapped.
- Good, because the text code is shorter and has one glyph cache.
- Bad, because Chinese, Japanese and Korean titles, emoji and rare
  symbols show as boxes unless `font` names a font that has them, for
  example Noto Sans CJK.

### Confirmation

Text tests run with the bundled DejaVu Sans and check that a missing
character is drawn with the main font's box. `fc-match` is called only
from `default_face`.

## Pros and Cons of the Options

### Keep the per-character `fc-match` fallback

- Good, because any script installed on the system displays.
- Bad, because it spawns a process per new character and keeps extra
  fonts and a per-character index in memory.
