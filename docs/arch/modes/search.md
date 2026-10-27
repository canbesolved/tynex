# Search mode

The only mode ([ADR-0050](../../adr/0050_search_only.md)).

Source: `src/modes/search.rs`, `src/search/fuzzy.rs` and
`src/search/fold.rs`.

## Behavior

| Key            | Action                              |
| -------------- | ----------------------------------- |
| Printable char | Append to query, refilter           |
| `Backspace`    | Delete last char, refilter          |
| `Up` / `Down`  | Move selection                      |
| `Enter`        | Activate selected window, close     |
| `Tab`          | Ignored                             |
| `Esc`          | Clear query; close if already empty |

The popup opens with the previous window selected, so `Enter` right
after the hotkey switches back like Alt+Tab. Typing starts at once;
the best match is selected after every change of the query. Whether
the window list shows before typing depends on `title_bar` (see
[config.md](../config.md#format)).

## Matching

- `frizbee` scores the query against the row title (`title`) and
  against the app name and id, joined by a space (Smith-Waterman,
  like `fzf`). So `firefox` finds Firefox windows even when
  `strip_app_name` removed the name from their titles.
- Smart case: case-insensitive unless the query has an uppercase
  letter.
- Typo tolerance: up to one typo per four query characters
  (`max_typos`), so looser matches can follow the best ones. Queries
  under four characters allow no typo.
- Results are sorted by the better score of title and app; ties
  keep the row order from `sort_by` (see
  [core.md](../core.md#row-order)).
- Match indices go into `View` so the layout can highlight them, for
  the title only. `frizbee` returns byte offsets of matched
  characters; they are merged into byte ranges.
- One entry point, `match_list_indices`, single-threaded: the window
  list is small, and each extra entry point adds about 1 MB to the
  binary ([ADR-0044](../../adr/0044_binary_budget.md)).

## Languages

Titles and queries can use any script. Rules from
[core.md](../core.md#text-and-languages) apply.

- **Unicode:** `frizbee` matches UTF-8 directly. A non-ASCII query
  switches it to its Unicode path, so Cyrillic, Greek, CJK and other
  scripts match character by character.
- **Case:** smart case uses Unicode case mapping (`ñandú` matches
  `Ñandú`). Letters whose other case differs in length or count
  (Turkish `İ`, German `ß`) match only in the typed case.
- **Accents:** an ASCII query also matches accented Latin letters
  (`cafe` matches `Café`, `zurich` matches `Zürich`). `fold.rs` keeps
  a folded copy of each title with a byte offset map back to the
  original, so highlights land on the real letters. A query with
  accents is not folded: another accent counts as a typo. The fold
  tables cover Latin-1 Supplement, Latin Extended-A and Latin Extended
  Additional, plus letters with a stroke (`Ø`, `Đ`, `Ł`); no crate is
  needed.
- **Input:** characters come from the active layout, including dead
  keys and compose. Chinese, Japanese and Korean need an input method;
  see [ROADMAP.md](../../../ROADMAP.md).

## Cost

With a few dozen windows, refiltering takes microseconds, so it runs
on every key press with no debounce.
