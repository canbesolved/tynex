// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

//! Daily counts of `tynex qs` switches and popup calls: one
//! `YYYY-MM-DD qs N show M` line per day, oldest first. Counts wait
//! in `Pending` until the platform writes them. Pure logic; the
//! platform reads the clock and the file.

use crate::core::event::{Action, Event};

/// What is counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Counter {
    Qs,   // A `tynex qs` that switched
    Show, // A popup call: the hotkey or `tynex show`
}

impl Counter {
    /// The counter `event` may add to, before the switcher takes it.
    #[must_use]
    pub fn of(event: &Event) -> Option<Self> {
        match event {
            Event::Show => Some(Self::Show),
            Event::QuickSwitch => Some(Self::Qs),
            _ => None,
        }
    }

    /// True when the switcher's `action` makes the event count: every
    /// popup call, but only a `qs` that switched.
    #[must_use]
    pub fn counts(self, action: &Action) -> bool {
        match self {
            Self::Show => true,
            Self::Qs => {
                matches!(action, Action::Activate(_) | Action::ActivateTab(_))
            }
        }
    }
}

/// The counts of one day.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Day {
    qs: u64,
    show: u64,
}

impl Day {
    /// Reads the counts after the date: `qs N show M`, or `N` from the
    /// first version, which counted only `qs`.
    fn parse(rest: &str) -> Option<Self> {
        let words: Vec<&str> = rest.split_whitespace().collect();

        if let [count] = words[..] {
            return Some(Self {
                qs: count.parse().ok()?,
                show: 0,
            });
        }
        let mut day = Self::default();

        for pair in words.chunks(2) {
            let [name, count] = pair else { return None };
            let count = count.parse().ok()?;

            match *name {
                "qs" => day.qs = count,
                "show" => day.show = count,
                _ => return None,
            }
        }
        Some(day)
    }

    fn add(&mut self, other: Self) {
        self.qs = self.qs.saturating_add(other.qs);
        self.show = self.show.saturating_add(other.show);
    }
}

/// Counts not written yet, per date, oldest first.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Pending {
    days: Vec<(String, Day)>,
}

impl Pending {
    /// Adds one `counter` on `date`.
    pub fn add(&mut self, date: &str, counter: Counter) {
        if self.days.last().is_none_or(|(day, _)| day != date) {
            self.days.push((date.to_owned(), Day::default()));
        }
        if let Some((_, day)) = self.days.last_mut() {
            let count = match counter {
                Counter::Qs => &mut day.qs,
                Counter::Show => &mut day.show,
            };

            *count = count.saturating_add(1);
        }
    }

    /// True when nothing waits to be written.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    /// Returns `text` with the pending counts added and forgets them.
    pub fn apply(&mut self, text: &str) -> String {
        let mut lines: Vec<String> = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_owned)
            .collect();

        for (date, pending) in self.days.drain(..) {
            let today = lines.last().and_then(|line| {
                let (day, rest) = line.split_once(' ')?;

                (day == date).then(|| Day::parse(rest))?
            });

            if today.is_some() {
                lines.pop();
            }
            let mut day = today.unwrap_or_default();

            day.add(pending);
            lines.push(format!("{date} qs {} show {}", day.qs, day.show));
        }

        let mut out = lines.join("\n");

        out.push('\n');
        out
    }
}

/// Formats a civil date as `YYYY-MM-DD`.
#[must_use]
pub fn date(year: i32, month: u32, day: u32) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
#[path = "switch_counter_tests.rs"]
mod tests;
