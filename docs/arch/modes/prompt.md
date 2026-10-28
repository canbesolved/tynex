# Prompt mode

The only mode ([ADR-0050](../../adr/0050_search_only.md)). It is
called prompt mode because it will do more than search
([ADR-0051](../../adr/0051_prompt_mode.md)).

Source: `src/modes/prompt.rs`. Matching is the
[search](../modules/search.md) module.

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
the best match is selected after every change of the query. The bar
on screen is the [input bar](../ui/input_bar.md). Matches are drawn
in the [output bar](../ui/output_bar.md). Open windows show when
`title_bar.enabled` is set (see [config.md](../config.md#format) and
[title_bar.md](../ui/title_bar.md)).
