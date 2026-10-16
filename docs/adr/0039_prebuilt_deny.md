---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Prebuilt cargo-deny in CI

## Context and Problem Statement

[ADR-0038](0038_ci_actions.md) installs `cargo-deny` in CI with
`cargo install`, which compiles it from source. The Dependencies job
takes 2 min 15 s on a cold cache, while the other jobs take under
30 s. How should CI get `cargo-deny`?

## Decision Drivers

- Fast CI runs, also on new branches without a cache.
- A fixed, verified version, no unreviewed third-party action.
- Lines within 80 columns.

## Considered Options

- Download the release binary and check a pinned SHA-256
- `cargo install` from source (ADR-0038)
- `EmbarkStudios/cargo-deny-action`

## Decision Outcome

Chosen option: "Download the release binary and check a pinned
SHA-256", because the job then takes seconds, and the archive is
verified against a hash stored in the workflow, not against a file
from the same release.

- `ci.yml` downloads
  `cargo-deny-<version>-x86_64-unknown-linux-musl.tar.gz` from the
  GitHub release, checks it with `sha256sum -c`, and installs the
  binary into `~/.cargo/bin`.
- The version and hash live in `DENY_VERSION` and `DENY_SHA256`. A
  version bump updates both; the hash is computed locally from the
  downloaded archive and compared with the published one.

This amends ADR-0038 for `cargo-deny` only; npm and pipx tools stay as
they are.

### Consequences

- Good, because the Dependencies job no longer compiles anything.
- Neutral, because a bump needs one manual hash check.

## Pros and Cons of the Options

### `cargo install` from source

- Good, because it needs no hash.
- Bad, because it compiles for minutes on every cold cache.

### `EmbarkStudios/cargo-deny-action`

- Good, because it is maintained by the `cargo-deny` authors.
- Bad, because the action name plus a commit hash exceeds 80 columns,
  and it runs in a Docker image.

## More Information

See [ci.yml](../../.github/workflows/ci.yml).
