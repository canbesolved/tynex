<!-- markdownlint-disable MD033 MD041 -->
<p align="center">
  <img src="assets/icon.svg" alt="tynex icon" width="128">
</p>

<h1 align="center">tynex</h1>

<p align="center">
  <b>T</b>hank <b>Y</b>ou, <b>NEX</b>t: efficient task switcher for
  Linux, macOS and Windows.
</p>
<!-- markdownlint-enable MD033 MD041 -->

Press a hotkey, see your windows in a centered list, press one letter,
and you are there. No mouse, no scrolling, no waiting.

> Status: early development toward 0.1.0 (see the [roadmap](ROADMAP.md)).
> Not ready for daily use yet.

## Features

- Centered popup with a vertical list of open windows.
- Each entry shows the app icon and the window title.
- Home row keys (`a s d f h j k l`) jump straight to a window.
- Fuzzy search mode: open the switcher and start typing.
- Low footprint: idle at 0 % CPU, under 10 MB of RAM, no GPU usage.
- Modular core: window sources, layouts and modes are plug-in modules.
- Configuration in a single TOML file.

## Supported environments

| Environment      | Session | Status  |
| ---------------- | ------- | ------- |
| KDE Plasma 6     | Wayland | 0.1.0   |
| Other DEs / WMs  | Any     | Planned |
| macOS, Windows   | -       | Planned |

Planned environments and features are listed in the
[roadmap](ROADMAP.md).

## Build from source

There are no prebuilt packages yet, and the repository is private for
now.

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
tynex show               # Open the switcher (hotkey mode)
tynex show --mode search # Open the switcher in search mode
tynex quit               # Stop the background service
```

On KDE the default hotkey is `Meta+Tab`. Change it in System Settings,
Shortcuts, `tynex`. On other desktops bind `tynex show` to any key.

In the popup:

- hotkey mode: press the letter next to a window;
- search mode: type to filter, `Enter` to switch;
- `Tab` toggles the modes, arrows move, `Esc` closes.

## Configuration

The config file is `~/.config/tynex/tynex.toml`:

```toml
desktop = "auto"       # auto | kde | mock
mode = "hotkey"        # hotkey | search
layout = "vertical_list"
keys = "asdfhjklgqwertyuiop"

[appearance]
font = "auto"
font_size = 14
width = 480
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
