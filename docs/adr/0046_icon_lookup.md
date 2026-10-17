---
status: "accepted"
date: "2026-10-17"
decision-makers: "CanBeSolved"
---
# Icon theme lookup

## Context and Problem Statement

Rows show app icons. A desktop file names the icon (`Icon=firefox`),
and the file lives in an icon theme: the user's theme (Breeze on
Plasma), its parents, then `hicolor`, in size directories described
by each theme's `index.theme`. Decoding is settled by
[ADR-0025](0025_rendering.md) (`resvg`, `tiny-skia` PNG). How does
`tynex` find the file, and which theme is the user's?

## Decision Drivers

- Footprint: few crates, no directory scans held in memory, no work
  while idle.
- Icons that match the Plasma setting, including Breeze Dark.
- The renderer stays portable: no OS types in `ui/`.

## Considered Options

- Own lookup in `platform/linux/icon_theme.rs`
- `freedesktop-icons` crate
- `linicon` crate

## Decision Outcome

Chosen option: "Own lookup in `platform/linux/icon_theme.rs`", because
the Icon Theme spec lookup is about 150 lines on top of the key file
reader that desktop files already need, and it adds no crate.

- Theme: `[Icons] Theme` from `kdeglobals` in the XDG config dirs,
  else `breeze`, the Plasma default.
- Chain: the theme, its `Inherits` depth first, `hicolor` last, then
  the `pixmaps` data dirs. Each theme loads once, on the first lookup.
- Size: the directory whose size range (`Fixed`, `Scalable`,
  `Threshold`, times `Scale`) is closest to the row icon size in
  physical pixels; PNG before SVG in one directory.
- Files are found with `stat` calls; directory listings are not
  cached, so Breeze's tens of thousands of names never sit in RAM.
- `ui/render/icons.rs` asks through an `IconLookup` trait, like font
  fallback, and caches the decoded image per icon name and size,
  misses included.
- Without a desktop file, the app id is tried as icon name.

### Consequences

- Good, because no new crate beyond `resvg` from ADR-0025.
- Good, because later platforms plug in their own `IconLookup`.
- Neutral, because the first lookup reads the theme indexes (about
  50 ms with Breeze) and a miss costs about 10 ms; both happen once
  per app, when its window arrives, not when the popup opens.
- Bad, because a changed icon theme shows after a daemon restart.
- Bad, because desktops other than Plasma get `breeze` or `hicolor`
  until their own setting is read.

### Confirmation

Unit tests in `icon_theme_tests.rs` build made-up themes and cover
size choice, inheritance, `hicolor`, `pixmaps` and refused names.
`cargo tree` shows no crate besides `resvg` and its dependencies.

## Pros and Cons of the Options

### `freedesktop-icons` crate

- Good, because it implements the spec and caches themes.
- Bad, because it adds `dirs`, `ini_core`, `once_cell`, `thiserror`,
  `tracing` and `xdg` for one lookup.
- Bad, because it can read the GTK theme setting, not the Plasma
  one.

### `linicon` crate

- Good, because it can list a theme's icons.
- Bad, because it is MPL-2.0, which is not in the license allow list
  of `deny.toml`.

## More Information

Revisit for the launcher (see [ROADMAP.md](../../ROADMAP.md)), which
also needs icons of apps without windows.
