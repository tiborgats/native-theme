# Breeze and an open combo box, readings of 2026-11-18

What Breeze draws on a combo box's own frame, the trigger, when its popup is open, with
the pointer away and with the pointer moved to the trigger, beside the closed combo box
at rest and under the pointer (decision 10 of the 0.7.2 KDE steps 4 and 5). Taken with
`combo_trigger.py`, a copy of [`2026-11-14/combo.py`](../2026-11-14/combo.py) reshaped
to grab the combo box itself (`c.grab()`) instead of the popup's list, against the
`kdeglobals` files the [`2026-11-13/` README](../2026-11-13/README.md#rebuilding-the-configuration)
rebuilds. Qt's `offscreen` platform: nothing appeared on the desktop. The images are not
kept.

| | |
|---|---|
| Date | 2026-11-18 |
| native-theme | `f02c3ae9` (no build: the run reads the installed Breeze only) |
| Versions | Breeze 6.8.2, plasma-integration 6.8.2 (the distribution's package query: `breeze 6.8.2-1.1`, `plasma-integration 6.8.2-1.1`); PySide6 and Qt as printed by `python3 -c "import PySide6, PySide6.QtCore as c; print(PySide6.__version__, c.qVersion())"`: 6.12.0, 6.12.0 |
| Environment | from a working folder outside the repository. `combo_trigger.py` under `env -i` with `HOME`, `PATH` and only: `QT_QPA_PLATFORM=offscreen`, `QT_QPA_PLATFORMTHEME=kde`, `XDG_CONFIG_HOME` (`cfg-light` / `cfg-dark`), `XDG_CONFIG_DIRS` (an empty folder). `HOME` was an empty folder in the working folder, not the user's home as on 2026-11-14 (the session may not read outside the project); PySide6 is the system's (`/usr/lib/python3.14/site-packages`). In that environment the 2026-11-14 `combo.py` printed `combo_light.txt` and `combo_dark.txt` byte for byte as recorded there |

- `combo_trigger_light.txt`, `combo_trigger_dark.txt`: `combo_trigger.py` under
  `cfg-light` / `cfg-dark`. The first line gives the style, the palette's `Window` and
  `Highlight`, `SH_ComboBox_Popup` and the combo box's geometry. Each state's `grab` line
  gives the pixels `x = 0..7` at mid height of the grabbed combo box (the frame line is
  `x = 1`; `x = 0` is the window behind it), one fill pixel at the middle (the text ends
  before it, the arrow is at the right end), `underMouse()`, `hasFocus()`, the state
  flags `initStyleOption` hands the style, the active popup and the cursor's global
  position. Each `events` line lists the mouse, enter, leave and hover events an
  application event filter saw for that step (repeats in a row folded), with their
  receivers.
- The states: `1_closed_away` (pointer at a point of the window outside the combo box);
  `2_closed_over` (`QTest.mouseMove(c, c.rect().center())`, then 300 ms, as `combo.py`
  waits); `3_open_away` (pointer moved away, then `c.showPopup()`, 300 ms);
  `4a_open_move_to_combo` (`QTest.mouseMove(c, c.rect().center())` with the popup open);
  `4b_open_move_to_popup` (the move sent to the popup at the trigger centre's global
  position, after `4b_prep_popup_move_away` moved the pointer away through the popup:
  without that step the cursor already stands at that position after 4a, and a first
  version of the script that sent the move straight after 4a logged no event for it).

## Commands

Every reading was taken on 2026-11-18. The configuration (`cfg-light/`, `cfg-dark/`,
`empty/`) was rebuilt with the `2026-11-13/` README's `curl` lines from breeze
`f0b1d753`; `sha256sum` gave the two sums that README lists.

```sh
mkdir -p empty home
for s in light dark; do
  env -i HOME="$PWD/home" PATH="$PATH" QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde \
    XDG_CONFIG_HOME="$PWD/cfg-$s" XDG_CONFIG_DIRS="$PWD/empty" python3 combo_trigger.py > "combo_trigger_$s.txt"
done
```

`combo_trigger.py` writes one `combo_trigger_<state>.png` per state into the working
folder.
