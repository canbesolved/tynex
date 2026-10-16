// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Stderr logger for the `log` facade, and crash output with a
//! backtrace. Level from `TYNEX_LOG`, default `warn`.

use std::backtrace::Backtrace;
use std::io::Write;

use log::{Level, LevelFilter, Log, Metadata, Record};

/// The only logger; its level lives in `log::max_level()`.
static LOGGER: StderrLogger = StderrLogger;

struct StderrLogger;

impl Log for StderrLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        allowed(metadata.target(), metadata.level(), log::max_level())
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let module = record.module_path().unwrap_or(record.target());
            let line =
                format_line(record.level(), module, &record.args().to_string());

            // One write per record; a broken stderr is ignored.
            let _ = writeln!(std::io::stderr().lock(), "{line}");
        }
    }

    fn flush(&self) {}
}

/// Registers the logger with the level from `TYNEX_LOG`.
///
/// An invalid value logs a warning and keeps `warn`. Call once, first.
pub fn init() {
    let value = std::env::var("TYNEX_LOG").ok();
    let level = value
        .as_deref()
        .map_or(Some(LevelFilter::Warn), parse_level);

    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(level.unwrap_or(LevelFilter::Warn));
    }
    if level.is_none() {
        log::warn!(
            "invalid TYNEX_LOG value {:?}, expected error, warn, info, debug \
             or trace; using warn",
            value.unwrap_or_default()
        );
    }
}

/// Logs panics at `error` level with the version and a backtrace.
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::force_capture();

        log::error!(
            "tynex {} crashed\n{info}\nbacktrace:\n{backtrace}",
            env!("CARGO_PKG_VERSION")
        );
    }));
}

/// Parses a `TYNEX_LOG` value, ignoring case and surrounding spaces.
fn parse_level(value: &str) -> Option<LevelFilter> {
    match value.trim().to_ascii_lowercase().as_str() {
        "error" => Some(LevelFilter::Error),
        "warn" => Some(LevelFilter::Warn),
        "info" => Some(LevelFilter::Info),
        "debug" => Some(LevelFilter::Debug),
        "trace" => Some(LevelFilter::Trace),
        _ => None,
    }
}

/// Records from `tynex` pass at `max`; other crates only at `warn` and
/// above, unless `max` is `trace`.
fn allowed(target: &str, level: Level, max: LevelFilter) -> bool {
    let ours = target == "tynex" || target.starts_with("tynex::");

    level <= max && (ours || level <= Level::Warn || max == LevelFilter::Trace)
}

/// Formats one record; the module path is added at `debug` and `trace`.
fn format_line(level: Level, module: &str, message: &str) -> String {
    let level_name = level.as_str().to_ascii_lowercase();

    if level <= Level::Info {
        return format!("tynex: {level_name}: {message}");
    }

    let module = module.strip_prefix("tynex::").unwrap_or(module);

    format!("tynex: {level_name}: {module}: {message}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_parse_case_insensitively() {
        assert_eq!(parse_level("debug"), Some(LevelFilter::Debug));
        assert_eq!(parse_level(" TRACE "), Some(LevelFilter::Trace));
        assert_eq!(parse_level("verbose"), None);
        assert_eq!(parse_level(""), None);
    }

    #[test]
    fn other_crates_only_warn_unless_trace() {
        let debug = LevelFilter::Debug;

        assert!(allowed("tynex::core", Level::Debug, debug));
        assert!(!allowed("tynex::core", Level::Trace, debug));
        assert!(!allowed("wayland_client", Level::Debug, debug));
        assert!(allowed("wayland_client", Level::Warn, debug));
        assert!(allowed("wayland_client", Level::Debug, LevelFilter::Trace));
        assert!(!allowed("tynexus", Level::Debug, debug)); // Not our crate
    }

    #[test]
    fn lines_match_the_documented_format() {
        assert_eq!(
            format_line(Level::Warn, "tynex::config", "created default config"),
            "tynex: warn: created default config"
        );
        assert_eq!(
            format_line(Level::Debug, "tynex::adapters::kde", "7 windows"),
            "tynex: debug: adapters::kde: 7 windows"
        );
        assert_eq!(
            format_line(Level::Trace, "wayland_client::conn", "event"),
            "tynex: trace: wayland_client::conn: event"
        );
    }
}
