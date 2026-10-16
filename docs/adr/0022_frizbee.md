---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Use frizbee for search

## Context and Problem Statement

[ADR-0014](0014_fuzzy_matcher.md) chose a fuzzy matching library for
search mode. Which one?

## Decision Drivers

- Actively developed and released.
- Match indices for highlighting.
- Good ranking, fast, few dependencies.
- License compatible with MIT.

## Considered Options

- `frizbee`
- `nucleo-matcher`
- `fuzzy-matcher`, `sublime_fuzzy`
- `simsearch`

## Decision Outcome

Chosen option: "`frizbee`", because it is the only candidate that is
both actively released (0.13.0, 2026-08-13) and fits the needs: no
required dependencies, match indices, smart case, optional typo
tolerance, MIT license. It is used by skim, atuin, television and
blink.cmp.

### Consequences

- Good, because the matcher gets regular fixes and releases.
- Good, because typo tolerance improves search for fast typists.
- Bad, because MSRV rises from 1.85 to 1.89.
- Bad, because the API is pre-1.0 and may change between minor
  versions; the wrapper in `src/search/fuzzy.rs` contains it.

### Confirmation

Unit tests in `src/search/` cover ranking, smart case and indices, so
a `frizbee` upgrade that changes behavior fails CI.

## Pros and Cons of the Options

### `nucleo-matcher`

- Good, because it is the fast, well-ranked matcher from the Helix
  editor.
- Bad, because its last release is 0.3.1 from 2024-02-20, while the
  repository keeps changing and a release request (#86) is open since
  2025-09; fixes do not reach users of the crate.
- Neutral, because it is MPL-2.0 (file-level copyleft).

### `fuzzy-matcher`, `sublime_fuzzy`

- Bad, because both are unmaintained since 2020.

### `simsearch`

- Good, because it is actively released.
- Bad, because it is a search index without character match indices.

## More Information

See [search.md](../arch/modes/search.md).
