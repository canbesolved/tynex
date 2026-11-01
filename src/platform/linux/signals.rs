// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `SIGINT`, `SIGTERM` and `SIGHUP` as a pollable fd, so `Ctrl+C`, `kill`
//! and a closed terminal stop the daemon like `tynex quit`.
//!
//! A handler writes the signal number to a pipe that the poll loop
//! reads: no wakeups while idle. The signal mask is left alone, and
//! `exec` resets handlers, so spawned apps get these signals as usual.

use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::sync::atomic::{AtomicI32, Ordering};

/// Signals that quit the daemon, with their names for the log.
const QUIT: [(libc::c_int, &str); 3] = [
    (libc::SIGINT, "SIGINT"),
    (libc::SIGTERM, "SIGTERM"),
    (libc::SIGHUP, "SIGHUP"),
];

/// Write end of the pipe for the handler; `-1` while none is set.
static PIPE: AtomicI32 = AtomicI32::new(-1);

/// The installed handlers and their pipe. Dropping it restores the
/// previous actions, so a second `Ctrl+C` during shutdown ends the
/// process at once.
pub struct Signals {
    read: OwnedFd,
    _write: OwnedFd, // Kept open for the handler
    old: [libc::sigaction; 3],
}

impl Signals {
    /// Installs the handlers. `None` keeps the default actions, which
    /// end the process without cleanup. One instance at a time.
    #[must_use]
    pub fn open() -> Option<Self> {
        let Some((read, write)) = pipe() else {
            log::warn!("cannot open a signal pipe, Ctrl+C skips cleanup");
            return None;
        };

        PIPE.store(write.as_raw_fd(), Ordering::Relaxed);
        // SAFETY: an all-zero `sigaction` is a valid value to fill in.
        let mut old: [libc::sigaction; 3] = unsafe { std::mem::zeroed() };

        for (slot, (signal, _)) in old.iter_mut().zip(QUIT) {
            if !install(signal, slot) {
                log::warn!("cannot catch quit signals, Ctrl+C skips cleanup");
                PIPE.store(-1, Ordering::Relaxed);
                return None;
            }
        }
        Some(Self {
            read,
            _write: write,
            old,
        })
    }

    /// Reads every pending signal. Returns the name of the last quit
    /// signal, or `None` when nothing arrived.
    #[must_use]
    pub fn take(&self) -> Option<&'static str> {
        let mut buf = [0u8; 16];
        let mut name = None;

        while let Ok(n @ 1..) = rustix::io::read(&self.read, &mut buf) {
            for &byte in &buf[..n] {
                name = QUIT
                    .iter()
                    .find(|(signal, _)| *signal == libc::c_int::from(byte))
                    .map(|(_, text)| *text)
                    .or(name);
            }
        }
        name
    }
}

/// The fd to poll for reading.
impl AsFd for Signals {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.read.as_fd()
    }
}

impl Drop for Signals {
    fn drop(&mut self) {
        for (old, (signal, _)) in self.old.iter().zip(QUIT) {
            // SAFETY: `old` is the action `sigaction` returned for
            // `signal`.
            unsafe {
                libc::sigaction(signal, old, std::ptr::null_mut());
            }
        }
        PIPE.store(-1, Ordering::Relaxed);
    }
}

/// Writes the signal number to the pipe. Async-signal-safe: an atomic
/// load and `write`, with `errno` kept.
extern "C" fn on_signal(signal: libc::c_int) {
    let fd = PIPE.load(Ordering::Relaxed);

    if fd < 0 {
        return;
    }
    let byte = u8::try_from(signal).unwrap_or(0);

    // SAFETY: `errno` is thread-local; `fd` stays open while `PIPE`
    // holds it, and a full pipe makes the write fail, not block.
    unsafe {
        let errno = *libc::__errno_location();

        libc::write(fd, (&raw const byte).cast(), 1);
        *libc::__errno_location() = errno;
    }
}

/// Sets `on_signal` for `signal` and saves the old action in `old`.
fn install(signal: libc::c_int, old: &mut libc::sigaction) -> bool {
    // SAFETY: the zeroed `sigaction` gets a handler, an empty mask and
    // flags before use.
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();

        action.sa_sigaction = on_signal as extern "C" fn(libc::c_int) as usize;
        action.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&raw mut action.sa_mask);
        libc::sigaction(signal, &raw const action, old) == 0
    }
}

/// A non-blocking, close-on-exec pipe: read end, write end.
fn pipe() -> Option<(OwnedFd, OwnedFd)> {
    let mut fds = [0; 2];

    // SAFETY: `fds` has room for both ends.
    let ok = unsafe {
        libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC | libc::O_NONBLOCK)
    };
    if ok != 0 {
        return None;
    }
    // SAFETY: `pipe2` returned two new fds that nothing else owns.
    Some(unsafe {
        (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1]))
    })
}

#[cfg(test)]
#[path = "signals_tests.rs"]
mod tests;
