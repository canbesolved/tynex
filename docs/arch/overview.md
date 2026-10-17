# Architecture overview

Entry point to the architecture docs. Each module has its own page.
The reasons behind each choice are in the
[decision records](../adr/index.md).

## Contents

- [stack.md](stack.md): tech stack and tooling.
- [core.md](core.md): core types, traits and state machine.
- [config.md](config.md): `tynex.toml` format and loading.
- [logging.md](logging.md): log levels and output format.
- [ipc.md](ipc.md): CLI to daemon communication.
- [adapters/mock.md](adapters/mock.md): mock adapter for tests.
- [adapters/kde.md](adapters/kde.md): KDE Plasma 6 adapter.
- [platforms/linux.md](platforms/linux.md): Wayland presenter, D-Bus.
- [platforms/portability.md](platforms/portability.md): macOS, Windows.
- [ui/render.md](ui/render.md): canvas, text and icons.
- [ui/layouts/vertical_list.md](ui/layouts/vertical_list.md):
  default layout.
- [modes/hotkey.md](modes/hotkey.md): letter switching.
- [modes/search.md](modes/search.md): fuzzy search.

## Goals

- Show the switcher in under one frame (16 ms) after the hotkey.
- Zero CPU use while idle, RSS under 10 MB, release binary under 10 MB
  ([ADR-0044](../adr/0044_binary_budget.md)).
- Every part (window source, UI, layout, mode, IPC) is a replaceable
  module behind a small trait.
- Linux first (KDE Plasma 6, Wayland), portable to macOS and Windows.

## Process model

- One binary, `tynex`, is both daemon and client; the command picks the
  role.
- One resident daemon, `tynex daemon`. While idle it is blocked on the
  event loop and does no work.
- The CLI, `tynex show [--mode search]`, sends one IPC call to the
  daemon, so any desktop can bind it to a hotkey.
- One thread: a single `poll` loop over the Wayland and D-Bus file
  descriptors drives the backend and the core. No async runtime, no
  locks, no channels.

## Module layout

```text
src/
  main.rs              Entry: logger, command dispatch
  lib.rs               Module tree, shared by the binary and tests
  cli.rs               Command line parsing with lexopt
  daemon.rs            tynex daemon: config loading, wiring
  core/                Pure logic, traits, state machine
  config/              tynex.toml: load, defaults, validation
  logger.rs            stderr logger, level from TYNEX_LOG
  ipc/                 Request and IpcError; transport per platform
  adapters/
    mod.rs             Registry, auto-detect by XDG_CURRENT_DESKTOP
    mock/              In-memory windows for tests and manual runs
    kde/               KWin script, runner activation
  platform/            Per-OS backends, gated by cfg(target_os)
    linux/             Wayland, D-Bus, event loop, fonts, paths,
                       desktop files, icon themes
    macos/             Later
    windows/           Later
  ui/                  Portable: no OS types
    render/            Canvas, text, icons
    layouts/           vertical_list (default)
    painter.rs         Text, theme and layout: size and draw the popup
  modes/               hotkey, search
  search/              Fuzzy matcher, accent folding
```

Dependency direction: every module may depend on `core`; `core`
depends on nothing else in the crate. `main.rs` is the only place that
wires concrete modules together.

## Startup

`tynex daemon` creates exactly one adapter and one presenter (see
[ADR-0005](../adr/0005_trait_modules.md)):

1. The OS backend is fixed at compile time by `cfg(target_os)`.
2. The presenter for the session is chosen (Wayland for 0.1.0).
3. The adapter is chosen from the `desktop` key, or detected from
   `XDG_CURRENT_DESKTOP` when it is `auto`.

Each seam is an enum with one variant per module, and only the chosen
variant is constructed. Other modules are never initialized and add
no file descriptors, threads or allocations. Their code stays on disk
until executed, so it adds nothing to RAM.

## Data flow

```text
KWin script --Update------> KdeSource --Event::Windows--> core cache
hotkey/CLI  --Show--------> core --View--> Presenter (map surface)
key press   --Event::Key--> Mode --Action::Activate(id)--> KdeSource
KdeSource   --Run(id)-----> KWin /WindowsRunner --> window activated
```

## Footprint budget

- Idle: 0 % CPU, no wakeups, RSS under 10 MB.
- Small dependency tree: about 80 crates in total (see
  [stack.md](stack.md)).
- Show to first frame under 16 ms, key to activation under 5 ms.
- No GPU context, no UI toolkit, no async runtime, no extra threads.
