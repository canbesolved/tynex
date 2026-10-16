#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT
# Regenerates the ADR log in docs/adr/index.md with adr-log.

set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"

cd "$root/docs/adr"
adr-log -d . -e adr_template.md -i index.md > /dev/null

# Trim trailing blank lines that adr-log appends on every run.
printf '%s\n' "$(< index.md)" > index.md
echo "Updated docs/adr/index.md"
