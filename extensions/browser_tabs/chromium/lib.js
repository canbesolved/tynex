// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

const MAX_TITLE = 256;
const MAX_URL = 1024;

function cut(text, max) {
    const chars = Array.from(String(text));

    return chars.length <= max ? String(text) : chars.slice(0, max).join("");
}

function clean(text) {
    return String(text).toWellFormed();
}

function iconKey(url) {
    try {
        const parsed = new URL(url);

        return parsed.origin === "null" ? parsed.protocol : parsed.origin;
    } catch {
        return "";
    }
}

function buildTabs(tabs) {
    return tabs
        .filter((tab) => !tab.incognito && tab.id !== -1)
        .map((tab) => ({
            id: tab.id,
            window: tab.windowId,
            active: Boolean(tab.active),
            last: tab.lastAccessed || 0,
            title: cut(clean(tab.title || ""), MAX_TITLE),
            url: cut(clean(tab.url || tab.pendingUrl || ""), MAX_URL),
            icon: iconKey(tab.url || tab.pendingUrl || ""),
        }));
}

function toBase64(bytes) {
    let text = "";

    for (const byte of bytes) {
        text += String.fromCharCode(byte);
    }
    return btoa(text);
}

globalThis.tynexLib = { cut, clean, iconKey, buildTabs, toBase64 };
