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

Press a hotkey, see your windows in a centered list, press one letter,
and you are there. No mouse, no scrolling, no waiting.

> Status: 0.1.2, for KDE Plasma 6 on Wayland. App icons follow in
> 0.1.3 (see the [roadmap](ROADMAP.md)).

## Features

- Centered popup with a vertical list of open windows.
- Each entry shows the window title, optionally with the app name
  (`"{app} : {title}"`); app icons come in 0.1.3.
- Home row keys (`a s d f h j k l`) jump straight to a window, and a
  window keeps its key until it closes.
- Fuzzy search mode: open the switcher and start typing; accents and
  small typos are forgiven.
- Low footprint: idle at 0 % CPU, under 10 MB of RAM, no GPU usage.
- Modular core: window sources, layouts and modes are plug-in modules.
- Configuration in a single TOML file.

## Supported environments

| Environment      | Session | Status  |
| ---------------- | ------- | ------- |
| KDE Plasma 6     | Wayland | 0.1.2   |
| Other DEs / WMs  | Any     | Planned |
| macOS, Windows   | -       | Planned |

Planned environments and features are listed in the
[roadmap](ROADMAP.md).

## Download

Each [release](https://github.com/canbesolved/tynex/releases) has
prebuilt binaries for Linux x86_64 and arm64 (glibc 2.35 or newer),
for example:

```bash
v=0.1.2 t=x86_64-unknown-linux-gnu     # or aarch64-unknown-linux-gnu
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
  Plasma 6 for 0.1.0), a D-Bus session bus and fontconfig.
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
tynex daemon             # Start the background service
tynex show               # Open the switcher (mode from the config)
tynex show --mode search # Open the switcher in search mode
tynex hide               # Close the switcher
tynex quit               # Stop the background service
```

On KDE the default hotkey is `Meta+Tab`. Change it in System Settings,
Shortcuts, `tynex`. On other desktops bind `tynex show` to any key.
Pressing the hotkey again switches to the previous window, or closes
the popup with `hotkey_action = "hide"`. The popup also closes when it
loses keyboard focus.

In the popup:

- hotkey mode: press the letter next to a window;
- search mode: type to filter, `Enter` to switch;
- `Tab` toggles the modes, arrows move, `Esc` closes.

## Configuration

The config file is `~/.config/tynex/tynex.toml`:

```toml
desktop = "auto"        # auto | kde | mock
mode = "hotkey"         # hotkey | search
hotkey_action = "last"  # hide | last
layout = "vertical_list"
keys = "asdfhjklgqwertyuiop"

[appearance]
font = "auto"
font_size = 14
width = 480
title_format = "{title}" # Row text, for example "{app} : {title}"
strip_app_name = true    # "Café — Firefox" becomes "Café"
```

On first start `tynex daemon` writes this file with defaults. Keys
left out use their built-in defaults. A syntax error, an unknown key
or an invalid value stops the daemon with a message naming the
problem. See [docs/arch/config.md](docs/arch/config.md) for the rules.

## Architecture

Small resident daemon, pure core, and pluggable modules behind traits:
window sources (adapters), presenters (platforms), layouts and modes.

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
