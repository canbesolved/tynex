# IPC

Source: `src/ipc/` (traits, commands) and `src/platform/*/` (transport).

## Purpose

Lets the short-lived CLI (`tynex show`) control the resident daemon.
Any desktop can bind the CLI to a hotkey, even without an adapter
hotkey.

## Commands

| CLI                        | Command        | Effect              |
| -------------------------- | -------------- | ------------------- |
| `tynex show`               | `Show(hotkey)` | Open in hotkey mode |
| `tynex show --mode search` | `Show(search)` | Open in search mode |
| `tynex hide`               | `Hide`         | Close the popup     |
| `tynex quit`               | `Quit`         | Stop the daemon     |

## Traits

```rust
/// Daemon side: receives commands and forwards them as events.
pub trait IpcServer {
    fn start(&mut self, tx: EventTx) -> Result<()>;
}

/// CLI side: sends one command and exits.
pub trait IpcClient {
    fn send(&self, cmd: Command) -> Result<()>;
}
```

## Transports

| Platform | Transport                                              |
| -------- | ------------------------------------------------------ |
| Linux    | D-Bus session bus, name `io.github.canbesolved.Tynex`  |
| macOS    | Unix socket (later)                                    |
| Windows  | Named pipe (later)                                     |

On Linux D-Bus is used because the KDE adapter needs it anyway, so it
adds no extra dependency. The server registers its object on the
daemon's shared connection (see
[linux.md](platforms/linux.md#d-bus)), next to the KDE adapter. A
second `tynex daemon` exits early when the bus name is already owned.
The CLI sends one method call and exits.
