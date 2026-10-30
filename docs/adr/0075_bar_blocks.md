---
status: "accepted"
date: "2026-10-30"
decision-makers: "CanBeSolved"
---
# Separate bar blocks

## Context and Problem Statement

The input bar and the icon strip are already separate rounded
blocks, with a transparent gap between them. The title list in
`titlebar` mode, and the output bar while typing, still sit inside
the input frame under a 1 px line. How should every bar look?

## Decision Drivers

- Each UI module is its own block.
- Every block uses the same rounded border and theme colors.
- No new config key and no new crate.

## Considered Options

- Each visible bar is its own rounded block
- Keep the 1 px separator inside the input frame

## Decision Outcome

Chosen option: "Each visible bar is its own rounded block", because
the icon strip already looks that way and the other bars should
match.

- The input bar, the icon strip, the title list and the output bar
  are each a rounded block. The block is the theme border outside
  and the theme background inside, with the theme radius.
- A transparent gap of 3 logical pixels separates adjacent blocks.
  It scales with the display. The desktop shows through.
- An empty query with `taskbar.mode = "off"` is the input block
  alone.
- The dropdown keeps its own rounded panel, in the same colors. It
  still opens over the title list or in the spare band.

### Consequences

- Good, because every bar shares one frame and one gap.
- Good, because a later bar, such as the info bar, uses the same
  block.
- Bad, because `titlebar` mode and a non-empty query make the popup
  taller by the gap and the block padding.

### Confirmation

`src/ui/frames.rs` draws each bar with one `frame` helper. A pixel
test checks that the gap under the input block is clear, and that
the lower block's top edge is the theme border, in `titlebar` mode
and while typing.

## Pros and Cons of the Options

### Each visible bar is its own rounded block

- Good, because the bars match the icon strip.
- Good, because the gap is already used between the input and the
  icons.
- Bad, because the popup is taller in `titlebar` mode and while
  searching.

### Keep the 1 px separator inside the input frame

- Good, because the popup stays shorter.
- Bad, because the title list and the output bar do not match the
  icon strip.
- Bad, because one frame holds two modules.

## More Information

Amends how bars are drawn in [ADR-0052](0052_ui_modules.md) and
[ADR-0054](0054_output_bar.md). Those records still decide which
module shows what. The gap matches
[ADR-0071](0071_dropdown_overlay.md). The item is 0.2.14 in
[ROADMAP.md](../../ROADMAP.md).
