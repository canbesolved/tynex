# Roadmap

Planned features for `tynex`, in the Now / Next / Later format. Items
move from Later to Next to Now as they are scheduled, and leave this
file once released. Released changes are listed in
[CHANGELOG.md](CHANGELOG.md), dropped items in
[DEPRECATIONS.md](DEPRECATIONS.md).

| Horizon | Release     | Meaning                                  |
| ------- | ----------- | ---------------------------------------- |
| Now     | 0.3.x–0.5.x | Scheduled releases                       |
| Next    | After 0.5.x | Committed once the 0.5.x horizon is done |
| Later   | Unplanned   | Accepted ideas, not scheduled yet        |

Rules:

- Items are user-facing features, not internal tasks.
- A design-changing item gets an ADR before work starts.
- Every item has a code, `RM-` plus at least four digits
(`RM-NNNN`), right after the checkbox. The code stays when the item
moves or is reworded.
- A new item takes the lowest free code, printed by
`scripts/rm_next.sh`. A code is free when it is in none of
`ROADMAP.md`, `CHANGELOG.md` and `DEPRECATIONS.md`.
- Work on an item names its code in the commit subject, the pull
request, the `CHANGELOG.md` entry and, for a dropped item, the
`DEPRECATIONS.md` entry. See [AGENTS.md](AGENTS.md#roadmap-codes).

## Now

### 0.4.0

- [ ] RM-0005 `tynex qs` (quick switch): back to last without opening
  the popup, the same way as `hotkeys.action = "last"`.

### 0.5.0

- [ ] RM-0062 Text editing in the input bar: `Ctrl+A` selects the
  whole query, `Ctrl+X` cuts the selection and `Ctrl+V` pastes from
  the clipboard. Dragging the mouse over the query selects text.
- [ ] RM-0067 **Breaking:** remove `taskbar.mode = "both"`. The popup
  shows either the icon strip or the title list, never both, so
  only one of them takes the keys. Values are `off`, `iconbar`
  (default) and `titlebar`. A file that still sets `both` stops the
  daemon until it is edited.
- [ ] RM-0066 Arrow keys open and close the icon dropdown:
  - `Down` on a selected icon with several windows opens its
    dropdown, like `Enter` and `Space`.
  - `Up` on the top title of an open dropdown closes it and leaves
    the icon selected.

## Next

Browser tabs:

- [ ] RM-0003 Open a new tab in the browser from the popup, or with the
  `hotkeys.new_tab` key while the popup is open.
- [ ] RM-0004 Browser bookmarks in the popup: search bookmarks and open
  one.

Window list:

- [ ] RM-0013 Close windows from the popup, like `Alt+F4`: `Ctrl+W` or a right
  click closes the selected window.
  - In the icon bar, a right click on an app with several windows
  opens its dropdown; a right click on a title there closes that
  window.

Launcher:

- [ ] RM-0015 App aliases: map a name to an installed app, for example
  `Firefox = "Web Browser"`. A query that matches the alias suggests
  launching that app.

## Later

Performance:

- [ ] RM-0016 Faster, more responsive and smoother UI overall: the popup opens,
  redraws and reacts to every key without visible delay.
- [ ] RM-0017 Performance: holding `Up` or `Down` moves the selection one row
  per key repeat, without skipping rows, and each frame is drawn
  before the next repeat.

Titles:

- [ ] RM-0018 Configurable prefixes on rows and titles:
  - Per app: `[apps.<app_id>]` with `prefix = "Web"` so launcher rows
  read `Web: Firefox`.
  - By pattern: `[title_prefix]` maps a pattern to a prefix on matching
  window titles, for example `"Jira" = "[J] "`. Patterns are regular
  expressions, so plain text matches as well.

Icon bar:

- [ ] RM-0019 `taskbar.iconbar.size = "auto"`: the icon side follows the
  screen size, `main.width` and the number of running apps, within
  the usual 8 to 128 logical pixels. A number still sets a fixed
  size.

Output bar:

- [ ] RM-0065 Pointer in the output list: hovering a row moves the
  selection highlight to it, and a click activates the row, as in
  the title list (ADR-0077).

Modes:

- [ ] RM-0020 Mode indicator: the sign at the start of the input bar (`>`
  today) shows the current mode and changes when you switch mode:
  prompt, AI, files and others. It comes with the second mode.

Browser tabs:

- [ ] RM-0021 Edge and Vivaldi: register the host for them too,
  once their manifest folders are confirmed.
- [ ] RM-0022 Firefox: same local bridge and search.
- [ ] RM-0063 `browser_tabs.match_first`: which part of a tab ranks
  first in search results, `"title"` (default) or `"url"`.
  - The URL part is the site's domain without `www.`, for example
  `github.com` or `reddit.com`, not the path.
  - With `"url"`, typing `reddit` lists every tab whose domain
  matches first, then the tabs that match only by title.
  - With `"title"`, title matches come first, then domain matches.
  - Inside each group, tabs follow usage stats (RM-0023).

Info bar:

- [ ] RM-0024 `info_bar` module: a status bar inside the popup that can replace
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

- [ ] RM-0006 `popup_delay`: milliseconds to wait before the popup
  appears. Default `200`. With `hotkeys.action = "last"`, a second
  hotkey press inside that window uses back to last instead of
  showing the popup (see Back to last in 0.4.0).
- [ ] RM-0025 `tynex reload`: re-read the config without restarting. An invalid
  file is reported and the running config stays in use.

Logging:

- [ ] RM-0032 Improve logging: clearer messages and useful context on warnings
  and errors. Config:
  - `log_level`: `info`, `warn`, `error` or `debug`.
  - `log_file`: path to the log file, default `~/.cache/tynex.log`.

Themes:

- [ ] RM-0033 Update the default theme to the Endgame color palette. Exact
  color values will be added when the palette is finalized.
- [ ] RM-0034 Configurable background, selection and font colors.
- [ ] RM-0035 Theme options: opacity, background, selection, search highlight
  and text colors, border width and color, font size and style.
- [ ] RM-0036 Custom themes in `~/.config/tynex/tynex.theme`, chosen with
  `dark_theme = "tynex.theme"` and `light_theme = "tynex.theme"`; one
  file may hold both a dark and a light variant.
- [ ] RM-0037 Follow the system: switch between the dark and light theme when
  the desktop color scheme changes.
- [ ] RM-0038 Import colors from the desktop color scheme.

Look:

- [ ] RM-0008 Popup animation: `animation = true` or `false` turns it
  on or off. `animation_speed = 1.0` scales how fast it runs, from
  `0.5` to `3.0` (`1.0` is normal speed).
- [ ] RM-0014 A shadow around the popup.
- [ ] RM-0041 Color emoji in titles.

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
[layer-shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1), [wlr toplevel](https://wayland.app/protocols/wlr-foreign-toplevel-management-unstable-v1),
[ext toplevel list](https://wayland.app/protocols/ext-foreign-toplevel-list-v1).

Adapters:

- [ ] RM-0045 wlroots-style compositors: `wlr-foreign-toplevel-management`.
- [ ] RM-0046 COSMIC: `ext-foreign-toplevel-list` and COSMIC toplevel
  management.
- [ ] RM-0047 X11: EWMH (`_NET_CLIENT_LIST`, `_NET_ACTIVE_WINDOW`) with
  `x11rb`.
- [ ] RM-0048 GNOME: Shell extension exposing windows over D-Bus.
- [ ] RM-0049 macOS: `CGWindowListCopyWindowInfo` and Accessibility API.
- [ ] RM-0050 Windows: `EnumWindows` and `SetForegroundWindow`.

Presenters:

- [ ] RM-0051 X11: override-redirect window with a keyboard grab.
- [ ] RM-0052 GNOME: popup without layer-shell.
- [ ] RM-0053 macOS: AppKit panel.
- [ ] RM-0054 Windows: Win32 popup.

Platform services:

- [ ] RM-0055 Unix socket IPC: fallback without a D-Bus session bus (minimal
  X11 setups without systemd) and default on macOS.
- [ ] RM-0056 Windows named pipe IPC.
- [ ] RM-0057 Font lookup on macOS and Windows (no `fc-match`).

Project:

Release milestones:

- 0.x milestone: define and test core functionality and design.
- 1.x milestone: Linux desktop support: KDE, GNOME, LXQt, Xfce,
Hyprland and COSMIC. Cinnamon, LXDE, MATE, Openbox and others
follow later.
- 2.x milestone: macOS implementation.
- 3.x milestone: Windows implementation.
- 4.x milestone: add more features and refactor.

- [ ] RM-0058 Packaging: systemd user unit and autostart entry.
- [ ] RM-0059 CI: `cargo check` for macOS and Windows targets.
- [ ] RM-0060 Own fuzzy matcher in place of `frizbee`, to shrink the binary by
  about 3.7 MB, if it can match its ranking and typo tolerance
  ([ADR-0044](docs/adr/0044_binary_budget.md)).
- [ ] RM-0061 Prebuilt binaries for macOS on Apple silicon and Windows x64 and
  arm64, added with each port (ADR-0042).
