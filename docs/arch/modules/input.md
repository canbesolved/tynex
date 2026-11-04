# Input module

Source: `src/modules/input/`. Sub-modules of the
[input bar](../ui/input_bar.md). They read the query and fill the
output list. They do not draw.

```text
modules/input/
  mod.rs         Runs search, then the launcher, then joins tabs
  launcher.rs    Groups windows and launch rows
  browser_tabs.rs  Puts tab rows between windows and launch rows
  exact.rs       Exact matches first
```

Prompt mode holds one `Input` and calls `refresh` after every query
change ([modes/prompt.md](../modes/prompt.md)).

## Search

Open windows are ranked with the
[search](search.md) module. A window whose title or app name
starts with the query is listed before the other matches
([ADR-0079](../../adr/0079_leading_match.md)). With
`input.launcher.mode = "off"`, the output list is only those
matches. The default mode is `"gui_only"`.

## Launcher

With `launcher.mode` set to `"all"` or `"gui_only"`, matching
installed apps join the output list
([ADR-0056](../../adr/0056_launcher_input.md)):

- A running app whose name or id matches lists all of its open
  windows. When `launcher.new_window` is set, that app also gets a
  launch row after the windows. The default title is one space,
  `+`, two spaces, then `New window`, in `#797979`.
  `launcher.new_window_text` and
  `launcher.new_window_text_color` change that title and color.
- A title-only match lists those windows, and the launch row when
  the app is installed.
- An app that is not running and matches: one launch row (title =
  name).
- A matching window with no desktop entry stays a window row.
- Open windows come first. An app whose title or name starts with
  the query comes before one that matches later in the text, so
  `c` lists Cursor, then its new-window row, before Google Chrome
  ([ADR-0079](../../adr/0079_leading_match.md)). The new-window row
  stays under that app ([ADR-0080](../../adr/0080_new_window_place.md)).
  Score breaks ties. Installed apps that are not running follow,
  so `c` still lists an open Cursor before Calf
  ([ADR-0078](../../adr/0078_windows_before_apps.md)).

## Exact matches first

The joined list (windows, then browser tabs, then launch rows) is
split into two bands with a stable sort
([ADR-0085](../../adr/0085_exact_first.md)):

1. Exact rows: the query appears as one run of letters, with the
   search's case and accent rules, in the title, or in the app name
   or short id of a window or launch row. A tab title includes its
   host. A new-window row is exact when its app has an exact
   window.
2. Fuzzy rows: every other match.

Each band keeps the order above, so `miro` lists the tab
`Overall - Miro — miro.com` before a Chromium window that matched
only through `chromium-browser`, while `c` still lists Cursor and
its new-window row before Google Chrome.

`gui_only` skips `Terminal=true` apps. The platform catalog supplies
`AppInfo` values; `Exec` stays on the platform
([platforms/linux.md](../platforms/linux.md)).
