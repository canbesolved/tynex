# Search mode

Source: `src/modes/search.rs`, `src/search/fuzzy.rs` and
`src/search/fold.rs`.

## Behavior

| Key            | Action                              |
| -------------- | ----------------------------------- |
| Printable char | Append to query, refilter           |
| `Backspace`    | Delete last char, refilter          |
| `Up` / `Down`  | Move selection                      |
| `Enter`        | Activate selected window, close     |
| `Tab`          | Switch to hotkey mode               |
| `Esc`          | Clear query; close if already empty |

Opening with `tynex show --mode search` (or `mode = "search"` in config)
lets the user type immediately after the hotkey. The best match is
selected after every change of the query. Switching back to hotkey
mode shows all windows again; the query stays for the next switch.

## Matching

- `frizbee` scores the query against `title` and `app_id`
  (Smith-Waterman, like `fzf`).
- Smart case: case-insensitive unless the query has an uppercase
  letter.
- Typo tolerance: up to one typo per four query characters
  (`max_typos`), so looser matches can follow the best ones. Queries
  under four characters allow no typo.
- Results are sorted by the better score of title and app id; ties
  keep the row order (previous window first, active window last).
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
- **Case:** smart case uses Unicode case mapping (`привет` matches
  `Привет`). Letters whose other case differs in length or count
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
