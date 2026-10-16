# Linux platform

Source: `src/platform/linux/`. Session: Wayland.

## Files

```text
platform/linux/
  mod.rs          Backend wiring
  event_loop.rs   Single poll loop over Wayland and D-Bus fds
  paths.rs        XDG config and runtime paths
  wayland/
    mod.rs        WaylandPresenter: impl Presenter
    globals.rs    Registry: compositor, shm, seat, layer-shell
    surface.rs    Layer-shell surface setup and teardown
    buffer.rs     wl_shm double buffer on a memfd
    keyboard.rs   xkb keymap, native key -> core Key
  dbus.rs         Shared D-Bus connection, dispatch, calls
  dbus_ipc.rs     DbusIpc: impl IpcServer, IpcClient
  fonts.rs        fc-match lookup: default and per-character fallback
```

## Wayland presenter

Built directly on `wayland-client` with `wayland-protocols-wlr` for the
layer-shell protocol. No client toolkit.

- Protocol: `wlr-layer-shell-unstable-v1` (supported by KWin, wlroots
  compositors, Hyprland, niri).
- Globals bound once at start: `wl_compositor`, `wl_shm`, `wl_seat`,
  `zwlr_layer_shell_v1`.
- Layer `overlay`, no anchors, so the compositor centers the surface.
- Keyboard interactivity `exclusive`: all keys go to the popup while it
  is shown. No global key grabbing.
- Size comes from `Layout::size`; resized only when the row count
  changes.
- `show`: create the surface, wait for `configure`, draw, commit.
- `hide`: destroy the surface. The connection, keymap and caches stay.
- Fractional scale: buffers are drawn at the output scale.

## Buffers

- One `memfd` (via `rustix`), mapped once, split into two `wl_shm`
  buffers.
- Draw into the free buffer, attach, damage only changed rows, commit.
- The renderer writes ARGB8888 directly into the mapping. No copies.
- The pool grows only when the popup grows.

## Keyboard

- The compositor sends the keymap as a file descriptor. It is mapped
  with `rustix` and loaded with `xkbcommon`
  (`Keymap::new_from_string`, default features off).
- Key events become keysyms and UTF-8 text with the current layout, so
  non-QWERTY layouts work, then `core::Key`.
- Dead keys and compose sequences (`´` then `e` gives `é`) use the
  `xkbcommon` compose table for the user's locale.
- For `Key::Char.latin`, the keysym of the same keycode is read from
  the first layout group that produces Latin letters
  (`key_get_syms_by_level`).
- Key repeat, for arrows and `Backspace` only, uses the rate and delay
  sent by the compositor and the poll timeout.

## Event loop

One thread, one `rustix::event::poll` call over:

- the Wayland connection fd;
- the D-Bus connection fd (`dbus` channel watch).

The poll timeout is the nearest deadline: key repeat while the popup is
shown, otherwise none, so an idle daemon never wakes up. Events from both
sources go to the core in order, so no locks or channels are needed.

## D-Bus

`dbus.rs` owns the daemon's single session bus connection
([ADR-0035](../../adr/0035_shared_dbus.md)). Every D-Bus user goes
through it; none opens its own connection.

- `dbus` crate over the system `libdbus`, non-blocking, no async
  runtime. Its fd is the only D-Bus fd in the poll loop.
- Owns the bus name `io.github.canbesolved.Tynex`; a second daemon
  finds it taken and exits.
- Users register an object path with a handler; incoming method calls
  are read with `read_write` and dispatched by path, interface and
  member.
- `call` sends a method call without blocking; an optional callback
  receives the reply or error in the main loop.

| User                  | Registers           | Calls                 |
| --------------------- | ------------------- | --------------------- |
| IPC (`dbus_ipc.rs`)   | `/io/.../Tynex`     | none                  |
| KDE adapter           | `/io/.../Tynex/Kde` | KWin scripting, `Run` |
| Later: GNOME, portals | as needed           | as needed             |

The CLI client opens its own short-lived connection, sends one call
and exits.

## Paths

- Config: `$XDG_CONFIG_HOME/tynex/`, else `~/.config/tynex/`.
- Runtime: `$XDG_RUNTIME_DIR/tynex/` for the KWin script file.
  - `$XDG_RUNTIME_DIR` is required. If it is unset, not a directory,
    not owned by the user or open to other users, the daemon exits
    with an error. There is no fallback to `/tmp`.
  - `tynex/` is created with mode `0700`. An existing `tynex/` that is
    a symlink, not owned by the user or open to others is rejected.
  - Files are written atomically: a temporary file is created with
    `O_EXCL` and mode `0600`, then renamed over the target.
- Home comes from `std::env::home_dir`; no extra crate.

## Fonts

`font = "auto"` runs `fc-match -f '%{file}' sans-serif` once at start.
The result is cached. No fontconfig library is linked.

Fallback fonts for characters missing from the default font are also
found with `fc-match` (see
[render.md](../ui/render.md#fonts-and-scripts)). Font files are
memory-mapped with `rustix`.
