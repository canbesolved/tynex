# Screen fit

Source: `src/ui/fit.rs`. The popup is never taller than the screen
([ADR-0069](../../adr/0069_screen_fit.md), amended by
[ADR-0073](../../adr/0073_popup_place.md)).

## Rules

- The limit is the shortest output height. `top`, and `center` before
  the open height is locked, keep 24 logical pixels above and below.
  After `center` locks that height, the limit is the screen minus the
  locked top margin minus 24 logical pixels at the bottom.
- Rows go one at a time until the popup fits:
  1. The title list and the output bar, down to one row. They scroll,
     so the selection stays visible.
  2. Then the dropdown and its spare room, down to one row. Only
     `iconbar` mode has that room; in `both` mode the dropdown
     overlays the title list.
- Until the presenter knows an output height, nothing shrinks.
- `output.max_rows` stays the upper bound. A popup that already fits
  is drawn as configured.

## Contract

- `fit(view, metrics, theme, screen, position, anchor)` is pure and
  returns the metrics the painter uses to size, draw and hit test the
  same frame. `anchor` is the logical height locked at open, or none
  before the first show.
- `Metrics.list_rows` caps the title list and the output bar.
  `Metrics.max_rows` caps the dropdown and its spare room.
- When the dropdown room shrinks, keys still step through up to
  `output.max_rows` titles; titles past the drawn rows are not shown.
