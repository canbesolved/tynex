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

```toml
desktop = "auto"         # auto | kde | mock
mode = "hotkey"          # hotkey | search
# Hotkey pressed again while the popup is open: "hide" closes the
# popup, "last" switches to the previously used window.
hotkey_action = "last"   # hide | last
# Row order: "key" keeps rows in letter order so they never move, "app"
# groups them by app name, "recently" puts the last used window first.
sort_by = "key"          # key | app | recently
layout = "vertical_list"
keys = "asdfhjklgqwertyuiop"

[appearance]
font = "auto"            # Font file path, or auto
font_size = 14
width = 480
max_rows = 19
# Row text: {app} is the app name, {title} the window title, for
# example "{app} : {title}".
title_format = "{title}"
# Drop " — <app name>" from the end of window titles.
strip_app_name = true
```

`desktop` selects the window source adapter. `auto` picks `kde` when
the colon-separated `XDG_CURRENT_DESKTOP` list contains `KDE`, and fails
otherwise; it never selects `mock`.

`hotkey_action` sets what pressing the hotkey (or `tynex show`) does
while the popup is open, in either mode: `hide` closes it, `last`
switches to the previously used window, or closes it when there is
none.

`appearance.title_format` sets the text of each row. `{app}` is the
app name (see [ADR-0045](../adr/0045_app_names.md)), `{title}` the
window title; other text is kept as is, for example
`"{app} : {title}"` gives `Cursor : My homelab project`. The result
is sanitized like a title, so an empty app name leaves no leading
space.

`appearance.strip_app_name` drops the app name from the end of a
title, after ` — `, ` – ` or ` - `: `Café — Firefox` becomes `Café`.
The end must equal the app name or the last part of the app id
(`konsole` for `org.kde.konsole`), ignoring case. A title that would
become empty stays as it is.

`sort_by` sets the order of the rows: `key` by letter, `app` by app
name, `recently` by last use. The popup always opens with the previous
window selected (see [core.md](core.md#row-order)).

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

| Key                         | Rule                                      |
| --------------------------- | ----------------------------------------- |
| `desktop`                   | `auto`, `kde` or `mock`                   |
| `mode`                      | `hotkey` or `search`                      |
| `hotkey_action`             | `hide` or `last`                          |
| `sort_by`                   | `key`, `app` or `recently`                |
| `layout`                    | `vertical_list`                           |
| `keys`                      | 1 to 26 unique ASCII lowercase letters    |
| `appearance.font`           | `auto` or a path to an existing font file |
| `appearance.font_size`      | 8 to 72                                   |
| `appearance.width`          | 200 to 2000                               |
| `appearance.max_rows`       | 1 to 50                                   |
| `appearance.title_format`   | `{app}` or `{title}`, no other braces     |
| `appearance.strip_app_name` | `true` or `false`                         |

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
  expected one of `desktop`, `mode`, `layout`, `keys`, `appearance`
tynex: error: ~/.config/tynex/tynex.toml:9:13: `appearance.font_size`
  = 120 is out of range, expected 8 to 72
```

## Module boundary

The config module returns a plain `Config` struct or a list of
`ConfigError` values. Other modules get only the sub-struct they need
(for example `AppearanceConfig`), never the whole file.

- `parse` is pure: the font file check is passed in, so tests need no
  disk.
- `load_or_create` returns `Loaded::Existing` or `Loaded::Created`, so
  the daemon can log a new default file.
- `ConfigError` prints `line:col: message`; the daemon adds the file
  path in front, as in the example above.
