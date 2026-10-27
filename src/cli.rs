// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Command line: `daemon`, `show`, `hide`, `quit`.

use std::ffi::OsString;

use lexopt::prelude::*;

/// Usage text for `--help` and argument errors.
pub const USAGE: &str = "\
Usage: tynex <command>

Commands:
  daemon         Start the background service
  show           Open the switcher
  hide           Close the switcher
  quit           Stop the background service

Options:
  -h, --help     Show this help
  -V, --version  Show the version";

/// What the user asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Daemon,
    Show,
    Hide,
    Quit,
    Help,
    Version,
}

/// Parses the arguments after the program name.
///
/// # Errors
///
/// Returns a message for unknown commands, options or values.
pub fn parse(
    args: impl IntoIterator<Item = OsString>,
) -> Result<Command, String> {
    let mut parser = lexopt::Parser::from_args(args);
    let mut command = None;

    while let Some(arg) = parser.next().map_err(|e| e.to_string())? {
        match arg {
            Short('h') | Long("help") => return Ok(Command::Help),
            Short('V') | Long("version") => return Ok(Command::Version),
            Value(value) if command.is_none() => {
                command = Some(match value.to_str() {
                    Some("daemon") => "daemon",
                    Some("show") => "show",
                    Some("hide") => "hide",
                    Some("quit") => "quit",
                    _ => {
                        return Err(format!(
                            "unknown command `{}`",
                            value.to_string_lossy()
                        ));
                    }
                });
            }
            arg => return Err(arg.unexpected().to_string()),
        }
    }

    match command {
        Some("daemon") => Ok(Command::Daemon),
        Some("show") => Ok(Command::Show),
        Some("hide") => Ok(Command::Hide),
        Some("quit") => Ok(Command::Quit),
        _ => Err("missing command".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(args: &[&str]) -> Result<Command, String> {
        parse(args.iter().map(OsString::from))
    }

    #[test]
    fn commands_parse() {
        assert_eq!(run(&["daemon"]), Ok(Command::Daemon));
        assert_eq!(run(&["show"]), Ok(Command::Show));
        assert_eq!(run(&["hide"]), Ok(Command::Hide));
        assert_eq!(run(&["quit"]), Ok(Command::Quit));
    }

    #[test]
    fn help_and_version_win() {
        assert_eq!(run(&["--help"]), Ok(Command::Help));
        assert_eq!(run(&["show", "-h"]), Ok(Command::Help));
        assert_eq!(run(&["-V"]), Ok(Command::Version));
    }

    #[test]
    fn mistakes_are_errors() {
        assert!(run(&[]).is_err());
        assert!(run(&["start"]).is_err());
        assert!(run(&["show", "--mode", "search"]).is_err()); // Removed
        assert!(run(&["show", "extra"]).is_err());
    }
}
