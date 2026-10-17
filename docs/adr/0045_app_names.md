---
status: "accepted, amended by ADR-0049"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# App names from desktop files

## Context and Problem Statement

Rows need a readable app name for `title_format = "{app} : {title}"`
and to drop the app name from the end of titles. The KWin script only
sends the desktop file id (`desktopFileName`) and the WM class
(`resourceClass`), for example `org.kde.dolphin` and `dolphin`. The
name users know, `Dolphin` or a translation of it, is the `Name` key
of the app's desktop file. How does `tynex` read it?

## Decision Drivers

- Footprint: no new crates without need, no work while idle.
- Localized names that match the desktop's language.
- Reusable by later adapters (wlroots, COSMIC) and for icons (0.1.3).

## Considered Options

- Own reader in `platform/linux/desktop_files.rs`
- `freedesktop-desktop-entry` crate

## Decision Outcome

Chosen option: "Own reader in `platform/linux/desktop_files.rs`",
because only `Name` from one group is needed, which takes about 100
lines with the standard library.

- Lookup: `<app id>.desktop` in `applications/` under the XDG data
  dirs, first match wins.
- Locale: `LC_ALL`, `LC_MESSAGES` or `LANG`, matched as the Desktop
  Entry spec says (`lang_COUNTRY@MODIFIER`, `lang_COUNTRY`,
  `lang@MODIFIER`, `lang`, then untranslated).
- Fallback: the WM class when no file or no `Name` is found.
- Each app id is read once, on its first window; hits and misses are
  cached in memory until the daemon exits.
- The KDE adapter names windows before sending them to the core, so
  the core stays free of I/O.

### Consequences

- Good, because no new crate; the release binary grows by about
  40 KB (5.61 MB to 5.65 MB).
- Good, because later adapters on Linux reuse the same lookup.
- Bad, because the subdirectory rule of desktop file ids
  (`kde-foo.desktop` in `applications/kde/`) is not supported.
- Bad, because a renamed or newly installed app shows its new name
  only after a daemon restart.

### Confirmation

Unit tests in `desktop_files_tests.rs` cover locale order, groups,
escapes, the data dir order and the cache. `cargo tree` shows no new
crate.

## Pros and Cons of the Options

### `freedesktop-desktop-entry` crate

- Good, because it covers the full spec, including actions and
  `Exec` field codes, which a launcher would need.
- Bad, because it adds `bstr`, `memchr`, `thiserror`, `unicase` and
  `xdg`, plus `gettext-rs` with default features, for one key.
- Bad, because it is MPL-2.0, which is not in the license allow list
  of `deny.toml`.

## More Information

Revisit when the launcher (see [ROADMAP.md](../../ROADMAP.md)) needs
`Exec`, `Terminal` and `NoDisplay`.
