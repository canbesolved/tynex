---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Dependency and script checks

## Context and Problem Statement

A manual review on 2026-10-15 found crates with stale releases. Such
checks must not depend on someone remembering to run them. The helper
scripts in `scripts/` also had no linter. Which tools check
dependencies and shell scripts?

## Decision Drivers

- Catch unmaintained, yanked or vulnerable crates automatically.
- Keep all licenses compatible with MIT.
- Catch shell script bugs before they run.

## Considered Options

- `cargo-deny` and `shellcheck`
- `cargo-audit` only, no script linting

## Decision Outcome

Chosen option: "`cargo-deny` and `shellcheck`", because `cargo-deny`
covers RustSec advisories, unmaintained and yanked crates, licenses,
duplicate versions and crate sources in one tool configured by
`deny.toml`, and `shellcheck` is the standard shell linter.

### Consequences

- Good, because `cargo deny check` replaces manual maintenance reviews.
- Good, because a new dependency with a non-allowed license fails.
- Bad, because the license allow list needs updates when a dependency
  uses a new permissive license.

### Confirmation

`cargo deny check` and `shellcheck scripts/*.sh` run before each commit
and later in CI.

## Pros and Cons of the Options

### `cargo-audit` only

- Good, because it is simple and focused on security advisories.
- Bad, because it does not check licenses, sources or duplicates.
