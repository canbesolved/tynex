# Input bar

Source: `src/ui/input_bar.rs`. The input bar at the top of the popup.
Typing starts here.

Prompt mode handles keys and the query. This module only draws the
bar. The mode is [modes/prompt.md](../modes/prompt.md). Sub-modules
that read the query live under [modules/input.md](../modules/input.md)
([ADR-0056](../../adr/0056_launcher_input.md)): search matches open
windows; the launcher matches installed apps. Neither draws.

## Look

The input bar is always on. It cannot be disabled. The
[output bar](output_bar.md) shows the result of what is typed here.
The title bar, the info bar and the icon bar can be turned off on
their own ([ADR-0053](../../adr/0053_ui_module_toggles.md)). Config
sections follow the modules
([ADR-0055](../../adr/0055_config_sections.md)).

```text
> firef
```

`>` is fixed. A row gap and one space separate it from the query.
The rest is the query, empty until you type. While it is empty, the
hint is drawn there in `hint.font_color`, at the same size as the
query. Typing hides the hint. A 1 px beam in the text color stands
at the cursor, 10% shorter than the text, including while the query
is empty. `Left` and `Right` move it. It does not blink.

The hint text is the first line of `hint.text`, a shell command run
when the popup opens ([ADR-0060](../../adr/0060_input_hint.md)).
Until that command exits, the hint steps every 400 ms through `.`,
`..`, `...`, and blank. The popup is drawn before the command
finishes. `hint.update` is the wait, in seconds, from that exit
until the next run. `0` runs once per open. Hiding the popup leaves
a running command alone; the next open does not start another until
it exits.

## Rules

- One row high: 2 x font size, times the output scale.
- The painter in `src/ui/painter.rs` draws the frame and the padding
  around this bar, then the output bar when the query is non-empty,
  then the [title bar](title_bar.md) when it is enabled and the query
  is empty.
