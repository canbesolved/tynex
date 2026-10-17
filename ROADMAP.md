# Roadmap

Planned features for `tynex`, in the Now / Next / Later format. Each
horizon maps to a [SemVer](https://semver.org) release. Items move from
Later to Next to Now as they are scheduled, and leave this file once
released. Released changes are listed in [CHANGELOG.md](CHANGELOG.md).

| Horizon | Release   | Meaning                             |
| ------- | --------- | ----------------------------------- |
| Now     | 0.1.x     | Patch releases, in the order below  |
| Next    | 0.2.0     | Committed for the following release |
| Later   | Unplanned | Accepted ideas, not scheduled yet   |

Rules:

- Items are user-facing features, not internal tasks.
- A design-changing item gets an ADR before work starts.

## Now: 0.1.x

Target: KDE Plasma 6 on Wayland.

### 0.1.3

- [ ] App icons in the list, from desktop files and the icon theme.

### 0.1.4

- [ ] Packaging: systemd user unit and autostart entry.

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
- [ ] Own fuzzy matcher in place of `frizbee`, to shrink the binary by
  about 3.7 MB, if it can match its ranking and typo tolerance
  ([ADR-0044](docs/adr/0044_binary_budget.md)).
- [ ] Prebuilt binaries for macOS on Apple silicon and Windows x64 and
  arm64, added with each port (ADR-0042).

[layer-shell]: https://wayland.app/protocols/wlr-layer-shell-unstable-v1
[wlr-toplevel]:
  https://wayland.app/protocols/wlr-foreign-toplevel-management-unstable-v1
[ext-toplevel]: https://wayland.app/protocols/ext-foreign-toplevel-list-v1
