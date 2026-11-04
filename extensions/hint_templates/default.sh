#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT
# Default input bar hint: date, time and battery level. The daemon
# writes it to ~/.local/share/tynex/hints/ when it is missing.

# Resolve the battery capacity file (supports BAT0, BAT1, etc.).
bat_files=(/sys/class/power_supply/BAT*/capacity)

if [[ -r "${bat_files[0]}" ]]; then
    battery="$(< "${bat_files[0]}")%"
else
    battery="N/A"
fi

printf '%(%d %B, %A | %T)T | Battery: %s\n' -1 "$battery"
