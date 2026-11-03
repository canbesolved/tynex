---
status: "accepted"
date: "2026-11-03"
decision-makers: "CanBeSolved"
---
# US keymap for popup keys

## Context and Problem Statement

Users who switch between a Latin and a non-Latin layout (Russian,
Greek, ...) often open the popup with the other layout active and
type `ащч` instead of `fox`. RM-0044 asks for `main.switch_layout`:
read keys with the US layout while the popup is open, whatever the
system layout is, on every desktop and OS. How should tynex get US
characters?

## Decision Drivers

- Nothing to restore: a crash or a lost focus must not leave the
  system on another layout.
- Portable: no desktop-specific layout API (KDE D-Bus, GNOME
  settings, `swaymsg`), and one US table for every OS.
- Footprint: no new crate, no work while idle or with the key off.
- User remaps of special keys (`caps:escape`) keep working.

## Considered Options

- Switch the system layout on show and back on hide
- Compile a second, US keymap with `xkbcommon`
- A US table in the core, keyed by physical key position

## Decision Outcome

Chosen option: "A US table in the core, keyed by physical key
position", because it changes no system state, is shared by every
platform and needs no XKB data.

- `core::us_layout::PhysicalKey` names the 47 character keys of a US
  keyboard after W3C `KeyboardEvent.code` (`KeyA`, `Digit1`,
  `Period`, ...). `us_char(key, shift, caps)` returns the US
  character; Caps Lock acts on letters only.
- Each platform maps its native key numbers to `PhysicalKey`: evdev
  on Linux, later virtual key codes on macOS and scan codes on
  Windows.
- A key uses the table when the system layout gives it a character
  or a dead key. Other keys (modifiers, `Enter`, arrows, keypad, `F`
  keys, remapped keys) keep the system layout. Shift and Caps Lock
  come from the system state. With `Ctrl`, `Alt` or `Meta`, the key
  is a combination of the unshifted US character.

### Consequences

- Good, because the desktop's layout and its indicator never change.
- Good, because macOS and Windows get the feature with a key number
  table, and the US table is tested without XKB data.
- Good, because it costs no memory or start time beyond a small
  table, also with the key on.
- Bad, because AltGr characters of the system layout (`ą`, `ß`) and
  the 102nd key (`IntlBackslash`) cannot be typed while the key is
  on.
- Bad, because tynex keeps its own US table instead of reading it
  from the system. The US layout is stable, so it rarely changes.

### Confirmation

Tests in `src/core/us_layout_tests.rs` cover the table. Tests in
`src/platform/linux/wayland/keyboard_us_tests.rs` use real `gr`,
`de`, `ru` and `caps:escape` keymaps.

## Pros and Cons of the Options

### Switch the system layout on show and back on hide

- Good, because AltGr and every key follow a real layout.
- Bad, because each desktop needs its own API, and some have none.
- Bad, because the layout indicator flickers, and a crash leaves the
  wrong layout active.

### Compile a second, US keymap with `xkbcommon`

- Good, because the US characters come from the system's XKB data.
- Bad, because it is Linux only: each OS needs its own way.
- Bad, because it costs some hundred KB of RAM and about a
  millisecond at start, and fails without US XKB data.

## More Information

The Latin fallback for letter hotkeys (`Key::Char.latin`) stays; with
the US table it is the US letter.
