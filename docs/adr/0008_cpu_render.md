---
status: "accepted"
date: "2026-10-15"
decision-makers: "CanBeSolved"
---
# Render on the CPU

## Context and Problem Statement

Without a toolkit (see [ADR-0007](0007_layer_shell.md)) `tynex` draws
its own rows, text and icons. Should it draw on the CPU or the GPU?

## Decision Drivers

- First frame well under 16 ms for a small surface.
- No GPU context or VRAM.
- Same code on every OS.

## Considered Options

- CPU rasterizing into a shared memory buffer
- GPU rendering (`wgpu`, OpenGL)

## Decision Outcome

Chosen option: "CPU rasterizing into a shared memory buffer", because
a 480 x 600 redraw takes well under 1 ms, needs no driver, and the
RGBA buffer can be handed to any presenter. The libraries are chosen
in [ADR-0025](0025_rendering.md).

### Consequences

- Good, because there is no GPU wake-up or VRAM use.
- Good, because the renderer is written once for all platforms.
- Bad, because large or animated surfaces would be slow; `tynex` has
  neither.

## Pros and Cons of the Options

### GPU rendering

- Good, because it scales to large animated surfaces.
- Bad, because context creation costs time and memory for no gain here.

## More Information

See [render.md](../arch/ui/render.md).
