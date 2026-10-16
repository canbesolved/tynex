---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Target KDE Plasma 6 on Wayland first

## Context and Problem Statement

`tynex` should work with every desktop through adapters, but the first
release needs one concrete target to prove the design.

## Decision Drivers

- The maintainer uses Plasma 6.7 on Wayland daily.
- Wayland is the default session on current distributions.
- A small first release ships sooner.

## Considered Options

- KDE Plasma 6, Wayland only
- KDE Plasma 6, Wayland and X11

## Decision Outcome

Chosen option: "KDE Plasma 6, Wayland only", because it covers the
maintainer's setup and the future default, and keeps the first release small.

### Consequences

- Good, because only one adapter and one presenter are needed.
- Bad, because X11 users wait for a later release.

### Confirmation

The adapter registry and presenter interface must allow adding X11
without touching the core (see [ADR-0005](0005_trait_modules.md)).
