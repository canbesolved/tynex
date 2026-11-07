// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! `~/.local/share/tynex/counter.txt`: `tynex qs` switches and
//! popup calls per local day. Counts wait in memory and are written
//! on a popup call at most once an hour, and on quit.

use std::time::{Duration, SystemTime};

use crate::core::event::Action;
use crate::core::switch_counter::{self, Counter, Pending};
use crate::platform::linux::data_file;

/// File name.
const FILE: &str = "counter.txt";

/// Shortest time between two writes. Wall clock time, so a suspend
/// counts.
const EVERY: Duration = Duration::from_secs(60 * 60);

/// Counts not written yet and when the file was last written.
pub struct SwitchCounter {
    on: bool, // `main.switch_counter`; off counts nothing
    pending: Pending,
    written: SystemTime,
}

impl SwitchCounter {
    /// Starts the hour now, so the first write is an hour away. Off,
    /// it counts and writes nothing.
    #[must_use]
    pub fn new(on: bool) -> Self {
        Self {
            on,
            pending: Pending::default(),
            written: SystemTime::now(),
        }
    }

    /// Counts `counter` if `action` makes it count. A popup call
    /// writes the file when the last write is an hour old.
    pub fn note(&mut self, counter: Option<Counter>, action: &Action) {
        let Some(counter) = counter.filter(|c| self.on && c.counts(action))
        else {
            return;
        };

        if let Some(date) = today() {
            self.pending.add(&date, counter);
        }
        let due = !self.written.elapsed().is_ok_and(|age| age < EVERY);

        if counter == Counter::Show && due {
            self.flush();
        }
    }

    /// Writes the pending counts, if any. Problems are logged and the
    /// counts are dropped, so a broken file never grows memory.
    pub fn flush(&mut self) {
        self.written = SystemTime::now();
        if self.pending.is_empty() {
            return;
        }
        match data_file::read(FILE) {
            Ok(old) => {
                let text = self.pending.apply(&old);

                if let Err(e) = data_file::write(FILE, &text) {
                    log::warn!("cannot write the qs counter: {e}");
                }
            }
            Err(e) => {
                log::warn!("cannot read the qs counter: {e}");
                self.pending = Pending::default();
            }
        }
    }
}

/// Today's date in the local time zone, `YYYY-MM-DD`.
fn today() -> Option<String> {
    let mut tm = std::mem::MaybeUninit::<libc::tm>::uninit();

    // SAFETY: `time` accepts a null pointer. `localtime_r` writes only
    // to `tm` and fills it when it returns non-null.
    let tm = unsafe {
        let now = libc::time(std::ptr::null_mut());

        if libc::localtime_r(&raw const now, tm.as_mut_ptr()).is_null() {
            return None;
        }
        tm.assume_init()
    };

    Some(switch_counter::date(
        tm.tm_year + 1900,
        u32::try_from(tm.tm_mon + 1).ok()?,
        u32::try_from(tm.tm_mday).ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn today_is_a_date() {
        let date = today().unwrap();

        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
    }

    #[test]
    fn counts_wait_for_the_hour() {
        let mut off = SwitchCounter::new(false);

        off.note(Some(Counter::Show), &Action::Redraw);
        assert!(off.pending.is_empty());

        let mut counter = SwitchCounter::new(true);

        counter.note(Some(Counter::Show), &Action::Redraw);
        counter.note(Some(Counter::Qs), &Action::None); // Did not switch
        assert!(!counter.pending.is_empty());
        assert_eq!(counter.pending.clone().apply(""), {
            let date = today().unwrap();

            format!("{date} qs 0 show 1\n")
        });
    }
}
