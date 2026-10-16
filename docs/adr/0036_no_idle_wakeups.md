---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBesolved"
---
# No idle wakeups

## Context and Problem Statement

[ADR-0034](0034_kwin_runner.md) removed the KDE `Poll` call and its
20 s `noop` reply. Three accepted records still describe that refresh:
[ADR-0009](0009_single_thread.md),
[ADR-0026](0026_poll_loop.md) and [ADR-0027](0027_dbus_crate.md). Which
statements in them still hold?

## Decision Drivers

- Accepted records are never rewritten.
- Readers must not follow outdated statements.

## Considered Options

- Amend the three records with this record
- Supersede the three records with full copies

## Decision Outcome

Chosen option: "Amend the three records with this record", because
their decisions (one thread, a `rustix` poll loop, the `dbus` crate)
stay valid; only statements about the 20 s refresh change.

- ADR-0009: `powertop` shows no wakeups from `tynex` while hidden,
  with no KDE exception.
- ADR-0026: the only poll deadline is key repeat while the popup is
  shown; an idle daemon has no timeout.
- ADR-0027: deferred replies are no longer a driver; the `dbus` crate
  is kept for its small tree and its fd in the poll loop.

Their status reads `accepted, amended by ADR-0036`.

### Consequences

- Good, because the idle budget is simply no wakeups.
- Neutral, because readers of the three records must also read this
  one.
