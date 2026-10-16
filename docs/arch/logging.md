# Logging

Source: `src/logger.rs`. A small logger for the `log` facade, written
in the project instead of `env_logger`
([ADR-0030](../adr/0030_logging.md)), and a panic hook that prints
crashes ([ADR-0032](../adr/0032_crash_output.md)).

## Levels

Set with the `TYNEX_LOG` environment variable. Default: `warn`.

| Level   | Use                                                  |
| ------- | ---------------------------------------------------- |
| `error` | The daemon cannot continue, for example a bad config |
| `warn`  | Something was skipped or fixed, the daemon continues |
| `info`  | Startup steps, selected adapter and presenter        |
| `debug` | Events, IPC calls, adapter updates                   |
| `trace` | Every key press and redraw; also libraries' records  |

```bash
TYNEX_LOG=debug tynex daemon
```

Records from other crates are shown only at `warn` and above, unless
the level is `trace`.

## Format

One line per record on stderr, no timestamps or colors:

```text
tynex: warn: created default config ~/.config/tynex/tynex.toml
tynex: debug: adapters::kde: window list updated, 7 windows
```

The module path is added at `debug` and `trace`, without the leading
`tynex::`. Under a systemd user unit the journal adds time and
priority.

## Setup

`main.rs` calls `logger::init()` first, before reading the config.
`init` parses `TYNEX_LOG`, sets the maximum level with
`log::set_max_level`, so disabled records cost one comparison, and
registers a static logger. An invalid `TYNEX_LOG` value prints a
warning and keeps `warn`.

## Crashes

`main.rs` installs a panic hook at startup. It does nothing until a
panic. Then it logs the crash at `error` level, like any other
message, and the process aborts.

```text
tynex: error: tynex 0.1.0 crashed
panicked at src/core/switcher.rs:42:18:
index out of bounds: the len is 0 but the index is 2
backtrace:
   0: tynex::core::switcher::Switcher::select
   1: tynex::main
```

Release builds keep symbol names (`strip = "debuginfo"`, about 75 KB),
so frames show function names instead of `<unknown>`. Source lines
need a debug build.

Crashes inside C libraries (`libdbus`, `libxkbcommon`) are signals,
not panics. `systemd-coredump` records them:

```bash
coredumpctl info tynex
```
