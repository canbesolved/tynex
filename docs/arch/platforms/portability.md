# Portability

How `tynex` stays portable to other desktops, macOS and Windows.

## Seams

Three traits isolate the operating system. Everything else is shared.

| Seam           | Linux (0.1.0)       | macOS (later)     | Windows (later) |
| -------------- | ------------------- | ----------------- | --------------- |
| `WindowSource` | KWin script adapter | Accessibility API | Win32 API       |
| `Presenter`    | Wayland layer-shell | AppKit `NSPanel`  | Win32 popup     |
| `IpcServer`    | D-Bus               | Unix socket       | Named pipe      |

## Rules

- `core`, `modes`, `modules`, `ui`, `config` and `ipc` traits
  are pure Rust and compile on all three systems.
- OS code lives only in `src/platform/<os>/` and `src/adapters/`.
- Linux-only crates are declared under
  `[target.'cfg(target_os = "linux")'.dependencies]`.
- Presenters receive a ready pixel buffer from the shared renderer and
  only blit it, so drawing is written once.
- Each backend maps native key events to the portable `core::Key`.
- Paths come from each platform's `paths.rs`, built on the standard
  library, never hard-coded.

## Planned backends

Planned desktops, systems and CI checks are listed in
[ROADMAP.md](../../../ROADMAP.md).
