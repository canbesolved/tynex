// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! tynex: efficient task switcher.

use std::process::ExitCode;

use tynex::cli::{Command, USAGE, parse};

/// Program entry point.
fn main() -> ExitCode {
    tynex::logger::init();
    tynex::logger::install_panic_hook();

    match parse(std::env::args_os().skip(1)) {
        Ok(Command::Help) => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Ok(Command::Version) => {
            println!("tynex {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(Command::Daemon) => tynex::daemon::run(),
        Ok(Command::Show(_) | Command::Hide | Command::Quit) => {
            log::error!("cannot reach the daemon: IPC is not available yet");
            ExitCode::FAILURE
        }
        Err(message) => {
            eprintln!("tynex: error: {message}\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}
