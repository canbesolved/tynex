# Roadmap

Planned features for `tynex`, in the Now / Next / Later format. Each
horizon maps to a [SemVer](https://semver.org) release. Items move from
Later to Next to Now as they are scheduled, and leave this file once
released. Released changes are listed in [CHANGELOG.md](CHANGELOG.md),
dropped items in [DEPRECATIONS.md](DEPRECATIONS.md).

| Horizon | Release               | Meaning                                  |
| ------- | --------------------- | ---------------------------------------- |
| Now     | 0.2.x                 | MVP with KDE Plasma 6 on Wayland support |
| Next    | 0.8.x - 1.0.0         | Committed for the following releases     |
| Later   | Unplanned             | Accepted ideas, not scheduled yet        |

Rules:

- Items are user-facing features, not internal tasks.
- A design-changing item gets an ADR before work starts.
- Reserved series: 0.8.x for performance, 0.9.x for security and
  stability, 1.0.0 for polishing and the first stable release.

## Now

Target: MVP with KDE Plasma 6 on Wayland support.

### 0.2.4

- [ ] A launch row reads `+ New window` instead of `Launch new window`.

### 0.2.5

- [ ] The input bar prompt is `>` instead of `/`.

### 0.2.6

- [ ] The input bar shows a line cursor.

### 0.2.7

- [ ] Hint text inside the input bar while the query is empty. It hides
  as soon as you type. Config under `[input_bar]`:
  - `hint.enabled`: `true` or `false`.
  - `hint.font_size`: hint text size.
  - `hint.font_color`: `"#RRGGBB"`.
  - `hint.string`: a shell command. On every launch its stdout is the
    hint text, for example `bash -c 'date'` or `cat /tmp/weather.txt`.

### 0.2.8

- [ ] `Space` works as `Enter`: it picks the selected match, so a
  search query no longer contains spaces.

### 0.2.9

- [ ] Icon bar: running apps shown as a row of app icons, one per app,
  to replace KDE's "Icons-only Task Manager" widget. It shows only
  with the popup, on demand.
  - An app with several windows shows their count under its icon.
  - A mouse click on an icon launches the app if it is not running,
    activates its window if it has one, or opens a dropdown list of
    its window titles if it has several.
  - The dropdown:
    - opens under the selected app;
    - lists windows alphabetically by title;
    - fits the longest title, up to 450 px wide, scaled with the
      display scale; longer titles end with an ellipsis (`…`);
    - slides in and out smoothly at about 120 frames per second.
  - The strip stays in place when the dropdown opens: the popup keeps
    a fixed size with transparent spare room.
  - Config: `[icon_bar]` with `enabled = true` (default) shows the
    strip with the input bar; with `false`, only the input bar shows
    until you type. Matches appear in the output bar.
  - Keys:
    - `Left` and `Right` move along the strip;
    - `Tab` moves through the matches.
  - Launching uses the launcher sub-module of the input bar.

### 0.2.10

- [ ] Each UI module is its own block: a small gap separates the bars
  (for example the input bar and the title bar), and every block uses
  the same rounded border and theme colors.

### 0.2.11

- [ ] When you search, the output bar lists open windows first, then
  installed apps from the launcher.

### 0.2.12

- [ ] Update the default theme to the Endgame color palette. Exact
  color values will be added when the palette is finalized.

## Next

### 0.8.x: performance

The whole 0.8.x series is reserved for performance work.

- [ ] Faster, more responsive and smoother UI overall: the popup opens,
  redraws and reacts to every key without visible delay.
- [ ] Performance: holding `Up` or `Down` moves the selection one row
  per key repeat, without skipping rows, and each frame is drawn
  before the next repeat.

### 0.9.x: security and stability

The whole 0.9.x series is reserved for security and stability fixes.
No items yet.

### 1.0.0: first stable release

Polishing and the first stable release. No items yet.

## Later

Window list:

- [ ] Close windows from the popup, like `Alt+F4`: `Ctrl+W` or a right
  click closes the selected window.
  - In the icon bar, a right click on an app with several windows
    opens its dropdown; a right click on a title there closes that
    window.

Titles:

- [ ] Rename a window from the popup: `F2` edits the selected title in
  place, `Enter` keeps it, `Esc` cancels. The new title lasts until the
  window closes.
- [ ] Configurable prefixes on rows and titles:
  - Per app: `[apps.<app_id>]` with `prefix = "Web"` so launcher rows
    read `Web: Firefox`.
  - By pattern: `[title_prefix]` maps a pattern to a prefix on matching
    window titles, for example `"Jira" = "[J] "`. Patterns are regular
    expressions, so plain text matches as well.

Icon bar:

- [ ] Pinned apps stay in the icon bar while they are not running.

Launcher:

- [ ] App aliases: map a name to an installed app, for example
  `Firefox = "Web Browser"`. A query that matches the alias suggests
  launching that app.

Info bar:

- [ ] `info_bar` module: a status bar inside the popup that can replace
  useful parts of the system panel (time and date, battery percentage,
  and the like). It runs only while the popup is open, so it adds no
  idle work.
  - Config: `[info_bar]` with `placement = "top"`, `"bottom"` or
    `"both"`.
  - Custom scripts: a shell script's output is shown in the bar; each
    output line becomes a row.
  - Plugins: date and time, battery, power usage in watts, CPU load,
    RAM usage.
  - Open questions: hotkeys for bar actions; a keyboard layout plugin.

Configuration:

- [ ] `tynex reload`: re-read the config without restarting. An invalid
  file is reported and the running config stays in use.

Themes:

- [ ] Theme options: opacity, background, selection, search highlight
  and text colors, border width and color, font size and style.
- [ ] Light theme next to the default dark one.
- [ ] Custom themes in `~/.config/tynex/tynex.theme`, chosen with
  `dark_theme = "tynex.theme"` and `light_theme = "tynex.theme"`; one
  file may hold both a dark and a light variant.
- [ ] Follow the system: switch between the dark and light theme when
  the desktop color scheme changes.
- [ ] Import colors from the desktop color scheme.

Look:

- [ ] Text shaping and right-to-left layout for Arabic, Hebrew and
  Indic scripts (`rustybuzz`).
- [ ] Color emoji in titles.

Languages:

- [ ] Input method support (`text-input-v3`) to type Chinese, Japanese
  and Korean in prompt mode.
- [ ] Phonetic search: match Chinese titles by pinyin and Japanese by
  romaji.
- [ ] Switch the keyboard layout to English when the popup opens and
  restore the previous one when it closes: `switch_layout = "us"`, off
  by default.

Platform support:

Two parts decide whether `tynex` works in an environment: a presenter
that shows the popup and an adapter that lists and activates windows.
Shared parts (core, modes, modules, config, renderer) are portable.

| Environment                 | Session | Popup        | Windows           |
| --------------------------- | ------- | ------------ | ----------------- |
| KDE Plasma                  | Wayland | layer-shell  | KWin script       |
| Sway, Hyprland, niri, river | Wayland | layer-shell  | wlr toplevel      |
| Wayfire, labwc, Treeland    | Wayland | layer-shell  | wlr toplevel      |
| Budgie 10.10, LXQt, Xfce    | Wayland | layer-shell  | wlr toplevel      |
| COSMIC                      | Wayland | layer-shell  | COSMIC protocols  |
| Cinnamon                    | Wayland | layer-shell  | none yet          |
| dwl, Qtile                  | Wayland | unverified   | unverified        |
| GNOME, Pantheon             | Wayland | other popup  | Shell extension   |
| Xfce, Cinnamon, MATE        | X11     | X11 popup    | EWMH              |
| i3, bspwm, awesome, openbox | X11     | X11 popup    | EWMH              |
| Qtile, xmonad, Deepin 23    | X11     | X11 popup    | EWMH              |
| dwm                         | X11     | X11 popup    | EWMH + patch      |
| macOS                       | -       | AppKit panel | Accessibility API |
| Windows                     | -       | Win32 popup  | Win32 API         |

- Popup "layer-shell": the Wayland presenter from 0.1.0 works as is.
- "wlr toplevel": the `wlr-foreign-toplevel-management` protocol, part
  of `wayland-protocols-wlr`, so the adapter needs no new crate.
- Support data from wayland.app, checked 2026-10-15:
  [layer-shell][layer-shell], [wlr toplevel][wlr-toplevel],
  [ext toplevel list][ext-toplevel].

Adapters:

- [ ] wlroots-style compositors: `wlr-foreign-toplevel-management`.
- [ ] COSMIC: `ext-foreign-toplevel-list` and COSMIC toplevel
  management.
- [ ] X11: EWMH (`_NET_CLIENT_LIST`, `_NET_ACTIVE_WINDOW`) with
  `x11rb`.
- [ ] GNOME: Shell extension exposing windows over D-Bus.
- [ ] macOS: `CGWindowListCopyWindowInfo` and Accessibility API.
- [ ] Windows: `EnumWindows` and `SetForegroundWindow`.

Presenters:

- [ ] X11: override-redirect window with a keyboard grab.
- [ ] GNOME: popup without layer-shell.
- [ ] macOS: AppKit panel.
- [ ] Windows: Win32 popup.

Platform services:

- [ ] Unix socket IPC: fallback without a D-Bus session bus (minimal
  X11 setups without systemd) and default on macOS.
- [ ] Windows named pipe IPC.
- [ ] Font lookup on macOS and Windows (no `fc-match`).

Project:

- [ ] Packaging: systemd user unit and autostart entry.
- [ ] CI: `cargo check` for macOS and Windows targets.
- [ ] Own fuzzy matcher in place of `frizbee`, to shrink the binary by
  about 3.7 MB, if it can match its ranking and typo tolerance
  ([ADR-0044](docs/adr/0044_binary_budget.md)).
- [ ] Prebuilt binaries for macOS on Apple silicon and Windows x64 and
  arm64, added with each port (ADR-0042).

[layer-shell]: https://wayland.app/protocols/wlr-layer-shell-unstable-v1
[wlr-toplevel]:
  https://wayland.app/protocols/wlr-foreign-toplevel-management-unstable-v1
[ext-toplevel]: https://wayland.app/protocols/ext-foreign-toplevel-list-v1
