---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Keymaps with xkbcommon

## Context and Problem Statement

The Wayland presenter must turn key events into keysyms and text using
the user's keyboard layout, which the compositor sends as an XKB
keymap. Which library should handle it?

## Decision Drivers

- Correct keysyms and text for any layout.
- Fewest crates, no unused features.
- Maintained code paths.

## Considered Options

- `xkbcommon` with default features off
- Keyboard support of `smithay-client-toolkit`
- `xkbcommon-dl`
- `xkbcommon-rs` (pure Rust port)

## Decision Outcome

Chosen option: "`xkbcommon` with default features off", because it is
a thin binding to the system `libxkbcommon` that every Wayland
desktop uses. The keymap fd is mapped with `rustix` and loaded with
`Keymap::new_from_string`, so the `wayland` and `x11` features are not
needed.

### Consequences

- Good, because keymaps behave exactly as in every other Wayland app.
- Bad, because building needs `libxkbcommon` headers.
- Neutral, because `xkbcommon` releases slowly (last release
  2025-08-09); it is a thin binding, so the risk is low.

## Pros and Cons of the Options

### Keyboard support of `smithay-client-toolkit`

- Good, because it hands out keysyms and text ready to use.
- Bad, because it requires the whole toolkit, which `tynex` does not
  use ([ADR-0028](0028_wayland_client.md)).

### `xkbcommon-dl`

- Good, because it loads `libxkbcommon` at runtime, no link-time
  dependency.
- Bad, because it has had no release or commit since 2024-02.

### `xkbcommon-rs`

- Good, because it is pure Rust.
- Bad, because it is young and little used.

## More Information

See [linux.md](../arch/platforms/linux.md).
