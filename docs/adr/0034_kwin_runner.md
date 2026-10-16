---
status: "accepted"
date: "2026-10-16"
decision-makers: "CanBeSolved"
---
# KWin bridge with runner activation

## Context and Problem Statement

[ADR-0010](0010_kwin_bridge.md) activates windows by answering a
pending `Poll` call from the KWin script, because a script cannot
receive D-Bus calls. D-Bus calls time out after 25 s, so the daemon
must answer `noop` every 20 s: the only wakeup of an idle daemon, and
held replies make the D-Bus code more complex. KWin also exposes its
window search over D-Bus at `org.kde.KWin /WindowsRunner`. Can the
daemon activate windows without the script?

## Decision Drivers

- No wakeups while idle.
- Simplest D-Bus code: no held replies or deadlines.
- Activation latency of a few ms.

## Considered Options

- Script for the list and shortcut, `WindowsRunner.Run` for activation
- Script with a pending `Poll` call (ADR-0010)
- `WindowsRunner` only, without a script

## Decision Outcome

Chosen option: "Script for the list and shortcut, `WindowsRunner.Run`
for activation", because the daemon calls KWin directly to activate a
window, so `Poll`, its 20 s refresh and held replies are gone.

- The script pushes the window list in most recently used order with
  `callDBus` and registers the global shortcut, as before.
- `activate(id)` calls
  `org.kde.krunner1.Run("0_{<uuid>}", "")` on
  `org.kde.KWin /WindowsRunner`, where `<uuid>` is the window's
  `internalId` from the list.
- On start the adapter checks that `/WindowsRunner` exists and fails
  with a clear error if not. There is no fallback to `Poll`.

Verified on Plasma 6.7: `Match("")` lists all windows with ids
`0_{<uuid>}` matching the script's `internalId`.

### Consequences

- Good, because an idle daemon has no wakeups at all.
- Good, because the script gets simpler and the D-Bus layer needs no
  held replies.
- Bad, because `/WindowsRunner` is KWin's search plugin endpoint, not
  a documented API; a KWin change can break activation.

### Confirmation

Manual test on Plasma 6: activation switches windows, and `powertop`
shows no wakeups from `tynex` while hidden.

## Pros and Cons of the Options

### Script with a pending `Poll` call

- Good, because it uses only the documented scripting API.
- Bad, because of the 20 s wakeup and held replies.

### `WindowsRunner` only, without a script

- Good, because no script is loaded.
- Bad, because results are not in most recently used order, carry a
  25 KB icon per window, and need polling to stay current; the global
  shortcut still needs a script or `kglobalaccel`.

## More Information

Supersedes [ADR-0010](0010_kwin_bridge.md). See
[kde.md](../arch/adapters/kde.md).
