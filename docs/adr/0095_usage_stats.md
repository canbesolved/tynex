---
status: "accepted"
date: "2026-11-05"
decision-makers: "CanBeSolved"
---
# Usage stats

## Context and Problem Statement

Output bar results are in a fixed order: exact matches, then the
rest, each band ordered windows, tabs, apps
([ADR-0085](0085_exact_first.md)). RM-0009 asked for a config to
change that order and RM-0023 for ranking by use. Which order puts
what the user wants first, with the least setup?

## Decision Drivers

- No new config key: the order learns from use.
- Exact matches stay above fuzzy ones.
- No idle work: the file is written only after a use.
- Scores stay small, bounded and private.

## Considered Options

- Usage scores inside each band, no config
- `output.priority`, a configured order of kinds
- Ranking by last use first, then by count

## Decision Outcome

Chosen option: "Usage scores inside each band, no config", because
the most used windows, tabs and apps come first without any setup,
and exact matches still lead.

- A use is a window activated, an app launched or a tab opened from
  the popup, also the hotkey pressed again. Focus changes outside
  tynex do not count.
- A window and its app share one score, by app id; a tab is counted
  by browser name and URL.
- A score is 0 to 100. A use adds 1. Going past 100 halves every
  score, so old uses fade and the order stays; a score that reaches
  0 is dropped.
- Both bands, exact and fuzzy, are sorted by score, highest first.
  The sort is stable, so equal scores, and all rows before any use,
  keep the order of ADR-0085.
- Scores live in `~/.local/share/tynex/stats.tsv`, one
  `score<TAB>app<TAB>id` or `score<TAB>tab<TAB>browser<TAB>url` line
  each, mode `0600`. It is read on start and written, atomically,
  after the popup closes with a change.
- Items that are gone are dropped: an app with no window and no
  desktop file when the popup opens (kept when the catalog was not
  scanned); a tab when its browser sends a list without its URL, so
  only a connected browser clears its tabs. Lists of one browser,
  such as two profiles, are joined first.
- Empty-query lists (the title list, the strip) keep their order.

RM-0009 is merged into RM-0023 and not shipped.

### Consequences

- Good, because frequent targets need fewer keys and no config.
- Good, because the file stays small: gone items are dropped and
  scores are capped.
- Bad, because the file holds URLs of tabs opened from the popup. It
  is private to the user and never leaves the machine.
- Bad, because a fresh install ranks as before until it has uses.

### Confirmation

Tests in `src/core/usage_tests.rs`, `src/core/switcher_usage_tests.rs`,
`src/modules/input/usage_order_tests.rs`,
`src/modules/browser_tabs/usage_tests.rs` and
`src/platform/linux/usage_file_tests.rs`.

## Pros and Cons of the Options

### `output.priority`, a configured order of kinds

- Good, because the order is predictable.
- Bad, because it is one more key and ignores which items are used.

### Ranking by last use first, then by count

- Good, because the last target is one key away.
- Bad, because it is two rules, and the hotkey already switches to
  the last window.
