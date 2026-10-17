# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Search mode: typing filters windows by title and app id with fuzzy
  matching (`frizbee`): smart case, one typo per four characters,
  accent folding (`cafe` finds `Café`) and highlighted matches. The
  release binary grows to about 5.4 MB; the budget is now 10 MB
  (ADR-0044).

## [0.1.0] - 2026-10-17

First release, for KDE Plasma 6 on Wayland.

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
  title sanitizing and safe cut points.
- Switcher state machine with `State`, `View` and the `Mode` trait.
- Config parsing, defaults and validation for `tynex.toml`, and the
  default file written on first start (`serde`, `toml`).
- Mock window source with sample windows, the `WindowSource` trait,
  an event queue, the adapter registry and an end-to-end test.
- Hotkey mode: letters (with Latin layout fallback), arrows, `Enter`,
  `/` to search and `Esc`.
- Renderer: canvas, text with font fallback, truncation at safe cut
  points and highlights, and a dark theme (`tiny-skia`, `skrifa`).
- Vertical list layout: key badges, selection, scrolling and a search
  bar.
- Wayland presenter: layer-shell popup, shared memory double buffer,
  fractional scale, and a `popup_demo` example.
- Keyboard: `xkbcommon` keymaps and compose, Latin fallback for any
  layout, key repeat for arrows and `Backspace`.
- System fonts via `fc-match` with per-character fallback, so titles in
  scripts like Chinese render; font files are memory-mapped.
- Logger: `TYNEX_LOG` levels, one line per record on stderr, and crash
  output with a backtrace.
- Command line (`daemon`, `show`, `hide`, `quit`, `--help`,
  `--version`); `tynex daemon` loads the config and exits with code 78
  on errors.
- Daemon over D-Bus: single instance, `tynex show`, `hide` and `quit`,
  and one poll loop for Wayland and D-Bus with no idle wakeups.
- KDE adapter for Plasma 6: a KWin script pushes the window list in
  most recently used order and registers `Meta+Tab`, and windows are
  activated through KWin's window search. `desktop = "auto"` picks it
  on KDE. Only KWin may call the adapter's D-Bus object.
- `hotkey_action`: pressing the hotkey again while the popup is open
  switches to the previous window (`"last"`, default) or hides it
  (`"hide"`).
- The popup closes when it loses keyboard focus.
- Pinned letter keys: a window keeps its letter until it closes
  (ADR-0041).

[Unreleased]: https://github.com/canbesolved/tynex/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/canbesolved/tynex/releases/tag/v0.1.0
