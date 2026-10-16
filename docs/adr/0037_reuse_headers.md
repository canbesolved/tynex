---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBeSolved"
---
# REUSE license headers

## Context and Problem Statement

`tynex` is MIT licensed, but only the root `LICENSE` file says so.
Copied files lose that context, and tools cannot tell the license of
a single file. How should copyright and license be recorded per file?

## Decision Drivers

- Every file has a machine-readable license.
- Little noise in docs and config files.
- Checked automatically, like the other style rules.

## Considered Options

- REUSE: SPDX headers in source files, `REUSE.toml` for the rest,
  checked with `reuse lint`
- SPDX headers in source files only, no check
- Root `LICENSE` only

## Decision Outcome

Chosen option: "REUSE", because it is the standard way to apply SPDX
identifiers to a whole repository, and `reuse lint` checks it.

- Source files (Rust, shell, JavaScript) start with
  `SPDX-FileCopyrightText` and `SPDX-License-Identifier` lines, after
  the shebang if any.
- `REUSE.toml` covers every other file with the same identifiers;
  a file's own header takes precedence (`precedence = "closest"`).
- `LICENSES/MIT.txt` holds the license text; the root `LICENSE` stays
  for GitHub and packagers.

### Consequences

- Good, because each source file states its license on its own.
- Good, because docs and config need no headers.
- Bad, because contributors install one more tool (`reuse`, packaged
  by Fedora, Debian and Arch).

### Confirmation

`reuse lint` passes before each commit.

## More Information

See the [REUSE specification](https://reuse.software/spec/) and the
coding style in [AGENTS.md](../../AGENTS.md).
