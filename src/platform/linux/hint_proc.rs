// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Spawn a hint command and read its stdout without blocking.

use std::io::{ErrorKind, Read};
use std::process::{Child, ChildStdout, Command, Stdio};

use rustix::fs::{OFlags, fcntl_getfl, fcntl_setfl};

/// Captured stdout is capped. The hint uses the first line only.
const MAX_OUT: usize = 4096;

/// One running hint command and the bytes read so far.
pub(super) struct Running {
    pub child: Child,
    pub stdout: ChildStdout,
    pub buf: Vec<u8>,
}

impl Drop for Running {
    fn drop(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

/// Returns the first line of `buf`, without trailing whitespace.
pub(super) fn first_line(buf: &[u8]) -> String {
    let text = String::from_utf8_lossy(buf);
    let line = text.lines().next().unwrap_or("");

    line.trim_end().to_owned()
}

/// Spawns `sh -c script` and returns before it exits.
pub(super) fn spawn(script: &str) -> std::io::Result<Running> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(script)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| std::io::Error::other("hint command has no stdout"))?;
    let flags = fcntl_getfl(&stdout)?;

    fcntl_setfl(&stdout, flags | OFlags::NONBLOCK)?;
    Ok(Running {
        child,
        stdout,
        buf: Vec::new(),
    })
}

/// Reads what is available. `true` means stdout reached EOF.
pub(super) fn drain(
    stdout: &mut ChildStdout,
    buf: &mut Vec<u8>,
) -> std::io::Result<bool> {
    let mut tmp = [0_u8; 512];

    loop {
        match stdout.read(&mut tmp) {
            Ok(0) => return Ok(true),
            Ok(n) => {
                let room = MAX_OUT.saturating_sub(buf.len());

                buf.extend_from_slice(&tmp[..n.min(room)]);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => return Ok(false),
            Err(e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
}
