---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Prebuilt release binaries

## Context and Problem Statement

Releases ship only source code, so every user needs a Rust toolchain
and development headers to try `tynex`. Which platforms get prebuilt
binaries, and how are they built?

## Decision Drivers

- Users install `tynex` without a Rust toolchain.
- Binaries run on common distributions, not only the newest.
- Builds are reproducible from the tag, with no manual steps.
- Third-party code in CI is pinned, as in
  [ADR-0038](0038_ci_actions.md).

## Considered Options

- GitHub Actions on native runners for each target, on tag push
- Cross-compiling every target from one Linux runner (`cross`, `zig`)
- No binaries, source releases only

## Decision Outcome

Chosen option: "GitHub Actions on native runners for each target, on
tag push", because each target builds with its own system libraries
and needs no cross toolchain.

| Platform              | Rust target                 | Runner             |
| --------------------- | --------------------------- | ------------------ |
| Linux x86_64          | `x86_64-unknown-linux-gnu`  | `ubuntu-22.04`     |
| Linux arm64           | `aarch64-unknown-linux-gnu` | `ubuntu-22.04-arm` |
| macOS (Apple silicon) | `aarch64-apple-darwin`      | `macos-14`         |
| Windows x64           | `x86_64-pc-windows-msvc`    | `windows-2022`     |
| Windows arm64         | `aarch64-pc-windows-msvc`   | `windows-11-arm`   |

- A workflow runs when a `v*` tag is pushed, builds each target with
  `cargo build --release` and attaches the archives to the GitHub
  release of that tag.
- Archives: `tynex-<version>-<target>.tar.gz` (`.zip` on Windows) with
  the binary, `README.md` and `LICENSE`, plus one `SHA256SUMS` file.
- Linux builds on Ubuntu 22.04, so binaries need glibc 2.35 or newer.
  They link the system `libdbus-1` and `libxkbcommon`, which every
  Plasma desktop has.
- macOS and Windows join the workflow with their ports; until then
  `tynex` does not build there.
- Binaries start with 0.1.1; earlier releases stay source only.

### Consequences

- Good, because users on the supported targets download and run one
  file.
- Good, because the same tag always builds the same way.
- Bad, because each target adds a runner and minutes to every release.
- Bad, because Linux binaries do not run on distributions older than
  glibc 2.35.

### Confirmation

Each release lists one archive per supported target and a
`SHA256SUMS` file; the workflow's actions are pinned to commit hashes.

## Pros and Cons of the Options

### Cross-compiling every target from one Linux runner

- Good, because one runner builds everything.
- Bad, because macOS and Windows need SDKs that are hard to set up on
  Linux, and Linux arm64 needs target system libraries.

### No binaries, source releases only

- Good, because there is nothing to maintain.
- Bad, because trying `tynex` needs a toolchain and headers.

## More Information

Planned in [ROADMAP.md](../../ROADMAP.md). Ports are listed under
Platform support there.
