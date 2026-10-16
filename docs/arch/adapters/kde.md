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

1. `KdeSource::start` checks that `/WindowsRunner` exists. If not, it
   fails with an error naming the missing KWin interface.
2. It writes `tynex.js` to `$XDG_RUNTIME_DIR/tynex/`, following the
   runtime directory rules in
   [linux.md](../platforms/linux.md#paths).
3. It calls `org.kde.kwin.Scripting.loadScript(path, "tynex")`, then
   `start`. A stale script with the same name is unloaded first.
4. On daemon exit, `unloadScript("tynex")` is called.

## Script responsibilities

- Track MRU order with `workspace.windowActivated`.
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
the list. `0_{...}` is the id format of KWin's window search results
(checked on Plasma 6.7). A failed call is logged at `warn` level when
its error reply arrives in the main loop.

## Payload format

To avoid a JSON dependency, the list is one string: records separated
by `\x1e`, fields by `\x1f`.

```text
<uuid> \x1f <desktopFileName> \x1f <resourceClass> \x1f <caption> \x1e ...
```

Control characters `\x1e` and `\x1f` are stripped from captions in the
script. `parse.rs` turns the string into `Vec<WindowInfo>`.

## D-Bus interface

Object `/io/github/canbesolved/Tynex/Kde`, interface
`io.github.canbesolved.Tynex.Kde`:

| Method   | Args     | Reply    | Called by   |
| -------- | -------- | -------- | ----------- |
| `Update` | `s` list | none     | KWin script |
| `Show`   | none     | none     | KWin script |

## Costs

- Idle: no wakeups; the script sends updates only when windows change.
- Show: no round trip, the list is cached.
- Activate: one `Run` call to KWin, about 1 ms.
