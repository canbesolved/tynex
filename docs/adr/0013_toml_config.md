---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# TOML config in tynex.toml

## Context and Problem Statement

Users configure mode, keys, layout and appearance, and later per-app
titles. Which file format and name?

## Decision Drivers

- Easy and safe to edit by hand.
- Well-maintained Rust parser with `serde`.
- Familiar to Rust users and tools.
- File name matches the app name.

## Considered Options

- TOML in `tynex.toml`, parsed with `toml`
- YAML in `tynex.conf`, parsed with `serde_yaml_ng`

## Decision Outcome

Chosen option: "TOML in `tynex.toml`, parsed with `toml`", because TOML
is the Rust ecosystem convention (`Cargo.toml`, `rustfmt.toml`,
Alacritty, Helix), the `toml` crate is maintained alongside Cargo, and
TOML has no indentation or implicit typing pitfalls.

### Consequences

- Good, because `serde` maps the file to typed structs; loading and
  validation rules are set in [ADR-0031](0031_strict_config.md).
- Good, because the `.toml` extension gives editors syntax support.
- Neutral, because per-app overrides use tables (`[apps.firefox]`),
  slightly more verbose than nested YAML.

## Pros and Cons of the Options

### YAML in `tynex.conf`

- Good, because it is compact for deeply nested data.
- Bad, because `serde_yaml` is archived and only forks remain.
- Bad, because indentation errors and implicit typing surprise users.

## More Information

See [config.md](../arch/config.md).
