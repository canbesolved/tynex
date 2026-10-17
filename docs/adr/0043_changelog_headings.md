---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Repeated changelog headings

## Context and Problem Statement

Keep a Changelog repeats `### Added`, `### Fixed` and the other
headings under every release. With two releases in `CHANGELOG.md`,
the default `MD024` rule of
[ADR-0020](0020_markdown_style.md) reports them as duplicates. How
should the lint treat them?

## Decision Drivers

- `CHANGELOG.md` keeps the Keep a Changelog format.
- Real duplicate headings are still reported.

## Considered Options

- `MD024` with `siblings_only: true`
- Disable `MD024` in `CHANGELOG.md` only
- Version names in every heading (`### Added in 0.1.1`)

## Decision Outcome

Chosen option: "`MD024` with `siblings_only: true`", because headings
under different parents are allowed while duplicates under one parent
are still reported, in every file. This amends ADR-0020:
`.markdownlint.yaml` now has two overrides, `MD013` and `MD024`.

### Consequences

- Good, because the changelog format stays standard.
- Neutral, because two sections with the same heading under one
  parent are still errors anywhere.
