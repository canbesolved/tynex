# Input module

Source: `src/modules/input/`. Sub-modules of the
[input bar](../ui/input_bar.md). They read the query and fill the
output list. They do not draw.

```text
modules/input/
  mod.rs         Runs search, then the launcher
  launcher.rs    Groups windows and launch rows
```

Prompt mode holds one `Input` and calls `refresh` after every query
change ([modes/prompt.md](../modes/prompt.md)).

## Search

Open windows are ranked with the
[search](search.md) module. With `input_bar.launcher.mode = "off"`
(default), the output list is only those matches.

## Launcher

With `launcher.mode` set to `"all"` or `"gui_only"`, matching
installed apps join the output list
([ADR-0056](../../adr/0056_launcher_input.md)):

- A running app whose name or id matches: all of its open windows,
  then a row titled `<app name>: New window`.
- A title-only match: those windows, then the launch row when the
  app is installed.
- An app that is not running and matches: one launch row (title =
  name).
- A matching window with no desktop entry stays a window row.
- Groups are ordered by match score; on a tie, a running app beats
  an idle one (so `c` prefers an open Cursor over Calf).

`gui_only` skips `Terminal=true` apps. The platform catalog supplies
`AppInfo` values; `Exec` stays on the platform
([platforms/linux.md](../platforms/linux.md)).
