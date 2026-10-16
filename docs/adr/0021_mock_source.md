---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Mock window source for testing

## Context and Problem Statement

The core, config, modes and popup need a window list to work with. The
KDE adapter needs a running Plasma session and is built later. How do
we test and run everything else before and without it?

## Decision Drivers

- Test core and config before the KDE adapter exists.
- Deterministic tests with no desktop, D-Bus or display.
- Manual popup runs on any compositor and later on any OS.

## Considered Options

- Mock adapter in `src/adapters/mock/`, selectable at runtime
- Test doubles only inside `#[cfg(test)]` modules

## Decision Outcome

Chosen option: "Mock adapter in `src/adapters/mock/`, selectable at
runtime", because the same module serves unit tests, integration tests
and manual runs (`desktop = "mock"`), and it proves the
`WindowSource` trait with a second implementation from day one.

### Consequences

- Good, because the build order becomes core and config, mock, popup,
  then KDE.
- Good, because macOS and Windows bring-up can start with the mock.
- Bad, because the mock ships in the binary; it is small, pure Rust
  and never chosen by `desktop = "auto"`.

### Confirmation

Core and config tests use `MockSource`; CI runs them without a display.

## More Information

See [mock.md](../arch/adapters/mock.md) and
[ADR-0005](0005_trait_modules.md).
