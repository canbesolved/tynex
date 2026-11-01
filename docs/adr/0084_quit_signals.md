---
status: "accepted"
date: "2026-11-01"
decision-makers: "CanBeSolved"
---
# Quit on signals

## Context and Problem Statement

`Ctrl+C` in the daemon's terminal, `kill`, or closing that terminal
ended the process at once. The KWin script stayed loaded and the
tabs socket stayed on disk until the next start cleaned it up. How
does the daemon stop on `SIGINT`, `SIGTERM` and `SIGHUP` with the same
cleanup as `tynex quit`?

## Decision Drivers

- The same shutdown path as `tynex quit`.
- No idle wakeups ([ADR-0036](0036_no_idle_wakeups.md)); the signal
  joins the single poll loop ([ADR-0026](0026_poll_loop.md)).
- Apps the daemon launches, and the hint command, keep normal signal
  handling.
- No heavy dependency ([ADR-0044](0044_binary_budget.md)).

## Considered Options

- A handler that writes to a pipe (self-pipe), with `libc`
- Blocked signals read from a `signalfd`
- The `signal-hook` crate

## Decision Outcome

Chosen option: "A handler that writes to a pipe (self-pipe), with
`libc`", because it leaves the signal mask alone, and `exec` resets
handlers, so every spawned child keeps default signal handling no
matter how it is started.

- `platform/linux/signals.rs` installs one `sigaction` handler for
  the three signals. It writes the signal number to a non-blocking,
  close-on-exec pipe and keeps `errno`.
- The poll loop reads the pipe, logs the signal at `info`, sends
  `Quit` to the switcher and leaves the loop like `tynex quit`. The
  daemon then restores the old actions, so a second `Ctrl+C` during
  shutdown ends the process at once, and stops the window source.
- `libc` is already in the tree through `dbus`, so it adds no code.

`signalfd` was tried first. It needs the signals blocked, and a test
showed that `std::process::Command` does not reset the mask in
children: launched apps would ignore `Ctrl+C` and `SIGTERM`.

### Consequences

- Good, because `Ctrl+C`, `kill` and a closed terminal unload the
  KWin script and remove the tabs socket.
- Good, because nothing runs while no signal arrives.
- Neutral, because the handler and the pipe need a little `unsafe`
  code, kept in one file.
- Bad, because `SIGKILL` still cannot be caught; the next start
  cleans up as before.

### Confirmation

A unit test sends `SIGTERM` to itself and reads it from the fd, and
checks that a spawned `sh` has none of the three signals blocked or
ignored. Manual: `Ctrl+C` on `tynex daemon` logs `SIGINT received,
quitting`, then `daemon stopped`, and leaves no `tabs.sock`.

## Pros and Cons of the Options

### A handler that writes to a pipe (self-pipe), with `libc`

- Good, because children are not affected.
- Good, because it is the classic async-signal-safe pattern.
- Bad, because the handler must use only async-signal-safe calls.

### Blocked signals read from a `signalfd`

- Good, because there is no handler at all.
- Bad, because the blocked mask is inherited by every child unless
  each spawn site resets it.

### The `signal-hook` crate

- Good, because it wraps the self-pipe pattern.
- Bad, because it is a new dependency for about 100 lines of code.

## More Information

Related: [ADR-0026](0026_poll_loop.md),
[ADR-0036](0036_no_idle_wakeups.md),
[ADR-0081](0081_tab_bridge.md) (the tabs socket).
