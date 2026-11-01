---
status: "accepted"
date: "2026-11-01"
decision-makers: "CanBeSolved"
---
# Extensions folder

## Context and Problem Statement

Some features ship code that the user installs into another
program: a browser extension now, and later add-ons for other
hosts. Where does that code live?

## Decision Drivers

- The Cargo crate stays the daemon, the modules and the adapters.
- A new host (Firefox, another browser) has an obvious place.
- Code the daemon embeds and loads itself is not an add-on.

## Considered Options

- `extensions/<feature>/<host>/` at the repo root
- Next to the adapter, under `src/`
- One folder per host, with no feature level

## Decision Outcome

Chosen option: "`extensions/<feature>/<host>/` at the repo root",
because the user installs these files into another program, and
the path names both the feature and the host.

`extensions/` is not part of the Cargo crate. Its sources still
carry SPDX headers, and its tests and scripts join the Tooling
checks.

The KWin script stays at `src/adapters/kde/tynex.js`. The binary
embeds it and loads it; the user never installs it.

### Consequences

- Good, because Firefox can be added beside the Chromium extension
  without moving it.
- Good, because `src/` stays code that `tynex` compiles.
- Neutral, because the folder is empty until a feature needs an
  add-on.

### Confirmation

Review: new add-on code lands under `extensions/`. The Chromium
extension tests run in CI.

## Pros and Cons of the Options

### `extensions/<feature>/<host>/` at the repo root

- Good, because feature and host are both visible.
- Bad, because a tiny add-on still gets two folders.

### Next to the adapter, under `src/`

- Good, because it sits beside the bridge that talks to it.
- Bad, because `src/` then holds files Cargo does not build.

### One folder per host, with no feature level

- Good, because the first extension has a shorter path.
- Bad, because a second add-on for the same host has no place
  that says which feature it belongs to.

## More Information

The browser extension is
[ADR-0081](0081_tab_bridge.md).
