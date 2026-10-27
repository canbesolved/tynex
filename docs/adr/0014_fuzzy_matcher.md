---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Fuzzy matching for search

## Context and Problem Statement

Search mode filters windows on every key press by title and app id.
Should it use a fuzzy matching library or a simple own match?

## Decision Drivers

- Good ranking quality with match positions for highlighting.
- Tolerance for small typos and mixed case.
- No allocations after setup.

## Considered Options

- Fuzzy matching library
- Hand-written subsequence match

## Decision Outcome

Chosen option: "Fuzzy matching library", because ranking by gaps,
word starts and case, plus typo tolerance, is far more than a few
lines of code, and good libraries do it in microseconds. The library
is chosen in [ADR-0022](0022_frizbee.md).

### Consequences

- Good, because refiltering a few dozen windows takes microseconds, so
  no debounce is needed.
- Bad, because it adds a dependency instead of 30 lines of own code.

## Pros and Cons of the Options

### Hand-written subsequence match

- Good, because it needs no dependency.
- Bad, because ranking is poor and typos are not tolerated.

## More Information

See [search.md](../arch/modules/search.md).
