---
status: "accepted"
date: "2026-11-05"
decision-makers: "CanBeSolved"
---
# Quick switch command

## Context and Problem Statement

`hotkeys.action = "last"` goes back to the previous window or tab
on a second hotkey press, so the popup opens first (ADR-0096).
`tynex qs` (RM-0005) goes back without it. While hidden there are
no tab lists, which `Places::back` uses to know that a tab is still
open, and no popup state, which the pending activation copies the
list from. How does a hidden quick switch reach a tab?

## Decision Drivers

- No popup, no surface, no extra wait.
- Same result as the second press with `last` where possible.
- No change to the KWin script or the browser extension.

## Considered Options

- Trust the history and ask the browser
- Ask for the tab lists first, then switch
- Keep the last popup's tab lists while hidden

## Decision Outcome

Chosen option: "Trust the history and ask the browser", because it
needs no wait and no stale data, and the browser already reports a
tab that is gone.

- New CLI command `tynex qs`, D-Bus method `QuickSwitch()`, core
  event `Event::QuickSwitch`.
- Popup open: the same switch back as a second press with `last`,
  whatever `hotkeys.action` says.
- Popup hidden: the previous place of `Places`. A window that is
  still there is activated; a tab gets `Action::ActivateTab`. With
  no previous place, the previous window; with none, nothing.
- A hidden tab switch notes a pending activation with the window
  that showed the tab (`Mode::on_quick_switch`). On success that
  window is raised, else the browser's window picked as usual. A
  closed tab fails in the browser, and nothing is raised.
- A window switch counts in the usage stats. A tab does not: its
  URL is known only from a list.

### Consequences

- Good, because a hidden switch is one D-Bus call and one activation.
- Good, because the second press and `qs` share `Switcher::back`.
- Bad, because a tab closed since the last list makes `qs` do
  nothing, where the popup would have fallen back to a window.
- Bad, because tab switches through `qs` are not counted.

### Confirmation

Tests in `src/core/switcher_back_tests.rs` and
`src/modules/browser_tabs/hidden_tests.rs`.

## Pros and Cons of the Options

### Ask for the tab lists first, then switch

- Good, because a closed tab is known before the switch.
- Bad, because every `qs` waits for each browser, up to
  `browser_tabs.wait`, and wakes every connected browser.

### Keep the last popup's tab lists while hidden

- Good, because tab uses could be counted.
- Bad, because the lists are stale and stay in memory while idle.
