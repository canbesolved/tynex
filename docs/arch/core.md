# Core

Source: `src/core/`. Pure logic: no I/O, no OS types, no threads. Fully
unit-testable and identical on every platform.

## Files

```text
core/
  window.rs     WindowId, WindowInfo
  app.rs        AppInfo: id, name, icon (no Exec)
  event.rs      Event, Action, Key
  traits.rs     WindowSource, Presenter, Mode
  state.rs      State of an open popup, shared with the modes
  switcher.rs   State machine (tests in switcher_tests.rs)
  view.rs       View, Row, RowTarget: what the presenter must draw
  order.rs      SortBy: row order (recently, app)
  icons.rs      One icon per running app, dropdown title order
  queue.rs      EventTx / EventRx: single-threaded event queue
  text.rs       Title sanitizing and safe cut points
  title.rs      Row titles: sanitizing and app name stripping
```

## Types

```rust
/// Opaque id given by the adapter (KWin uses a UUID string).
pub struct WindowId(pub String);

/// One switchable window.
pub struct WindowInfo {
    pub id: WindowId,
    pub app_id: String,  // Desktop file name or WM class
    pub app_name: String, // Shown name, from a desktop file or WM class
    pub title: String,
    pub icon: Option<String>, // Icon name or path
}

/// Inputs to the core.
pub enum Event {
    Windows(Vec<WindowInfo>), // Full list, most recently used first
    Show,
    Key(Key),
    Click(Click),
    Hover(Click),
    Hide,
    Quit,
}

/// Outputs of the core.
pub enum Action {
    None,
    Redraw,
    Activate(WindowId),
    Launch(String), // App id
    Close,
}
```

`Click` is where a pointer press landed: `Icon(index)` on the strip,
`Menu(index)` on an open dropdown, or `Elsewhere` in the popup.
`Hover` carries the same place while the pointer moves. The icon
under the pointer becomes selected.

`AppInfo` is `id`, `name`, `icon`. `Exec` stays on the platform.
A `Row` has a `RowTarget`: `Window` or `Launch`. `State` keeps open
windows in `rows` (title bar) and the result list in `output`
(output bar and selection).

`Key` is a portable enum (`Char { text, latin }`, `Enter`, `Esc`,
`Tab`, `BackTab`, `Up`, `Down`, `Left`, `Right`, `Home`, `End`,
`Backspace`, `Delete`, `WordLeft`, `WordRight`). `BackTab` is
`Shift+Tab`. Each platform maps native key
events to it:

- `text`: the character the active layout produces, after dead keys
  and compose (`α` on a Greek layout, `é` with a French one).
- `latin`: the character of the same physical key in the first Latin
  layout of the keymap, if any (`a` for the key that gives `α`). No
  mode uses it since hotkey mode was removed.

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

/// Turns key presses into switcher actions.
pub trait Mode {
    fn on_key(&mut self, key: Key, s: &mut State) -> Action;
    fn on_click(&mut self, click: Click, s: &mut State) -> Action;
    fn on_hover(&mut self, click: Click, s: &mut State) -> Action;
    /// Recomputes output rows and highlights; default does nothing.
    fn refresh(&mut self, s: &mut State) {}
}
```

`EventTx` and `EventRx` (`queue::channel`) share one queue
(`Rc<RefCell<VecDeque<Event>>>`). Adapters and IPC push events with
`EventTx::send`; the event loop drains `EventRx::recv` into the
switcher. Everything runs on one thread, so no locks are needed, and
adapters never see the platform event loop. `SourceError` carries a
message for the log.

`WindowSource` and `Mode` live in `core/traits.rs`. The popup is drawn
by UI modules in `ui/` ([input bar](ui/input_bar.md),
[taskbar](ui/taskbar.md)), which read a `View` and never the
switcher. The core depends on nothing else in the crate. `Presenter`
is added with its module.

## Switcher state machine

```text
          Show                 Activate / Launch / Close
 Hidden ─────────────> Visible ─────────────────────────> Hidden
                       │    ▲
                 Key   │    │ Redraw
                       └────┘
```

- `Hidden`: only `Windows` updates the cached list; keys are ignored.
  Each title in the cache is replaced by its row title (see
  [Row title](#row-title)).
- `Show`: snapshot the cached list, build `State` with the rows in
  `sort_by` order (see [Row order](#row-order)), and select the
  previous window. A `Show` while visible is a second hotkey press: it
  closes the popup, or with `HotkeyAction::Last` returns `Activate`
  for the previous window.
- `Visible`: keys go to the mode. A mode returning `Activate`,
  `Launch` or `Close` hides the popup.
- `Windows` while visible: refresh the rows, keep the query, the
  cursor and the selection by id.
- `Hide` and `Quit` close an open popup. The daemon loop stops itself
  on `Quit`.

`Switcher<M>` is generic over its mode, prompt
([ADR-0050](../adr/0050_search_only.md)), so calls are static. After
`Show` and `Windows` it calls the mode's `refresh`, so prompt can
refilter.

## View

`View` borrows the open `State`: query, cursor, selected index, the
open windows, the output rows, the running-app icons, the selected
icon, and whether a dropdown is open. Each `Row` has an id, an
icon, row text (`title`), an optional title color, highlight byte
ranges, and the app name and id for search. A row with no title
color uses the theme text color. `windows` is every open window,
for the title list.
`rows` is the current result, for the output bar: the matches while
a query is typed. The title list is shown when `taskbar.mode` is
`titlebar` or `both` and the query is empty. Popup modules read the
`View`, never the switcher state, which keeps rendering
replaceable. Building a
`View` allocates nothing.

## Row title

`row_title` in `title.rs` gives each window the title its row shows,
with `taskbar.titlebar.strip_app_name` from the config
([config.md](config.md#format)):

1. Sanitize the title and the app name (see below).
2. With `strip_app_name`, drop `<app>` after ` — `, ` – ` or ` - `
   from the end of the title when `<app>` is the app name or the last
   part of the app id, ignoring case; never down to an empty title.

The switcher applies it on each `Windows` event. The app name stays a
separate field of `Row` for search. The title list draws the icon
when `show_icons` is set, then the title. The output bar always draws the
icon and the title.

## Text and languages

Titles can be in any language and script. The core treats them as
Unicode text, never as bytes or ASCII
([ADR-0033](../adr/0033_unicode_text.md)).

- **Encoding:** every string is a Rust `String`, valid UTF-8.
  Adapters convert at the boundary; invalid bytes become `U+FFFD`.
- **Sanitizing:** when a `Windows` event arrives, each title and app
  name is cleaned once: control characters (C0, C1) become spaces, bidi
  override and isolate controls (`U+202A` to `U+202E`, `U+2066` to
  `U+2069`) are removed so a title cannot reorder the row, invisible
  characters that fonts draw as boxes are removed (bidi marks `U+061C`,
  `U+200E`, `U+200F`; zero width space `U+200B`; word joiner and
  invisible operators `U+2060` to `U+2064`; byte order mark `U+FEFF`),
  runs of whitespace collapse to one space, and the ends are trimmed.
  Letters, case and accents are never changed.
- **Offsets:** positions in a title (highlights, truncation) are byte
  offsets on `char` boundaries. Highlight ranges in `View` are byte
  ranges.
- **Truncation:** cuts only on `char` boundaries and never between a
  base letter and its combining marks (`U+0300` to `U+036F`, `U+1AB0`
  to `U+1AFF`, `U+1DC0` to `U+1DFF`, `U+20D0` to `U+20FF`, `U+FE20`
  to `U+FE2F`).

Prompt mode ([prompt.md](modes/prompt.md)), search
([modules/search.md](modules/search.md)) and rendering
([render.md](ui/render.md)) build on these rules.

## Row order

`order` in `order.rs` arranges the rows by
`taskbar.titlebar.sort_by` from the config
([ADR-0048](../adr/0048_row_order.md), amended by
[ADR-0050](../adr/0050_search_only.md)). Every order starts from the
most recently used order with the active window last, and sorts
stably from there:

- `recently` (default): no sorting; the previous window comes first
  and the active window last.
- `app`: by app name, ignoring case, then by recent use.

The popup opens with the previous window selected in every order, so
`Enter` or `Space` switches back like Alt+Tab when the title list is
on, the icon strip is off, or that app has one window
([ADR-0061](../adr/0061_prompt_keys.md),
[ADR-0064](../adr/0064_title_priority.md)). The icon strip uses most
recently used order on its own, not `sort_by`. With the title list
off and the strip showing, `Enter` on an app that has several
windows opens its dropdown ([ADR-0062](../adr/0062_icon_bar.md)).
