---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBeSolved"
---
# Unicode text handling

## Context and Problem Statement

Window titles come in any language: Chinese, Russian, French and many
more. `tynex` must display them, search them and select windows with
letter keys whatever keyboard layout is active. How should text be
handled without heavy crates?

## Decision Drivers

- Correct display of common scripts out of the box.
- Search that works for non-Latin scripts and accented Latin letters.
- Hotkey letters that work with any active layout.
- Fewest crates; RAM only for fonts actually used.

## Considered Options

- Own small rules on top of existing crates: UTF-8 everywhere,
  sanitized titles, `fc-match` font fallback, own accent fold table,
  Latin key fallback from `xkbcommon`
- Full Unicode stack: `unicode-normalization`, `unicode-segmentation`,
  `cosmic-text` with shaping and font fallback
- ASCII-only matching and a single font

## Decision Outcome

Chosen option: "Own small rules on top of existing crates", because
`frizbee`, `ab_glyph`, `xkbcommon` and `fc-match` already cover most
of the work, and the missing parts are small tables and lookups.

- Titles are UTF-8 and sanitized once (control and bidi override
  characters removed); letters are never changed.
- Offsets are byte offsets on `char` boundaries; truncation never
  splits a letter from its combining marks.
- Missing glyphs are found with `fc-match` by character, cached, and
  font files are memory-mapped.
- An ASCII query matches accented Latin letters through a fold table
  with an offset map for highlights.
- `Key::Char` carries the produced character and the character of the
  same key in the first Latin layout, for hotkey mode.

### Consequences

- Good, because Latin, Cyrillic, Greek and CJK titles display and
  match correctly with no new crate.
- Good, because a CJK fallback font costs RAM only for used pages.
- Good, because hotkey letters work with a Russian or Greek layout
  active.
- Bad, because Arabic, Hebrew and Indic scripts need shaping and
  right-to-left layout, which come later.
- Bad, because typing Chinese, Japanese or Korean needs an input
  method, which comes later.
- Neutral, because accent folding covers Latin scripts only; other
  scripts match as typed.

### Confirmation

Unit tests cover sanitizing, truncation at combining marks, accent
folding with highlight offsets, Cyrillic smart case and the Latin key
fallback.

## Pros and Cons of the Options

### Full Unicode stack

- Good, because shaping, normalization and grapheme rules are
  complete.
- Bad, because it adds many crates and megabytes of tables for cases a
  window list rarely meets.

### ASCII-only matching and a single font

- Good, because it is the simplest.
- Bad, because non-Latin titles show as boxes and cannot be searched.

## More Information

See [core.md](../arch/core.md#text-and-languages),
[search.md](../arch/modes/search.md) and
[render.md](../arch/ui/render.md).
