---
status: "proposed"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# Hotkey notation

## Context and Problem Statement

Planned `[hotkeys]` keys bind keys to actions: `rename`, `new_tab`,
`select` and `space` (RM-0001, RM-0003, RM-0012). They need one way
to write a key or a key combination in `tynex.toml` that users know
from other programs, that parses the same on every platform, and
that can name a single modifier such as the left Shift. Which
notation do we use?

## Decision Drivers

- Familiar: users have seen it in desktop and editor settings.
- One notation for every platform (Linux, macOS, Windows ports).
- Can name a key alone, a combination, and the left or right side
  of a modifier.
- Easy to parse and validate without a new crate.

## Considered Options

- Accelerator syntax with W3C UI Events key names
- GTK accelerator syntax (`<Control><Shift>t`)
- XKB keysym names (`Shift_L`, `period`)
- Free text, parsed loosely

## Decision Outcome

Chosen option: "Accelerator syntax with W3C UI Events key names",
because it is the `Modifier+Key` form used by KDE and Qt
(`QKeySequence`), Electron, VS Code and browsers, and the key names
come from a published standard that is not tied to one platform.

Syntax:

- A binding is modifiers and one key joined by `+`:
  `"Ctrl+Shift+T"`, `"Meta+Space"`. Spaces around `+` are allowed:
  `"Meta + Space"` is the same binding.
- Matching is case-insensitive for names: `"meta+space"` works.
- Modifiers: `Ctrl`, `Alt`, `Shift`, `Meta`. `Super` and `Win` are
  accepted as `Meta`, `Control` as `Ctrl`.
- Key names follow the W3C UI Events `KeyboardEvent.code` values for
  named keys: `Space`, `Enter`, `Tab`, `Escape`, `Backspace`,
  `Delete`, `F1` to `F24`, `ArrowUp`, `Home`, and the sided
  modifiers `ShiftLeft`, `ShiftRight`, `ControlLeft`, `AltLeft`,
  `MetaLeft` and their `Right` forms.
- A single printable character is the character the key types,
  for example `"."` or `","`. It follows the active layout, with the
  same Latin fallback as other keys.
- A modifier alone, such as `"ShiftLeft"`, fires when it is pressed
  and released with no other key in between, so `Shift+A` still
  types `A`.
- A `[hotkeys]` key takes one binding or a list of bindings, like
  `browser_tabs.browsers` takes one name or a list:
  `select = ["Enter", "Space"]`. Any binding in the list fires the
  action, as VS Code and Qt allow several shortcuts per action.
- An empty string `""` or an empty list `[]` turns the action off.
- A binding used by two actions, for example `Space` in both
  `select` and `space`, stops the daemon with an error that names
  both keys.
- An unknown name or a malformed binding stops the daemon with the
  line and column, like other invalid values.

Planned defaults in this notation: `rename = "."`,
`select = ["Enter", "Space"]`, `space = "ShiftLeft"`.

### Consequences

- Good, because one parser serves every `[hotkeys]` key and every
  platform port.
- Good, because the KWin shortcut (`"Meta+Tab"`) already uses this
  form.
- Bad, because W3C names for sided modifiers (`ShiftLeft`) differ
  from XKB (`Shift_L`); the docs list the accepted names.

### Confirmation

Unit tests in the config module parse every documented name, the
aliases, spaces around `+`, case, and reject unknown names.

## Pros and Cons of the Options

### Accelerator syntax with W3C UI Events key names

- Good, because it is the most widely seen form.
- Good, because W3C names cover sided modifiers.
- Neutral, because Qt and W3C name a few keys differently (`Esc`
  and `Escape`); aliases cover the common ones.

### GTK accelerator syntax (`<Control><Shift>t`)

- Good, because GTK and GNOME settings use it.
- Bad, because it is less known outside GNOME and harder to read.

### XKB keysym names (`Shift_L`, `period`)

- Good, because the Linux keyboard stack uses them directly.
- Bad, because they are Linux-only and unfamiliar to most users.

### Free text, parsed loosely

- Good, because users can write anything.
- Bad, because errors are silent and behavior differs by guess.

## More Information

- W3C UI Events KeyboardEvent code values:
  <https://www.w3.org/TR/uievents-code/>.
- Accept once the first `[hotkeys]` key binding is implemented, and
  update `docs/arch/config.md` in that change.
