# OS probe on windows-11-arm, run 42307873328 (2026-11-06): WP-reads

[The run](https://github.com/tiborgats/native-theme/actions/runs/42307873328): `os-probe.yml`'s probe job on
`windows-11-arm` (dispatched with `-f jobs=probe,winui-host,android-host,kde-render -f images='["windows-11-arm"]'`),
green. The probe is `tools/os-probe` on `v0.8.0-tools-hosts-m1b` at `a4b2af0a`, on the runner image
win11-vs2026-arm64 20261101.203.1 (Windows 11 Enterprise 25H2, 10.0.26200, UBR 9725; `probe.toml` `[os]`, `[probe]`).
It is the probe's half of the WP-reads pass of the 0.8.0 measurement plan (register rows U503, U505, U508); the WinUI
host's half (GetCaretBlinkTime, DirectWrite, the text pass) is in `docs/evidence/winui-host/`.

New in this run's `probe.toml`, beside the earlier runs' reads:
- `[windows.system_parameters]`: `SPI_GETKEYBOARDDELAY` 1 and `SPI_GETKEYBOARDSPEED` 31 (the settings' indices, 0-3
  and 0-31; their millisecond values are not published), `SPI_GETCARETTIMEOUT` 5000 (ms), `SPI_GETMOUSEVANISH` 1;
- `[windows.registry.'HKCU\Control Panel\Desktop']`: `CursorBlinkRate` "530", `CaretTimeout` 5000, `CaretWidth` 1,
  `FontSmoothing` "2", `FontSmoothingType` 2, `FontSmoothingGamma` 0, `FontSmoothingOrientation` 1;
- `[windows.registry.'HKCU\Control Panel\Keyboard']`: `KeyboardDelay` "1", `KeyboardSpeed` "31";
- `HKCU\Software\Microsoft\Accessibility` `TextScaleFactor` and every value under
  `HKCU\Software\Microsoft\Avalon.Graphics\DISPLAY1`: absent (error 2), so DirectWrite and the text size run on their
  defaults.

These are the runner image's settings, which may differ from a fresh installation's: `SPI_GETMOUSEVANISH` reads 1,
where the SystemParametersInfo reference states the default is off.

Kept here: `os-probe-windows-11-arm/probe.toml` (the whole read, 17 KB). Not kept: nothing else was written.
