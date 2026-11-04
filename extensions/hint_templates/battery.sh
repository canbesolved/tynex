#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 CanBeSolved (@canbesolved)
# SPDX-License-Identifier: MIT
# Hint template: battery level and state, or "No battery".

for bat in /sys/class/power_supply/BAT*; do
    [[ -r "$bat/capacity" ]] || continue
    state="Unknown"
    [[ -r "$bat/status" ]] && state="$(< "$bat/status")"
    printf 'Battery: %s%% (%s)\n' "$(< "$bat/capacity")" "$state"
    exit 0
done

echo "No battery"
