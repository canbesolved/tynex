# Configuration

Source: `src/config/`.

```text
config/
  mod.rs         Config types, parse(text, font_exists)
  raw.rs         serde layout with defaults and value spans
  validate.rs    Value checks, all errors at once
  error.rs       ConfigError with line and column
  file.rs        load_or_create(path): read, or write the default
  default.toml   Default file, embedded with include_str!
  tests.rs       Unit tests
```

## Location

`<config dir>/tynex/tynex.toml`, TOML format. The config dir comes from
the platform module (`paths.rs`), using only the standard library:

- Linux: `$XDG_CONFIG_HOME`, else `~/.config`;
- macOS (later): `~/Library/Application Support`;
- Windows (later): `%APPDATA%`.

Parsed with `toml` (features `parse`, `serde`, `std`) into `serde`
structs with `deny_unknown_fields` and `#[serde(default)]`.

## Format

The default file. It is kept in `default.toml` and embedded with
`include_str!`; the `Default` impls must match it (a unit test checks
this). Every key is optional: a missing key takes the value shown here.

Sections stay in this order: `[main]`, `[hotkeys]`, `[input]`,
`[output]`, then `[taskbar]`
([ADR-0055](../adr/0055_config_sections.md), amended by
[ADR-0065](../adr/0065_taskbar.md) and
[ADR-0066](../adr/0066_short_sections.md)).

```toml
[main]
desktop = "auto"             # auto | kde | mock
font = "auto"                # Font file path, or auto
font_size = 14
width = 480
position = "center"          # center | top

[hotkeys]
# Hotkey pressed again while the popup is open: "hide" closes the
# popup, "last" switches to the previously used window.
action = "last"              # hide | last

[input]
# Installed apps in the output bar: "off" hides them, "all" includes
# terminal apps, "gui_only" skips Terminal=true.
launcher.mode = "gui_only"   # off | all | gui_only
# Extra row under a running app. false hides that row.
launcher.new_window = true
# One space, a plus, two spaces, then "New window".
launcher.new_window_text = " +  New window"
launcher.new_window_text_color = "#8da787" # #RRGGBB
# Hint in the empty input bar. false hides it.
hint.enabled = true
hint.font_color = "#8da787" # #RRGGBB
# Shell command. Its first stdout line is the hint.
hint.text = 'date +"%A %b %-d %T %Y"'
# Seconds after the previous exit. 0 runs once per open.
hint.update = 0

[output]
# Cap for the output bar, the title list and an icon dropdown.
max_rows = 19

[taskbar]
# "off" hides the strip and the title list until you type.
# "iconbar" shows one icon per running app.
# "titlebar" shows the open windows.
# "both" shows the strip, then the title list.
mode = "iconbar"             # off | iconbar | titlebar | both
# Row order: "recently" puts the last used window first, "app" groups
# windows by app name.
titlebar.sort_by = "recently" # recently | app
# Draw the app icon before the title. false shows titles only.
titlebar.show_icons = true
# Drop " — <app name>" from the end of window titles.
titlebar.strip_app_name = false
# Icon side in logical pixels, scaled with the display.
iconbar.size = 24
# Selection border outside the icon, in logical pixels.
iconbar.border = 4
```

`main.desktop` selects the window source adapter. `auto` picks `kde`
when the colon-separated `XDG_CURRENT_DESKTOP` list contains `KDE`, and
fails otherwise; it never selects `mock`.

`main.position` is where the popup sits
([ADR-0073](../adr/0073_popup_place.md)): `center` (default) or
`top`. `center` places it in the middle of the shortest output when
it opens, then keeps that top edge while the output bar or a dropdown
grows downward. `top` keeps it 24 logical pixels from the top of the
screen. A missing key stays centered. Any other value stops the
daemon.

`hotkeys.action` sets what pressing the hotkey (or `tynex show`) does
while the popup is open: `hide` closes it, `last` switches to the
previously used window, or closes it when there is none.

`input.launcher.mode` adds installed apps to the output bar
([ADR-0056](../adr/0056_launcher_input.md)): `"off"`, `"all"`, or
`"gui_only"` (default; skips `Terminal=true`).

`launcher.new_window` is the extra row under a running app
([ADR-0059](../adr/0059_new_window_row.md)). It defaults to `true`.
`false` hides that row; an app that is not running is still one
launch row titled with its name. `launcher.new_window_text` is the
title. The default is one space, `+`, two spaces, then `New window`.
`launcher.new_window_text_color` is `#RRGGBB`. The default
`#8da787` is a little dimmer than the theme text. A missing key
uses the default. An empty title or a color that is not `#RRGGBB`
stops the daemon.

`input.hint` is the text in the empty input bar
([ADR-0060](../adr/0060_input_hint.md)). `hint.enabled` defaults to
`true`. `hint.text` is a shell command. The default is
`date +"%A %b %-d %T %Y"`. Its first stdout line is the hint. An
empty command shows nothing. `hint.font_color` is `#RRGGBB`. The
default `#8da787` matches the new-window row. The hint uses
`main.font_size`. `hint.update` is seconds from 0 to 86400. `0`
runs the command once each time the popup opens. A larger value
runs it again while the popup stays open, waiting that long after
the previous command exits. A bad color or a wait outside that
range stops the daemon.

`output.max_rows` caps the output bar, the title list and an
icon dropdown. On a short screen fewer rows are drawn, so the popup
fits (see [fit.md](ui/fit.md)). A title list or output bar longer
than its rows scrolls, with a scrollbar (see
[scroll.md](ui/scroll.md)).

`taskbar.mode` chooses the taskbar while the query is empty
([ADR-0065](../adr/0065_taskbar.md), amended by
[ADR-0066](../adr/0066_short_sections.md)): `off`, `iconbar`
(default), `titlebar` or `both`. `iconbar` is one icon per running
app. `titlebar` is the open windows. `both` is the strip, then the
title list. Typing hides whichever is showing. Clearing the query
with `Backspace` or `Esc` shows it again. `off` leaves only the
input bar until you type. Matches then appear in the output bar.

`icon_bar.enabled` and `title_bar.enabled` were removed in 0.2.10.
`taskbar.mode` replaces both. A config that still sets either
`enabled` key stops the daemon with an unknown key error. A config
that still sets `mode` or `keys`, removed with hotkey mode in 0.1.6,
stops the daemon too
([ADR-0050](../adr/0050_search_only.md)).

`[input_bar]`, `[output_bar]`, `[title_bar]` and `[icon_bar]` were
removed in 0.2.11. Their keys moved to `[input]`, `[output]` and
`[taskbar]`. `taskbar.mode` values `icons` and `titles` became
`iconbar` and `titlebar`. A file that still uses an old section or
an old mode value stops the daemon
([ADR-0066](../adr/0066_short_sections.md)).

`taskbar.titlebar.show_icons` draws the app icon before the title
(default `true`). `false` shows titles only, with no icon slot.
A window without an icon keeps an empty slot when icons are on.
The output bar always draws the icon
([ADR-0058](../adr/0058_title_bar_row.md)).

`title_bar.show_app_icon`, the app name column, was removed in 0.2.3.
It replaces `appearance.show_app` and earlier `title_format`. A
config that still sets `show_app_icon`, `show_app` or `title_format`
stops the daemon with an unknown key error.

`layout` was removed in 0.1.7
([ADR-0052](../adr/0052_ui_modules.md)). The popup is UI modules, so
there is nothing to choose. The input bar is always on. The taskbar
mode picks the strip, the title list, both or neither
([ADR-0053](../adr/0053_ui_module_toggles.md), amended by
[ADR-0065](../adr/0065_taskbar.md)). A config that still
sets `layout` stops the daemon with an unknown key error.

`[appearance]` and the old top-level keys (`desktop`, `hotkey_action`,
`sort_by`) were removed in 0.2.0
([ADR-0055](../adr/0055_config_sections.md)). A config that still has
them stops the daemon with an unknown key error.

`taskbar.titlebar.strip_app_name` drops the app name from the end
of a title, after ` — `, ` – ` or ` - `: `Café — Firefox` becomes
`Café`.
The end must equal the app name or the last part of the app id
(`konsole` for `org.kde.konsole`), ignoring case. A title that would
become empty stays as it is.

`taskbar.titlebar.sort_by` sets the order of the rows: `recently`
(default) by last use, `app` by app name. The popup always opens
with the previous window selected (see
[core.md](core.md#row-order)). `key`, the letter order, was removed
in 0.1.6 and is rejected.

`taskbar.iconbar.size` is the icon side in logical pixels, default
`24`, from 8 to 128. It is multiplied by the output scale, like
`main.font_size`. A value outside that range stops the daemon.
`taskbar.iconbar.border` is the selection outline in the same
units, default `4`, from 1 to 32. A missing key uses the default.
The strip shows when `taskbar.mode` is `iconbar` or `both`
([ADR-0062](../adr/0062_icon_bar.md)).

Planned options are listed in [ROADMAP.md](../../ROADMAP.md).

## Loading rules

The config is read once, when `tynex daemon` starts
([ADR-0031](../adr/0031_strict_config.md)).

- **No file:** the daemon writes the default file above, logs its
  path at `warn` level, then continues with it.
- **Missing key or table:** the built-in default is used.
- **Syntax error:** exit.
- **Unknown key:** exit, so typos never pass silently. A key renamed
  or removed in a release is unknown too; the message points to the
  release notes.
- **Wrong type or invalid value:** exit.

`tynex` never changes an existing config file. The CLI commands
(`show`, `hide`, `quit`) do not read the config.

## Validation

| Key                               | Rule                                   |
| --------------------------------- | -------------------------------------- |
| `main.desktop`                    | `auto`, `kde` or `mock`                |
| `main.font`                       | `auto` or a path to an existing font   |
| `main.font_size`                  | 8 to 72                                |
| `main.width`                      | 200 to 2000                            |
| `main.position`                   | `center` or `top`                      |
| `hotkeys.action`                  | `hide` or `last`                       |
| `input.launcher.mode`             | `off`, `all` or `gui_only`             |
| `launcher.new_window`             | `true` or `false`                      |
| `launcher.new_window_text`        | a non-empty string                     |
| `launcher.new_window_text_color`  | `#RRGGBB`                              |
| `input.hint.enabled`              | `true` or `false`                      |
| `input.hint.font_color`           | `#RRGGBB`                              |
| `input.hint.update`               | 0 to 86400                             |
| `output.max_rows`                 | 1 to 50                                |
| `taskbar.mode`                    | `off`, `iconbar`, `titlebar` or `both` |
| `taskbar.titlebar.sort_by`        | `recently` or `app`                    |
| `taskbar.titlebar.show_icons`     | `true` or `false`                      |
| `taskbar.titlebar.strip_app_name` | `true` or `false`                      |
| `taskbar.iconbar.size`            | 8 to 128                               |
| `taskbar.iconbar.border`          | 1 to 32                                |

Syntax and type errors stop at the first error, as reported by `toml`.
Value checks run on the parsed file and report every invalid key at
once.

## Errors

Each problem is logged at `error` level (stderr, see
[logging.md](logging.md)) with the file path, line and column when
known, the key, and what is expected. The daemon then exits with code
78 (`EX_CONFIG`). Under a systemd user unit the journal keeps the
messages.

```text
tynex: error: ~/.config/tynex/tynex.toml:1:1: unknown key `adaptr`,
  expected one of `main`, `hotkeys`, `input`, `output`, `taskbar`
tynex: error: ~/.config/tynex/tynex.toml:4:13: `main.font_size`
  = 120 is out of range, expected 8 to 72
```

## Module boundary

The config module returns a plain `Config` struct or a list of
`ConfigError` values. Other modules get only the sub-struct they need
(for example `MainConfig` or `TaskbarConfig`), never the whole file.

- `parse` is pure: the font file check is passed in, so tests need no
  disk.
- `load_or_create` returns `Loaded::Existing` or `Loaded::Created`, so
  the daemon can log a new default file.
- `ConfigError` prints `line:col: message`; the daemon adds the file
  path in front, as in the example above.
