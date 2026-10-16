# Agent rules

Rules for humans and AI agents working on `tynex`. Read
[docs/arch/overview.md](docs/arch/overview.md) and the page of the
module you touch before changing code.

## Project layout

- `src/`: Rust source code.
- `docs/`: documentation.
- `docs/arch/`: architecture, one page per module. The tree mirrors
  `src/` (for example `src/adapters/kde/` -> `docs/arch/adapters/kde.md`).
- `docs/arch/stack.md`: tech stack. Every crate is listed there.
- `docs/adr/`: architecture decision records (MADR 4.0).
- `scripts/`: helper scripts (`new_adr.sh`, `adr_log.sh`).
- `assets/`: images and other media.
- `ROADMAP.md`: planned features (Now / Next / Later).
- `CHANGELOG.md`: notable changes, Keep a Changelog format.
- `CONTRIBUTING.md`: developer environment setup and workflow.

## Principles

- Footprint first: no idle CPU work, no polling loops, no GPU context,
  no heavy dependencies. Justify every new crate.
- Modularity: the core (`src/core/`) is pure logic with no I/O. New
  window managers, layouts, modes and backends plug in through traits.
- Portability: no OS-specific types outside `src/platform/` and
  `src/adapters/`. Gate them with `cfg(target_os = "...")`.
- Be concise in code, comments, docs and commit messages.

## Documentation

- Keep docs in sync with code in the same commit: a change to a module
  updates its page in `docs/arch/`.
- A new module gets a new page and a link in `docs/arch/overview.md`.
- A new dependency gets a row and a reason in `docs/arch/stack.md`.
- Notable changes get an entry under `## [Unreleased]` in
  `CHANGELOG.md` (`Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`,
  `Security`), in the same commit. On release, the section becomes
  `## [x.y.z] - YYYY-MM-DD` and released items leave `ROADMAP.md`.
- A renamed or removed config key is a breaking change: it gets a
  `Changed` or `Removed` entry in `CHANGELOG.md` with the old key, the
  new key if any, and starts with **Breaking:**. Old config files stop
  the daemon until edited (see `docs/arch/config.md`).
- Deferred features go to `ROADMAP.md`, not into module docs. Module
  docs describe what exists or is in the current release.

## Decisions (ADR)

- Every significant decision (architecture, dependency, protocol,
  tooling) gets an ADR in `docs/adr/`, in the same commit as the
  change.
- Create one with `scripts/new_adr.sh "Title" short_slug`, fill the
  template, set `status: "accepted"` once agreed.
- Never rewrite an accepted ADR. Write a new one and set the old
  status to `superseded by ADR-NNNN`.
- Refresh the log with `scripts/adr_log.sh` (needs `adr-log`). Keep
  titles and slugs short so log lines fit in 80 columns.

## Coding style

- Max line width: 80 characters in source code and docs.
- Naming: underscores (`snake_case`, `file_name.rs`); hyphens only where
  underscores are not allowed. File names are lowercase, except
  conventional ones (`README.md`, `AGENTS.md`, `CLAUDE.md`,
  `CONTRIBUTING.md`, `ROADMAP.md`, `CHANGELOG.md`, `LICENSE`,
  `LICENSES/MIT.txt`, `REUSE.toml`).
- A source file must not exceed 250 lines. Split it into modules first.
- Every source file (Rust, shell, JavaScript) starts with SPDX / REUSE
  identifiers, after the shebang if any. Other files (docs, config,
  assets) are covered by `REUSE.toml`. The comment rules below do not
  apply to these two lines:

  ```rust
  // SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
  // SPDX-License-Identifier: MIT
  ```

- Comments start with a capital letter and end with a period.
- Inline (trailing) comments start with a capital letter and have no
  final period.
- Rust code follows the
  [Rust Style Guide](https://doc.rust-lang.org/style-guide/), enforced
  by `rustfmt`:
  - one blank line between items (functions, types, impls, modules);
  - inside a function, one blank line separates logical groups of
    statements;
  - never more than one blank line in a row;
  - imports in three groups, one blank line apart: `std`, external
    crates, `crate` (`group_imports = "StdExternalCrate"`).
- Item order in a file: SPDX header, module docs, `use`, constants,
  types and their impls, functions, `#[cfg(test)] mod tests` last.
- Markdown follows the default `markdownlint` rules (CommonMark style):
  ATX headings (`#`), one blank line around headings, lists and code
  blocks, never two blank lines in a row, `-` for bullets, a language
  on every fenced code block.

```rust
// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Window list handling.

use std::cmp::Reverse;

use crate::core::WindowInfo;

/// Maximum number of entries shown at once.
const MAX_ENTRIES: usize = 19;

/// Returns windows sorted by most recent use.
fn sorted(windows: &[WindowInfo]) -> Vec<&WindowInfo> {
    let mut list: Vec<_> = windows.iter().collect();

    list.sort_by_key(|w| Reverse(w.last_used)); // Newest first
    list.truncate(MAX_ENTRIES);
    list
}
```

## Tooling

Run before every commit. All must pass. CI
(`.github/workflows/ci.yml`) runs the same checks on every pull request.

```bash
cargo +nightly fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check
markdownlint-cli2 "**/*.md"
shellcheck scripts/*.sh
reuse lint
scripts/adr_log.sh   # When ADRs changed
```

Nightly `rustfmt` is used only for formatting, because `wrap_comments`,
`comment_width` and `group_imports` are unstable options. Builds use
stable.

## Privacy

- Never put a personal email address in the repository: files, commit
  messages, commit metadata, test fixtures or screenshots.
- Identify people by name and GitHub handle, for example
  `CanBeSolved (@canbesolved)`.
- Commit with the GitHub no-reply address
  (`<id>+<login>@users.noreply.github.com`, set with
  `git config user.email`). On GitHub, enable "Keep my email addresses
  private" and "Block command line pushes that expose my email".
- No other personal data either: home paths (`/home/<user>`), user and
  host names, IP addresses, tokens or keys. Examples and fixtures use
  made-up data, never real window titles from your desktop.
- Check privacy before every commit:
  - `git config user.email` prints the no-reply address;
  - `git diff --cached` contains no email address (other than
    `git@github.com`), home path, user or host name, IP address, token
    or key;
  - the commit message contains none of these either.
- If personal data or a secret was pushed, deleting it in a new commit
  is not enough: tell the maintainer, revoke any secret, and remove it
  from history with `git filter-repo`.

## Git

- Agents never commit or push on their own. The maintainer reviews the
  changes and defines the scope of each commit first. Leave changes
  unstaged and propose the commit message.
- Default branch: `main`. All commits reach `main` only through a pull
  request; never commit or push to `main` directly. The only exception
  is the initial commit that creates `main`.
- Work on a branch named `<type>/<short_description>`, using the
  Conventional Commits type, for example `feat/kde_adapter` or
  `docs/roadmap`.
- A pull request covers one scope, passes all checks from Tooling, and
  is merged by the maintainer after review.
- Commits follow [Conventional Commits](https://www.conventionalcommits.org):
  `feat:`, `fix:`, `docs:`, `refactor:`, `perf:`, `test:`, `chore:`.
- One logical change per commit. Subject line under 72 characters.
- Remote: `git@github.com:CanBeSolved/tynex.git`.

## Commits and pull requests

- A pull request description has two sections:
  - `## Summary`: what changes and why, as a short list;
  - `## Checks`: each Tooling command run and its result, for example
    `reuse lint`: compliant. Name checks that were skipped and why.
- Do not add a `Co-Authored-By` trailer to commit messages.
- Do not add a "Generated with Claude Code" footer (or any similar
  agent attribution) to pull request descriptions.
