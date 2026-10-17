# Mock adapter

Source: `src/adapters/mock/`. Target: any system, no desktop needed.

A `WindowSource` with a fixed, in-memory window list. It lets the core,
config, modes and presenters run and be tested without KDE or any other
desktop. It is built before the KDE adapter.

## Files

```text
adapters/mock/
  mod.rs        MockSource: impl WindowSource
  fixtures.rs   Built-in sample windows
```

## Behavior

- `start` sends one `Event::Windows` with the fixture list in most
  recently used (MRU) order.
- `activate(id)` records the id, moves that window to the front of the
  MRU list and sends the updated list, like a real desktop would.
- Unknown ids return an error, so error paths are testable.
- No threads, timers or I/O.

## Fixtures

Built-in list of ten windows (ids `mock-01` to `mock-10`) with
realistic app ids and names and made-up titles, including edge cases:

- a title longer than the row width;
- two windows of the same app;
- a window without an icon;
- titles ending in the app name after ` — `, for `strip_app_name`.

Text cases for the rules in
[core.md](../core.md#text-and-languages):

- Spanish (`Música — Dolphin`), for Unicode smart case;
- Chinese (`微信`), for font fallback;
- accented French (`Café — Firefox`), for accent folding;
- a combining accent (`e` + `U+0301`) near the truncation point;
- control characters and a right-to-left override (`U+202E`), for
  sanitizing.

The registry in `src/adapters/mod.rs` builds it only for
`desktop = "mock"`. Tests use `pick`, which chooses the adapter without
building it, so they need no session bus.

## Use

- Unit and integration tests: build `MockSource` directly, push events,
  then assert on `View` and on recorded activations.
- Manual runs: `desktop = "mock"` in `tynex.toml` shows the popup with
  fixture windows on any compositor with layer-shell.
- `desktop = "auto"` never selects the mock adapter.

## Test API

```rust
impl MockSource {
    /// Creates a source with the built-in fixtures.
    pub fn new() -> Self;

    /// Creates a source with the given windows, in MRU order.
    pub fn with_windows(windows: Vec<WindowInfo>) -> Self;

    /// Returns ids passed to `activate`, oldest first.
    pub fn activations(&self) -> &[WindowId];
}
```
