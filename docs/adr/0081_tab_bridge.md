---
status: "accepted"
date: "2026-11-01"
decision-makers: "CanBeSolved"
---
# Browser tab bridge

## Context and Problem Statement

`ROADMAP.md` plans tab search: a browser extension gives `tynex` the
open tabs through a local bridge, with no polling, and `Enter`
switches to the tab. An extension cannot listen on a socket, and the
bridge must work on Linux, macOS and Windows, with or without a
D-Bus session bus. Which bridge, which transport, and when does tab
data flow?

## Decision Drivers

- One transport and one code path on every OS.
- No idle wakeups ([ADR-0036](0036_no_idle_wakeups.md)), no
  keepalive traffic, no traffic while the user browses.
- Any part may start, stop or crash in any order, and the next
  popup open works with no user action.
- `Enter` switches to the exact tab shown, or does nothing.
- Local only, private to the user, without an auth scheme.
- Few new dependencies ([ADR-0044](0044_binary_budget.md)).

## Considered Options

- Native messaging to a host, AF_UNIX socket from host to daemon
- Native messaging to a host, D-Bus from host to daemon
- WebSocket from the extension to the daemon on `127.0.0.1`
- Native messaging to a host, a different transport per OS (D-Bus,
  Unix socket, named pipe)

For when data flows: tabs pushed on every change, or fetched when
the popup opens.

## Decision Outcome

Chosen option: "Native messaging to a host, AF_UNIX socket from host
to daemon", with tabs fetched when the popup opens, because it is
the only option that is the same on every OS, needs no keepalive,
sends nothing while the user browses, and keeps the bridge private
to the user without auth.

- Stage 1 supports Brave, Chrome and Chromium, with the extension
  installed in Developer mode. Other Chromium browsers come later.
  Stage 2 publishes the same extension, with the same id, on the
  Chrome Web Store. Firefox comes later on the same design.
- The browser starts the `tynex` binary through a per-user native
  messaging manifest; only the listed extension ids may start it.
  `tynex` runs as the host when its arguments match the browser's
  documented form exactly; there is no subcommand for it.
- The `[browser_tabs]` config section (`enabled`, `browsers`)
  turns the feature on; the allowed extension id is built in. The
  daemon writes and removes the per-user manifests on every start,
  and leaves browsers with a system-wide manifest from a package
  alone. There is no install command.
- The host connects to `tabs.sock` in the user's runtime folder;
  the daemon listens. The daemon binds a temporary name, sets mode
  `0600` and renames it, so the socket appears only once it is
  ready, and the host waits for it with inotify. One connection is
  one source; its close drops the source.
- Both hops use native messaging frames: a 32-bit length in native
  byte order and UTF-8 JSON. The host forwards frames unchanged and
  adds only `hello` (browser pid) and `status` (daemon reachable,
  for the extension's badge).
- When the popup opens, the daemon sends `list`; each extension
  answers `tabs`, then `icons` with favicons the daemon does not
  have yet, taken from the browser's favicon cache. Tab lists are
  dropped when the popup closes; icons stay until the connection
  closes.
- `activate` uses the tab id only. A tab that is gone is not
  replaced and its URL is not opened. The extension answers
  `activated`; only on success does the daemon raise the browser
  window through its window source. KWin needs the window pid for
  this, recorded in ADR-0082.
- The daemon parses JSON with `serde_json`; the host parses none.
- The feature is two parts. `browser_tabs` is a module, like the
  title bar: pure logic for tab lists, rows, ranking and the window
  pick, off unless `[browser_tabs] enabled` is set. The Unix socket
  bridge is the `browser_tabs` adapter; it hands the core one
  `TabsUpdate` event type. There is no tab source trait: every
  browser, Firefox included, uses the same native messaging host and
  socket, so a trait would have one implementation. The platform
  event loop drives the adapter directly.

The module design is in
[browser_tabs.md](../arch/modules/browser_tabs.md). The adapter,
protocol and limits are in
[browser_tabs.md](../arch/adapters/browser_tabs.md).

### Consequences

- Good, because Linux, macOS and Windows share the frames, the
  messages, the host loop and the daemon logic.
- Good, because nothing flows while the user browses or while the
  popup is closed, and tab lists never outlive a popup.
- Good, because the list is exact at the moment of use, and
  `Enter` never acts on a different tab.
- Good, because a closed connection is the only liveness signal
  needed, and an open native port keeps the Chrome service worker
  (105+) alive with no traffic.
- Good, because no network port is open, and the socket file is
  private to the user.
- Good, because the same socket design can later carry the CLI IPC
  on systems without D-Bus, under its own ADR.
- Bad, because each popup open costs one round trip per browser, and
  a hung browser contributes no tabs that time.
- Bad, because a tab opened while the popup is open shows only at
  the next open.
- Bad, because native messaging needs a manifest per browser in
  fixed folders; the daemon writes into the browsers' config
  folders, which is why `browser_tabs.enabled` is off by default.
- Bad, because stale socket files and single-instance checks are
  written by hand, which D-Bus gave for free
  ([ADR-0012](0012_dbus_ipc.md)).
- Bad, because `serde_json` is a new dependency, and Windows will
  need a small crate for AF_UNIX.
- Bad, because rows need in-memory icons, a new case for the
  renderer.
- Bad, because Flatpak and Snap browsers cannot start the host
  without extra setup.
- Neutral, because host mode depends on the browsers' argument
  form, which is documented and stable.

### Confirmation

Unit tests cover frames, messages, base64, the core tab logic and
the window pick. Socket and host tests in a temporary folder cover
the atomic bind, `hello`, `status`, caps, stale files, reconnects,
late replies and source drops. Registration tests use a temporary `HOME`. The
extension's pure helpers run under `node --test`. The manual
checklist in the adapter page passes on Brave, Chrome and Chromium on
KDE Plasma 6 Wayland, and the idle check shows no wakeups.

## Pros and Cons of the Options

### Native messaging to a host, AF_UNIX socket from host to daemon

- Good, because it is the same on Linux, macOS and Windows 10 1803+.
- Good, because file permissions make it private to the user.
- Good, because connection close is the liveness signal.
- Bad, because stale socket cleanup is written by hand.

### Native messaging to a host, D-Bus from host to daemon

- Good, because the daemon already uses D-Bus on Linux, and
  `NameOwnerChanged` reports starts and stops.
- Bad, because minimal Linux setups, macOS and Windows have no
  session bus, so another transport is needed anyway.

### WebSocket from the extension to the daemon on `127.0.0.1`

- Good, because there is no host and nothing to register besides
  the extension.
- Bad, because Chrome keeps the service worker alive only with a
  message every 20 to 30 s, which breaks the idle budget.
- Bad, because a TCP port is open to every local user and can clash
  with other software; web pages can reach it too, so `Origin`
  checks are needed from day one.
- Bad, because the extension must poll to find a daemon that is not
  running yet.

### Native messaging to a host, a different transport per OS

- Good, because each OS uses its usual IPC.
- Bad, because three transports must be written, tested and kept in
  step.

### Tabs pushed on every change

- Good, because results are ready without a round trip.
- Bad, because every title change while browsing sends a full list,
  which needs debouncing, change detection, and a cache in the host
  and the daemon.
- Bad, because a cached list can be stale at `Enter`, which led to a
  URL fallback that could switch to a different tab.

## More Information

Answers the "needs an ADR for the bridge protocol" note of the
Browser tabs item in `ROADMAP.md`. Related:
[ADR-0011](0011_kwin_payload.md) (amended by ADR-0082),
[ADR-0012](0012_dbus_ipc.md), [ADR-0026](0026_poll_loop.md),
[ADR-0036](0036_no_idle_wakeups.md),
[ADR-0055](0055_config_sections.md). Revisit when the CLI IPC moves
to a socket, when Firefox support starts, or when Flatpak browsers
need support.
