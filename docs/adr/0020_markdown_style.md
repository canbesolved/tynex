---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Follow default Markdown style

## Context and Problem Statement

Docs need one Markdown style that every contributor follows and
`markdownlint-cli2` checks ([ADR-0018](0018_lint_tooling.md)). Which
style should they follow?

## Decision Drivers

- Docs familiar to any contributor and renderer.
- Formatting fully checked by `markdownlint-cli2`.
- Keep the 80-column limit.

## Considered Options

- Default `markdownlint` rules, `MD013` at 80 columns

## Decision Outcome

Chosen option: "Default `markdownlint` rules, `MD013` at 80 columns",
because they encode the common Markdown guidelines (CommonMark, ATX
headings, single blank lines, fenced code with a language) with only
the line length tuned.

### Consequences

- Good, because docs and code both follow standard styles.
- Good, because `.markdownlint.yaml` has a single override.

### Confirmation

`markdownlint-cli2 "**/*.md"` passes before each commit.
