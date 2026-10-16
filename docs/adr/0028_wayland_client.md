---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBesolved"
---
# Wayland via wayland-client

## Context and Problem Statement

The presenter is a single layer-shell popup
([ADR-0007](0007_layer_shell.md)). Which Wayland client library should
`tynex` use?

## Decision Drivers

- Fewest crates, no unused features.
- Same maintainers and maturity as the rest of the Rust Wayland
  ecosystem.

## Considered Options

- `wayland-client` + `wayland-protocols-wlr`
- `smithay-client-toolkit` (sctk)
- `wayrs-client`

## Decision Outcome

Chosen option: "`wayland-client` + `wayland-protocols-wlr`", because
they are the base layers sctk itself is built on, maintained by
Smithay, and need few crates. The missing parts (globals, shared
memory buffers, key repeat) are a few hundred lines in
`src/platform/linux/wayland/`. Keymaps use `xkbcommon`
([ADR-0023](0023_xkbcommon.md)).

### Consequences

- Good, because sctk's unused parts are not pulled in.
- Good, because buffer handling fits the single poll loop
  ([ADR-0026](0026_poll_loop.md)).
- Bad, because `tynex` owns the buffer, keymap and key repeat code.

## Pros and Cons of the Options

### `smithay-client-toolkit`

- Good, because it handles buffers and keyboard for us.
- Bad, because it pulls 40 crates, including cursor themes, window
  decorations and extra protocols that a single popup never uses.

### `wayrs-client`

- Good, because it is the leanest option (11 crates).
- Bad, because its user base is small and the last release was in
  2025-06.

## More Information

See [linux.md](../arch/platforms/linux.md).
