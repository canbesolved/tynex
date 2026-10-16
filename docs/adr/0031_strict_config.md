---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBeSolved"
---
# Config defaults and validation

## Context and Problem Statement

The config is a TOML file ([ADR-0013](0013_toml_config.md)). Users set
only the keys they care about, and a mistake in the file must not go
unnoticed. How should the daemon treat missing keys and errors?

## Decision Drivers

- Simple rules, easy to explain and to implement.
- Every mistake is visible at once.
- A new release with new keys works with an old file.

## Considered Options

- Defaults for missing keys, exit on any error
- All keys required, exit on any error
- Defaults for missing keys, warnings for errors

## Decision Outcome

Chosen option: "Defaults for missing keys, exit on any error", because
a key the user never wrote has a safe default, while a key the user
wrote wrong must be fixed, not silently ignored.

- **Missing key or table:** the built-in default is used.
- **Syntax error, unknown key, wrong type or invalid value:** a
  message names the file, line, key and what is expected, and the
  daemon exits with code 78 (`EX_CONFIG`).
- **Key renamed or removed in a release:** it is an unknown key, so
  the daemon exits; the message points to the release notes. The file
  is never changed by `tynex`.
- **No file:** the daemon writes the default file, logs its path, and
  continues.

### Consequences

- Good, because typos such as `adaptr` are caught immediately.
- Good, because a release that adds a key works with existing files.
- Good, because the file holds only what the user wrote.
- Bad, because a release that renames or removes a key stops the
  daemon until the user edits the file. Rule: the release notes
  (`CHANGELOG.md`) must list such changes, see `AGENTS.md`.

### Confirmation

Unit tests in `src/config/` cover defaults for missing keys, each
error class, and the default file written on first start.

## More Information

See [config.md](../arch/config.md).
