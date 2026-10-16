---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Isolate OS code for portability

## Context and Problem Statement

`tynex` starts on Linux but must later run on macOS and Windows. OS
types leaking into shared code would make that a rewrite.

## Decision Drivers

- Port by adding backends, not by editing shared code.
- Shared renderer, modes, search and config.

## Considered Options

- Three seams (`WindowSource`, `Presenter`, `IpcServer`) with OS code
  only in `src/platform/` and `src/adapters/`
- Cross-platform UI toolkit
- Separate code base per OS

## Decision Outcome

Chosen option: "Three seams with OS code only in `src/platform/` and
`src/adapters/`", because everything else stays pure Rust and compiles
everywhere, and no toolkit is needed (see
[ADR-0007](0007_layer_shell.md)).

### Consequences

- Good, because the renderer outputs a pixel buffer that any presenter
  can blit.
- Good, because each backend maps native keys to a portable `Key`.
- Bad, because each OS needs its own presenter and adapter.

### Confirmation

Later CI runs `cargo check` for Linux, macOS and Windows targets.
Linux-only crates sit under `cfg(target_os = "linux")`.

## More Information

See [portability.md](../arch/platforms/portability.md).
