---
status: "accepted"
date: "2026-11-05"
decision-makers: "CanBeSolved"
---
# Input keys

## Context and Problem Statement

`Space` confirms the selected row, the same as `Enter`, so a query
never holds a space ([ADR-0061](0061_prompt_keys.md)). Titles such
as `Release notes` are easier to find with two words, and some users
want `Space` to type. Which keys confirm, and which type a space
(RM-0012)?

## Decision Drivers

- The default keeps `Enter` and `Space` as they are.
- A space is one tap away without moving the hands.
- Every key is set in `[hotkeys]`, in the hotkey notation
  ([ADR-0087](0087_hotkey_notation.md)).

## Considered Options

- `hotkeys.select` and `hotkeys.space`, default a left Shift tap
- One switch, `space_confirms = true | false`
- Keep `Space` as the only confirm key and add nothing

## Decision Outcome

Chosen option: "`hotkeys.select` and `hotkeys.space`, default a left
Shift tap", because the default popup does not change, and any key,
combination or tap can confirm or type a space.

- `hotkeys.select` confirms the selected row. Default
  `["Enter", "Space"]`.
- `hotkeys.space` types a space at the cursor. Default
  `"ShiftLeft"`: a left Shift pressed and released with no other
  key, so `Shift+A` still types `A`.
- Each takes a key, a list or `""`. A key in both stops the daemon
  as a conflict. To type spaces with `Space`, set `select = "Enter"`
  and `space = "Space"`.
- A combination that no binding takes still counts as its plain key,
  so `Ctrl+Enter` confirms while `Enter` does.
- While a title is renamed, `Enter` keeps it and `Space` types, as
  before.

This amends ADR-0061. The body of that record is unchanged.

### Consequences

- Good, because a query can hold spaces without giving up `Space` to
  confirm.
- Good, because the taps already reach the popup: no new protocol
  and no new crate.
- Bad, because a left Shift tap typed by mistake adds a space; a
  stray space changes the matches, and `Backspace` removes it.

### Confirmation

Tests in `src/config/input_keys_tests.rs` and
`src/modes/prompt_keys_tests.rs`.

## Pros and Cons of the Options

### One switch, `space_confirms = true | false`

- Good, because it is one boolean.
- Bad, because no other key can type a space or confirm.

### Keep `Space` as the only confirm key and add nothing

- Good, because nothing changes.
- Bad, because a query cannot hold a space.
