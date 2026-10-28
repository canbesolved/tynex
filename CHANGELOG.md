# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `title_bar.show_icons` (default `true`) draws the app icon before
  each title-bar title. `false` shows titles only (ADR-0058).

### Removed

- **Breaking:** `title_bar.show_app_icon` and the app name column.
  A config that still sets `show_app_icon` stops the daemon
  (ADR-0058).

## [0.2.2] - 2026-10-29

The title bar hides while the query is non-empty.

### Changed

- The title bar hides as soon as you type, and shows again when the
  query is cleared, if it is enabled (ADR-0057).

## [0.2.0] - 2026-10-28

Config sections and launcher. Breaking: `[appearance]` and the old
top-level keys are removed.

### Added

- Launcher as an input-bar sub-module: matching installed apps appear
  in the output bar; `Enter` starts them (ADR-0056).
  - Config: `input_bar.launcher.mode = "off"` (default), `"all"`, or
    `"gui_only"`.
  - Linux: `.desktop` catalog and `Exec` spawn in
    `platform/linux/apps/`.

### Changed

- `title_bar.show_app_icon` draws the app name column in the title
  bar only; the output bar shows icon and title.
- **Breaking:** the config file is one section per part and module:
  `[main]`, `[hotkeys]`, `[input_bar]`, `[output_bar]`,
  `[title_bar]` (ADR-0055).
  - `desktop`, `font`, `font_size` and `width` move to `[main]`.
  - `hotkey_action` becomes `hotkeys.action`.
  - `max_rows` becomes `output_bar.max_rows`.
  - `title_bar` becomes `title_bar.enabled`.
  - `sort_by` becomes `title_bar.sort_by`.
  - `show_app` becomes `title_bar.show_app_icon`.
  - `strip_app_name` becomes `title_bar.strip_app_name`.
  - `[appearance]` is removed. A file that still has an old key
    stops the daemon until the file is edited.

## [0.1.7] - 2026-10-28

UI modules. Breaking: the `layout` config key is removed.

### Changed

- The popup is UI modules: input bar, output bar, title bar, info
  bar and icon bar. Each can be changed on its own. The input bar is
  always on. The title bar lists open windows only. Search matches
  are the output bar, which will also show later results of the input
  (ADR-0052, ADR-0053, ADR-0054).

### Removed

- **Breaking:** the `layout` config key. A file that still sets it
  stops the daemon until the key is removed.

## [0.1.6] - 2026-10-27

Prompt mode only, with `title_bar`. Breaking: hotkey mode is removed.

### Added

- `title_bar` in `[appearance]` shows the window list under the search
  prompt before you type (default `false`: only the prompt until you
  type) (ADR-0050).

### Changed

- **Breaking:** prompt mode is the only mode. The popup opens with the
  previous window selected, so `Enter` right away switches back.
- The only mode is called prompt mode, because it will do more than
  search (ADR-0051).
- `sort_by` defaults to `"recently"`.
- `strip_app_name` in `[appearance]` defaults to `false`: titles keep
  the app name.

### Removed

- **Breaking:** hotkey mode and its letter keys. The `mode` and `keys`
  config keys and `sort_by = "key"` are removed; a config file with
  them stops the daemon until they are removed. Set
  `title_bar = true` to keep a window list on screen.
- **Breaking:** `tynex show --mode`. The D-Bus `Show` call accepts only
  an empty mode.

## [0.1.5] - 2026-10-18

Popup rows in columns. Breaking: `title_format` is removed.

### Added

- Popup rows in columns: key, icon, app name and title.
  `show_app = true` in `[appearance]` shows the app name column
  (default `false`); its width follows `width` (ADR-0049).

### Removed

- **Breaking:** `title_format` in `[appearance]`, replaced by
  `show_app`. A config file with `title_format` stops the daemon
  until the key is removed.

## [0.1.4] - 2026-10-18

Rows that stay in place: `sort_by`.

### Added

- `sort_by` sets the row order: `"key"` (default) keeps rows in the
  order of their letters, so they never move while switching; `"app"`
  groups them by app name; `"recently"` is the order before this
  release. The popup opens with the previous window selected in every
  order (ADR-0048).

### Changed

- Rows follow their letters by default instead of most recently used
  order; set `sort_by = "recently"` for the old order.

## [0.1.3] - 2026-10-17

App icons and one font for all text.

### Added

- App icons in the list: `Icon=` from the app's desktop file, found
  in the Plasma icon theme, its parents and `hicolor`; PNG and SVG
  (`resvg`). The release binary grows to about 6.6 MB (ADR-0046).

### Changed

- One font for all text: `font` from the config, or the system
  sans-serif font. Characters it lacks show as boxes; there is no
  per-character fallback font any more, so set `font` to a font with
  your scripts, for example Noto Sans CJK (ADR-0047).
- Titles and app names drop invisible characters (bidi marks such as
  `U+200E`, zero width space, word joiner, byte order mark), which
  would show as boxes with one font.

## [0.1.2] - 2026-10-17

App names and title format.

### Added

- App names from desktop files: the localized `Name` of the app's
  desktop file, else the WM class (ADR-0045).
- `title_format` in `[appearance]` sets the row text, for example
  `"{app} : {title}"` gives `Cursor : My homelab project`. Default
  `"{title}"`.
- `strip_app_name` in `[appearance]` drops the app name from the end
  of titles: `Café — Firefox` becomes `Café`. On by default.

### Changed

- Search matches the app name as well as the title and app id.

## [0.1.1] - 2026-10-17

Prompt mode and prebuilt Linux binaries.

### Added

- Prompt mode: typing filters windows by title and app id with fuzzy
  matching (`frizbee`): smart case, one typo per four characters,
  accent folding (`cafe` finds `Café`) and highlighted matches. The
  release binary grows to about 5.4 MB; the budget is now 10 MB
  (ADR-0044).
- Prebuilt binaries for Linux x86_64 and arm64 on each GitHub release,
  with a `SHA256SUMS` file, built by `.github/workflows/release.yml`
  (ADR-0042).

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

[Unreleased]: https://github.com/canbesolved/tynex/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/canbesolved/tynex/releases/tag/v0.2.2
[0.2.0]: https://github.com/canbesolved/tynex/releases/tag/v0.2.0
[0.1.7]: https://github.com/canbesolved/tynex/releases/tag/v0.1.7
[0.1.6]: https://github.com/canbesolved/tynex/releases/tag/v0.1.6
[0.1.5]: https://github.com/canbesolved/tynex/releases/tag/v0.1.5
[0.1.4]: https://github.com/canbesolved/tynex/releases/tag/v0.1.4
[0.1.3]: https://github.com/canbesolved/tynex/releases/tag/v0.1.3
[0.1.2]: https://github.com/canbesolved/tynex/releases/tag/v0.1.2
[0.1.1]: https://github.com/canbesolved/tynex/releases/tag/v0.1.1
[0.1.0]: https://github.com/canbesolved/tynex/releases/tag/v0.1.0
