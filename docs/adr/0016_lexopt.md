---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Use lexopt for CLI parsing

## Context and Problem Statement

The CLI has a handful of subcommands and flags (`daemon`, `show
--mode`, `hide`, `quit`). Which argument parser?

## Decision Drivers

- Binary size and compile time.
- Correct handling of flags and values.

## Considered Options

- `lexopt`
- `clap`
- Manual `std::env::args` matching

## Decision Outcome

Chosen option: "`lexopt`", because it is a tiny, dependency-free,
correct lexer for arguments; a few flags do not justify `clap`.

### Consequences

- Good, because it adds almost nothing to the binary.
- Bad, because help text and errors are written by hand.
