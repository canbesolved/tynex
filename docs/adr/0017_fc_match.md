---
status: "accepted, amended by ADR-0040"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Resolve fonts with fc-match

## Context and Problem Statement

With `font = "auto"`, `tynex` must find the system's default sans-serif
font file to give to `ab_glyph`. How?

## Decision Drivers

- No extra linked library or build dependency.
- Respect the user's fontconfig settings.
- One-time cost at start-up only.

## Considered Options

- Run `fc-match -f '%{file}' sans-serif` once at start
- Link `fontconfig` through a Rust binding
- Scan font directories with `fontdb`

## Decision Outcome

Chosen option: "Run `fc-match` once at start", because fontconfig is
present on every Linux desktop, the result honours user settings, and
nothing extra is linked.

### Consequences

- Good, because `fontconfig-devel` is not needed to build.
- Bad, because it spawns one process at start, a few ms.
- Bad, because macOS and Windows need their own lookup.
