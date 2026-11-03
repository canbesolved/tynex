#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT
# Prints the lowest free roadmap code (RM-NNNN).
# A code is free when it is in none of ROADMAP.md, CHANGELOG.md and
# DEPRECATIONS.md. Usage: scripts/rm_next.sh

set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
files=("$root/ROADMAP.md" "$root/CHANGELOG.md" "$root/DEPRECATIONS.md")

# Used numbers, one per line, without leading zeros.
used="$(grep -ohE 'RM-[0-9]{4,}' "${files[@]}" \
    | sed -E 's/^RM-0*//; s/^$/0/' | sort -nu || true)"

next=1
while grep -qx "$next" <<< "$used"; do
    next=$((next + 1))
done

printf 'RM-%04d\n' "$next"
