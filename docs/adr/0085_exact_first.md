---
status: "accepted"
date: "2026-11-01"
decision-makers: "CanBeSolved"
---
# Exact matches first

## Context and Problem Statement

The output list is ordered by source: matching windows, then browser
tabs, then launch rows ([ADR-0078](0078_windows_before_apps.md)).
Fuzzy matching also hits scattered letters, so `miro` matched a
Chromium window through `chro`**`mi`**`um-b`**`ro`**`wser` and listed
it above the tab titled `Overall - Miro`. Which row comes first when
one contains the query as typed and another only matches fuzzily?

## Decision Drivers

- What the user typed, as typed, is the strongest signal.
- Keep the existing order where it works: windows before tabs before
  launch rows, leading matches first
  ([ADR-0079](0079_leading_match.md)), the new-window row under its
  app ([ADR-0080](0080_new_window_place.md)).
- No new config key, no score tuning.

## Considered Options

- Two bands: exact rows, then fuzzy rows, each in the current order
- Three bands: leading, then contained, then fuzzy
- Raise the score of contiguous matches inside the fuzzy ranking

## Decision Outcome

Chosen option: "Two bands: exact rows, then fuzzy rows, each in the
current order", because it fixes the case above with one rule and
leaves every ordering inside a band as it was.

- A row is exact when the query appears in it as one run of letters,
  with the search's case and accent rules: the title for every row,
  plus the app name or short app id for windows and launch rows. A
  tab title includes its host.
- A new-window row is exact when its app has an exact window, so it
  stays under those windows.
- The bands are a stable sort of the final output list
  (`modules/input/exact.rs`), after windows, tabs and launch rows
  are joined.

This refines ADR-0078: open windows still come before launch rows
within a band, but an app whose name contains the query can come
before a window that only matches fuzzily.

### Consequences

- Good, because a row that contains the query is never below one
  that does not.
- Good, because leading matches, grouping and the new-window row
  behave as before inside each band.
- Neutral, because a typo-tolerant match (five or more characters)
  is fuzzy, so it lands in the second band.
- Bad, because an idle app can now come before an open window that
  matches only fuzzily.

### Confirmation

`exact_tests.rs` covers an exact tab above a fuzzy window, band order
with the new-window row, the app name, and the `miro` case through
`Input::refresh`. The launcher tests for ADR-0078, 0079 and 0080
still pass.

## Pros and Cons of the Options

### Two bands: exact rows, then fuzzy rows, each in the current order

- Good, because it is one stable sort on top of the current order.
- Bad, because a leading tab stays under a contained window.

### Three bands: leading, then contained, then fuzzy

- Good, because a leading match of any source comes first.
- Bad, because a leading idle app then beats an open window that
  contains the query, which goes against ADR-0078 more than needed.

### Raise the score of contiguous matches inside the fuzzy ranking

- Good, because there is no extra pass.
- Bad, because scores from windows, tabs and apps are ranked
  separately and never compared, so the source order still wins.

## More Information

Refines [ADR-0078](0078_windows_before_apps.md). Related:
[ADR-0079](0079_leading_match.md),
[ADR-0080](0080_new_window_place.md),
[ADR-0081](0081_tab_bridge.md).
