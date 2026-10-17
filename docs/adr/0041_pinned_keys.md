---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Window keys pinned until close

## Context and Problem Statement

[ADR-0015](0015_key_order.md) gives letters in most recently used
(MRU) order, so every switch moves the letters, and the user must read
the popup before each key press. How can a window keep its letter?

## Decision Drivers

- A window's letter is learned once and stays valid.
- Fastest keys first, and any keyboard layout works (as in ADR-0015).
- The previous window stays one key press away.
- No config option: one built-in behavior.

## Considered Options

- Pin each window's letter until the window closes
- Letters in MRU order (ADR-0015)
- Letters by app, shared by all windows of the app

## Decision Outcome

Chosen option: "Pin each window's letter until the window closes",
because letters stop moving, so a known window needs no look at the
popup.

- Letters still come from `keys`, home row first.
- On daemon start, the windows get letters in MRU order, as in
  ADR-0015.
- A new window gets the first free letter in `keys` order.
- A closed window frees its letter. A freed letter goes to the most
  recent window without one, if any.
- Windows beyond the number of letters have no letter until one is
  freed.
- Rows stay in MRU order; only the letters are fixed.
- Pins are kept in memory by window id; a daemon restart assigns them
  again.

### Consequences

- Good, because letters can be learned and pressed without looking.
- Bad, because the previous window is no longer always on `a`;
  `hotkey_action = "last_app"` (pressing the hotkey again) covers that
  switch instead.
- Bad, because letters are not in row order, so the popup shows them
  scattered.

### Confirmation

Unit tests in `src/core/keys.rs`: letters survive MRU changes, new
windows get the first free letter, freed letters go to the most recent
window without one.

## Pros and Cons of the Options

### Letters in MRU order (ADR-0015)

- Good, because the previous window is always on `a`.
- Bad, because letters move on every switch.

### Letters by app, shared by all windows of the app

- Good, because an app keeps its letter across restarts.
- Bad, because two windows of one app need a second key press or
  extra letters.

## More Information

Supersedes [ADR-0015](0015_key_order.md). Later, per-app keys in
`[apps.<app_id>]` (see [ROADMAP.md](../../ROADMAP.md)) reserve letters,
which pinning then skips. See [core.md](../arch/core.md) and
[hotkey.md](../arch/modes/hotkey.md).
