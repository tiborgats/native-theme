#!/bin/sh
# Usage: ./run.sh <script.py> <light|dark> [proxy]   (from this folder)
# The set-up of the 2026-11-13 and 2026-11-20 runs: private kdeglobals, no system config,
# a private HOME, platform offscreen, Plasma's platform theme, scale 1.
D=$(cd "$(dirname "$0")" && pwd)
export XDG_CONFIG_HOME="$D/cfg-$2" XDG_CONFIG_DIRS="$D/empty" HOME="$D/home"
export QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde QT_SCALE_FACTOR=1
unset QT_STYLE_OVERRIDE QT_SCREEN_SCALE_FACTORS QT_AUTO_SCREEN_SCALE_FACTOR QT_ENABLE_HIGHDPI_SCALING
cd "$D" && exec nice -n 19 ionice -c 3 python3 "$@"
