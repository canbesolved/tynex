# Contributing

Thanks for helping with `tynex`. Read [AGENTS.md](AGENTS.md) for the
coding style and rules, and [docs/arch/overview.md](docs/arch/overview.md)
for the architecture.

## Developer environment

### Requirements

Build:

- Rust stable 1.89 or newer, plus nightly `rustfmt`.
- `gcc`, `pkg-config`, Wayland, xkbcommon and D-Bus development
  headers.
- Node.js and `npm` for the docs tools (`markdownlint-cli2`,
  `adr-log`).
- `cargo-deny` for dependency checks, `shellcheck` for scripts,
  `reuse` for license headers.

Runtime:

- Linux with a Wayland compositor that supports layer-shell (KDE
  Plasma 6 for 0.1.0).
- D-Bus session bus.
- fontconfig (`fc-match`) for `font = "auto"`.

### 1. System packages

Fedora:

```bash
sudo dnf install -y gcc pkgconf-pkg-config wayland-devel \
  libxkbcommon-devel dbus-devel nodejs-npm ShellCheck reuse
```

Debian / Ubuntu:

```bash
sudo apt install -y build-essential pkg-config libwayland-dev \
  libxkbcommon-dev libdbus-1-dev npm shellcheck reuse
```

Arch Linux:

```bash
sudo pacman -S --needed base-devel pkgconf wayland libxkbcommon dbus \
  npm shellcheck reuse
```

### 2. Rust toolchain

Fedora ships `rustup` as a package:

```bash
sudo dnf install -y rustup
rustup-init -y --profile minimal -c rustfmt,clippy
```

Other distributions:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --profile minimal -c rustfmt,clippy
```

Then add the nightly formatter (used only for `cargo +nightly fmt`):

```bash
source "$HOME/.cargo/env"
rustup toolchain install nightly --profile minimal -c rustfmt
```

### 3. Check tools

```bash
sudo npm install -g markdownlint-cli2 adr-log
cargo install --locked cargo-deny
```

### 4. Verify the setup

```bash
git clone git@github.com:canbesolved/tynex.git
cd tynex
cargo build
cargo +nightly fmt --check
cargo clippy --all-targets -- -D warnings
cargo deny check
markdownlint-cli2 "**/*.md"
shellcheck scripts/*.sh
reuse lint
```

All commands must finish without errors.

## Workflow

1. Create a branch `<type>/<short_description>`, for example
   `feat/kde_adapter`.
2. Make one scoped change. Update the module page in `docs/arch/`, add
   an ADR for significant decisions (`scripts/new_adr.sh`), and add an
   entry under `[Unreleased]` in [CHANGELOG.md](CHANGELOG.md). A
   renamed or removed config key is listed as **Breaking:**.
3. Run the checks:

   ```bash
   cargo +nightly fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo deny check
   markdownlint-cli2 "**/*.md"
   shellcheck scripts/*.sh
   reuse lint
   scripts/adr_log.sh   # When ADRs changed
   ```

4. Commit with [Conventional Commits](https://www.conventionalcommits.org)
   and open a pull request to `main`. The maintainer reviews and merges.

## Footprint checks

`tynex` has a strict budget (see
[docs/arch/overview.md](docs/arch/overview.md)). Before a release, or
for changes on the hot path, measure a release build:

```bash
cargo build --release
ls -lh target/release/tynex           # Binary size, target < 10 MB
ps -o rss=,cmd= -C tynex              # Resident memory, target < 10 MB
sudo powertop                         # Idle wakeups, target ~0
```

Mention the numbers in the pull request when they change.
