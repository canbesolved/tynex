---
status: "accepted"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# Hint scripts

## Context and Problem Statement

The default hint was one `date` command in `input.hint.text`.
RM-0029 makes the default richer (date, time and battery level),
which is too long for one config line, and users should be able to
edit it. RM-0031 adds ready-made hint scripts for common status
lines. Where do the default hint and these templates live?

## Decision Drivers

- Easy to read and edit, without TOML quoting.
- Never overwrites a user's edits.
- No new crate; the binary stays self-contained.

## Considered Options

- A longer one-line command in `input.hint.text`
- A script in the repository, written to the home folder on start

## Decision Outcome

Chosen option: "A script in the repository, written to the home
folder on start", because the script stays readable and editable
and the config line stays short.

- The scripts are in `extensions/hint_templates/` (bash):
  `default.sh` (date, time, battery), `date_time.sh`, `battery.sh`
  and `system.sh` (CPU load, memory, uptime). They read `/proc` and
  `/sys` only.
- Only `default.sh` is built into the binary, with `include_str!`.
  The other templates stay in the repository; users copy the ones
  they want into the hints folder.
- The default `input.hint.text` is
  `~/.local/share/tynex/hints/default.sh`; `sh -c` expands `~`.
- On start, when `~/.local/share/tynex/hints/` is missing, the
  daemon creates it and writes `default.sh` with mode `0755`.
- When the folder exists, `default.sh` is written only when the hint
  is on, `hint.text` is the default and nothing is at that path.
- An existing file or link is never replaced.
- The folder is under the home folder, not `$XDG_DATA_HOME`, so it
  always matches the default `hint.text`.

### Consequences

- Good, because users edit a plain script, and their edits stay.
- Good, because the binary carries one script, not every template;
  more templates cost nothing at run time.
- Good, because a copied template is one config line away:
  `hint.text = "~/.local/share/tynex/hints/system.sh"`.
- Good, because existing config files keep their own `hint.text`:
  nothing changes for them.
- Good, because the folder is created once, with no work while
  idle.
- Bad, because the templates need `bash` (arrays and
  `printf '%(...)T'`).
- Bad, because the other templates take a manual copy from the
  repository.
- Bad, because a written `default.sh` is not updated when a new
  release changes it; deleting it gets the new one.

### Confirmation

Tests in `src/platform/linux/hint_file_tests.rs`. `shellcheck` runs
on `extensions/hint_templates/*.sh` in CI.

## Pros and Cons of the Options

### A longer one-line command in `input.hint.text`

- Good, because nothing is written outside the config file.
- Bad, because a battery lookup in one TOML string is hard to read
  and to edit.

## More Information

The hint command itself is unchanged
([ADR-0060](0060_input_hint.md)): any shell command still works.
