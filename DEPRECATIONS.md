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
- Hotkey mode with letter keys, the `mode` and `keys` config keys and
  `tynex show --mode`: added in 0.1.0, removed in 0.1.6 (**Breaking**).
  Prompt mode is the only mode; `title_bar` shows the window list
  ([ADR-0050](docs/adr/0050_search_only.md)).
- `sort_by = "key"`, rows in letter order: added in 0.1.4, removed in
  0.1.6 (**Breaking**). `sort_by` defaults to `recently`.
- `layout` config key (`vertical_list`): added in 0.1.0, removed in
  0.1.7 (**Breaking**). The popup is UI modules
  ([ADR-0052](docs/adr/0052_ui_modules.md)).
- `[appearance]` and the top-level keys `desktop`, `hotkey_action`
  and `sort_by`: added in 0.1.0, removed in 0.2.0 (**Breaking**).
  Replaced by `[main]`, `[hotkeys]`, `[output_bar]` and
  `[title_bar]`. `show_app` becomes `title_bar.show_app_icon`
  ([ADR-0055](docs/adr/0055_config_sections.md)).
- `title_bar.show_app_icon`, the app name column: added in 0.2.0,
  removed in 0.2.3 (**Breaking**). Replaced by `title_bar.show_icons`
  ([ADR-0058](docs/adr/0058_title_bar_row.md)).
- `title_bar.enabled` (added in 0.2.0) and `icon_bar.enabled` (added
  in 0.2.9): removed in 0.2.10 (**Breaking**). Replaced by
  `taskbar.mode` ([ADR-0065](docs/adr/0065_taskbar.md)).
- `[input_bar]` and `[output_bar]` (added in 0.2.0), `[title_bar]`
  (added in 0.2.0) and `[icon_bar]` (added in 0.2.9): removed in
  0.2.11 (**Breaking**). Replaced by `[input]`, `[output]` and
  `taskbar.titlebar` / `taskbar.iconbar`
  ([ADR-0066](docs/adr/0066_short_sections.md)).
- `taskbar.mode` values `icons` and `titles`: added in 0.2.10,
  removed in 0.2.11 (**Breaking**). Replaced by `iconbar` and
  `titlebar` ([ADR-0066](docs/adr/0066_short_sections.md)).

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
  prompt mode is to be the only mode.
- Key badge background and text colors in the theme options. Planned
  for Later, dropped on 2026-10-27: `title_bar` rows show no key
  letters.
- Grid and horizontal strip layouts. Planned for Later, dropped on
  2026-10-28: the popup is UI modules, not a choice of layout
  ([ADR-0052](docs/adr/0052_ui_modules.md)).
