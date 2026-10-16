# Linux platform

Source: `src/platform/linux/`. Session: Wayland.

## Files

```text
platform/linux/
  mod.rs          Backend wiring
  event_loop.rs   Single poll loop over Wayland and D-Bus fds
  paths.rs        XDG config and runtime paths
  wayland/
    mod.rs        WaylandPresenter: connect, show, draw, hide, dispatch
    state.rs      Dispatch state: globals, popup, events
    surface.rs    Layer-shell surface setup, commit and teardown
    buffer.rs     wl_shm double buffer on a memfd
    input.rs      wl_seat and wl_keyboard events
    keyboard.rs   xkb keymap, compose, repeat, native key -> core Key
    error.rs      PresenterError
  dbus.rs         Shared D-Bus connection, dispatch, calls
  dbus_ipc.rs     DbusIpc: impl IpcServer, IpcClient
  fonts.rs        fc-match lookup: default and per-character fallback
```

## Wayland presenter

Built directly on `wayland-client` (pure Rust backend, no
`libwayland` linked) with `wayland-protocols-wlr` for the layer-shell
protocol. No client toolkit.

- Protocol: `wlr-layer-shell-unstable-v1` (supported by KWin, wlroots
  compositors, Hyprland, niri).
- Globals bound once at start: `wl_compositor`, `wl_shm`,
  `zwlr_layer_shell_v1` (required), `wp_viewporter` and
  `wp_fractional_scale_manager_v1` (optional). `wl_seat` comes with the
  keyboard.
- Layer `overlay`, no anchors, so the compositor centers the surface.
- Keyboard interactivity `exclusive`: all keys go to the popup while it
  is shown. No global key grabbing.
- Size comes from `Layout::size` in physical pixels; the surface size
  is that divided by the scale, rounded up.
- `show`: create the surface, wait for `configure`, draw, commit.
- `draw`: returns false until the surface is configured or while both
  buffers are held by the compositor; the caller draws again on the
  next event.
- `hide`: destroy the surface and free the buffers, so an idle daemon
  holds no pixel memory. The connection, keymap and caches stay.
- Fractional scale: with `wp_fractional_scale_v1` the buffer is drawn
  at the preferred scale (for example 1.25) and `wp_viewport` sets the
  surface size. Without it, the integer `preferred_buffer_scale` and
  `set_buffer_scale` are used. A `Scale` event asks for a redraw.

Try it in a Wayland session: `cargo run --example popup_demo` shows the
mock windows; a letter or `Enter` prints the chosen window, `Esc`
closes, and without input it closes after ten seconds.

## Buffers

- One `memfd` (via `rustix`), mapped once, split into two `wl_shm`
  buffers. A buffer is free again after the compositor's `release`.
- Draw into the free buffer, attach, damage, commit.
- The renderer writes straight into the mapping. With `ABGR8888`
  (same byte order as the renderer) there is no copy; with the
  mandatory `ARGB8888` red and blue are swapped in place.
- Buffers are recreated when the popup size changes.

## Keyboard

- `wl_seat` (version 4 or later, for repeat info) is bound at start;
  its keyboard is taken when the seat has one. Without a seat the
  popup still shows but ignores keys.
- The compositor sends the keymap as a file descriptor. It is read
  with `pread` (no mapping, no `unsafe`) and loaded with `xkbcommon`
  (`Keymap::new_from_string`, default features off).
- Key events become keysyms and UTF-8 text with the current layout, so
  non-QWERTY layouts work, then `core::Key`: `Enter` (also keypad),
  `Esc`, `Tab`, arrows, `Backspace`, or `Char` when the text is one
  printable character. Ctrl combinations produce no `Char`.
- Dead keys and compose sequences (`´` then `e` gives `é`) use the
  `xkbcommon` compose table for the locale in `LC_ALL`, `LC_CTYPE` or
  `LANG`.
- For `Key::Char.latin`, the level-1 keysym of the same keycode is read
  from the first layout where it is an ASCII letter
  (`key_get_syms_by_level`).
- Key repeat, for arrows and `Backspace` only, uses the rate and delay
  sent by the compositor. `next_deadline` gives the poll timeout and
  `dispatch` reports due repeats; leaving focus or hiding stops it.

Tests use real keymaps from the system's XKB data (`us,ru`), so CI
installs `libxkbcommon-dev`.

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
