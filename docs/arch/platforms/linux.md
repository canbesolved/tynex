# Linux platform

Source: `src/platform/linux/`. Session: Wayland.

## Files

```text
platform/linux/
  mod.rs          Backend wiring
  event_loop.rs   Single poll loop over Wayland and D-Bus fds
  paths.rs        XDG config, data and runtime paths, atomic writes
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
  desktop_files.rs  App names from desktop files
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
- When keyboard focus leaves the open popup (`wl_keyboard.leave`),
  the presenter reports `FocusLost` and the event loop hides the
  popup.
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

Tests use real keymaps from the system's XKB data (`us,gr`), so CI
installs `libxkbcommon-dev`.

## Event loop

One thread, one `rustix::event::poll` call over:

- the Wayland connection fd;
- the D-Bus connection fd (`dbus` channel watch).

The poll timeout is the nearest deadline: key repeat while the popup is
shown, otherwise none, so an idle daemon never wakes up. Events from both
sources go to the core in order, so no locks or channels are needed.

`event_loop.rs` owns the parts (`Parts`: switcher, window source,
presenter, bus, painter, event queue). Each turn it:

1. drains the event queue into the switcher; new window lists first
   resolve their fonts (`Painter::prepare`);
2. applies the action: `Redraw` sizes and draws the popup, `Activate`
   hides it and calls the window source, `Close` hides it;
3. flushes Wayland requests and D-Bus calls, then polls;
4. dispatches Wayland events (keys, configure, scale, closed) and
   D-Bus calls, which push new events into the queue.

A `Quit` request hides the popup and ends the loop. Measured with the
mock source: about 6 MB RSS and no context switches while idle.

## D-Bus

`dbus.rs` owns the daemon's single session bus connection
([ADR-0035](../../adr/0035_shared_dbus.md)). Every D-Bus user goes
through it; none opens its own connection.

- `dbus` crate over the system `libdbus`, non-blocking, no async
  runtime. Its fd is the only D-Bus fd in the poll loop.
- Owns the bus name `io.github.canbesolved.Tynex` (`RequestName` with
  `DO_NOT_QUEUE`); a second daemon finds it taken and exits with
  "tynex daemon is already running".
- `Bus` is a cheap handle (`Rc`); clones share the connection, so the
  IPC server and the adapter each keep one.
- Users register an object path with a handler; incoming method calls
  are read with `read_write` and dispatched by path, interface and
  member.
- `call` sends a method call without blocking; its callback receives
  the reply or error in a later `dispatch`. The loop flushes these
  calls before it polls.
- `call_blocking` waits up to 2 s for the reply. It is used only at
  startup and exit (`RequestName`, `GetNameOwner`, KWin scripting).

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
    `O_EXCL` and mode `0600`, then renamed over the target
    (`write_atomic`).
  - The user's id for the owner checks comes from
    `rustix::process::getuid`.
- Data: `$XDG_DATA_HOME`, else `~/.local/share`, then each entry of
  `$XDG_DATA_DIRS`, else `/usr/local/share:/usr/share`. Relative
  entries are skipped (`data_dirs`).
- Home comes from `std::env::home_dir`; no extra crate.

## Desktop files

`DesktopFiles` gives each window the `Name` of its app's desktop file
([ADR-0045](../../adr/0045_app_names.md)). Adapters call it before
sending a window list; the KDE adapter does.

- File: `<app id>.desktop` in `applications/` under each data dir, in
  the order above; the first file found wins. Ids that are empty,
  start with `.` or contain `/` are not looked up.
- Key: `Name` in the `[Desktop Entry]` group; other groups such as
  actions are skipped. Escapes `\s`, `\n`, `\t`, `\r`, `\\` are
  decoded. At most 1 MiB of a file is read.
- Locale: the first non-empty of `LC_ALL`, `LC_MESSAGES` and `LANG`.
  `sr_RS.UTF-8@latin` tries `Name[sr_RS@latin]`, `Name[sr_RS]`,
  `Name[sr@latin]`, `Name[sr]`, then `Name`, as the Desktop Entry
  spec says. `C` and `POSIX` use `Name`.
- Fallback: without a file or a `Name`, the adapter's value stays
  (the WM class for KDE).
- Cache: each app id is looked up once, hits and misses alike, and
  kept until the daemon exits. A renamed app shows its new name after
  a restart.
- Not supported: the subdirectory rule for ids (`kde-foo` in
  `applications/kde/foo.desktop`).

## Fonts

`fonts.rs` loads the default face with `default_face`: the path from
`font`, or `fc-match -f '%{file}:%{index}' sans-serif` for `auto`. No
fontconfig library is linked; `fc-match` runs without a shell.

`FcFallback` implements `FontLookup`: for a character the loaded fonts
lack it runs `fc-match` with `sans-serif:charset=<hex>` (see
[render.md](../ui/render.md#fonts-and-scripts)). Files are cached by
path and face index, so each is mapped once.

- fontconfig keeps the named instance of a variable font in the upper
  16 bits of the index (`262144` is instance 4 of face 0); only the
  lower 16 bits are the face index.
- Font files are mapped read-only and private with `rustix` and never
  unmapped, because `Face` needs `&'static` data. Only used pages count
  towards RAM.
