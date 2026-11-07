---
status: "accepted"
date: "2026-11-07"
decision-makers: "CanBeSolved"
---
# Switch counter

## Context and Problem Statement

How often is the quick switch used compared with the popup? A daily
count of both answers it, kept on disk across restarts. Usage stats
(ADR-0095) already keep a file in `~/.local/share/tynex/`. Where do
the counts live, when are they written, and is counting on by
default?

## Decision Drivers

- No idle work, no timers, no new crate.
- Usage scores stay exact for ranking.
- A day's counts are easy to read by hand.
- Nothing new is written for users who did not ask for it.

## Considered Options

- A separate counter file, written at most once an hour
- `qs` and `show` items in the usage stats file
- A write on every count

## Decision Outcome

Chosen option: "A separate counter file, written at most once an
hour", because the counts are plain totals per day, while usage
scores are capped at 100 and halved to fade.

- `main.switch_counter`, default `false`. Off, nothing is counted
  and no file is written.
- `~/.local/share/tynex/counter.txt`, one `YYYY-MM-DD qs N show M`
  line per local day, oldest first. The date comes from
  `localtime_r` (`libc`, already a dependency).
- `qs` counts a quick switch that switched, from the KDE shortcut or
  `tynex qs`. `show` counts every popup call, from the hotkey or
  `tynex show`.
- Counts wait in memory, per date. A popup call writes them when the
  last write is at least an hour old by the wall clock, so a suspend
  counts. Quitting writes them too.
- Both files share `data_file.rs`: capped reads, atomic `0600`
  writes.
- The usage stats file is renamed to `stats.tsv`, next to
  `counter.txt` (ADR-0095). The daemon does not move the old file;
  users rename it by hand to keep their scores.

### Consequences

- Good, because a count costs no I/O until the hour is up.
- Good, because usage ranking is untouched.
- Bad, because a killed daemon loses up to an hour of counts.
- Bad, because counts made without a popup call wait until the next
  popup call or quit.

### Confirmation

Tests in `src/core/switch_counter_tests.rs`,
`src/platform/linux/data_file_tests.rs`,
`src/platform/linux/counter_file.rs` and
`src/config/history_tests.rs`.

## Pros and Cons of the Options

### `qs` and `show` items in the usage stats file

- Good, because one file and one parser.
- Bad, because scores stop at 100 and every pass halves all scores,
  so counts break and app ranking fades with each popup call.
- Bad, because one item per day grows the file read on every start.

### A write on every count

- Good, because nothing is lost on a kill.
- Bad, because every hotkey press reads and writes a file.
