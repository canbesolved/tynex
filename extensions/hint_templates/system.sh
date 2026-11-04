#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT
# Hint template: CPU load, memory in use and uptime, from /proc.

read -r load _ < /proc/loadavg

while read -r key value _; do
    case "$key" in
        MemTotal:) total=$value ;;
        MemAvailable:) available=$value ;;
    esac
done < /proc/meminfo

used=$(((total - available) * 10 / 1048576)) # Tenths of a GiB
all=$((total * 10 / 1048576))

read -r up _ < /proc/uptime
up=${up%.*}

printf 'CPU %s | RAM %d.%d/%d.%d GiB | Up %dh %02dm\n' "$load" \
    $((used / 10)) $((used % 10)) $((all / 10)) $((all % 10)) \
    $((up / 3600)) $((up % 3600 / 60))
