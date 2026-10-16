---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Follow the Rust Style Guide

## Context and Problem Statement

Rust code needs one layout that every contributor follows and the
formatter enforces. Should `tynex` follow the official
[Rust Style Guide](https://doc.rust-lang.org/style-guide/) or a custom
code style?

## Decision Drivers

- Code familiar to any Rust contributor.
- Layout enforced by `rustfmt`, not by review.
- 80-column limit for code and comments.

## Considered Options

- Rust Style Guide
- Custom code style

## Decision Outcome

Chosen option: "Rust Style Guide", because the code looks like
idiomatic Rust and `rustfmt` enforces it with a few options
([ADR-0018](0018_lint_tooling.md)).

Rules:

- At most one blank line between items and between statement groups.
- Imports in three groups: `std`, external crates, `crate`.
- 80 columns for code and comments.

### Consequences

- Good, because `rustfmt` defaults already match the official style.
- Good, because new contributors learn no custom layout rules.
- Neutral, because project rules outside layout (naming, comments,
  file length) live in [AGENTS.md](../../AGENTS.md) and are checked in
  review.

### Confirmation

`cargo +nightly fmt --check` passes before each commit.

## Pros and Cons of the Options

### Rust Style Guide

- Good, because it is the default of `rustfmt` and most Rust projects.
- Bad, because wrapping comments at 80 columns and grouping imports
  need nightly `rustfmt` options.

### Custom code style

- Good, because it can fit personal taste, for example two blank lines
  so sections stand out visually.
- Bad, because it is non-standard Rust and needs unstable `rustfmt`
  options such as `blank_lines_upper_bound`.
