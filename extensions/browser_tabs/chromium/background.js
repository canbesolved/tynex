// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

importScripts("lib.js");

const HOST = "io.github.canbesolved.tynex";
const MAX_ICONS = 512; // Daemon cap per connection
const BATCH = 64; // Icons per message
const MAX_ICON_BYTES = 16 * 1024;
const lib = globalThis.tynexLib;
const sentIcons = new Set(); // Keys sent since the daemon connected
let port = null;
let delay = 1000;

function badge(text, color) {
    chrome.action.setBadgeText({ text });
    chrome.action.setBadgeBackgroundColor({ color });
}

function connect() {
    if (port) {
        return;
    }
    try {
        port = chrome.runtime.connectNative(HOST);
    } catch {
        badge("!", "#c0392b");
        retry();
        return;
    }
    badge("…", "#7f8c8d");
    port.onMessage.addListener(onMessage);
    port.onDisconnect.addListener(() => {
        void chrome.runtime.lastError; // Host missing or exited
        port = null;
        badge("!", "#c0392b");
        retry();
    });
}

function retry() {
    setTimeout(connect, delay);
    delay = Math.min(delay * 2, 60000);
}

function onMessage(msg) {
    if (!msg || msg.v !== 1) {
        return;
    }
    switch (msg.type) {
        case "status":
            if (msg.daemon) {
                sentIcons.clear(); // A new daemon has no icons
                badge("", "#000000");
                delay = 1000;
            } else {
                badge("…", "#7f8c8d");
            }
            break;
        case "list":
            list(msg.req).catch(() => {});
            break;
        case "activate":
            activate(msg.req, msg.id);
            break;
    }
}

function post(msg) {
    if (port) {
        port.postMessage(msg);
    }
}

async function list(req) {
    const tabs = lib.buildTabs(await chrome.tabs.query({}));

    post({ v: 1, type: "tabs", req, browser: browserName(), tabs });
    await sendIcons(tabs);
}

function browserName() {
    const brands = navigator.userAgentData
        ? navigator.userAgentData.brands
        : [];
    const brand = brands.find(
        (item) => item.brand !== "Chromium" && !item.brand.startsWith("Not"),
    );

    return brand ? brand.brand : "Chromium";
}

async function sendIcons(tabs) {
    const keys = [];

    for (const tab of tabs) {
        if (sentIcons.size >= MAX_ICONS) {
            break;
        }
        if (tab.icon && !sentIcons.has(tab.icon)) {
            sentIcons.add(tab.icon); // A key that failed is not retried
            keys.push(tab.icon);
        }
    }
    for (let i = 0; i < keys.length; i += BATCH) {
        const part = keys.slice(i, i + BATCH);
        const pngs = await Promise.all(part.map(fetchIcon));
        const icons = part
            .map((key, j) => ({ key, png: pngs[j] }))
            .filter((icon) => icon.png);

        if (icons.length) {
            post({ v: 1, type: "icons", icons });
        }
    }
}

async function fetchIcon(origin) {
    const page = encodeURIComponent(origin);
    const url =
        `chrome-extension://${chrome.runtime.id}/_favicon/` +
        `?pageUrl=${page}&size=32`;

    try {
        const response = await fetch(url);

        if (!response.ok) {
            return null;
        }
        const bytes = new Uint8Array(await response.arrayBuffer());

        return bytes.length > MAX_ICON_BYTES ? null : lib.toBase64(bytes);
    } catch {
        return null;
    }
}

async function activate(req, id) {
    try {
        const tab = await chrome.tabs.get(id);

        await chrome.tabs.update(id, { active: true });
        const win = await chrome.windows.get(tab.windowId);
        const update = { focused: true };

        if (win.state === "minimized") {
            update.state = "normal";
        }
        await chrome.windows.update(tab.windowId, update);
        post({ v: 1, type: "activated", req, ok: true, window: tab.windowId });
    } catch {
        post({ v: 1, type: "activated", req, ok: false, window: -1 });
    }
}

chrome.alarms.create("tynex", { periodInMinutes: 1 });
chrome.alarms.onAlarm.addListener(connect);
connect();
