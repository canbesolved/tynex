// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

// KWin script for tynex: pushes the window list to the daemon in most
// recently used order and registers the shortcuts that show the
// switcher and switch back. Loaded by the daemon; see
// docs/arch/adapters/kde.md.

const SERVICE = "io.github.canbesolved.Tynex";
const PATH = "/io/github/canbesolved/Tynex/Kde";
const INTERFACE = "io.github.canbesolved.Tynex.Kde";
const RECORD = "\x1e";
const FIELD = "\x1f";

let mru = []; // Most recently used first

function id(w) {
    return String(w.internalId);
}

function switchable(w) {
    return (w.normalWindow || w.dialog) && !w.skipSwitcher &&
        !w.specialWindow && !w.popupWindow;
}

// Separators would split a record, so they never reach the daemon.
function clean(value) {
    return String(value || "").replace(/[\x1e\x1f]/g, "");
}

function push() {
    const records = mru.filter(switchable).map((w) => [
        clean(id(w)),
        clean(w.desktopFileName),
        clean(w.resourceClass),
        clean(w.caption),
        String(w.pid || 0),
    ].join(FIELD));

    callDBus(SERVICE, PATH, INTERFACE, "Update", records.join(RECORD));
}

function without(w) {
    return mru.filter((x) => id(x) !== id(w));
}

// KWin may report a new window as activated before it reports it added.
function added(w) {
    if (!mru.some((x) => id(x) === id(w))) {
        mru.push(w); // Moves to the front once activated
    }
    w.captionChanged.connect(push);
    push();
}

function removed(w) {
    mru = without(w);
    push();
}

function activated(w) {
    if (w) {
        mru = [w].concat(without(w));
    }
    push();
}

// Start from the stacking order, topmost first, as the closest MRU guess.
mru = workspace.stackingOrder.slice().reverse();
mru.forEach((w) => w.captionChanged.connect(push));
workspace.windowAdded.connect(added);
workspace.windowRemoved.connect(removed);
workspace.windowActivated.connect(activated);
registerShortcut("tynex", "tynex: Show switcher", "Meta+Shift+F11", () => {
    callDBus(SERVICE, PATH, INTERFACE, "Show");
});
registerShortcut("tynex_quick_switch", "tynex: Quick switch",
    "Meta+Shift+F12", () => {
    callDBus(SERVICE, PATH, INTERFACE, "QuickSwitch");
});
activated(workspace.activeWindow);
