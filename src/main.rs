// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! tynex: efficient task switcher.

use std::process::ExitCode;

use tynex::cli::{Command, USAGE, parse};
use tynex::ipc::Request;

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
        Ok(Command::Show) => request(Request::Show),
        Ok(Command::Hide) => request(Request::Hide),
        Ok(Command::Quit) => request(Request::Quit),
        Err(message) => {
            eprintln!("tynex: error: {message}\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Sends `req` to the daemon and reports a failure.
fn request(req: Request) -> ExitCode {
    match tynex::platform::linux::dbus_ipc::send(req) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            log::error!("{e}");
            ExitCode::FAILURE
        }
    }
}
