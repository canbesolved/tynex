# Browser tabs

Source: `src/modules/browser_tabs/`, `src/core/browser_tabs.rs`.
Protocol and transport:
[adapters/browser_tabs.md](../adapters/browser_tabs.md).
Decision: [ADR-0081](../../adr/0081_tab_bridge.md).

## Goal

While a query is typed, open Brave, Chrome and Chromium tabs join
the output bar after window matches and before launcher rows. `Enter` switches
to that tab. With `[browser_tabs] enabled = false` prompt mode has no
tab session and the popup is unchanged.

## Files

```text
modules/browser_tabs/
  mod.rs    Session: applies TabsUpdate, the pending activation
  list.rs   Cleaning, row titles, search order, fallback icon
  pick.rs   browser_window: the KWin window to raise
  tests.rs  Unit tests
  hidden_tests.rs  Tests for a tab opened by `tynex qs`
```

## What the module owns

Pure logic, no I/O:

- The tab lists of the open popup, one list per connected browser.
  They live on `State` and die with the popup.
- Row titles and search order.
- The pending activation, which outlives the popup until the
  browser answers.
- The pick of the KWin window to raise.

`Session::apply` takes one `TabsUpdate`:

| Update      | Effect                                              |
| ----------- | --------------------------------------------------- |
| `List`      | Cleans and stores the list and the browser's window |
|             | ids, drops scores of that browser's closed tabs;    |
|             | ignored while hidden                                |
| `Gone`      | Drops that source's list and pending activation     |
| `Icons`     | Redraw while the popup is open                      |
| `Activated` | `Activate` the picked window, or nothing            |

After `List` or `Gone`, prompt mode refilters the output and keeps
the selected row. A late reply is also dropped by the adapter.

## Config

```toml
[browser_tabs]
enabled = false   # true: register hosts, open the socket
browsers = "auto" # "auto", a name or a list: brave, chrome, chromium
```

`enabled` defaults to `false`, because `true` writes into the
browsers' config folders. `browsers` is `"auto"` (every installed
supported browser), one name, or a list of `brave`, `chrome` and
`chromium`; `"brave"` equals `["brave"]`.
Each name covers that browser's release channels, such as Brave
Origin or Chrome Beta. An unknown name is a config error. The
socket opens only when a selected browser is installed. The allowed
extension id is built in, not a config key.

## Cleaning

`clean` keeps the 10 000 tabs with the newest `last`, then sanitizes
titles and URLs like window titles and cuts them to 256 and 1024
chars. Sorting first means dropped tabs are never cleaned.

## Rows

Tab rows appear only in the output bar, and only while the query
is not empty. Order: window matches, then tab matches, then
launcher rows ([ADR-0078](../../adr/0078_windows_before_apps.md)),
with rows that contain the query as typed above fuzzy ones
([ADR-0085](../../adr/0085_exact_first.md)).

The row title is `<title> — <host>`. `<host>` is the URL's host
without a leading `www.`. A URL with no host uses its scheme. When
the title is the URL, the row is just the URL. Search matches that
title, so the host is searchable. The path is not matched.

Among tabs: a title that starts with the query comes first
([ADR-0079](../../adr/0079_leading_match.md)), then score, then
`last` descending.

A window row and its active tab row are not both listed: the tab
row stays and the window row is dropped
([ADR-0093](../../adr/0093_tab_dedupe.md)). `List` stores the ids of
the windows whose pid is the browser's. A window is dropped when its
id is in that set, an active tab row of that list is in the results,
and the window title is the tab title, alone or before a separator
(`Notes - Brave`). The join in
[modules/input.md](input.md) applies it.

## Activation

`Enter` on a tab row returns `Action::ActivateTab`. Prompt mode
notes the pending activation before the popup state is dropped:
source, pid, the source's list and the tab id. A new one replaces
it. There is no timer. The event loop hides the popup and the
adapter sends `activate`.

`tynex qs` while hidden has no list: `note_hidden` keeps the source,
pid, tab id and the window that last showed the tab
([ADR-0098](../../adr/0098_quick_switch.md)).

When the browser reports success, the noted window is raised if it
is still open; otherwise `browser_window` picks the KWin window. The
module returns `Action::Activate` for it. On failure
nothing is raised. A reply from another source is ignored.

## Window pick

`browser_window` in `pick.rs`:

1. Candidates are cached windows whose `pid` equals the browser's
   pid, in most-recently-used order. None: return nothing.
2. One candidate: return it.
3. Titles to look for: the tab that was active in the browser
   window, and the tab that was activated. Drop empty ones.
4. Return the first candidate whose title starts with one of them.
   None: return nothing.

The extension has already focused the window, so a miss leaves that
result in place. The pid comes from the host
([ADR-0082](../../adr/0082_kwin_pid.md)).

## Icon

The row uses the favicon when the renderer has it. Otherwise it
uses the fallback chosen when the list arrived: the icon of a
cached window with that pid, else the browser's theme icon
(`brave-browser`, `google-chrome`), else the lowercased browser
name with dashes.

## Tests

- Lists are kept only while the popup is visible; a new list from a
  source replaces the old one; `Gone` drops it.
- A list keeps the window ids of its browser's pid.
- Cleaning keeps the newest tabs and caps titles.
- The fallback icon prefers a window of the browser.
- Row titles, hosts and search order.
- The activation uses the noted list, once; a failed reply raises
  nothing.
- Every branch of `browser_window`: no pid, one candidate, the old
  title, the target title, a `<2>` suffix, and no title match.
