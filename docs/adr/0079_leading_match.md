---
status: "accepted"
date: "2026-10-31"
decision-makers: "CanBeSolved"
---
# Leading window matches

## Context and Problem Statement

Open windows already come before installed apps (ADR-0078). Inside
that list, score can still put a later match above one that starts
with the query. `c` can list `Google Chrome` before `Cursor Agents`.
Should a window that starts with the query stay on top?

## Decision Drivers

- The first letters you type are how a window begins.
- A match later in the title, such as `Chrome` in `Google Chrome`,
  is a weaker switch target.
- The same rule applies with the launcher off.
- No new config key.

## Considered Options

- Windows whose title or app name starts with the query come first
- Keep score order inside the window band
- Treat the first letter only, ignoring the rest of the query

## Decision Outcome

Chosen option: "Windows whose title or app name starts with the
query come first", because that is the window the typed letters
were aimed at.

- After ranking, open windows split in two. A window whose title
  or app name starts with the query comes first, then the other
  matching windows. Score, then the original order, breaks ties
  inside each part.
- The check uses the same case and accent rules as search. An
  uppercase letter in the query makes it case-sensitive. An ASCII
  query also matches an accented start (`c` matches `Café`).
- A match that begins later in the text does not count. `c` does
  not treat `Google Chrome` as a leading window.
- Launch rows still follow the windows (ADR-0078).

This amends ADR-0078. The body of that record is unchanged.

### Consequences

- Good, because `c` lists `Cursor Agents` before `Google Chrome`.
- Good, because the rule does not depend on which score is higher.
- Bad, because a stronger match later in the title is no longer
  the first row.

### Confirmation

`starts_with_query` covers a leading title, a later match, an
accent and smart case. A launcher test types `c` against
`Google Chrome` and `Cursor Agents`.

## Pros and Cons of the Options

### Windows whose title or app name starts with the query come first

- Good, because the window that begins with the query stays first.
- Good, because `chrome` still prefers `Chrome` over `Cursor`.
- Bad, because score no longer picks the first window.

### Keep score order inside the window band

- Good, because the closest fuzzy match stays first.
- Bad, because `Google Chrome` can sit above `Cursor Agents`.

### Treat the first letter only, ignoring the rest of the query

- Good, because a one-letter query is simple.
- Bad, because `ch` would still lift every window that starts
  with `c`.

## More Information

Ships in 0.2.17. Amends [ADR-0078](0078_windows_before_apps.md).
Behavior is in [input.md](../arch/modules/input.md).
