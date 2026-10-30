# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.10] - 2026-10-30

The icon strip and the title list are one `[taskbar]` module.

### Added

- `[taskbar]` with `mode`: `off`, `icons` (default), `titles` or
  `both`. The icon strip and the title list are one module
  (ADR-0065).

### Changed

- **Breaking:** `icon_bar.enabled` and `title_bar.enabled` are
  replaced by `taskbar.mode`.
  - Both keys on becomes `both`.
  - Only `icon_bar.enabled` on becomes `icons`.
  - Only `title_bar.enabled` on becomes `titles`.
  - Both keys off becomes `off`.
  - A file that still sets either `enabled` key stops the daemon
    until the file is edited.

## [0.2.9] - 2026-10-29

One icon per running app, under the input bar.

### Added

- `[icon_bar]` draws one icon per running app under the input bar
  while the query is empty. `enabled` defaults to `true`.
  `icon_size` defaults to `24` logical pixels (8 to 128) and scales
  with the display. The row is centered, or shrunk until it fits.
  An app with several windows shows a count and opens a dropdown
  of its titles. `Left` and `Right` move along the strip. `Enter`,
  `Space` and a left click activate one window, open the dropdown,
  or launch. The dropdown slides over 150 ms (ADR-0062).

### Changed

- `icon_bar.border` sets the selection outline in logical pixels,
  scaled with the display. The default is `4`, from 1 to 32.
- The icon bar is its own rounded frame, 3 logical pixels below the
  input bar. The gap scales with the display and shows the desktop.
- A gap of 3 logical pixels separates the icon bar from the title
  bar, and the dropdown from the icon bar. Both scale with the
  display.
- `Tab` moves to the next icon while the strip is showing and the
  title bar is off. An open dropdown still takes `Tab` first
  (ADR-0063).
- Moving the pointer onto an icon selects it and closes an open
  dropdown. Moving off the icons leaves the selection where it is.

### Fixed

- The selected icon is outlined by a border outside the icon.
- The last title in an icon dropdown stays on screen when the slide
  ends.
- With the title bar on, `Enter` and `Space` activate the selected
  window instead of opening the icon dropdown (ADR-0064).

## [0.2.8] - 2026-10-29

`Space` confirms the selected match, and `Tab` walks the visible list.

### Changed

- `Space` picks the selected match, the same as `Enter`. A search
  query no longer contains spaces (ADR-0061).
- `Tab` moves the selection through the list on screen: open windows
  in the title bar while the query is empty and `title_bar.enabled`
  is set, and matches in the output bar while a query is typed. With
  neither list showing, `Tab` does nothing (ADR-0061).

## [0.2.7] - 2026-10-29

The input bar shows a hint while the query is empty.

### Added

- `hint.enabled`, `hint.text`, `hint.font_color` and `hint.update`
  live under `[input_bar]`. `hint.text` is a shell command. Its
  first stdout line is the hint, centered in the popup, at
  `main.font_size`. The popup opens before the command exits.
  Until then the hint steps through `.`, `..`, `...` and blank.
  `hint.update` is the wait in seconds after the command exits;
  `0` runs once per open. The default command is
  `date +"%A %b %-d %T %Y"` and the default color is `#8da787`
  (ADR-0060).

### Fixed

- A hint that arrives while both popup buffers are busy is drawn
  when a buffer is released, instead of staying on the loading dot
  until the next key.
- A hint refresh keeps the previous text until the new line is
  ready, so the loading dots do not flash over it.

## [0.2.6] - 2026-10-29

The input bar shows a beam cursor.

### Added

- A row gap and one space separate `>` from the query.
- The input bar draws a 1 px beam at the cursor, 10% shorter than
  the text, in the text color. `Left` and `Right` move it by one
  character, `Home` and `End` jump, and `Ctrl+Left` and
  `Ctrl+Right` move by word. Typing inserts at the cursor.
  `Backspace` and `Delete` remove the character before it and at
  it. The beam stays while the query is empty and does not blink.

## [0.2.5] - 2026-10-29

The input bar prompt is `>`.

### Changed

- The input bar prompt is `>` instead of `/`.

## [0.2.4] - 2026-10-29

The launcher is on by default. A running app gets a new-window row.

### Added

- `launcher.new_window` under `[input_bar]` turns the extra launch
  row under a running app on or off. `launcher.new_window_text` and
  `launcher.new_window_text_color` (`#RRGGBB`) set its title and
  color. The default title is one space, `+`, two spaces, then
  `New window`. The default color is `#8da787` (ADR-0059).

### Changed

- The default `launcher.mode` is `gui_only` instead of `off`.
- A launch row for a running app reads one space, `+`, two spaces,
  then `New window`, instead of `Launch new window`.

## [0.2.3] - 2026-10-29

The title bar row is the app icon and the window title. Breaking:
`show_app_icon` is removed.

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

[Unreleased]: https://github.com/canbesolved/tynex/compare/v0.2.10...HEAD
[0.2.10]: https://github.com/canbesolved/tynex/releases/tag/v0.2.10
[0.2.9]: https://github.com/canbesolved/tynex/releases/tag/v0.2.9
[0.2.8]: https://github.com/canbesolved/tynex/releases/tag/v0.2.8
[0.2.7]: https://github.com/canbesolved/tynex/releases/tag/v0.2.7
[0.2.6]: https://github.com/canbesolved/tynex/releases/tag/v0.2.6
[0.2.5]: https://github.com/canbesolved/tynex/releases/tag/v0.2.5
[0.2.4]: https://github.com/canbesolved/tynex/releases/tag/v0.2.4
[0.2.3]: https://github.com/canbesolved/tynex/releases/tag/v0.2.3
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
