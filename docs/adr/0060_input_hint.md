---
status: "accepted"
date: "2026-10-29"
decision-makers: "CanBeSolved"
---
# Input bar hint

## Context and Problem Statement

The empty input bar should show a hint whose text comes from a shell
command. The popup has to open before that command exits, and a second
command must not start while the first is still running. Where do the
keys live, and when does the command run?

## Decision Drivers

- The keys belong under `[input_bar]`
  ([ADR-0055](0055_config_sections.md)).
- One font still draws the text ([ADR-0047](0047_one_font.md)).
- The popup opens in the same turn. A slow command must not delay it.
- An idle daemon has no poll timeout
  ([ADR-0036](0036_no_idle_wakeups.md)).
- An invalid color or wait stops the daemon
  ([ADR-0031](0031_strict_config.md)).

## Considered Options

- Spawn `sh -c` and draw before it exits; one child at a time
- Block the show path on `Command::output`
- A fixed hint string, with no command

## Decision Outcome

Chosen option: "Spawn `sh -c` and draw before it exits; one child at a
time", because the hint can be a date or a slow script, and the popup
still appears at once.

- `hint.enabled` defaults to `true`.
- `hint.font_color` is `#RRGGBB`. The default `#8da787` matches the
  new-window row. Anything else stops the daemon.
- `hint.text` is the command. The default is
  `date +"%A %b %-d %T %Y"`. An empty command shows nothing.
- There is no `hint.font_size`. The hint uses `main.font_size`.
- `hint.update` is seconds from 0 to 86400. `0` runs once per open.
  A larger value runs again while the popup stays open. The wait
  starts when the previous process exits.
- While the command runs, the hint cycles every 400 ms through `.`,
  `..`, `...`, and empty. The first frame is drawn on the open turn.
- Hiding the popup does not kill the child. Showing it again does
  not spawn a second command until that child exits. Stdout collected
  while hidden is discarded.
- A spawn failure or a non-zero exit logs a warning and keeps the
  previous hint, which is empty on the first failure.
- The child pipe stays in the poll set until exit. Animation and
  update waits exist only while the popup is open.

### Consequences

- Good, because a slow command cannot hold the first frame.
- Good, because two commands never overlap.
- Bad, because a command that ignores `SIGKILL` would still be the
  one child until it exits. `Drop` kills a child that is still
  running when the daemon stops.

### Confirmation

Config tests cover the defaults, a custom hint, a bad color and an
update above 86400. Hint tests check the dot cycle, a spawn that
returns before `sleep` ends, the first stdout line, and a missing
command. Painter tests check that the hint color is drawn only while
the query is empty.

## Pros and Cons of the Options

### Spawn `sh -c` and draw before it exits; one child at a time

- Good, because the popup and the command are independent.
- Good, because the poll loop already has deadlines for key repeat.
- Bad, because the daemon tracks a child and a pipe.

### Block the show path on `Command::output`

- Good, because the code is shorter.
- Bad, because the popup waits for the command.

### A fixed hint string, with no command

- Good, because nothing is spawned.
- Bad, because the date and other live text need a command.

## More Information

The command runs as the user, from the user's config file. Related:
[ADR-0056](0056_launcher_input.md).
