---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Pure core with trait-based modules

## Context and Problem Statement

Window listing, display, layouts and switching modes must be replaced
or extended without breaking other parts. How are parts connected?

## Decision Drivers

- Add a desktop, layout or mode by adding one module.
- Test the switcher logic without a display.
- Keep coupling explicit and small.
- Only the modules the system needs use RAM and CPU.

## Considered Options

- Pure core with traits and compile-time registries
- Dynamic plugins loaded at runtime
- One crate without internal boundaries

## Decision Outcome

Chosen option: "Pure core with traits and compile-time registries",
because it gives clear module boundaries at zero runtime cost.

The core owns `WindowSource`, `Presenter`, `Layout`, `Mode` and the
`Event` / `Action` protocol. Concrete modules depend on the core, never
on each other. Only `main.rs` wires them together.

At startup the daemon creates exactly one adapter and one presenter:

- **Presenter:** the OS is chosen at compile time (`cfg(target_os)`),
  so other systems' code is not in the binary. Within an OS (later
  Wayland or X11 on Linux), the session decides at runtime.
- **Adapter:** chosen at runtime from the `desktop` key, or from
  `XDG_CURRENT_DESKTOP` when it is `auto`.
- **Dispatch:** each seam is an enum with one variant per module, for
  example `enum Source { Kde(KdeSource), Mock(MockSource) }`. Only the
  selected variant is constructed, and calls stay static.

Unselected modules are never initialized: no connections, scripts,
buffers or file descriptors, and nothing in the poll loop. Their code
is in the binary, but the kernel loads code pages on demand, so pages
never executed add nothing to RAM. A module that needs its own system
library (for example `libxcb` for X11) keeps it behind a cargo
feature, so builds without it do not link or load the library.

### Consequences

- Good, because the core is unit-testable with mock modules.
- Good, because static dispatch keeps it fast.
- Good, because RAM and CPU use depend only on the selected adapter
  and presenter, not on how many are built in.
- Neutral, because the binary file grows with each module; disk size,
  not RAM.
- Bad, because third-party plugins need a rebuild.

### Confirmation

Code review: `src/core/` imports nothing else from the crate and does
no I/O. Unit tests check that selecting one adapter constructs no
other.

## Pros and Cons of the Options

### Dynamic plugins loaded at runtime

- Good, because unused modules are not even in memory maps.
- Bad, because Rust has no stable ABI, so plugins must match the exact
  compiler and `tynex` version, or go through a C interface.
- Bad, because it adds `dlopen` and plugin discovery, while on-demand
  paging already keeps unused built-in code out of RAM.

### One crate without internal boundaries

- Good, because it is the least code at first.
- Bad, because every new desktop touches the switcher logic.

## More Information

See [core.md](../arch/core.md).
