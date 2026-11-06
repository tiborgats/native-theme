# KDE render host, run 42307873328 (2026-11-06): KH-reads

[The run](https://github.com/tiborgats/native-theme/actions/runs/42307873328): `os-probe.yml`'s kde-render job
(dispatched with `-f jobs=probe,winui-host,android-host,kde-render`), green. The harness is `tools/kde-host/run.sh` on
`v0.8.0-tools-hosts-m1b` at `a4b2af0a`, in the KDE container `ghcr.io/tiborgats/native-theme-kde:2026-10-29` (digest
`sha256:ee7ed074bae04f8e963ca3748daaa6da8ba09019b48615305387ae230bf2db00`, pulled without credentials; the Arch Linux
Archive snapshot 2026/10/29): qt6-base 6.12.0-3, kirigami 6.31.0-1, fontconfig 2:2.18.3-2, noto-fonts
1:2026.10.01-1, ttf-hack 3.003-7 (the container's `packages.txt`, `pacman -Q`). It is the KH-reads pass of the 0.8.0
measurement plan, for register rows U400 and U404.

Each read runs with `QT_QPA_PLATFORM=offscreen` and `QT_QPA_PLATFORMTHEME=kde` in fresh XDG directories holding
BreezeLight's kdeglobals (which sets no font, so the platform theme's defaults apply).

- `fonts/fonts.toml` (`tools/kde-host/fonts.py`): the screen's logical DPI (96), the fonts the platform theme reports
  (`QFontDatabase::systemFont`: general `Noto Sans,10`, fixed `Hack,10`, smallest readable `Noto Sans,8`, title) and
  `Noto Sans 10` and `Hack 10` asked by name, each with the font Qt resolved (`QFontInfo`) and its `QFontMetrics`
  and `QFontMetricsF`. Noto Sans 10 resolves to Noto Sans Regular, 13 px; `height()` 18 (ascent 14, descent 4),
  `QFontMetricsF::height()` 17.6875. The second read with `QT_FONT_DPI=96` wrote a byte-identical file (not kept).
- `qml/index.txt` (`tools/kde-host/buttons.qml`, BreezeLight, run 1): Kirigami's `Units.iconSizes.sizeForLabels` is
  16, with a `FontMetrics` of the default font: Noto Sans 10 pt, height 17.6875.
- `fonts/fontconfig.txt`, `fonts/font-files.sha256`: fontconfig's version, the files `fc-match` gives for the
  default families, and the SHA-256 of `NotoSans-Regular.ttf` and `Hack-Regular.ttf`.
- `fonts/fc-match-sorted.tsv`: `fc-match -s` (fontconfig's sorted match, `FcFontSort`, which Qt's family fallbacks
  follow), its first 20 fonts each, for `Noto Sans`, `Hack`, `sans-serif`, `serif`, `monospace`, `emoji`, and for
  `Noto Sans:lang=<l>` and `Hack:lang=<l>` over 22 languages.
- `fonts/fc-list.tsv`: the 649 installed fonts (family, style, file).

What the reads show about the container: it installs `noto-fonts` and `ttf-hack` and the fonts their dependencies
pull in, and no CJK or colour emoji font (`fc-list.tsv` has no `Noto Sans CJK` and no `Noto Color Emoji`), so the
CJK and emoji rows of `fc-match-sorted.tsv` fall back to Latin fonts. Whether these lists state Plasma's fallbacks or
only this container's is the facts round's question.

Not kept (only on the run's page, for 90 days): the renders and their indexes (spike K's passes, unchanged; runs 1
and 2 byte-identical), `fonts/dpi-96/`, the scheme files, `run.txt`, `packages.txt`, `pull.txt`, `qml-help.txt`.
