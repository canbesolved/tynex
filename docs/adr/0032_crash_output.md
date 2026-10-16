---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBesolved"
---
# Crash output with backtrace

## Context and Problem Statement

When the daemon crashes, the output must show where. Release builds
abort on panic and are stripped, so the default panic output has no
backtrace, and a backtrace would show only `<unknown>` frames. How
should `tynex` print crashes?

## Decision Drivers

- A readable backtrace for every panic, without a debugger.
- Same output as other messages, no extra files.
- No cost while running: nothing is captured until a crash.
- Binary size target under 3 MB.

## Considered Options

- Panic hook with a backtrace, symbol names kept in release builds
- Default panic output only
- Crash reporting crate (`sentry`, `human-panic`)

## Decision Outcome

Chosen option: "Panic hook with a backtrace, symbol names kept in
release builds", because the standard library already captures
backtraces, and keeping only symbol names costs about 75 KB.

- `main.rs` installs the hook with `std::panic::set_hook` at startup.
- On panic the hook captures `std::backtrace::Backtrace` and logs the
  version, panic message, source location and backtrace at `error`
  level, through the same output as other messages
  ([ADR-0030](0030_logging.md)). No separate report files are
  written.
- The release profile uses `strip = "debuginfo"` instead of
  `strip = true`: function names stay, debug info is removed.
- Crashes in C libraries (`libdbus`, `libxkbcommon`) raise a signal
  that no panic hook sees; `systemd-coredump` records them
  (`coredumpctl info tynex`).

### Consequences

- Good, because a crash shows where it happened, in the usual output.
- Good, because the hook adds about 10 KB and runs only on a crash.
- Neutral, because symbol names add about 75 KB to the binary
  (measured on a stub: 301 KB with `strip = true`, 376 KB with
  `strip = "debuginfo"`).

### Confirmation

A test build with a forced panic prints a backtrace that names
`tynex` functions.

## Pros and Cons of the Options

### Default panic output only

- Good, because it needs no code.
- Bad, because there is no backtrace in release builds.

### Crash reporting crate

- Good, because reports are formatted and may be uploaded.
- Bad, because it adds crates and features `tynex` does not need.

## More Information

See [logging.md](../arch/logging.md).
