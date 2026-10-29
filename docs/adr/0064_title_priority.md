---
status: "accepted"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Title bar confirms over icons

## Context and Problem Statement

[ADR-0062](0062_icon_bar.md) gives `Enter` and `Space` to the icon
strip whenever it is showing. With the title bar on, confirming the
selected window then opens the icon dropdown when that app has
several windows. Which selection should those keys confirm?

## Decision Drivers

- The title bar is the list of windows on screen.
- `Enter` and `Space` confirm the selection the user can see.
- A click on an icon is aimed at that icon, not at the title list.
- An open dropdown is its own selection until it closes.
- No new `Key` variant and no config key.

## Considered Options

- The title bar wins `Enter` and `Space` while it is showing
- The icon strip keeps `Enter` and `Space` whenever it is showing
- `Enter` always confirms the title bar, even inside a dropdown

## Decision Outcome

Chosen option: "The title bar wins `Enter` and `Space` while it is
showing", because that list is the selection, and the dropdown
stays a click on the icon.

- While the strip is showing and `title_bar.enabled` is set, and
  the dropdown is closed, `Enter` and `Space` activate the selected
  title. They do not open the dropdown.
- With the title bar off, `Enter` and `Space` still match a click
  on the selected icon: one window activates, several open the
  dropdown, none launches.
- A left click on an icon still does that icon action when the
  title bar is on.
- An open dropdown still takes `Enter` and `Space`, and `Up`,
  `Down` and `Tab`. `Esc` still closes it first.

This amends ADR-0062. The body of that record is unchanged.

### Consequences

- Good, because confirming a title switches to that window.
- Good, because a click can still open the dropdown.
- Bad, because `Enter` and a click on the selected icon differ
  while the title bar is on.

### Confirmation

`src/modes/prompt_icon_tests.rs`: with the title bar on, `Enter`
and `Space` activate the selected title and leave the dropdown
closed. A click still opens it, and `Enter` then activates the
highlighted title.

## Pros and Cons of the Options

### The title bar wins `Enter` and `Space` while it is showing

- Good, because the visible window list keeps confirm.
- Good, because the dropdown remains available from a click.
- Bad, because confirm and a click on the icon are not the same
  key while both bars show.

### The icon strip keeps `Enter` and `Space` whenever it is showing

- Good, because one key always means the selected icon.
- Bad, because the selected title cannot be confirmed.

### `Enter` always confirms the title bar, even inside a dropdown

- Good, because confirm never depends on the dropdown.
- Bad, because a highlighted dropdown title has no confirm key.

## More Information

The icon strip is 0.2.9 in [ROADMAP.md](../../ROADMAP.md). Related:
[ADR-0061](0061_prompt_keys.md), [ADR-0062](0062_icon_bar.md).
