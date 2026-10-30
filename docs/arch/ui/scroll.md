# Scrolling lists

Source: `src/ui/scroll.rs`. The title list and the output bar scroll
when they hold more rows than they show
([ADR-0070](../../adr/0070_list_scroll.md)).

## Rules

- Rows shown: `output.max_rows`, or fewer on a short screen
  ([fit.md](fit.md)). Scrolling is on only when the list is longer.
- One wheel notch moves 3 rows. Touchpad scrolling adds up pixels and
  moves one row per row height. The list stops at its first and last
  page.
- The wheel moves the rows, not the selection. The selected row may
  scroll out of sight.
- A new selection (keys, a new query) moves the rows the least that
  shows it.
- The wheel works anywhere over the popup and moves the list on
  screen: the title list while the query is empty, the output bar
  while typing.
- Closing the popup forgets the position.

## Scrollbar

- A thumb 3 logical pixels wide at the right edge of the rows, in the
  text color at 40% opacity. No track.
- Its height is the shown share of the list, at least twice its
  width. Its position follows the first row shown.
- Rows leave 6 logical pixels free for it while the list scrolls.
- No dragging or track clicks.

## Contract

- `Viewport` lives in the painter, like the dropdown slide. `place`
  sets `Metrics.first_row` for a frame; `scroll` moves it.
- `Metrics.first_row = None` follows the selection only, for tests
  and callers without a viewport.
- Drawing reads only `View` and `Metrics`.
