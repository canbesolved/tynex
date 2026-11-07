---
status: "accepted"
date: "2026-11-07"
decision-makers: "CanBeSolved"
---
# File size limit

## Context and Problem Statement

`AGENTS.md` says a source file must not exceed 250 lines. The limit
keeps files focused and easy to review, and an agent can read a
whole file at once. Sometimes it decides where code goes instead of
the design: while adding the quick switch and the switch counter,
four files crossed 250 by 1 to 4 lines. Two splits improved the
code (`RawMain` moved next to its checks, a hook took one `Place`
instead of three arguments), one was neutral, and one only made
room (`Switcher::rows` moved into `switcher_history.rs`). `Switcher`
now spans five files. Should the limit change?

## Decision Drivers

- Files stay small, focused and readable in one pass.
- A split follows a responsibility, never a line count.
- The rule is checked with `wc -l`, with no new tool.
- `rustfmt`'s vertical style and the required doc comments use many
  lines.

## Considered Options

- Keep the hard limit of 250 lines
- 250 lines as a target, 300 as a hard limit with a reason
- Count only code lines: no blank, comment or doc comment lines
- Raise the hard limit to 400 lines

## Decision Outcome

Chosen option: "250 lines as a target, 300 as a hard limit with a
reason", because it keeps files small while letting a change finish
where it belongs, and it is still checked with `wc -l`.

- A source file should stay under 250 lines. Past 250, split it by
  responsibility when one is there.
- Between 251 and 300 lines, the file stays whole when a split would
  only make room. The pull request `## Summary` names the file and
  the reason.
- A file never exceeds 300 lines. Split it into modules first.
- Lines are counted as `wc -l` counts them, the SPDX header and
  comments included. Test files (`*_tests.rs`) follow the same rule.
- `AGENTS.md`, Coding style, changes in the same commit as this
  record.

### Consequences

- Good, because a file a few lines over no longer forces a split
  that only makes room.
- Good, because splits that improve the design still happen, now by
  choice.
- Bad, because the 250 to 300 band needs judgment, and files may
  drift toward 300.
- Neutral, because files that are split today stay split.

### Confirmation

Review: a pull request with a source file over 250 lines names it
and the reason in `## Summary`. `wc -l` on the changed files shows
none over 300.

## Pros and Cons of the Options

### Keep the hard limit of 250 lines

- Good, because it is simple and needs no judgment.
- Bad, because a file 1 to 4 lines over forces a split, sometimes
  one that only makes room.

### Count only code lines: no blank, comment or doc comment lines

- Good, because writing docs never forces a split.
- Bad, because `wc -l` no longer checks it: it needs a counting
  script or a new tool such as `tokei`.
- Bad, because a file can grow long with comments and still pass.

### Raise the hard limit to 400 lines

- Good, because splits become rare.
- Bad, because files grow larger than one focused responsibility
  and are harder to read in one pass.

## More Information

A CI step can enforce the 300-line hard limit with `wc -l` over
`src/**/*.rs`, `scripts/*.sh` and the JavaScript sources. It is not
part of this decision.
