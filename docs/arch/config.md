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

Sections stay in this order: `[main]`, `[hotkeys]`, `[input_bar]`,
`[output_bar]`, then the other modules
([ADR-0055](../adr/0055_config_sections.md)).

```toml
[main]
desktop = "auto"             # auto | kde | mock
font = "auto"                # Font file path, or auto
font_size = 14
width = 480

[hotkeys]
# Hotkey pressed again while the popup is open: "hide" closes the
# popup, "last" switches to the previously used window.
action = "last"              # hide | last

[input_bar]

[output_bar]
# Cap for the output bar and the title bar.
max_rows = 19

[title_bar]
# Show the open windows. Search matches are the output bar, shown
# once you type.
enabled = false
# Row order: "recently" puts the last used window first, "app" groups
# windows by app name.
sort_by = "recently"         # recently | app
# Show the app name in its own column before the title.
show_app_icon = false
# Drop " — <app name>" from the end of window titles.
strip_app_name = false
```

`main.desktop` selects the window source adapter. `auto` picks `kde`
when the colon-separated `XDG_CURRENT_DESKTOP` list contains `KDE`, and
fails otherwise; it never selects `mock`.

`hotkeys.action` sets what pressing the hotkey (or `tynex show`) does
while the popup is open: `hide` closes it, `last` switches to the
previously used window, or closes it when there is none.

`[input_bar]` is empty today. `launcher.mode` is planned for 0.2.1
([ROADMAP.md](../../ROADMAP.md)).

`output_bar.max_rows` caps both the output bar and the title bar.

`title_bar.enabled` shows the open windows. With `false` (default)
only the input bar shows until you type. Matches then appear in the
output bar, whether or not the title bar is on. It replaces hotkey
mode, which 0.1.6 removed with the `mode` and `keys` keys
([ADR-0050](../adr/0050_search_only.md)); a config that still has them
stops the daemon with an unknown key error.

`title_bar.show_app_icon` adds an app name column before the title
(see [ADR-0045](../adr/0045_app_names.md) for where names come from and
[title_bar.md](ui/title_bar.md) for the columns). It replaces
`appearance.show_app` and earlier `title_format`; a config that still
has them stops the daemon with an unknown key error.

`layout` was removed in 0.1.7
([ADR-0052](../adr/0052_ui_modules.md)). The popup is UI modules, so
there is nothing to choose. The input bar is always on; the title
bar can be turned off on its own
([ADR-0053](../adr/0053_ui_module_toggles.md)). A config that still
sets `layout` stops the daemon with an unknown key error.

`[appearance]` and the old top-level keys (`desktop`, `hotkey_action`,
`sort_by`) were removed in 0.2.0
([ADR-0055](../adr/0055_config_sections.md)). A config that still has
them stops the daemon with an unknown key error.

`title_bar.strip_app_name` drops the app name from the end of a
title, after ` — `, ` – ` or ` - `: `Café — Firefox` becomes `Café`.
The end must equal the app name or the last part of the app id
(`konsole` for `org.kde.konsole`), ignoring case. A title that would
become empty stays as it is.

`title_bar.sort_by` sets the order of the rows: `recently` (default)
by last use, `app` by app name. The popup always opens with the
previous window selected (see [core.md](core.md#row-order)). `key`,
the letter order, was removed in 0.1.6 and is rejected.

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

| Key                        | Rule                                      |
| -------------------------- | ----------------------------------------- |
| `main.desktop`             | `auto`, `kde` or `mock`                   |
| `main.font`                | `auto` or a path to an existing font file |
| `main.font_size`           | 8 to 72                                   |
| `main.width`               | 200 to 2000                               |
| `hotkeys.action`           | `hide` or `last`                          |
| `output_bar.max_rows`      | 1 to 50                                   |
| `title_bar.enabled`        | `true` or `false`                         |
| `title_bar.sort_by`        | `recently` or `app`                       |
| `title_bar.show_app_icon`  | `true` or `false`                         |
| `title_bar.strip_app_name` | `true` or `false`                         |

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
  expected one of `main`, `hotkeys`, `input_bar`, `output_bar`,
  `title_bar`
tynex: error: ~/.config/tynex/tynex.toml:4:13: `main.font_size`
  = 120 is out of range, expected 8 to 72
```

## Module boundary

The config module returns a plain `Config` struct or a list of
`ConfigError` values. Other modules get only the sub-struct they need
(for example `MainConfig` or `TitleBarConfig`), never the whole file.

- `parse` is pure: the font file check is passed in, so tests need no
  disk.
- `load_or_create` returns `Loaded::Existing` or `Loaded::Created`, so
  the daemon can log a new default file.
- `ConfigError` prints `line:col: message`; the daemon adds the file
  path in front, as in the example above.
