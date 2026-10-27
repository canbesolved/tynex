# Deprecations

History of features that were deprecated, removed, or dropped before
they shipped. Current changes are in [CHANGELOG.md](CHANGELOG.md),
planned ones in [ROADMAP.md](ROADMAP.md). Entries are never deleted.

## Deprecated

Features that still work but will be removed in a later release.

None yet.

## Removed

Features that shipped in a release and were removed later.

- `title_format` in `[appearance]`: added in 0.1.2, removed in 0.1.5
  (**Breaking**). Replaced by `show_app`
  ([ADR-0049](docs/adr/0049_popup_columns.md)).

## Never shipped

Features that were planned or designed, then dropped before a release.

- Keys reassigned in most recently used order: the first letter always
  went to the most recent window. Designed for 0.1.0, dropped on
  2026-10-17 for keys pinned until the window closes
  ([ADR-0015](docs/adr/0015_key_order.md), superseded by
  [ADR-0041](docs/adr/0041_pinned_keys.md)).
- Default key order `keys = "asdfjkl;qweruipghty"`, with `;` as a key.
  Planned for 0.1.7, dropped on 2026-10-27: hotkey mode is to be
  removed.
- Per-app keys: `key = "w"` in `[apps.<app_id>]` reserved a letter for
  the app's window. Planned for Later, dropped on 2026-10-27 for the
  same reason.
- More switching modes. Planned for Later, dropped on 2026-10-27:
  search mode is to be the only mode.
- Key badge background and text colors in the theme options. Planned
  for Later, dropped on 2026-10-27: `title_bar` rows show no key
  letters.
