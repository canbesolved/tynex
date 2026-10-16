# Hotkey mode

Source: `src/modes/hotkey.rs`. Default mode.

## Behavior

| Key            | Action                              |
| -------------- | ----------------------------------- |
| Assigned key   | Activate that window, close         |
| `Up` / `Down`  | Move selection                      |
| `Enter`        | Activate selected window, close     |
| `/` or `Tab`   | Switch to search mode               |
| `Esc`          | Close without switching             |
| Other keys     | Ignored                             |

## Notes

- Keys come from `keys.rs` in the core, default order
  `asdfhjklgqwertyuiop`: home row first.
- The first key (`a`) points to the previously used window, so
  hotkey then `a` acts like a quick Alt+Tab.
- A key matches by the produced character first (`Key::Char.text`),
  then by the same physical key in the first Latin layout
  (`Key::Char.latin`). With a Russian layout active, the key that
  gives `ф` still selects the window marked `a`. Shift does not matter:
  `S` selects the window marked `s`.
- `Up` and `Down` wrap around the list; `Enter` on an empty list does
  nothing.
- `/` sets the state to search mode; the switcher then refreshes
  search so it can filter right away.
