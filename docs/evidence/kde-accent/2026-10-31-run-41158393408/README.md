# KDE accent run, run 41158393408 (2026-10-31)

[The run](https://github.com/tiborgats/native-theme/actions/runs/41158393408): `os-probe.yml`'s kde-accent job
(dispatched with `-f jobs=kde-accent`), green. The harness is `scripts/measure_kde_accent.sh` on `v0.8.0-tools` at
`dd4be8a6`, on the pairs of `tools/kde-host/accent-pairs.tsv`, in the KDE container
`ghcr.io/tiborgats/native-theme-kde:2026-10-29` (digest
`sha256:ee7ed074bae04f8e963ca3748daaa6da8ba09019b48615305387ae230bf2db00`, pulled without credentials; the Arch Linux
Archive snapshot 2026/10/29, `archlinux@2026-10-29`): plasma-workspace 6.8.2-1, breeze 6.8.2-1, kcolorscheme
6.31.0-1, kconfig 6.31.0-1, kconfigwidgets 6.31.0-1, kguiaddons 6.31.0-1, qt6-base 6.12.0-3 (the manifest's
`[builds]`, `pacman -Q`). It is the specification's S17 *The accent run*: what Plasma's colour scheme applier
(`applyScheme()`, plasma-workspace@6.8.2 `kcms/colors/colorsapplicator.cpp`) writes into kdeglobals for each scheme
file and accent.

Each pair runs in fresh `XDG_CONFIG_HOME`, `XDG_DATA_HOME` and `XDG_CACHE_HOME` directories with
`QT_QPA_PLATFORM=offscreen` under `dbus-run-session`: `plasma-apply-colorscheme <predecessor>`, then
`plasma-apply-colorscheme <scheme>` (checked: `ColorScheme` names it, absent for BreezeLight, the default, and
`ColorSchemeHash` is the file's SHA-1), then, for an accent, `plasma-apply-colorscheme --accent-color <accent>`. Each
step exited 0 and changed kdeglobals, no pair had an `AccentColor` stored before its scheme's apply, and the three
check pairs' kdeglobals equal their `same as` pair's byte for byte, so the scheme applied first leaves nothing behind.
The nine Breeze pairs' kdeglobals are byte-identical to spike K's run
[40975872811](https://github.com/tiborgats/native-theme/actions/runs/40975872811).

The schemes: BreezeLight and BreezeDark, the preset's own files (a `Colors:Header` group with an `[Inactive]` group,
no tint, no accent-coloured title bar); BreezeClassic (`accentActiveTitlebar=true`, no `Colors:Header` group, the one
colour group the others have and it lacks); and SUSECafe, `SUSE-Cafe@2f66c168 SUSECafe.colors` (`TintFactor=0.15`,
`TitlebarIsAccentColored=true`, a `Colors:Header` group of two keys), fetched by the job at its pinned commit and
checked by SHA-256 (`tools/kde-host/accent-schemes.tsv`). The installed Breeze files' SHA-256s are those of
`breeze@6.8.2 colors/`. The accents are spike K's: `#e5a50a` and `#5e2ca5`.

| Pair | Scheme file | Accent | Predecessor | Same as | kdeglobals SHA-256 | Committed |
|---|---|---|---|---|---|---|
| `light-none` | `BreezeLight` (SHA-256 `c46470b75e7df3e1…`) | `-` | `BreezeDark` | `-` | `30b0712e60f2efe538b7f3c2ef7a4f042fb236470014c75b33a1e2eddf841d50` | no (no accent) |
| `light-e5a50a` | `BreezeLight` (SHA-256 `c46470b75e7df3e1…`) | `#e5a50a` | `BreezeDark` | `-` | `2ad46e1cbb0d643d362fe2a0afc60f93bd2f1cab09750f66dde6941d2e2e9c6b` | yes |
| `light-5e2ca5` | `BreezeLight` (SHA-256 `c46470b75e7df3e1…`) | `#5e2ca5` | `BreezeDark` | `-` | `31595b7679b89a3de4cc97c2702cee7e55be2a62963137de0cf0d8542e935770` | yes |
| `dark-none` | `BreezeDark` (SHA-256 `11fce1d8ab97b529…`) | `-` | `BreezeClassic` | `-` | `c9df398081add7ddee7e0166970649c3e87df4caffc5ebc9018d889532aafb1a` | no (no accent) |
| `dark-e5a50a` | `BreezeDark` (SHA-256 `11fce1d8ab97b529…`) | `#e5a50a` | `BreezeClassic` | `-` | `11b5c55b5b8b924ad432f044549b82c89873c066fe13e2ef98900a0f9357be4c` | yes |
| `dark-5e2ca5` | `BreezeDark` (SHA-256 `11fce1d8ab97b529…`) | `#5e2ca5` | `BreezeClassic` | `-` | `5d9a0c0570b6f998e0d9687fce0eb41b07ced90f49a8ca3f85b9817e8bfc8f82` | yes |
| `classic-none` | `BreezeClassic` (SHA-256 `19dcbd1883c03d2d…`) | `-` | `BreezeDark` | `-` | `a6dc1499e63e73889e56734fbb02c2ccd40b874e137bc6cd9e14d73630ed5062` | no (no accent) |
| `classic-e5a50a` | `BreezeClassic` (SHA-256 `19dcbd1883c03d2d…`) | `#e5a50a` | `BreezeDark` | `-` | `cf88bd93a06a6403c047d88b9eb94530543853fea8da243488f8023ce960d3e1` | yes |
| `classic-5e2ca5` | `BreezeClassic` (SHA-256 `19dcbd1883c03d2d…`) | `#5e2ca5` | `BreezeDark` | `-` | `b7a9177abcaf34474bb2ed9ccc9a4b4d0202f63cc1307cf60849f29651f0b1f7` | yes |
| `susecafe-none` | `SUSECafe` (SHA-256 `5225742f994f5689…`) | `-` | `BreezeDark` | `-` | `f523fee7d5b0dfacb229fd666ef958933b96abd1fc22d3eeaf00cfbeaa3b4a4e` | no (no accent) |
| `susecafe-e5a50a` | `SUSECafe` (SHA-256 `5225742f994f5689…`) | `#e5a50a` | `BreezeDark` | `-` | `b0b5dac9b6cf3a17838c8e77de5672303feb91acb0b97cd17423fdf43884a5cb` | yes |
| `susecafe-5e2ca5` | `SUSECafe` (SHA-256 `5225742f994f5689…`) | `#5e2ca5` | `BreezeDark` | `-` | `88127ed178f5c8fd11798b20f8bceab67b5f0ccfef322c6e5e2353afaaf26c01` | yes |
| `light-none-after-classic` | `BreezeLight` (SHA-256 `c46470b75e7df3e1…`) | `-` | `BreezeClassic` | `light-none` | `30b0712e60f2efe538b7f3c2ef7a4f042fb236470014c75b33a1e2eddf841d50` | no (no accent) |
| `classic-none-fresh` | `BreezeClassic` (SHA-256 `19dcbd1883c03d2d…`) | `-` | `none` | `classic-none` | `a6dc1499e63e73889e56734fbb02c2ccd40b874e137bc6cd9e14d73630ed5062` | no (no accent) |
| `susecafe-none-after-classic` | `SUSECafe` (SHA-256 `5225742f994f5689…`) | `-` | `BreezeClassic` | `susecafe-none` | `f523fee7d5b0dfacb229fd666ef958933b96abd1fc22d3eeaf00cfbeaa3b4a4e` | no (no accent) |

Kept here: one folder per accent pair, holding its resulting `kdeglobals`, which `docs/facts/kde/accent.toml` cites
(`run.toml` maps each folder to `kde-breeze`). Not kept (only on the run's page, for 90 days): the no-accent and
check pairs' kdeglobals, each step's kdeglobals and output, `manifest.toml`, `run.txt`, `pull.txt` and
`schemes.txt`; the SHA-256s above are the manifest's.
