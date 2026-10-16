---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Standard library paths

## Context and Problem Statement

`tynex` needs the config and runtime directories. The `dirs` crate
(4 crates) provides them, but `std::env::home_dir` is no longer
deprecated since Rust 1.86 (checked on 1.99 with warnings as errors).
Is `dirs` needed?

## Decision Drivers

- Fewest crates.
- Correct XDG behavior on Linux.

## Considered Options

- Standard library and XDG variables in each `platform/*/paths.rs`
- `dirs`

## Decision Outcome

Chosen option: "Standard library and XDG variables", because on Linux
the rules are two lines each (`$XDG_CONFIG_HOME` or `~/.config`,
`$XDG_RUNTIME_DIR`), and each platform module will implement its
own paths when ported.

### Consequences

- Good, because `dirs` and its 3 dependencies are not needed.
- Bad, because macOS and Windows paths must be written when ported.
