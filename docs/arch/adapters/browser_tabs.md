# Browser tabs adapter

Source: `src/adapters/browser_tabs/`. The extension lives in
`extensions/browser_tabs/chromium/`
([ADR-0083](../../adr/0083_extensions_folder.md)).
Module: [modules/browser_tabs.md](../modules/browser_tabs.md).
Decision: [ADR-0081](../../adr/0081_tab_bridge.md).

The adapter is the Unix socket bridge between the daemon and browser
extensions. It is Linux-only for now (`cfg(target_os = "linux")`).
The Linux event loop drives it directly; there is no tab source
trait, because every browser uses this one transport.

## Files

```text
adapters/browser_tabs/
  mod.rs       start (daemon side), run_host (host side)
  frame.rs     Length-prefixed frames, pure
  msg.rs       JSON messages in and out
  base64.rs    Favicon decoding, no crate
  bind.rs      Socket file: single instance, stale files, rename
  channels.rs  Supported release channels and their folders
  socket.rs    Socket: connections, requests
  recv.rs      Incoming frames, closed connections
  conn.rs      One host connection: buffers, close reason
  host.rs      Relay: stdin and stdout to the socket
  pump.rs      Host poll loop
  watch.rs     inotify watch on the runtime folder
  register.rs  Native messaging manifests
```

## Flow

```text
 Browser (one per profile)      tynex host             tynex daemon
+--------------------------+   +-----------------+   +---------------+
| extension service worker |   | tynex, started  |   | tabs socket   |
|   answers "list"         |<--| by the browser  |<--|   "list" on   |
|   answers "activate"     |-->| stdin to socket |-->|   popup open  |
+--------------------------+   +-----------------+   +---------------+
```

1. The browser starts the host when the extension calls
   `connectNative`. The host lives as long as that port and
   connects to the daemon's socket.
2. The popup opens: the daemon sends `list`. Each extension answers
   with its tabs, then with favicons the daemon does not have yet.
3. `Enter` on a tab row: the daemon sends `activate`. On `ok` the
   module raises the browser window through KWin.
4. The popup closes: tab lists are dropped. Favicons stay until the
   connection closes.

Nothing flows while the user browses or while the popup is closed.
A closed connection is the only liveness signal.

## Frames

A frame is a 32-bit length in native byte order, then that many
bytes of UTF-8 JSON. `frame.rs` is pure: a reader yields whole
frames as bytes arrive. A length of `0` or over the cap stops the
reader for good, and the owner closes the stream.

| Direction              | Cap    |
| ---------------------- | ------ |
| Toward the daemon      | 32 MiB |
| Toward the browser     | 1 MiB  |
| Host out buffer        | 8 MiB  |
| Daemon out buffer      | 64 KiB |

## Messages

Every message has `"v": 1` and `"type"`. A peer ignores another
version. `req` is chosen by the daemon; a reply carries it.

| Type        | From      | To        | When                |
| ----------- | --------- | --------- | ------------------- |
| `hello`     | host      | daemon    | first frame         |
| `status`    | host      | extension | daemon reachability |
| `list`      | daemon    | extension | popup opens         |
| `tabs`      | extension | daemon    | reply to `list`     |
| `icons`     | extension | daemon    | new favicons        |
| `activate`  | daemon    | extension | `Enter` on a tab    |
| `activated` | extension | daemon    | reply to `activate` |

`hello` carries the browser pid (`getppid()`). `status` is
`{"v":1,"type":"status","daemon":true}` and drives the badge.
`true` also tells the extension that the icon cache is empty.

`tabs` carries `browser`, then rows: `id`, `window`, `active`,
`last` (ms, `0` if missing), `title`, `url` and `icon` (the origin,
or the scheme when the URL has no host). A row whose ids do not fit
32 bits is dropped.

`icons` carries base64 PNGs, at most 64 per message and 16 KiB
each. Padding is accepted only at the end.

`activate` carries the tab id only. `activated` has `ok` and
`window`; on failure `ok` is false and `window` is `-1`. A missing
tab is not replaced and its URL is not opened.

`msg.rs` reads a frame twice with `serde_json`: once for `v` and
`type`, skipping every other value, then into the struct of that
type. No `Value` tree is built. A known type with bad fields is
skipped. The host parses no JSON: it builds `hello` and `status`
with `format!` and forwards every other frame unchanged.

## Socket

Linux: `$XDG_RUNTIME_DIR/tynex/tabs.sock`, inside the private
runtime folder from [platforms/linux.md](../platforms/linux.md). No
fallback to `/tmp`. A path longer than the platform limit is logged
at `error` and tabs stay off. The path is not configurable: the
host reads no config.

`bind.rs` runs on start:

1. Connect to `tabs.sock`. Success means another daemon serves it:
   log at `warn` and leave tabs off.
2. Connection refused, and `lstat` shows a socket owned by the user
   (not a symlink or a regular file): remove it. Not found: go on.
   The same check clears a leftover `.tabs.sock.new`.
3. Bind `.tabs.sock.new`, set mode `0600`, and rename it to
   `tabs.sock`.

The rename makes the socket appear only once it listens with its
final mode, so a host that wakes on the new name always connects.
The file is removed when the daemon exits.

The host connects; the daemon only listens. At most 8 connections;
a ninth is closed at once. Each connection is one source. `SourceId`
numbers are not reused while the daemon runs. The first frame must
be a version 1 `hello`; anything else, bad JSON included, closes
the connection. Later bad JSON is logged once per connection and
skipped.

One activation is pending at a time. A reply with another `req` or
from another source is ignored. A `tabs` reply is used only while
the popup is open and only for the latest `list`. `listing` stays
true until every host asked by the latest `list` answered or left;
the event loop holds the first draw that long, at most
`browser_tabs.wait` ms ([ADR-0097](../../adr/0097_history.md)).

A connection closes on EOF, a read or write error, a bad frame
length, or a full out buffer (a browser that stops reading). Every
close goes through one path: the connection is shut down, which
wakes the poll loop, and the next `drive` drops it and reports
`TabsUpdate::Gone`. The module then drops that source's rows and
pending activation, and the painter drops its favicons.

`drive` never blocks. It reads at most 1 MiB per connection per
call; the level-triggered poll wakes again for the rest.

## Host

`tynex` has no host subcommand. Before CLI parsing, one argument of
the form `chrome-extension://` plus 32 letters from `a` to `p` plus
`/` selects the host. Anything else is the normal CLI. The browser
starts this process; the manifest cannot pass `daemon`.

stdout carries frames only. Logs go to stderr. One thread polls
stdin, the socket and an inotify watch (`IN_CREATE`, `IN_MOVED_TO`)
on the runtime folder. stdin is read from fd 0 directly, because
`std`'s buffered stdin could hold bytes that `poll` does not see.

The watch is set before the first connect, so a socket that appears
in between still wakes the host. Any watch event, or a timer when
disconnected without inotify, leads to one connect attempt. The
timer backs off from 1 s to 60 s, and that fallback is logged once
at `info`.

The host exits `0` on stdin EOF or a write error on stdout, and `1`
on a bad frame from the browser. It does not exit because the
daemon is down. Frames from the browser are dropped while the
daemon is disconnected. Without a runtime folder the host sends
`status` `false` and waits for stdin to close.

## Registration

On every daemon start, including when tabs are off, per-user host
manifests are made to match the config. The file is
`NativeMessagingHosts/io.github.canbesolved.tynex.json` inside each
release channel's profile folder. It is written only when missing
or different, with `write_atomic`, and only if that channel is
installed and no system-wide manifest for that browser exists. A
system-wide manifest wins, and the per-user copy is removed.

A channel counts as installed when its profile folder has a
`Local State` file, which every Chromium browser writes on first
run. A bare folder does not count, so a folder left behind, or one
holding only an old manifest, is cleaned up instead.

`browsers = "auto"` uses every installed channel of every supported
browser. A name or a list uses only those browsers, with all their
installed channels; a listed browser with none is logged. Channels
of other browsers are not probed or logged: only a manifest this
daemon wrote there earlier is removed. The tabs socket is opened
only when at least one selected channel is installed.

| Channel       | Profile folder under `~/.config/`      |
| ------------- | -------------------------------------- |
| Brave         | `BraveSoftware/Brave-Browser/`         |
| Brave Beta    | `BraveSoftware/Brave-Browser-Beta/`    |
| Brave Nightly | `BraveSoftware/Brave-Browser-Nightly/` |
| Brave Origin  | `BraveSoftware/Brave-Origin/`          |
| Chrome        | `google-chrome/`                       |
| Chrome Beta   | `google-chrome-beta/`                  |
| Chrome Dev    | `google-chrome-unstable/`              |
| Chromium      | `chromium/`                            |

System-wide folders: Brave `/etc/opt/brave/native-messaging-hosts/`,
Chrome `/etc/opt/chrome/native-messaging-hosts/`, for every channel
of that browser, and Chromium `/etc/chromium/native-messaging-hosts/`.

Edge and Vivaldi are planned
([ROADMAP.md](../../../ROADMAP.md)). "Ours" means this file name
and a `path` whose file name is `tynex`. Other files are left
alone.

`path` is `current_exe()`. `allowed_origins` lists the built-in
extension id only.

## Extension

Stage 1 installs the unpacked extension in
`extensions/browser_tabs/chromium/`. The manifest is Manifest V3
with permissions `tabs`, `nativeMessaging`, `alarms` and
`favicon`. There are no content scripts and no host permissions.

The service worker connects to `io.github.canbesolved.tynex`,
answers `list` and `activate`, and shows a badge: no badge when
connected, a grey ellipsis when the daemon is down, a red
exclamation mark when the host cannot be started or exits.
Reconnects back off from 1 s to 60 s. An alarm every minute starts
the worker again if it was stopped with no port open. Incognito
tabs are never listed.

Favicons come from the browser's own cache, as 32 px PNG, through
the extension favicon URL, fetched 64 at a time. A key is sent at
most once per daemon connection, and one that failed is not
retried on that connection. At most 512 keys are sent.

### Install

1. Set `[browser_tabs] enabled = true` and restart the daemon.
2. Open `brave://extensions` or `chrome://extensions`, turn on
   Developer mode, and load the `extensions/browser_tabs/chromium/`
   folder.
3. The badge clears once the daemon is connected.

The extension id is derived from the public `key` in the manifest:
the first 32 hex digits of the SHA-256 of the key bytes, with `0`
to `f` mapped to `a` to `p`. The private key is not in the
repository. Stage 2 replaces this id with the Chrome Web Store id;
the code does not change apart from that value.
`build.sh` packs the zip for the store.

The icons follow the logo. 48 and 128 px are drawn from
`assets/icon.svg`; 16 and 32 px from `assets/icon_small.svg`, the
logo without its idle rows so it stays legible in the toolbar:

```bash
cd extensions/browser_tabs/chromium/icons
rsvg-convert -w 16 -h 16 ../../../../assets/icon_small.svg -o 16.png
rsvg-convert -w 32 -h 32 ../../../../assets/icon_small.svg -o 32.png
rsvg-convert -w 48 -h 48 ../../../../assets/icon.svg -o 48.png
rsvg-convert -w 128 -h 128 ../../../../assets/icon.svg -o 128.png
```

## Logging

Every record starts with `browser tabs:`. Titles and URLs are never
logged; browser names, ids and counts are.

| Level   | Daemon                                  | Host                    |
| ------- | --------------------------------------- | ----------------------- |
| `info`  | Browsers used, socket ready,            | Waiting, connected,     |
|         | each connect and disconnect with reason | lost the daemon, exit   |
| `debug` | Per-browser state, manifest up to date, | Started, frames dropped |
|         | `list` sent, tab counts, favicons,      | while disconnected      |
|         | `activate` and its reply, window raised |                         |
| `warn`  | Socket in use, bad JSON, a listed       | Connect errors, frames  |
|         | browser missing, write errors           | dropped (daemon slow)   |
| `error` | Socket path too long                    | Bad frame, stdin error  |

Daemon, with `TYNEX_LOG=debug tynex daemon` (module paths shortened):

```text
tynex: debug: ...register: browser tabs: Brave: installed
tynex: debug: ...register: browser tabs: Brave Beta: not installed
tynex: info: browser tabs: using Brave
tynex: info: browser tabs: socket ready, waiting for hosts
tynex: info: browser tabs: source 1 (browser, pid 4242) connected
tynex: debug: ...socket: browser tabs: list 2 sent to 1 browser(s)
tynex: debug: ...recv: browser tabs: 37 tabs from source 1 (Brave, ...)
tynex: debug: ...socket: browser tabs: popup closed, idle
tynex: info: browser tabs: source 1 (Brave, pid 4242) disconnected:
             closed by the host
```

Host: its stderr is the browser's. Start the browser from a terminal
to see it, for example `TYNEX_LOG=debug brave-browser`:

```text
tynex: info: browser tabs: host waiting for the daemon
tynex: info: browser tabs: host connected to the daemon
tynex: info: browser tabs: host lost the daemon (daemon closed), waiting
```

## Safety

No network sockets. The socket file is mode `0600` in a folder only
the user can open. Only the extension id in the manifest can start
the host. The extension never opens URLs. Private tabs are not
listed. Titles and URLs are not logged.

A process of the same user can connect and add fake rows. It is
sent `activate` only for its own rows. Checking the peer
executable is later work.

## Limits

| Value                    | Limit       |
| ------------------------ | ----------- |
| Title                    | 256 chars   |
| URL                      | 1024 chars  |
| Tabs per browser         | 10 000      |
| Icons per connection     | 512         |
| Icon file                | 16 KiB      |
| Icon side                | 64 px       |
| Icons per message        | 64          |
| Connections              | 8           |
| Read per connection      | 1 MiB/turn  |
| Reconnect backoff        | 1 s to 60 s |
| Watchdog alarm           | 1 min       |

The extension cuts titles and URLs; the module cuts them again, so
the limits hold for any peer. Flatpak and Snap browsers cannot
start the host without extra setup, and their pid does not match
the KWin window. Desktops without a KWin raise rely on the
extension's own focus request.

## Tests

Frames, messages, base64, the socket (in a temporary folder), the
host relay and registration (with a temporary home) are covered by
`cargo test`. The extension helpers run under `node --test`. On
Brave (including Brave Origin), Chrome and Chromium, KDE Plasma 6
Wayland, check at least:

1. Start the browser, then the daemon. Search a tab, press `Enter`,
   and the tab and its window come forward.
2. Favicons show on the first open and are not sent again later.
3. Restart the daemon with the browser open. The next open lists
   tabs.
4. Kill the daemon with `SIGKILL` and start it again. The stale
   socket is replaced.
5. Set `enabled = false`, restart, reload the extension. The badge
   shows an exclamation mark.
6. Close a tab while the popup is open, then `Enter` on its row.
   Nothing happens and the browser is not raised.
7. Activate a tab in a minimized window, a maximized window and a
   window on another virtual desktop.
8. Two profiles. Each row opens in the right profile.
9. With the popup closed, `tynex` and the host use no CPU.
