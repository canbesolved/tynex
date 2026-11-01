#!/bin/sh
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT

# Builds the zip uploaded to the Chrome Web Store. The store listing
# itself is not created here.

set -eu

cd "$(dirname "$0")"
mkdir -p dist
rm -f dist/tynex-browser-tabs.zip
zip -qr dist/tynex-browser-tabs.zip \
    manifest.json lib.js background.js icons
