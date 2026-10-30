---
status: "accepted, amended by ADR-0075"
date: "2026-10-28"
decision-makers: "CanBeSolved"
---
# Output bar for input results

## Context and Problem Statement

The title bar drew two things: the open windows, and the windows a
query matched. [ADR-0053](0053_ui_module_toggles.md) says each UI
module has one job and can change on its own. Search results are not
the open-window list, and later input can ask for other results:
installed apps, or an answer from a model. Where do those results go?

## Decision Drivers

- The title bar has one job: show the open windows.
- Any result of the input should have one place to appear.
- That place has to work before apps and model answers exist.
- Turning the title bar off must not hide search matches.

## Considered Options

- An output bar for every result of the input
- Keep matches in the title bar

## Decision Outcome

Chosen option: "An output bar for every result of the input", because
the result depends on the request, not on which windows are open.

- The title bar (`src/ui/title_bar.rs`) draws the open windows. It
  ignores the query. `appearance.title_bar` shows or hides it.
- The output bar (`src/ui/output_bar.rs`) draws the result of the
  input bar. A non-empty query shows the matched windows, with
  search highlights and the selection. An empty query shows nothing.
- Later results use the same module: found applications, a model
  answer, and anything else the input asks for. They are not new
  bars.
- The painter stacks the input bar, then the output bar when it has
  something to show, then the title bar when it is enabled. Both
  lists can be on screen together.
- The output bar has no off switch. It is how a request becomes
  visible. The input bar stays always on.

This replaces the part of ADR-0053 that drew matches in the title
bar. ADR-0053 still decides that modules toggle independently and
that the input bar cannot be turned off.

### Consequences

- Good, because a new kind of result changes the output bar only.
- Good, because the title bar can stay a plain window list.
- Bad, because a query with the title bar on makes the popup taller:
  matches and open windows are both drawn.

### Confirmation

`src/ui/output_bar.rs` draws `View::rows` when the query is not
empty. `src/ui/title_bar.rs` draws `View::windows` only when
`title_bar` is set. A painter test checks that a query adds the
output bar above the title bar.

## Pros and Cons of the Options

### An output bar for every result of the input

- Good, because search matches, apps and model answers share one
  module.
- Good, because disabling the title bar still leaves somewhere for
  matches.
- Bad, because two lists can show at once.

### Keep matches in the title bar

- Good, because the popup stays one list tall.
- Bad, because the title bar both lists windows and answers a query.
- Bad, because a model answer would have no module of its own without
  pretending it is a window.

## More Information

The info bar and the icon bar stay separate modules. They are planned
in [ROADMAP.md](../../ROADMAP.md).
