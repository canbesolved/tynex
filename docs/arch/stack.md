# Tech stack

Every dependency must earn its place: small, maintained, no hidden
runtime cost. Adding a crate needs a line in this file and an ADR.
`cargo deny check` enforces maintenance, advisories and licenses.

## Language

- Rust, edition 2024, MSRV 1.89, stable toolchain.
- Why: native speed and size like C, memory safety, mature Wayland and
  D-Bus crates, and first-class cross-compilation for macOS and
  Windows.

## Runtime crates

| Crate                   | Purpose                | OS    | Notes             |
| ----------------------- | ---------------------- | ----- | ----------------- |
| `wayland-client`        | Wayland client         | linux |                   |
| `wayland-protocols-wlr` | Layer-shell protocol   | linux | `client`          |
| `xkbcommon`             | Keymaps, keysyms, text | linux | defaults off      |
| `rustix`                | poll, memfd, mmap      | linux | `event fs mm`     |
| `dbus`                  | D-Bus via libdbus      | linux | defaults off      |
| `tiny-skia`             | 2D drawing, PNG        | all   | `png-format`      |
| `resvg`                 | SVG icons              | all   | defaults off      |
| `ab_glyph`              | Glyph rasterizing      | all   |                   |
| `frizbee`               | Fuzzy matching         | all   |                   |
| `serde`                 | Config structs         | all   | `derive`          |
| `toml`                  | Config parser          | all   | `parse serde std` |
| `lexopt`                | CLI arguments          | all   |                   |
| `log`                   | Logging facade         | all   |                   |

The full tree is about 70 crates, including build-time proc macros
(`serde_derive`, `wayland-scanner`). A clean release build of the
dependencies takes about 15 s.

The standard library covers paths (`std::env::home_dir`, XDG
variables), and the logger for `log` is written in the project. No
extra threads and no async runtime are used.

## Decisions

Why each part was chosen, with rejected options, is recorded in ADRs:

- Language: [ADR-0002](../adr/0002_rust.md).
- Popup without UI toolkit: [ADR-0007](../adr/0007_layer_shell.md).
- KWin payload without JSON: [ADR-0011](../adr/0011_kwin_payload.md).
- TOML config: [ADR-0013](../adr/0013_toml_config.md),
  defaults and validation: [ADR-0031](../adr/0031_strict_config.md).
- CLI parser: [ADR-0016](../adr/0016_lexopt.md).
- Font lookup: [ADR-0017](../adr/0017_fc_match.md).
- Tooling: [ADR-0018](../adr/0018_lint_tooling.md),
  [ADR-0019](../adr/0019_rust_style.md),
  [ADR-0020](../adr/0020_markdown_style.md),
  [ADR-0024](../adr/0024_dependency_checks.md).
- Fuzzy matching: [ADR-0014](../adr/0014_fuzzy_matcher.md),
  [ADR-0022](../adr/0022_frizbee.md).
- Rendering, text and icons: [ADR-0008](../adr/0008_cpu_render.md),
  [ADR-0025](../adr/0025_rendering.md).
- Threads and event loop: [ADR-0009](../adr/0009_single_thread.md),
  [ADR-0026](../adr/0026_poll_loop.md).
- D-Bus: [ADR-0012](../adr/0012_dbus_ipc.md),
  [ADR-0027](../adr/0027_dbus_crate.md),
  [ADR-0035](../adr/0035_shared_dbus.md).
- Wayland and keyboard: [ADR-0028](../adr/0028_wayland_client.md),
  [ADR-0023](../adr/0023_xkbcommon.md).
- Paths without `dirs`: [ADR-0029](../adr/0029_std_paths.md).
- Logging: [ADR-0030](../adr/0030_logging.md),
  crash output: [ADR-0032](../adr/0032_crash_output.md).

## Build profile

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "debuginfo"     # Keep symbol names for backtraces
```

## Tooling

| Tool                | Use                                     |
| ------------------- | --------------------------------------- |
| `rustfmt` (nightly) | Rust Style Guide, comment wrapping      |
| `clippy`            | Linting, `all` + `pedantic` as warnings |
| `cargo test`        | Unit tests, mainly `core` and `config`  |
| `cargo-deny`        | Advisories, maintenance, licenses       |
| `markdownlint-cli2` | Markdown lint, 80 columns               |
| `shellcheck`        | Lint shell scripts in `scripts/`        |
| `adr-log`           | ADR index (`docs/adr/index.md`)         |
| `.editorconfig`     | Editor defaults                         |
