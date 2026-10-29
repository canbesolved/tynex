// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Hint command: spawn without waiting, then read stdout on exit.

use std::os::fd::{AsFd, BorrowedFd};
use std::time::{Duration, Instant};

use super::hint_proc::{self, Running};
use crate::config::HintConfig;

/// Dot animation step while the command runs.
const TICK: Duration = Duration::from_millis(400);

const FRAMES: [&str; 4] = [".", "..", "...", ""];

/// The hint command for one daemon. At most one child runs.
pub struct Hint {
    command: String,
    update: Duration,
    last: String, // Last successful stdout
    step: u32,
    tick: Option<Instant>,   // Next animation frame
    repeat: Option<Instant>, // Next spawn, measured from exit
    reap: Option<Instant>,   // Retry after stdout closed early
    child: Option<Running>,
}

impl Hint {
    /// Builds a hint runner from the config. A disabled hint or an
    /// empty command never spawns.
    #[must_use]
    pub fn new(config: &HintConfig) -> Self {
        let command = if config.enabled {
            config.text.clone()
        } else {
            String::new()
        };

        Self {
            command,
            update: Duration::from_secs(u64::from(config.update)),
            last: String::new(),
            step: 0,
            tick: None,
            repeat: None,
            reap: None,
            child: None,
        }
    }

    /// Starts a command, or resumes the one still running.
    ///
    /// Returns the string to draw. The popup must not wait on this.
    pub fn on_show(&mut self, now: Instant) -> String {
        self.repeat = None;
        if self.command.is_empty() {
            return String::new();
        }
        if self.child.is_some() {
            self.tick = Some(now + TICK);
            return frame(self.step).to_owned();
        }
        self.start(now)
    }

    /// Drops animation and repeat waits. A running child is left alone.
    pub fn on_hide(&mut self) {
        self.tick = None;
        self.repeat = None;
    }

    /// Returns when the loop should wake for the animation or a repeat.
    #[must_use]
    pub fn deadline(&self) -> Option<Instant> {
        [self.tick, self.repeat, self.reap]
            .into_iter()
            .flatten()
            .min()
    }

    /// Returns the child's stdout, if a command is running.
    #[must_use]
    pub fn stdout_fd(&self) -> Option<BorrowedFd<'_>> {
        self.child.as_ref().map(|run| run.stdout.as_fd())
    }

    /// Reads the child and advances the animation.
    ///
    /// `Some` is the new hint string. Stdout from a child that exits
    /// while `open` is false is discarded. A failure keeps `last`.
    pub fn poll(
        &mut self,
        now: Instant,
        open: bool,
        query_empty: bool,
    ) -> Option<String> {
        let finished = self.take_exit(now, open);

        self.arm_tick(now, open, query_empty);
        if finished.is_some() {
            return finished;
        }
        if self.repeat.is_some_and(|at| at <= now) && self.child.is_none() {
            self.repeat = None;
            if open {
                return Some(self.start(now));
            }
        }
        self.advance(now)
    }

    /// Spawns `command`. The caller draws before the child exits.
    fn start(&mut self, now: Instant) -> String {
        match hint_proc::spawn(&self.command) {
            Ok(run) => {
                self.child = Some(run);
                self.step = 0;
                self.tick = Some(now + TICK);
                frame(0).to_owned()
            }
            Err(e) => {
                log::warn!("cannot start hint command: {e}");
                self.tick = None;
                self.last.clone()
            }
        }
    }

    /// Reaps the child when it has exited.
    fn take_exit(&mut self, now: Instant, open: bool) -> Option<String> {
        let eof = self.drain_child();
        let status = match self.child.as_mut()?.child.try_wait() {
            Ok(status) => status,
            Err(e) => {
                log::warn!("cannot wait on hint command: {e}");
                self.child = None;
                self.tick = None;
                self.reap = None;
                return open.then(|| self.last.clone());
            }
        };
        let Some(status) = status else {
            // Stdout can close before the process is waitable.
            self.reap = eof.then(|| now + Duration::from_millis(10));
            return None;
        };
        if let Some(run) = self.child.as_mut() {
            let _ = hint_proc::drain(&mut run.stdout, &mut run.buf);
        }
        let run = self.child.take()?;

        self.tick = None;
        self.reap = None;
        if !open {
            return None;
        }
        if !status.success() {
            log::warn!("hint command exited: {status}");
            return Some(self.last.clone());
        }
        let line = hint_proc::first_line(&run.buf);

        self.last.clone_from(&line);
        let secs = u32::try_from(self.update.as_secs()).unwrap_or(0);

        self.repeat = next_update(secs, now);
        Some(line)
    }

    /// Reads pending stdout. `true` means the pipe is at EOF.
    fn drain_child(&mut self) -> bool {
        let Some(run) = self.child.as_mut() else {
            return false;
        };

        hint_proc::drain(&mut run.stdout, &mut run.buf).unwrap_or_else(|e| {
            log::warn!("cannot read hint command: {e}");
            false
        })
    }

    /// Arms the animation only while the popup shows an empty query.
    fn arm_tick(&mut self, now: Instant, open: bool, query_empty: bool) {
        if self.child.is_none() || !open || !query_empty {
            self.tick = None;
            return;
        }
        if self.tick.is_none() {
            self.tick = Some(now + TICK);
        }
    }

    /// Moves to the next dot frame when the tick is due.
    fn advance(&mut self, now: Instant) -> Option<String> {
        if self.tick.is_none_or(|at| at > now) {
            return None;
        }
        self.step = self.step.wrapping_add(1);
        self.tick = Some(now + TICK);
        Some(frame(self.step).to_owned())
    }
}

/// Returns the loading frame for `step`: `.`, `..`, `...`, then empty.
#[must_use]
pub(crate) fn frame(step: u32) -> &'static str {
    FRAMES[step as usize % FRAMES.len()]
}

/// Returns when the next run may start. `0` does not repeat.
#[must_use]
pub(crate) fn next_update(secs: u32, exit: Instant) -> Option<Instant> {
    (secs > 0).then(|| exit + Duration::from_secs(u64::from(secs)))
}

#[cfg(test)]
#[path = "hint_tests.rs"]
mod tests;
