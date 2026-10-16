#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT
# Creates a new MADR record in docs/adr/ and refreshes the ADR log.
# Usage: scripts/new_adr.sh "Decision title" [short_slug]
# Keep slugs short so docs/adr/index.md lines fit in 80 columns.

set -euo pipefail
set -o noclobber

root="$(cd "$(dirname "$0")/.." && pwd)"
adr_dir="$root/docs/adr"
template="$adr_dir/adr_template.md"

if [[ $# -lt 1 || $# -gt 2 || -z "$1" ]]; then
    echo "Usage: $0 \"Decision title\" [short_slug]" >&2
    exit 1
fi

title="$1"

# Next number: highest existing NNNN prefix plus one.
last="$(find "$adr_dir" -maxdepth 1 -name '[0-9][0-9][0-9][0-9]_*.md' \
    -printf '%f\n' | sort | tail -n 1 | cut -c1-4)"
next="$(printf '%04d' $((10#${last:-0} + 1)))"

# Slug: lowercase, non-alphanumerics to single underscores.
slug="$(printf '%s' "${2:-$title}" | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9]+/_/g; s/^_+|_+$//g')"
if [[ -z "$slug" ]]; then
    echo "Slug is empty: pass a slug with letters or digits" >&2
    exit 1
fi

file="$adr_dir/${next}_${slug}.md"

# Fill title, status and date from the template.
safe_title="$(printf '%s' "$title" | sed 's/[&/\]/\\&/g')"
sed -e '/^# MADR 4.0.0 template/d' \
    -e "s/^# {Short title.*}$/# $safe_title/" \
    -e 's/^status: .*/status: "proposed"/' \
    -e "s/^date: .*/date: \"$(date +%F)\"/" \
    "$template" > "$file"

echo "Created ${file#"$root"/}"

# Refresh the ADR log when adr-log is installed.
if command -v adr-log > /dev/null; then
    "$root/scripts/adr_log.sh"
else
    echo "adr-log not found: index not updated" >&2
fi
