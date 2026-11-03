---
status: "accepted"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# Roadmap item codes

## Context and Problem Statement

Roadmap items have no stable name. The maintainer and AI agents refer
to them by paraphrase, which is ambiguous, and an item loses its
trace once it leaves `ROADMAP.md`. How do we name an item exactly,
from planning to release or drop?

## Decision Drivers

- One exact handle for prompts, commits, pull requests, the changelog
  and deprecations.
- Works offline and in plain text, so agents can grep it.
- No new tool or service.

## Considered Options

- No codes, describe items in words
- GitHub issues as item IDs
- `RM-NNNN` codes in `ROADMAP.md`

## Decision Outcome

Chosen option: "`RM-NNNN` codes in `ROADMAP.md`", because it gives an
exact, greppable name that lives in the repository and needs no
network.

- Format: `RM-` plus at least four digits, zero padded, right after
  the checkbox. Past `RM-9999` the number grows to five digits.
- A new item takes the lowest free code: one that is in none of
  `ROADMAP.md`, `CHANGELOG.md` and `DEPRECATIONS.md`.
  `scripts/rm_next.sh` prints it.
- The code is carried in the commit subject, the pull request, the
  `CHANGELOG.md` entry and, for a dropped item, the `DEPRECATIONS.md`
  entry. It is never removed from those files, so it is not reused
  for another feature.
- Branch names do not need the code: they are deleted after the
  squash merge, and some branches cover no item or several.
- Rules for agents are in `AGENTS.md`, section "Roadmap codes".

### Consequences

- Good, because a request such as "do RM-0012" is unambiguous.
- Good, because `git log --grep RM-0012` and a grep over the docs
  show the whole history of an item.
- Bad, because every new item needs a code, and two open pull requests
  may pick the same one. No stacked pull requests keeps this rare;
  the second one takes a new code on rebase.

### Confirmation

Review checks that every `- [ ]` item in `ROADMAP.md` has a code and
that no code repeats:
`grep -o 'RM-[0-9]\{4,\}' ROADMAP.md | sort | uniq -d` prints nothing.

## Pros and Cons of the Options

### No codes, describe items in words

- Good, because nothing to maintain.
- Bad, because wording changes, and agents may pick the wrong item.

### GitHub issues as item IDs

- Good, because GitHub links issues, commits and pull requests.
- Bad, because it needs the network and a second place to plan in,
  apart from `ROADMAP.md`.

### `RM-NNNN` codes in `ROADMAP.md`

- Good, because plain text, offline, next to the item.
- Bad, because codes are assigned by hand, helped by
  `scripts/rm_next.sh`.

## More Information

Existing `CHANGELOG.md` and `DEPRECATIONS.md` entries are not
back-filled; codes start with the items in `ROADMAP.md` on
2026-11-03.
