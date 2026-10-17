# KDE adapter

Source: `src/adapters/kde/`. Target: KDE Plasma 6, Wayland session.

## Files

```text
adapters/kde/
  mod.rs        KdeSource: impl WindowSource
  kwin.rs       Script loading and window activation via KWin D-Bus
  bridge.rs     D-Bus object the KWin script calls
  parse.rs      Window list decoding
  tynex.js      KWin script, embedded with include_str!
```

App names come from desktop files through
`platform/linux/desktop_files.rs`
([linux.md](../platforms/linux.md#desktop-files)).

D-Bus access goes through the shared connection in
`platform/linux/dbus.rs` ([linux.md](../platforms/linux.md#d-bus)).
The adapter only registers its object and makes its calls.

## How it works

On Wayland, clients cannot list other windows. KWin offers two ways
in ([ADR-0034](../../adr/0034_kwin_runner.md)):

- **Window list:** a KWin script runs inside the compositor, sees all
  windows in most recently used (MRU) order, and pushes the list to
  the daemon with `callDBus`.
- **Activation:** KWin's window search, `org.kde.KWin /WindowsRunner`
  (`org.kde.krunner1`), activates a window by id with `Run`. The
  daemon calls it directly; the script is not involved.

## Lifecycle

The registry builds `KdeSource` for `desktop = "kde"`, or for `auto`
when `XDG_CURRENT_DESKTOP` contains `KDE`.

1. `KdeSource::start` checks that `/WindowsRunner` exists. If not, it
   fails with an error naming the missing KWin interface.
2. It asks the bus for KWin's unique name (`GetNameOwner`), used by
   the sender check below.
3. It writes `tynex.js` to `$XDG_RUNTIME_DIR/tynex/`, following the
   runtime directory rules in
   [linux.md](../platforms/linux.md#paths).
4. It registers the bridge object, then calls
   `org.kde.kwin.Scripting.loadScript(path, "tynex")` and `start`. A
   stale script with the same name is unloaded first.
5. On daemon exit, `WindowSource::stop` calls `unloadScript("tynex")`
   and deletes the file.

Startup and exit calls block for up to 2 s each; activation never
blocks.

## Script responsibilities

- Track MRU order with `workspace.windowActivated`, starting from the
  stacking order, topmost first.
- Push the list with `callDBus(..., "Update", payload)` on
  `windowAdded`, `windowRemoved`, `windowActivated` and
  `captionChanged`. Only normal, switchable windows are sent (skip
  `skipSwitcher`, docks, desktops, special windows).
- Register the global shortcut with `registerShortcut("tynex", ...,
  "Meta+Tab", ...)`. It calls `Show` on the daemon. Users rebind it in
  System Settings, Shortcuts.

## Activation

`activate(id)` sends one call to KWin and does not wait for the reply:

```text
org.kde.KWin /WindowsRunner org.kde.krunner1.Run("0_{<uuid>}", "")
```

`<uuid>` is the window's `internalId`, the same id the script sends in
the list; `parse.rs` strips its braces, `activate` adds them back.
`0_{...}` is the id format of KWin's window search results (checked on
Plasma 6.7). A failed call is logged at `warn` level when its error
reply arrives in the main loop.

## Payload format

To avoid a JSON dependency, the list is one string: records separated
by `\x1e`, fields by `\x1f`.

```text
<uuid> \x1f <desktopFileName> \x1f <resourceClass> \x1f <caption> \x1e ...
```

Control characters `\x1e` and `\x1f` are stripped from captions in the
script. `parse.rs` turns the string into `Vec<WindowInfo>`, in the
order sent:

- `app_id` is `desktopFileName`, or `resourceClass` when it is empty;
- `app_name` is `resourceClass`, replaced by the `Name` of the app's
  desktop file when there is one
  ([ADR-0045](../../adr/0045_app_names.md));
- `title` is the caption, sanitized later by the core;
- `icon` is not set;
- records without exactly four fields or with an empty id are
  skipped, and a repeated id keeps only its first record.

## D-Bus interface

Object `/io/github/canbesolved/Tynex/Kde`, interface
`io.github.canbesolved.Tynex.Kde`:

| Method   | Args     | Reply    | Called by   |
| -------- | -------- | -------- | ----------- |
| `Update` | `s` list | none     | KWin script |
| `Show`   | none     | none     | KWin script |

`Show` opens the popup in the config's `mode`.

Only KWin may call the object: a call whose sender is not KWin's
unique name gets an error reply and is logged at `warn` level, so
other programs in the session cannot inject a fake window list or open
the popup. If KWin restarts, its unique name changes and the daemon
must be restarted.

## Costs

- Idle: no wakeups; the script sends updates only when windows change.
- Show: no round trip, the list is cached.
- App names: one desktop file read per app id, on its first window;
  later updates use the cache.
- Activate: one `Run` call to KWin, about 1 ms.
