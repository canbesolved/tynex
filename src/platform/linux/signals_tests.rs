// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Signals arrive on the fd, and children keep default signal handling.
//! One test: handlers are process-wide.

use super::Signals;

/// SIGHUP (1), SIGINT (2) and SIGTERM (15) in a `/proc` signal mask.
const QUIT_BITS: u64 = 0x4003;

/// A hex mask line of the `sh` child, such as `SigBlk`.
fn child_mask(field: &str) -> u64 {
    let out = std::process::Command::new("sh")
        .args(["-c", &format!("grep {field} /proc/self/status")])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    let hex = text.split_whitespace().last().unwrap();

    u64::from_str_radix(hex, 16).unwrap()
}

#[test]
fn signals_reach_the_fd_and_not_children() {
    let signals = Signals::open().unwrap();

    assert_eq!(signals.take(), None);
    // SAFETY: the handler only writes to the pipe.
    unsafe {
        libc::pthread_kill(libc::pthread_self(), libc::SIGTERM);
    }
    assert_eq!(signals.take(), Some("SIGTERM"));
    assert_eq!(signals.take(), None);
    assert_eq!(child_mask("SigBlk") & QUIT_BITS, 0);
    assert_eq!(child_mask("SigIgn") & QUIT_BITS, 0);
}
