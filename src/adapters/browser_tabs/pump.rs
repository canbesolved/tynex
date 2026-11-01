// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Host poll loop over stdin, the daemon socket and the folder watch.

use std::io::{self, Write};
use std::os::fd::AsFd;
use std::path::Path;
use std::time::Duration;

use rustix::event::{PollFd, PollFlags, Timespec, poll};
use rustix::fd::OwnedFd;
use rustix::fs::{OFlags, fcntl_getfl, fcntl_setfl};
use rustix::io::Errno;

use super::host::Relay;
use super::watch;

/// First and last retry delay when inotify is missing.
const RETRY: (Duration, Duration) =
    (Duration::from_secs(1), Duration::from_secs(60));

/// What woke the loop.
#[derive(Default)]
struct Ready {
    sock: bool,
    watch: bool,
    timeout: bool,
}

/// Runs until the browser closes stdin: `0`, or `1` on a bad frame.
pub fn run(dir: &Path) -> u8 {
    let mut relay = Relay::new(dir);
    let watch = watch::open(dir); // Before the first connect: no gap
    let mut retry = RETRY.0;
    let stdin = io::stdin();

    if watch.is_none() {
        log::info!("browser tabs: no inotify, retrying on a timer");
    }
    if let Ok(flags) = fcntl_getfl(stdin.as_fd()) {
        let _ = fcntl_setfl(stdin.as_fd(), flags | OFlags::NONBLOCK);
    }
    if !relay.try_connect() {
        log::info!("browser tabs: host waiting for the daemon");
        relay.status(false);
    }

    loop {
        let out = std::mem::take(&mut relay.to_browser);

        if io::stdout()
            .write_all(&out)
            .and_then(|()| io::stdout().flush())
            .is_err()
        {
            log::info!("browser tabs: host stdout closed, exiting");
            return 0;
        }
        let ready = wait(&relay, watch.as_ref(), retry);

        match read_stdin(&mut relay) {
            Some(code) => return code,
            None if ready.sock => relay.on_socket(),
            None => {}
        }
        relay.flush();
        if let Some(fd) = watch.as_ref().filter(|_| ready.watch) {
            watch::drain(fd);
        }
        if relay.socket().is_none() {
            if relay.try_connect() {
                retry = RETRY.0;
            } else if ready.timeout {
                retry = (retry * 2).min(RETRY.1);
            }
        }
    }
}

/// Blocks until a descriptor is ready. The timer runs only while
/// disconnected without a watch.
fn wait(relay: &Relay, watch: Option<&OwnedFd>, retry: Duration) -> Ready {
    let stdin = io::stdin();
    let sock = relay.socket();
    let timer = sock.is_none() && watch.is_none();
    let timeout = timer.then(|| Timespec::try_from(retry).ok()).flatten();
    let sock_flags = if relay.wants_write() {
        PollFlags::IN | PollFlags::OUT
    } else {
        PollFlags::IN
    };
    let mut fds = vec![PollFd::new(&stdin, PollFlags::IN)];

    if let Some(sock) = sock {
        fds.push(PollFd::new(sock, sock_flags));
    }
    if let Some(fd) = watch {
        fds.push(PollFd::new(fd, PollFlags::IN));
    }
    let woke = poll(&mut fds, timeout.as_ref()).unwrap_or(0);
    let watch_at = 1 + usize::from(sock.is_some());

    Ready {
        sock: sock.is_some() && !fds[1].revents().is_empty(),
        watch: watch.is_some() && !fds[watch_at].revents().is_empty(),
        timeout: timer && woke == 0,
    }
}

/// Feeds stdin to the relay. `Some` is the exit code.
///
/// Reads fd 0 directly: `std`'s buffered stdin could hide bytes from
/// `poll`.
fn read_stdin(relay: &mut Relay) -> Option<u8> {
    let stdin = io::stdin();
    let mut buf = [0u8; 8192];

    loop {
        match rustix::io::read(stdin.as_fd(), &mut buf) {
            Ok(0) => {
                log::info!("browser tabs: browser closed the host, exiting");
                return Some(0);
            }
            Ok(n) => {
                if relay.on_stdin(&buf[..n]).is_err() {
                    log::error!("browser tabs: bad frame from the browser");
                    return Some(1);
                }
            }
            Err(Errno::AGAIN) => return None,
            Err(Errno::INTR) => {}
            Err(e) => {
                log::error!("browser tabs: host stdin: {e}");
                return Some(1);
            }
        }
    }
}
