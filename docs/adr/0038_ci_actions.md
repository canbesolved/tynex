---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# CI with GitHub Actions

## Context and Problem Statement

The Tooling checks in `AGENTS.md` run only on contributors' machines.
A pull request can be merged although a check fails, and the README
has no build status. Where and how should the checks run
automatically?

## Decision Drivers

- Every pull request runs every Tooling check.
- No secrets, and the least permissions possible.
- Third-party code in CI is pinned and reviewable.
- Lines stay within 80 columns.

## Considered Options

- GitHub Actions, actions pinned to commit hashes, tools installed at
  fixed versions
- GitHub Actions with third-party actions for every tool
- No CI, local checks only

## Decision Outcome

Chosen option: "GitHub Actions, actions pinned to commit hashes, tools
installed at fixed versions", because it runs next to the pull
requests at no cost for a public repository, and keeps third-party
code to four widely used actions.

- `.github/workflows/ci.yml` runs on pull requests and on pushes to
  `main`, with `contents: read` permission only.
- Jobs: format, lint and test (nightly `rustfmt`, stable `clippy` and
  tests); minimum Rust version (`cargo check` with 1.89); dependencies
  (`cargo deny`); docs, scripts and licenses (`markdownlint-cli2`,
  `shellcheck`, `reuse lint`, ADR index up to date).
- `actions/checkout`, `dtolnay/rust-toolchain`, `Swatinem/rust-cache`
  and `actions/setup-node` are pinned to full commit hashes. Other
  tools are installed with npm, pipx and cargo at fixed versions,
  because their action names plus a hash exceed 80 columns.

### Consequences

- Good, because broken checks are visible on every pull request.
- Good, because the README can show a live build badge.
- Bad, because pinned action hashes and tool versions must be bumped
  by hand.

### Confirmation

The CI badge is green on `main`, and each pull request shows the four
jobs.

## More Information

See [ci.yml](../../.github/workflows/ci.yml) and the Tooling section of
[AGENTS.md](../../AGENTS.md).
