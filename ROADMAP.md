# Roadmap

Planned features for `tynex`, in the Now / Next / Later format. Each
horizon maps to a [SemVer](https://semver.org) release. Items move from
Later to Next to Now as they are scheduled, and leave this file once
released. Released changes are listed in [CHANGELOG.md](CHANGELOG.md).

| Horizon | Release   | Meaning                             |
| ------- | --------- | ----------------------------------- |
| Now     | 0.1.0     | First release, in development       |
| Next    | 0.2.0     | Committed for the following release |
| Later   | Unplanned | Accepted ideas, not scheduled yet   |

Rules:

- Items are user-facing features, not internal tasks.
- Scheduling an item in Now or Next opens a GitHub issue and milestone.
- A design-changing item gets an ADR before work starts.

## Now: 0.1.0

Target: KDE Plasma 6 on Wayland.

- [ ] Resident daemon and CLI (`daemon`, `show`, `hide`, `quit`).
- [ ] KDE adapter: KWin script bridge, `Meta+Tab` shortcut.
- [ ] Centered popup with a vertical list: icon and title per window.
- [ ] App names from desktop files (`Name=`, localized), with
  `resourceClass` as fallback.
- [ ] Row text format: `title_format = "{app} : {title}"` in
  `[appearance]`, for example `Cursor : My homelab project`. Default
  `"{title}"`.
- [ ] Drop the app name from the end of titles:
  `strip_app_name = true` (default) in `[appearance]` turns
  `Café — Firefox` into `Café` (separators ` — `, ` – `, ` - `).
- [ ] Hotkey pressed again while the popup is open:
  `hotkey_action = "last"` (default) switches to the previous window,
  `"hide"` closes it.
- [ ] Pinned keys: a window keeps its letter until it closes, so
  letters no longer move when windows are switched. Rows stay in MRU
  order; a freed letter goes to the most recent window without one
  ([ADR-0041](docs/adr/0041_pinned_keys.md)).
- [ ] The popup hides when it loses keyboard focus.
- [ ] Hotkey mode: home row letters switch windows.
- [ ] Search mode: fuzzy filtering while typing.
- [ ] Configuration in `tynex.toml`.
- [ ] Packaging: systemd user unit and autostart entry.

Build order:

1. Core and config with unit tests, no I/O.
2. Mock window source to test core and config end to end.
3. Wayland popup and hotkey mode, driven by the mock window source.
4. KDE adapter with the KWin script, daemon and CLI over D-Bus.
5. Search mode, icons and app names from desktop files, packaging.

## Next: 0.2.0

- [ ] Hide app icons: `show_icons = false` in `[appearance]`.
- [ ] Per-app settings: an `[apps.<app_id>]` table per app, starting
  with a custom title, for example `[apps.firefox]` with
  `title = "Web"`.
- [ ] `tynex reload`: re-read the config without restarting. An invalid
  file is reported and the running config stays in use.

## Later

Layouts and modes:

- [ ] More layouts: grid, horizontal strip.
- [ ] More switching modes.
- [ ] Per-app keys: `key = "w"` in `[apps.<app_id>]` reserves a letter
  for the app's window; automatic pinning skips reserved letters.

Launcher:

- [ ] `launcher` module: find installed GUI apps and show them in the
  list with icon and name, filtered with fuzzy search, to start an app
  from the same popup.
  - Config: `launcher = "off"` (default), `"all"` or `"gui_only"`.
    `gui_only` skips terminal apps (`Terminal=true`).
  - Linux: `.desktop` files from `$XDG_DATA_HOME/applications` and
    `$XDG_DATA_DIRS`; `NoDisplay` and `Hidden` entries are never shown.
  - A running app lists its open windows first, then a "Launch new
    window" entry. An app that is not running has one launch entry.
  - Later sources: macOS app bundles, Windows Start menu shortcuts.

Look:

- [ ] Theme options in the config and import of desktop colors.
- [ ] Text shaping and right-to-left layout for Arabic, Hebrew and
  Indic scripts (`rustybuzz`).
- [ ] Color emoji in titles.

Languages:

- [ ] Input method support (`text-input-v3`) to type Chinese, Japanese
  and Korean in search mode.
- [ ] Phonetic search: match Chinese titles by pinyin and Japanese by
  romaji.

Platform support:

Two parts decide whether `tynex` works in an environment: a presenter
that shows the popup and an adapter that lists and activates windows.
Shared parts (core, modes, search, config, renderer) are portable.

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

- [ ] CI: `cargo check` for macOS and Windows targets.

[layer-shell]: https://wayland.app/protocols/wlr-layer-shell-unstable-v1
[wlr-toplevel]:
  https://wayland.app/protocols/wlr-foreign-toplevel-management-unstable-v1
[ext-toplevel]: https://wayland.app/protocols/ext-foreign-toplevel-list-v1
