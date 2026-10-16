---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBeSolved"
---
# Own stderr logger on the log facade

## Context and Problem Statement

`tynex` must print errors and warnings, and debug and trace messages
that users and developers can turn on without a rebuild. Ready-made
loggers bring filter syntax, colors and timestamps that a small daemon
does not need. How should logging work?

## Decision Drivers

- Errors and warnings always visible; debug output on demand.
- Fewest crates.
- Standard macros (`error!` to `trace!`), so library logs show up too.

## Considered Options

- `log` + own stderr logger
- `log` + `env_logger` with default features off
- Own macros without `log`
- `tracing`

## Decision Outcome

Chosen option: "`log` + own stderr logger", because the `log` facade
has no dependencies and a logger for it is about 50 lines in
`src/logger.rs`, with no extra crate.

- Level from `TYNEX_LOG` (`error`, `warn`, `info`, `debug`, `trace`),
  default `warn`.
- Records from `tynex` are shown at the chosen level; records from
  other crates only at `warn` and above, or at every level with
  `trace`.
- Output goes to stderr, one line per record:
  `tynex: warn: message`. At `debug` and `trace` the module is added:
  `tynex: debug: adapters::kde: message`.
- No timestamps or colors; under a systemd user unit the journal adds
  time and priority.

### Consequences

- Good, because `env_logger` and its 2 crates are not needed.
- Good, because the output format is ours and stays stable.
- Bad, because there are no per-module filters like `RUST_LOG`; one
  level for `tynex` is enough for a small program.

### Confirmation

Unit tests in `src/logger.rs` cover level parsing, the default level
and the line format.

## Pros and Cons of the Options

### `log` + `env_logger` with default features off

- Good, because `RUST_LOG` syntax with module filters is familiar.
- Bad, because it adds 2 crates for filters `tynex` does not need.

### Own macros without `log`

- Good, because it needs no crate at all.
- Bad, because log records from libraries are lost, and the macros are
  non-standard.

### `tracing`

- Good, because it supports spans and structured fields.
- Bad, because it is heavier than `tynex` needs.

## More Information

See [logging.md](../arch/logging.md).
