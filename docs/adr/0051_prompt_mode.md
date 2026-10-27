---
status: "accepted"
date: "2026-10-27"
decision-makers: "CanBeSolved"
---
# Prompt mode, broader than search

## Context and Problem Statement

[ADR-0050](0050_search_only.md) left one mode and called it search
mode, because typing filters windows. Later plans use that same mode
for more than a query: a task list, launching an app, renaming a
title, closing a window. The name "search mode" describes only the
filter. What should the mode be called?

## Decision Drivers

- The name should still fit when the prompt does more than search.
- Search is a module, not a mode.
- One mode, as decided in ADR-0050.

## Considered Options

- Call the mode prompt mode
- Keep the name search mode

## Decision Outcome

Chosen option: "Call the mode prompt mode", because the mode will
cover more than search. Filtering windows by a typed query is
the search module; the mode is the prompt around it.

- Docs, the roadmap, the changelog, deprecations and module comments
  say prompt mode.
- The mode is `PromptMode` in `src/modes/prompt.rs`. Its page is
  `docs/arch/modes/prompt.md`.
- Search is the `search` module in `src/modules/search/`. Its page
  is `docs/arch/modules/search.md`.

This amends the name used in ADR-0050. ADR-0050 still decides one
mode, `title_bar`, and the removed `mode`, `keys` and
`sort_by = "key"`.

### Consequences

- Good, because a task list, a launcher and other prompt actions are
  not described as search.
- Good, because search is a module the mode calls, not the mode.
- Bad, because accepted records, including ADR-0050, still say search
  mode. Those records stay as written.

### Confirmation

`docs/arch/modes/prompt.md` is titled Prompt mode and links this ADR.
`src/modes/prompt.rs` defines `PromptMode`. Search lives in
`src/modules/search/` and `docs/arch/modules/search.md`.

## Pros and Cons of the Options

### Call the mode prompt mode

- Good, because the name still fits when the prompt does more than
  search.
- Good, because search remains the name of the filter, as a module.
- Bad, because the word changes in docs that already shipped.

### Keep the name search mode

- Good, because the name matches today's filter and no docs change.
- Bad, because the name becomes wrong once the prompt does more than
  search.

## More Information

Related plans are the task list and the launcher in
[ROADMAP.md](../../ROADMAP.md).
