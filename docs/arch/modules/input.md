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
[search](search.md) module. With `input.launcher.mode = "off"`,
the output list is only those matches. The default mode is
`"gui_only"`.

## Launcher

With `launcher.mode` set to `"all"` or `"gui_only"`, matching
installed apps join the output list
([ADR-0056](../../adr/0056_launcher_input.md)):

- A running app whose name or id matches: all of its open windows,
  then a new-window row when `launcher.new_window` is set. The
  default title is one space, `+`, two spaces, then `New window`,
  in `#8da787`. `launcher.new_window_text` and
  `launcher.new_window_text_color` change that title and color.
- A title-only match: those windows, then the launch row when the
  app is installed.
- An app that is not running and matches: one launch row (title =
  name).
- A matching window with no desktop entry stays a window row.
- Open windows come first, best score first. Installed apps that
  are not running follow. A new-window row stays under its app, so
  `c` still lists an open Cursor before Calf even when Calf scores
  higher ([ADR-0078](../../adr/0078_windows_before_apps.md)).

`gui_only` skips `Terminal=true` apps. The platform catalog supplies
`AppInfo` values; `Exec` stays on the platform
([platforms/linux.md](../platforms/linux.md)).
