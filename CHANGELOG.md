# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project scaffold: Cargo package, release profile tuned for size,
  `rustfmt`, `clippy`, `markdownlint` and EditorConfig settings.
- Architecture docs in `docs/arch/`, one page per module, and the tech
  stack in `docs/arch/stack.md`.
- Architecture decision records in `docs/adr/` (MADR 4.0) with
  `scripts/new_adr.sh` and `scripts/adr_log.sh`.
- `README.md`, `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`,
  `ROADMAP.md` and this changelog.
- MIT license and GitHub Sponsors funding config.
- CI on GitHub Actions running all Tooling checks on each pull request,
  and README badges.
- Core types (`WindowId`, `WindowInfo`, `Event`, `Action`, `Key`),
  title sanitizing, safe cut points and letter key assignment.
- Switcher state machine with `State`, `View` and the `Mode` trait.
- Config parsing, defaults and validation for `tynex.toml`, and the
  default file written on first start (`serde`, `toml`).
- Mock window source with sample windows, the `WindowSource` trait,
  an event queue, the adapter registry and an end-to-end test.
- Hotkey mode: letters (with Latin layout fallback), arrows, `Enter`,
  `/` to search and `Esc`.
- Renderer: canvas, text with font fallback, truncation at safe cut
  points and highlights, and a dark theme (`tiny-skia`, `skrifa`).

[Unreleased]: https://github.com/canbesolved/tynex/commits/main
