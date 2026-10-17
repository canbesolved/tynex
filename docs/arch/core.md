# Core

Source: `src/core/`. Pure logic: no I/O, no OS types, no threads. Fully
unit-testable and identical on every platform.

## Files

```text
core/
  window.rs     WindowId, WindowInfo
  event.rs      Event, Action, Key
  traits.rs     WindowSource, Presenter, Layout, Mode
  state.rs      State of an open popup, shared with the modes
  switcher.rs   State machine (tests in switcher_tests.rs)
  view.rs       View and Row: what the presenter must draw
  keys.rs       Letters pinned to windows
  queue.rs      EventTx / EventRx: single-threaded event queue
  text.rs       Title sanitizing and safe cut points
```

## Types

```rust
/// Opaque id given by the adapter (KWin uses a UUID string).
pub struct WindowId(pub String);

/// One switchable window.
pub struct WindowInfo {
    pub id: WindowId,
    pub app_id: String,  // Desktop file name or WM class
    pub title: String,
    pub icon: Option<String>, // Icon name or path
}

/// Inputs to the core.
pub enum Event {
    Windows(Vec<WindowInfo>), // Full list, most recently used first
    Show(ModeKind),
    Key(Key),
    Hide,
    Quit,
}

/// Outputs of the core.
pub enum Action {
    None,
    Redraw,
    Activate(WindowId),
    Close,
}
```

`Key` is a portable enum (`Char { text, latin }`, `Enter`, `Esc`,
`Tab`, `Up`, `Down`, `Backspace`). Each platform maps native key
events to it:

- `text`: the character the active layout produces, after dead keys
  and compose (`ф` on a Russian layout, `é` with a French one).
- `latin`: the character of the same physical key in the first Latin
  layout of the keymap, if any (`a` for the key that gives `ф`). Hotkey
  mode uses it, so letters work whatever layout is active.

## Traits

```rust
/// Source of windows: a desktop environment or window manager.
pub trait WindowSource {
    fn start(&mut self, tx: EventTx) -> Result<(), SourceError>;
    fn activate(&mut self, id: &WindowId) -> Result<(), SourceError>;
    /// Undoes what `start` set up; called once on exit.
    fn stop(&mut self) {}
}

/// Puts a rendered view on screen.
pub trait Presenter {
    fn show(&mut self, view: &View);
    fn update(&mut self, view: &View);
    fn hide(&mut self);
}

/// Arranges entries on a canvas.
pub trait Layout {
    fn size(&self, view: &View, m: &Metrics) -> Size;
    fn draw(&self, view: &View, canvas: &mut Canvas);
}

/// Turns key presses into switcher actions.
pub trait Mode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action;
    /// Recomputes shown rows and highlights; default does nothing.
    fn refresh(&mut self, s: &mut State) {}
}
```

`EventTx` and `EventRx` (`queue::channel`) share one queue
(`Rc<RefCell<VecDeque<Event>>>`). Adapters and IPC push events with
`EventTx::send`; the event loop drains `EventRx::recv` into the
switcher. Everything runs on one thread, so no locks are needed, and
adapters never see the platform event loop. `SourceError` carries a
message for the log.

`WindowSource` and `Mode` live in `core/traits.rs`. `Layout` lives in
`ui/layouts/` because it draws on a `Canvas`, and the core depends on
nothing else in the crate. `Presenter` is added with its module.

## Switcher state machine

```text
          Show(mode)              Activate / Close
 Hidden ─────────────> Visible ─────────────────────> Hidden
                       │    ▲
                 Key   │    │ Redraw
                       └────┘
```

- `Hidden`: only `Windows` updates the cached list and the pinned
  letters; keys are ignored.
- `Show`: snapshot the cached list with its pinned letters, build
  `State`. A `Show` while visible is a second hotkey press: it closes
  the popup, or with `HotkeyAction::Last` returns `Activate` for
  the previous window.
- `Visible`: keys go to the active `Mode`. `Tab` swaps modes; a mode
  may also switch by setting `state.mode`, and the switcher then calls
  the new mode's `refresh`. A mode returning `Activate` or `Close`
  hides the popup.
- `Windows` while visible: refresh the rows and keep the selection by
  id. Letters of closed windows are not reused until the popup closes.
- `Hide` and `Quit` close an open popup. The daemon loop stops itself
  on `Quit`.
- The currently active window is moved to the end of the rows.

`Switcher<H, S>` is generic over the hotkey and search modes, so calls
are static. After `Show`, `Tab` and `Windows` it calls the mode's
`refresh`, so search can refilter.

## View

`View` borrows the open `State`: mode, query, selected index, and the
shown rows in display order. Each `Row` has an id, key, icon, title
and highlight byte ranges. Layouts read the `View`, never the switcher
state, which keeps rendering replaceable. Building a `View` allocates
nothing.

## Text and languages

Titles can be in any language and script. The core treats them as
Unicode text, never as bytes or ASCII
([ADR-0033](../adr/0033_unicode_text.md)).

- **Encoding:** every string is a Rust `String`, valid UTF-8.
  Adapters convert at the boundary; invalid bytes become `U+FFFD`.
- **Sanitizing:** when a `Windows` event arrives, each title is
  cleaned once: control characters (C0, C1) become spaces, bidi
  override and isolate controls (`U+202A` to `U+202E`, `U+2066` to
  `U+2069`) are removed so a title cannot reorder the row, runs of
  whitespace collapse to one space, and the ends are trimmed. Letters,
  case and accents are never changed.
- **Offsets:** positions in a title (highlights, truncation) are byte
  offsets on `char` boundaries. Highlight ranges in `View` are byte
  ranges.
- **Truncation:** cuts only on `char` boundaries and never between a
  base letter and its combining marks (`U+0300` to `U+036F`, `U+1AB0`
  to `U+1AFF`, `U+1DC0` to `U+1DFF`, `U+20D0` to `U+20FF`, `U+FE20`
  to `U+FE2F`).

Search ([search.md](modes/search.md)) and rendering
([render.md](ui/render.md)) build on these rules.

## Key assignment

`Pins` in `keys.rs` gives each window a letter from the configured
string (default `asdfhjklgqwertyuiop`) and keeps it until the window
closes ([ADR-0041](../adr/0041_pinned_keys.md)). Rows stay in most
recently used order, so letters are not in row order.

- The first window list gets letters in row order: the previous window
  gets `a`, the active window comes last.
- Later, windows without a letter take free letters in key string
  order, most recent window first. A new window gets the first free
  letter.
- A closed window frees its letter. While the popup is open, freed
  letters are held back until it closes, so a letter on screen never
  moves to another window.
- Windows beyond the key string get no letter until one is freed;
  they are reachable with arrows or search.
- Pins live in memory; a daemon restart assigns them again.
