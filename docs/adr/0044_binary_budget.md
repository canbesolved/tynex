---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Binary size budget of 10 MB

## Context and Problem Statement

The release binary had a 3 MB target. Search mode with `frizbee`
([ADR-0022](0022_frizbee.md)) grows it from 1.60 MB to 5.35 MB,
because `frizbee` compiles its matcher for each SIMD instruction set
and typo count, and has no option to turn that off. Keep `frizbee` or
keep the budget?

## Decision Drivers

- Search quality: ranking like `fzf`, typo tolerance, match indices.
- Footprint: binary size, RSS and idle CPU.
- Maintained dependencies.

## Considered Options

- Keep `frizbee`, raise the budget to 10 MB
- Switch to `nucleo-matcher` (about +0.2 MB), keep 3 MB
- Own matcher without dependencies, keep 3 MB

## Decision Outcome

Chosen option: "Keep `frizbee`, raise the budget to 10 MB", because
the feature set matters more than a few megabytes of disk, and only
the code paths in use are paged into memory, so RSS and idle CPU stay
within their budgets.

- The release binary target is under 10 MB.
- `src/search/fuzzy.rs` calls a single `frizbee` entry point,
  `match_list_indices`; each extra one adds about 1 MB.
- An own matcher stays a long-term option in
  [ROADMAP.md](../../ROADMAP.md).

### Consequences

- Good, because search keeps typo tolerance and SIMD speed.
- Bad, because downloads and installs are about three times larger.

### Confirmation

Release builds are measured as described in `CONTRIBUTING.md`, and the
size is named in the pull request when it changes.

## Pros and Cons of the Options

### Switch to `nucleo-matcher`

- Good, because it adds about 0.2 MB and folds accents itself.
- Bad, because it has no typo tolerance and no release since 2024.

### Own matcher without dependencies

- Good, because it is the smallest and fully under our control.
- Bad, because good ranking and typo tolerance take real work
  ([ADR-0014](0014_fuzzy_matcher.md)).
