---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Delimited string KWin payload

## Context and Problem Statement

The KWin script sends the window list to the daemon with `callDBus`.
Converting JS arrays and objects to D-Bus types is unreliable, so the
list travels as one string. Which encoding?

## Decision Drivers

- Robust through `callDBus` type conversion.
- No extra dependency in the daemon.
- Trivial and fast to parse.

## Considered Options

- Records split by `\x1e`, fields by `\x1f`
- JSON string parsed with `serde_json`
- D-Bus array of structs

## Decision Outcome

Chosen option: "Records split by `\x1e`, fields by `\x1f`", because
these ASCII separators never appear in real titles, parsing is a
couple of `split` calls, and no JSON crate is needed.

### Consequences

- Good, because the binary avoids `serde_json`.
- Bad, because the format is custom and must be documented.
- Bad, because the script must strip `\x1e` and `\x1f` from captions.

### Confirmation

Unit tests in `src/adapters/kde/parse.rs` cover empty lists, empty
fields and separator stripping.
