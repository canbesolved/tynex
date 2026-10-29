<!-- markdownlint-disable MD033 MD041 -->
<p align="center">
  <img src="assets/icon.svg" alt="tynex icon" width="128">
</p>

<h1 align="center">tynex</h1>

<p align="center">
  <code><b>T</b>hank <b>Y</b>ou, <b>NEX</b>t: efficient task switcher
  for Linux, macOS and Windows.</code>
</p>

<p align="center">
  <a href="https://github.com/canbesolved/tynex/actions/workflows/ci.yml"><img
    alt="CI" src=
    "https://github.com/canbesolved/tynex/actions/workflows/ci.yml/badge.svg"
    ></a>
  <a href="LICENSE"><img alt="License: MIT"
    src="https://img.shields.io/github/license/canbesolved/tynex"></a>
  <a href="Cargo.toml"><img alt="Rust 1.89+"
    src="https://img.shields.io/badge/rust-1.89+-orange?logo=rust"></a>
  <a href="https://api.reuse.software/info/github.com/canbesolved/tynex"><img
    alt="REUSE compliant"
    src="https://api.reuse.software/badge/github.com/canbesolved/tynex"></a>
  <a href="https://github.com/sponsors/canbesolved"><img alt="Sponsor"
    src="https://img.shields.io/badge/sponsor-❤-ea4aaa?logo=githubsponsors"></a>
</p>
<!-- markdownlint-enable MD033 MD041 -->

Press a hotkey, type a few letters of a window, press `Enter`, and you
are there. Or press `Enter` right away to go back to the previous
window. No mouse, no scrolling, no waiting.

> Status: 0.2.3, for KDE Plasma 6 on Wayland. The icon bar follows
> in 0.2.x (see the [roadmap](ROADMAP.md)).

## Features

- Centered popup with an input bar: start typing at once; accents
  and small typos are forgiven.
- Optional list of open windows (`title_bar.enabled`) while the query
  is empty; the list hides as soon as you type and returns when you
  clear the query. Order is most recently used first or grouped by app
  (`title_bar.sort_by`).
- Matches appear in the output bar as you type; with
  `input_bar.launcher.mode` set, installed apps appear there too.
- Each entry shows the app icon and the window title. The title bar
  can hide its icons with `title_bar.show_icons = false`.
- Low footprint: idle at 0 % CPU, under 10 MB of RAM, no GPU usage.
- Modular core: window sources, UI modules and modes are plug-in
  modules.
- Configuration in a single TOML file.

## Supported environments

| Environment      | Session | Status  |
| ---------------- | ------- | ------- |
| KDE Plasma 6     | Wayland | 0.2.3   |
| Other DEs / WMs  | Any     | Planned |
| macOS, Windows   | -       | Planned |

Planned environments and features are listed in the
[roadmap](ROADMAP.md).

## Download

Each [release](https://github.com/canbesolved/tynex/releases) has
prebuilt binaries for Linux x86_64 and arm64 (glibc 2.35 or newer),
for example:

```bash
v=0.2.3 t=x86_64-unknown-linux-gnu     # or aarch64-unknown-linux-gnu
url=https://github.com/canbesolved/tynex/releases/download/v$v
curl -LO "$url/tynex-$v-$t.tar.gz" -LO "$url/SHA256SUMS"
sha256sum -c --ignore-missing SHA256SUMS
tar -xzf "tynex-$v-$t.tar.gz"
install -Dm755 "tynex-$v-$t/tynex" ~/.local/bin/tynex
```

They need `libdbus-1` and `libxkbcommon`, which every Plasma desktop
has. Binaries for macOS (Apple silicon) and Windows (x64, arm64) come
with their ports (see the [roadmap](ROADMAP.md)).

## Build from source

Requirements:

- Linux with a Wayland compositor that supports layer-shell (KDE
  Plasma 6 for the current release), a D-Bus session bus and fontconfig.
- Rust 1.89 or newer ([rustup](https://rustup.rs)).
- `gcc`, `pkg-config`, Wayland, xkbcommon and D-Bus development
  headers, for example on Fedora:

  ```bash
  sudo dnf install gcc pkgconf-pkg-config wayland-devel \
    libxkbcommon-devel dbus-devel
  ```

Build and install to `~/.local/bin`:

```bash
git clone git@github.com:CanBeSolved/tynex.git
cd tynex
cargo build --release
install -Dm755 target/release/tynex ~/.local/bin/tynex
```

Packages for Debian, Ubuntu and Arch are listed in
[CONTRIBUTING.md](CONTRIBUTING.md).

## Usage

```bash
tynex daemon  # Start the background service
tynex show    # Open the switcher
tynex hide    # Close the switcher
tynex quit    # Stop the background service
```

On KDE the default hotkey is `Meta+Tab`. Change it in System Settings,
Shortcuts, `tynex`. On other desktops bind `tynex show` to any key.
Pressing the hotkey again switches to the previous window, or closes
the popup with `hotkeys.action = "hide"`. The popup also closes when
it loses keyboard focus.

In the popup, type to filter, `Enter` to switch, arrows to move, and
`Esc` to clear the query or close. `Enter` right after opening
switches to the previous window, like Alt+Tab.

## Configuration

The config file is `~/.config/tynex/tynex.toml`:

```toml
[main]
desktop = "auto"             # auto | kde | mock
font = "auto"                # Or a font file with your scripts (CJK)
font_size = 14
width = 480

[hotkeys]
action = "last"              # hide | last

[input_bar]
launcher.mode = "off"        # off | all | gui_only

[output_bar]
max_rows = 19

[title_bar]
enabled = false              # Window list before typing
sort_by = "recently"         # recently | app
show_icons = true            # false: titles only
strip_app_name = false       # true: "Café — Firefox" becomes "Café"
```

On first start `tynex daemon` writes this file with defaults. Keys
left out use their built-in defaults. A syntax error, an unknown key
or an invalid value stops the daemon with a message naming the
problem. See [docs/arch/config.md](docs/arch/config.md) for the rules.

Upgrading to 0.2.0: move keys into the new sections. `desktop`,
`font`, `font_size` and `width` go under `[main]`. `hotkey_action`
becomes `hotkeys.action`. `sort_by`, `title_bar`, `show_app` and
`strip_app_name` move under `[title_bar]` as `sort_by`, `enabled`,
`show_app_icon` and `strip_app_name`. `max_rows` moves to
`[output_bar]`. Delete `[appearance]`. Removed and renamed keys are
listed in [CHANGELOG.md](CHANGELOG.md).

Upgrading to 0.2.3: delete `title_bar.show_app_icon`. The app name
column is gone. `title_bar.show_icons` (default `true`) draws the
icon; `false` shows titles only.

## Architecture

Small resident daemon, pure core, and pluggable modules behind traits:
window sources (adapters), presenters (platforms), UI modules and
modes.

- [Overview](docs/arch/overview.md)
- [Tech stack](docs/arch/stack.md)
- [Decision records (ADR)](docs/adr/index.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the developer environment
setup and workflow, and [AGENTS.md](AGENTS.md) for coding style and
rules. Commits follow Conventional Commits and go through pull
requests.

## Donations

Donations are accepted via
[GitHub Sponsors](https://github.com/sponsors/CanBeSolved) and help keep
`tynex` alive.

## License

[MIT](LICENSE) © CanBeSolved
([@canbesolved](https://github.com/canbesolved))
