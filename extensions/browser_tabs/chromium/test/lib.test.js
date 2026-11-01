// SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
// SPDX-License-Identifier: MIT

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { createContext, runInContext } from "node:vm";

const sandbox = { URL, btoa };
const context = createContext(sandbox);

sandbox.globalThis = sandbox;
runInContext(
    readFileSync(new URL("../lib.js", import.meta.url), "utf8"),
    context,
);

const { cut, clean, iconKey, buildTabs, toBase64 } = sandbox.tynexLib;

test("cut stops on code points", () => {
    assert.equal(cut("café", 3), "caf");
    assert.equal(cut("ab", 5), "ab");
});

test("clean keeps well-formed text", () => {
    assert.equal(clean("a\uD800"), "a\uFFFD");
});

test("iconKey uses the origin", () => {
    assert.equal(
        iconKey("https://www.example.com/a"),
        "https://www.example.com",
    );
    assert.equal(iconKey("about:blank"), "about:");
    assert.equal(iconKey("not a url"), "");
});

test("buildTabs drops private and id -1", () => {
    const tabs = buildTabs([
        {
            id: -1,
            windowId: 1,
            title: "x",
            url: "https://a.example",
        },
        {
            id: 2,
            windowId: 3,
            incognito: true,
            title: "secret",
            url: "https://b.example",
        },
        {
            id: 4,
            windowId: 3,
            active: true,
            title: "Notes",
            url: "https://notes.example/a",
            lastAccessed: 9,
        },
    ]);

    assert.equal(tabs.length, 1);
    assert.equal(tabs[0].id, 4);
    assert.equal(tabs[0].icon, "https://notes.example");
    assert.equal(tabs[0].last, 9);
});

test("toBase64 encodes bytes", () => {
    assert.equal(toBase64(new Uint8Array([97])), "YQ==");
});
