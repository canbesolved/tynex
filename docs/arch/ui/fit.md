# Screen fit

Source: `src/ui/fit.rs`. The popup is never taller than the screen
([ADR-0069](../../adr/0069_screen_fit.md)).

## Rules

- The limit is the shortest output height, in physical pixels, minus
  24 logical pixels above and below, scaled like the rest.
- Rows go one at a time until the popup fits:
  1. The title list and the output bar, down to one row. They scroll,
     so the selection stays visible.
  2. Then the dropdown and its spare room, down to one row.
- Until the presenter knows an output height, nothing shrinks.
- `output.max_rows` stays the upper bound. A popup that already fits
  is drawn as configured.

## Contract

- `fit(view, metrics, theme, screen)` is pure and returns the metrics
  the painter uses to size, draw and hit test the same frame.
- `Metrics.list_rows` caps the title list and the output bar.
  `Metrics.max_rows` caps the dropdown and its spare room.
- When the dropdown room shrinks, keys still step through up to
  `output.max_rows` titles; titles past the drawn rows are not shown.
