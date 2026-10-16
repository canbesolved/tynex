---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Formatting and linting tooling

## Context and Problem Statement

Code and docs have an 80-column limit and a fixed style. Which tools
enforce it, and on which toolchain?

## Decision Drivers

- Enforce the style automatically, not by review.
- Keep builds on the stable toolchain.

## Considered Options

- Nightly `rustfmt` for formatting only, stable for builds, `clippy`,
  `markdownlint-cli2`
- Stable `rustfmt` only

## Decision Outcome

Chosen option: "Nightly `rustfmt` for formatting only", because only
nightly supports `wrap_comments`, `comment_width = 80` and
`group_imports = "StdExternalCrate"`, while code still builds on
stable.

Rules: `rustfmt.toml` (`max_width = 80`, `comment_width = 80`,
`wrap_comments`, `group_imports`), `clippy` with `all` and `pedantic`
as warnings, `markdownlint-cli2` with `MD013` at 80, `.editorconfig`
for editors. The style itself is set in
[ADR-0019](0019_rust_style.md) and [ADR-0020](0020_markdown_style.md).

### Consequences

- Good, because the style is checked by one command per tool.
- Good, because comments are wrapped at 80 columns automatically.
- Bad, because contributors need the nightly toolchain installed.

### Confirmation

Before each commit:

```bash
cargo +nightly fmt --check
cargo clippy --all-targets -- -D warnings
markdownlint-cli2 "**/*.md"
```

## Pros and Cons of the Options

### Stable `rustfmt` only

- Good, because no nightly toolchain is needed.
- Bad, because long comments are not wrapped or reported, so the
  80-column rule relies on review.
