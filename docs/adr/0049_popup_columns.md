---
status: "accepted"
date: "2026-10-18"
decision-makers: "CanBeSolved"
---
# Popup columns

## Context and Problem Statement

`title_format` ([ADR-0045](0045_app_names.md)) joins the app name and
the title into one string per row, for example `"{app} : {title}"`.
App names of different lengths then push titles to different places,
and the list is hard to scan. How should rows show the app name?

## Decision Drivers

- Titles start at the same place in every row.
- Few config keys; sizes follow what the user already sets.
- The core gives data, the layout decides how it looks.

## Considered Options

- Columns: key, icon, app name, title; `show_app` toggles the app name
- Keep `title_format`
- Columns with configurable widths

## Decision Outcome

Chosen option: "Columns: key, icon, app name, title", because aligned
columns are easier to scan and need only one switch.

- `Row` keeps the app name and the stripped title as separate fields;
  `row_title` replaces `TitleFormat`.
- `show_app = false` (default) in `[appearance]` hides the app name
  column; `strip_app_name` still drops the name from titles.
- The app name column takes 30 % of the text width after the icon;
  the title takes the rest. Both are cut with an ellipsis.
- `title_format` is removed. It is a breaking change: a config file
  that still has it stops the daemon with an unknown key error.

This amends ADR-0045 where it names `title_format` as the use of app
names.

### Consequences

- Good, because titles line up and app names read as a column.
- Good, because the format parser and its errors are gone.
- Bad, because users with `title_format` must edit their config.
- Bad, because custom text around names, such as `[{app}]`, is no
  longer possible.

### Confirmation

Layout tests check that the app column is drawn only with `show_app`;
config tests check that `title_format` is an unknown key.

## Pros and Cons of the Options

### Keep `title_format`

- Good, because users choose any text.
- Bad, because titles do not line up.

### Columns with configurable widths

- Good, because users can tune each column.
- Bad, because it adds keys that `width` already covers well enough.
