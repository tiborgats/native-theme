# Whole-codebase review — finding ledger

The plan is `docs/todo_code-review-plan.md`; entry format, lenses, severities and triage
rules are there (*The finding ledger*). Branch `v0.6.2-rc1`, from `main` at `92893ce1`.

## Status

| M | Status |
|---|--------|
| M0 | done (2026-10-11) |
| M1 | not started |
| M2a–M2e | Steps 1–6 done 2026-10-11 (stop point A, branch `m2-research`, D7); Step 3 probes, the M2d release pin and the version gaps await the maintainer; Steps 7–15 not started |
| M3–M9 | not started |

## Baseline (M0, 2026-10-11)

### Gates

`./scripts/check_release.sh` on `v0.6.2-rc1` at `10e037b1` plus the uncommitted timing test:
84 checks, 82 ✅, 1 ⚠️ (the tag `v0.6.1` already exists: the version is not bumped yet), 1 ❌
(visual assets stale since `92893ce1`'s GIF change, cleared by the next
`generate_assets_release.sh`); 37 min 22 s. A first run stopped at clippy on the timing
test's `IconRole` import, unused without the icon features; the import moved into the
feature-gated tests (the plan's copy too).

### Cross-target clippy

`cargo clippy -p native-theme --all-targets -- -D warnings` with `--all-features` and with
`--no-default-features`, for `x86_64-apple-darwin` and `x86_64-pc-windows-msvc`: 8 errors on
macOS and 11 on Windows, all fixed in M0:

- `needless_return` (6 sites): `detect.rs` (dark-mode and reduced-motion readers),
  `model/icons.rs` (`detect_icon_theme_outcome`), `pipeline.rs` (`platform_preset_name`),
  `presets.rs` (`detect_platform`), `watch/mod.rs` (both platform arms); the `#[cfg]` blocks
  are one per OS, so each final `return x;` became the block's value. Five
  `#[allow(unreachable_code)]` went with them; the one on `detect_system_font_dpi` stays (its
  blocks are not exclusive: removing it fails on both targets);
- `needless_borrow` (2): `Context::from_waker(&waker)` in both non-Linux `from_system`s,
  where `Waker::noop()` is already a `&Waker`;
- `field_reassign_with_default` (3, Windows): `cbSize` set in the initialiser of
  `NONCLIENTMETRICSW`, `HIGHCONTRASTW` and `SHSTOCKICONINFO`; no `unsafe` line changed;
- one test import (`icons.rs`, `load_system_icon_by_name_tests`) unused on macOS without
  features: explicit `super::` paths instead of `use super::*`.

After: all six runs clean; native-theme's 1104 tests pass on Linux. `check_release.sh`'s
cross-target section now runs the four clippy runs (both targets, both feature sets).

### Timings

`cargo test -p native-theme --release --all-features --test timings -- --ignored --nocapture --test-threads=1`,
three runs on the maintainer's machine (KDE Plasma session), each value a median (21 runs
per preset, 5 for the readers, 11 for icons); release profile.

| Path | Run 1 | Run 2 | Run 3 |
|------|-------|-------|-------|
| `Theme::preset` parse, every preset | 1.02–1.38 µs | 1.03–1.18 µs | 1.02–1.38 µs |
| `Theme::resolve`, per preset and mode | 1.30–1.50 ms; 3.36 ms ×1 | 1.30–1.37 ms; 3.37–3.49 ms ×12 | 1.30–1.46 ms; 3.36–3.42 ms ×6, 5.18 ms ×1 |
| `SystemTheme::from_system` cold / warm | 9.12 / 3.63 ms | 9.66 / 8.30 ms | 9.41 / 5.64 ms |
| `AccessibilityPreferences::from_system` | 11.57 ms | 12.15 ms | 8.35 ms |
| Material, all 42 roles | 431 ns | 431 ns | 431 ns |
| freedesktop `breeze` cold / warm | 39.2 / 1.76 ms | 40.7 / 1.77 ms | 40.2 / 1.86 ms |
| freedesktop `breeze-dark` cold / warm | 37.6–40.9 / 1.82–2.07 ms (three runs) | | |
| freedesktop `Adwaita` cold / warm | 5.0–7.7 / 2.58–2.92 ms (three runs) | | |

(The `from_system` and accessibility rows are from the last three of six runs; the first
three gave warm `from_system` 5.55, 5.61, 24.34 ms and accessibility 7.59, 9.42, 11.86 ms.)

Noise floor: the resting resolve times agree within about 0.2 ms; the readers vary by
about 2×. Observations for M3/M4, not yet findings:

- `resolve` is about 1.3 ms for any preset, against about 1 µs to parse one, and jumps
  by about 2 ms (to 3.4, once 5.2): a fixed-step wait inside resolution is the suspect
  (`ResolutionContext::from_system`, `model/mod.rs:461`, runs on every call) [M3 R2];
- 1 µs is too fast to parse a 500-line TOML, so `Theme::preset` presumably returns a
  cached copy: confirm it is a cache, and what it costs to clone [M3 R4];
- a cold freedesktop lookup took 375 ms on 2026-10-10 (cold disk cache) and 38–41 ms
  here (warm page cache); warm lookups take 1.8–2.9 ms for 42 icons [M4 R4].

### Lint inventory

`cargo clippy --workspace --all-features --all-targets --message-format=json -- -W clippy::pedantic -W clippy::nursery`:
3075 hits (1495 in library code, 1462 in the showcases, 118 in tests). Per crate:
native-theme-gpui 1117, native-theme-egui 728, native-theme 614, native-theme-iced 375,
native-theme-build 112, native-theme-derive 65, native-theme-egui-widgets 61 (and 3 in
generated or third-party files). The most frequent in library code: `doc_markdown` 344,
`missing_const_for_fn` 195, `redundant_pub_crate` 140, `use_self` 90,
`too_long_first_doc_paragraph` 68, `suboptimal_flops` 56, `match_same_arms` 55,
`uninlined_format_args` 52, `option_if_let_else` 48, `too_many_lines` 39. Worth a
reviewer's look rather than a blanket fix: `redundant_clone` 15 (13 in native-theme),
`needless_pass_by_ref_mut` 14 (gpui), the `cast_*` lints 88, `float_cmp` 12 (9 in iced),
`unnecessary_wraps` 8. Each milestone regenerates the list for its own crate (plan,
template Step 1).

### Dependencies

- `cargo machete` (0.9.2): no unused dependency in any workspace crate.
- `cargo tree -d -e normal --all-features`: duplicated versions per crate, counting a
  crate once however many versions it has: native-theme 7, native-theme-iced 10,
  native-theme-egui 10, native-theme-gpui 32. In native-theme the real ones are upstream's:
  `miniz_oxide` 0.8.9 (png) / 0.9.1 (flate2 under png), `roxmltree` 0.20.0 (fontdb's
  fontconfig-parser) / 0.21.1 (usvg), `syn` 2 / 3 (proc macros); the `serde`, `serde_core`
  and `toml_datetime` lines are one version built with two feature sets. None is ours to fix.

### Semver

`cargo semver-checks` 0.51.0, `-p native-theme --baseline-version 0.6.1 --all-features`:
202 checks pass, 58 skipped; no semver update required. The other crates are checked when
their milestone closes.

## Calibration

Measured cost per milestone, and the estimate for the rest.

- **M2, Steps 1–2 and 4–5, all five parts (one Workflow, 2026-10-11):** 41 Opus agents at high effort (5 version maps, 14 researchers, 5 preset-validators, 17 verifiers), 5.41M subagent tokens, 36.5 min wall-clock; the plan's +1000k per part estimated 5.0M. Per-agent figures were not recorded; the triage and the ledger ran in the session.

## M1 — Design review

### M2 decisions (2026-10-11)

- **Step 3, M2a and M2b (maintainer):** the OS probe is built now on `m2-research` and run in CI before the `macos-14`
  runner image becomes unsupported (2026-11-02), on `macos-14`, `macos-15`, `macos-26`, `windows-2022` and `windows-2025`;
  the CI dispatch is the maintainer's. It needs `unsafe` FFI (AppKit control sizes, `GetSystemMetricsForDpi`,
  `GetSysColor`, `SystemParametersInfoW` are `unsafe` in objc2-app-kit 0.3 and windows 0.62), so per the strict rule it
  waits for the maintainer's answer on that before any code is written.
- **Step 3, M2d (maintainer):** the Compose reference app is skipped: Step 2 found a token or a Compose constant for every
  Material cell. Revisited only if Step 13 meets a detail with neither.
- **M2d release pin (settled by logic; the maintainer may veto):** D6 asks for a *stable* release and the *same* release
  for both presets. Only Compose Material 3 1.5.0 can satisfy both, since 1.4.0 keeps Expressive `internal`. So both
  presets pin 1.5.0 stable. Step 13 waits for that release (Steps 13–15 wait for M1's stop point A anyway); the Step 2
  research done at 1.5.0-beta01 (`19991f2c`) is re-diffed against the stable tag then. If 1.5.0 is not stable when
  Step 13 is due, the maintainer chooses between waiting and pinning the latest 1.5.0 pre-release.

## M2a — macOS

**Status:** Steps 1, 2, 4, 5 and 6 done 2026-10-11, beside M1 (D7), in the `m2-research` worktree; Step 3 awaits the maintainer; Step 8 takes the *Step 8* rows after M1's stop point A; Step 7 not started. One Workflow ran all five parts (41 Opus agents at high effort, 5.41M subagent tokens, 36.5 min). Triage values: *fix* (Step 7), *Step 8* / *Step 13* (the part's build steps, already decided), *maintainer (Step 3 probe)*, *maintainer (version gap)*, *M4* (reader code), *defer* (an existing docs/todo.md entry), *drop*.

The preset targets macOS 14 (`macos-sonoma.toml:1-3`), and 14 has had no security update since 14.8.9 (2026-08-06): Apple
supports 27, 26 and 15 today. No source separates 15 from 14 (WebKit's pre-Tahoe tables carry no 14/15 conditional, Apple
published no AppKit notes after 14), so D4's "Sequoia only if it differs" stays open until the probe compares the
`macos-14` and `macos-15` runners. WebKit at `de320ba3` is the only primary source of Tahoe control metrics, and only for its
own vector drawing of HTML controls (`ENABLE_FORM_CONTROL_REFRESH`, macOS ≥ 26): verifiers rated each such value PLAUSIBLE,
not CONFIRMED, because AppKit may draw differently. No theme file in WebKit has a macOS 27 conditional; Apple calls 27 a
refinement of Liquid Glass ("updated window shapes", menu symbols hidden, a `tabs` segmented-control role). 137 rows have no
primary source at all, almost all control metrics.

**Step 3 (probe) is the main open question:** of the 30 *differs* rows, 10 conflict at the target version and need a
measurement, and the Tahoe values of 15 more rest on WebKit alone. The `macos-14` runner image is fully unsupported from
2026-11-02 (brownouts from 2026-10-05, actions/runner-images #13518), so a Sonoma probe must run before then.

### Step 1 — version map

| Version | Released | Support on 2026-10-11 | Generation |
|---|---|---|---|
| 14 macOS Sonoma | 26 Sep 2023, from Apple's security archive https://support.apple.com/en-us/121012 (row 'macOS Sonoma 14', link https://support.apple.com/kb/HT213940), read 2026… | Looks out of security support since 14 Sep 2026. Its last update is 14.8.9, released August 6, 2026 (https://support.apple.com/en-us/148172). There is no Sonoma entry in the 14 Sep 2026 or 28 Sep 2026 releases on https://support.apple.com/en-us/100100 (page da… | G1 Big Sur-era design (macOS 11-15) |
| 15 macOS Sequoia | 16 Sep 2024 (https://support.apple.com/en-us/100100, row 'macOS Sequoia 15', link https://support.apple.com/en-us/121238) | In support. 15.8 was released September 14, 2026 (https://support.apple.com/en-us/149043) and 15.8.1 on 28 Sep 2026 (https://support.apple.com/en-us/100100). It is the oldest version still patched. | G1 Big Sur-era design (macOS 11-15) |
| 26 macOS Tahoe | 15 Sep 2025 (https://support.apple.com/en-us/100100, row 'macOS Tahoe 26', link https://support.apple.com/en-us/125110). Announced June 9, 2025 (https://www.app… | In support. 26.7 came on 14 Sep 2026 and 26.7.1 on 28 Sep 2026 (https://support.apple.com/en-us/100100; https://support.apple.com/en-us/122868) | G2 Liquid Glass |
| 27 macOS Golden Gate | September 14, 2026 (https://support.apple.com/en-us/149035). 27.0.1 followed on 28 Sep 2026 (https://support.apple.com/en-us/100100; https://support.apple.com/e… | Current release. https://support.apple.com/en-us/100100 says 'The latest version of macOS is 27.0.1'. | G2 Liquid Glass, refined (see the generation notes; the control metrics are not… |

Generations:

- **G1 Big Sur-era design** (14 Sonoma, 15 Sequoia (began with 11 Big Sur)): Big Sur (June 22, 2020) brought this design: 'Everything from the curvature of window corners to the palette of colors and materials has been refined'. Apple's Sequoia 15 press release (June 10, 2024) announces no system or control redesign; the only 'redesigned' item is Safari's Reader. Whether 15 differs from 14 at the control level cannot be settled from Apple's announcements. Apple published no AppKit release notes after Sonoma 14 (the index at developer.apple.com/tutorials/data/documentation/macos-release-notes.json lists AppKit notes for 10.14 through 14 only), so Step 2 or the probe (macos-14 against macos-15 runners) must decide. Source: https://www.apple.com/newsroom/2020/06/apple-introduces-macos-big-sur-with-a-beautiful-new-design/ (dated June 22, 2020); https://www.apple.com/newsroom/2024/06/macos-sequoia-takes-productivity-and-intelligence-on-mac-to-new-heights/ (June 10, 2024); https://developer.apple.com/tutorials/data/documentation/macos-release-notes.json (Last-Modified 2026-10-03). All read 2026-10-11.
- **G2 Liquid Glass** (26 Tahoe (26.0-26.7.1)): Apple calls it its 'broadest software design update ever'. Liquid Glass extends 'from the smallest elements ... buttons, switches, sliders, text' to sidebars and toolbars, and controls are now 'concentric with the rounded corners of modern hardware and app windows'. The menu bar is transparent. 26.1 added a Liquid Glass setting with a 'default clear look or a new tinted look'. Apps can opt out on 26: the Info.plist key UIDesignRequiresCompatibility=YES runs an app in a compatibility mode that 'displays the app as it looks when built against previous versions of the SDKs' (introducedAt macOS 26.0). The system ignores the key when an app is built for macOS 27 or later. So on macOS 26 a given a… Source: https://www.apple.com/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/ (June 9, 2025); https://www.apple.com/newsroom/2025/06/macos-tahoe-26-makes-the-mac-more-capable-productive-and-intelligent-than-ever/ (June 9, 2025); https://support.apple.com/en-us/122868 ('macOS Tahoe 26.1' section); https://developer.apple.com/tutorials/data/documentation/bundleresources/infor…
- **G2 Liquid Glass, Golden Gate refinement (decide during Step 2 whether it is a separate generation)** (27 Golden Gate (27.0, 27.0.1)): Apple calls it a refinement, not a new material. Its words: 'Updates to Liquid Glass improve readability, and a new slider lets you customize how it looks, from ultra-clear to fully tinted. Uniform toolbars, edge-to-edge sidebars, and more detailed app icons'; apple.com/os/macos adds 'more uniform refraction and improved contrast ... updated window shapes and menu bar icons'. The macOS 27 AppKit release notes list visible control changes. NSMenu hides menu item symbol images by default ('similar to the behavior prior to macOS 26.0'), for apps linked on 26.0 or later. NSSegmentedControl gains a role property with a 'tabs' role that has 'a distinct visual appearance'. NSTitlebarAccessoryViewCo… Source: https://support.apple.com/en-us/127257 (published September 28, 2026); https://www.apple.com/os/macos/ ('Design refinements'); https://developer.apple.com/tutorials/data/documentation/macos-release-notes/macos-27-release-notes.json, section 'AppKit' (Last-Modified 2026-10-03). All read 2026-10-11.

Pinned sources:

- Apple security releases (support lifecycle evidence): https://support.apple.com/en-us/100100 — page 'Published Date: September 28, 2026', read 2026-10-11; archive for 2022-2023: https://support.apple.com/en-us/121012; covers Release dates and the last security update of each macOS version (14/15/26/27). This is the only lifecycle evidence: Apple publishes no end-of-support dates.
- Apple Platform Deployment: About software updates (Apple's only stated patch policy): https://support.apple.com/guide/deployment/about-software-updates-depc4c80847a/web — read 2026-10-11; covers The note reads: 'Because of dependency on architecture and system changes to any current version of Apple operating systems (for example, iOS 27, macOS 27, and so on), not all known security issues ar…
- Apple HIG JSON API: https://developer.apple.com/tutorials/data/design/human-interface-guidelines/<page>.json — Unversioned: it serves only the current text, with no version or tag. Each page carries metadata.customMetadata.alert-date, a 'Change log' table and an HTTP Last-Modified. Read 2026-10-11: typography.json Last-Modified 2026-08-06 (change log latest 2025-12-16)…; covers G2 (26/27) guidance only. The HIG served today does not give Sonoma or Sequoia values: color values were replaced on 2025-06-09. A G1 value from the HIG needs a dated pre-2025-06-09 archived copy. A W…
- AppKit documentation JSON: https://developer.apple.com/tutorials/data/documentation/appkit/<symbol>.json (e.g. .../appkit/nscolor.json) — Unversioned. Each symbol carries metadata.platforms[].introducedAt. Read 2026-10-11: appkit.json Last-Modified 2026-09-11; nscolor.json 2026-09-11; nscontrol/controlsize-swift.enum.json 2026-08-14.; covers API availability per version (introducedAt). It gives no pixel metrics.
- macOS release notes (AppKit sections): https://developer.apple.com/tutorials/data/documentation/macos-release-notes/macos-26-release-notes.json and .../macos-27-release-notes.json; AppKit notes for 14: .../appkit-release-notes-for-macos-14 — Last-Modified 2026-10-03 (both), read 2026-10-11; covers Behaviour changes in 26 and 27 (menu item images, segmented-control tabs role, titlebar accessories). Apple published no separate AppKit release notes for 15, 26 or 27.
- UIDesignRequiresCompatibility (Liquid Glass opt-out): https://developer.apple.com/tutorials/data/documentation/bundleresources/information-property-list/uidesignrequirescompatibility.json — Last-Modified 2026-09-18, read 2026-10-11; covers An app on macOS 26 can keep the pre-26 look. The key is ignored when an app is built for macOS 27 or later.
- WebKit (RenderThemeMac/Cocoa, mac controls, ScrollbarThemeMac, WTF platform flags): https://github.com/WebKit/WebKit/tree/de320ba3ff90229f983988d42d4b0f83315459da — main @ de320ba3ff90229f983988d42d4b0f83315459da (committed 2026-10-10T17:07:54-07:00); covers G1 path: Source/WebCore/platform/graphics/mac/controls/*Mac.mm (AppKit NSCell-based, no version conditionals: values come from AppKit at run time on the OS it runs on), Source/WebCore/rendering/mac/Re…
- Apple newsroom and product pages (generation evidence): https://www.apple.com/newsroom/2020/06/apple-introduces-macos-big-sur-with-a-beautiful-new-design/ ; https://www.apple.com/newsroom/2024/06/macos-sequoia-takes-productivity-and-intelligence-on-mac-to-… — pages dated 2020-06-22, 2024-06-10, 2025-06-09, (122868 undated in extract), 2026-09-28, undated; all read 2026-10-11; covers Which visual generation each version belongs to.
- actions/runner-images: https://github.com/actions/runner-images/blob/8197087fc536320d1441203fdb5da9ae1b44b863/README.md — main @ 8197087fc536320d1441203fdb5da9ae1b44b863 (2026-10-09T18:46:46Z); image READMEs under images/macos/ at the same commit; issues #13518 and #14404 read 2026-10-11; covers macOS runner images available for the probe (Step 3).

Preset target: Both presets claim only Sonoma (macOS 14) and no other version. native-theme/src/presets/macos-sonoma.toml:1-3 reads '# macOS Sonoma theme' / '# Source: Apple Human Interface Guidelines, NSColor system colors' / name = "macOS Sonoma". native-theme/src/presets/macos-sonoma-live.toml:1-6 reads '# macOS Sonoma live preset - geometry/metrics only (no colors, fonts, or icons)' ... name = "macOS Sonoma". native-theme/src/presets/README.md:60-61 lists 'macOS Sonoma standalone theme' and 'macOS Sonoma base for the macOS reader'. The reader picks the preset with no version check: native-theme/src/pipeline.rs:408-413 (platform_preset_name returns "macos-sonoma" on any macOS) and pipeline.rs:631 (MacosReader + "macos-sonoma-live"). docs/platform-facts.md §1.1 (line 51) states no macOS version for the column as a whole. Only a few rows name a version: :159-162 (macOS 14+ APIs), :173 and :1155 ('thro…

CI runner images: At actions/runner-images 8197087fc536320d1441203fdb5da9ae1b44b863 (2026-10-09), README.md:30-36 lists these macOS images: - macOS 14 arm64: labels `macos-14`, `macos-14-xlarge`; image macOS 14.8.9 (23J631), version 20260831.0302.1. x64: `macos-14-large`. Deprecated: issue #13518 says deprecation began July 6, 2026, the image is 'fully unsupported by November 2nd, 2026', with brownouts Oct 5-31 2026 (including Oct 12, 16, 19, 23, 26, 29, 30, 14:00-24:00 UTC), and states 'GitHub Actions maintains the latest two stable versions'. - macOS 15 arm64: labels `macos-15`, `macos-15-xlarge`; image macOS 15.7.9 (24G830), version 20260907.0337.1. x64: `macos-15-intel`, `macos-15-large`. - macOS 26 arm64: labels `macos-latest`, `macos-26`, `macos-26-xlarge`; image macOS 26.6.2 (25G83), version 20260907.0351.1. x64: `macos-26-intel`, `macos-26-large`, `macos-latest-large`. - macOS 27: there is no `macos-27` label. The only macOS 27 image is `xcode-27` / `xcode-27-xlarge` (arm64), marked 'preview' (issue #14404). Since that issue's 'Edit 2026.09.16' it 'uses MacOS 27 OS as its base OS'. images/macos/xcode-27-arm64-Readme.md:7-10 gives 'macOS 27.0.1 (26A434)', Darwin 27.0.0, image version 20261006…

<details><summary>Version-map notes</summary>

Support status on 2026-10-11, from Apple's own pages:
- In support: 27 (current), 26 and 15.
- Sonoma 14 has had no security release since 14.8.9 (6 Aug 2026) and is left out of Safari 27. That fits Apple dropping it when 27 shipped, but Apple states no end-of-support date. Apple's only written policy is the deployment-guide note (see pinned_sources).
- Usage share: Apple publishes none, so the field is left empty.

WebKit macOS-26 conditionals at de320ba3ff90229f983988d42d4b0f83315459da:
- Source/WTF/wtf/PlatformEnableCocoa.h:1189-1194: ENABLE_FORM_CONTROL_REFRESH when PLATFORM(MAC) && __MAC_OS_X_VERSION_MIN_REQUIRED >= 260000.
- Source/WTF/Scripts/Preferences/UnifiedWebPreferences.yaml:2489-2497: the runtime gate FormControlRefreshEnabled (default true, WebCore: false).
- Source/WTF/wtf/PlatformHave.h:
  - :1726-1730: HAVE_MATERIAL_HOSTING (>= 160000; no macOS 16 exists, so in effect 26).
  - :1791-1794: HAVE_APPKIT_SCROLLBAR_COLOR_SPI (>= 260000). It is not used in the theme files checked.
  - :1869-1872: HAVE_NSVIEW_CORNER_CONFIGURATION (>= 260100).
  - :1971-1973: HAVE_NSGLASSEFFECTVIEW_EFFECT_IS_INTERACTIVE (>= 270000). This is the only macOS-27 flag found, and the theme files do not use it.
- Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm:
  - :112-212: formControlRefreshEnabled, controlTintLuminanceThreshold, highContrastOutlineThickness, drawFocusRingForPathForVectorBasedControls.
  - :282-296 and :318-332.
  - :413-733: Liquid Glass switch (liquidGlassSwitchThumbColor, renderThemePaintLiquidGlassSwitchTrack/Thumb, idealSwitchSize, idealThumbWidth, largeDefinedSwitchHeight, largeDefinedThumbHeight(Ratio), maximumDefinedSwitchHeight, logicalRefreshedSwitchWidth).
  - :945-1264 and :1273-1283: HAVE(MATERIAL_HOSTING) glass media controls.
  - :1458-4806: every adjust*/paint*ForVectorBasedControls function.
  - :4810-5390: per-appearance dispatch.
- Source/WebCore/rendering/cocoa/RenderThemeCocoa.h: :41-60, :68-74, :95-98, :198-288.
- Source/WebCore/rendering/mac/RenderThemeMac.mm: 20 blocks, each handing off to RenderThemeCocoa when settings.formControlRefreshEnabled(). Lines :169-171 (!ENABLE), :213-216, :230-233, :266-269, :279-282, :427-448 (!ENABLE, activeButtonTextColor), :728-730, :805-808, :1155-1160, :1190-1193, :1277-1282, :1364-1369, :1448-1451, :1510-1515, :1553-1558, :1578-1583, :1608-1613, :1625-1630, :1656-1661, :1753-1756.
- Source/WebCore/platform/mac/ScrollbarThemeMac.mm:660-665: the scroll corner is not painted because tracks are 'pill shaped'.
- Source/WebCore/platform/graphics/mac/controls/*.mm (ControlFactoryMac, ButtonMac, ToggleButtonMac, SwitchMac, SliderTrackMac, ProgressBarMac and others): no 26 or 27 conditionals. They draw through AppKit, so they show whatever look the OS they run on has.

New findings (not in docs/todo.md):
1. Every WebKit citation in platform-facts.md is unpinned (blob/main): lines 174, 179, 195, 198, 200, 203, 217, 1674, 1676, 1680, 1682, 1684, 1688, 1701.
2. Lines 217 and 1676 cite Source/WebCore/platform/graphics/mac/controls/SwitchThumbMac.mm, which does not exist at de320ba3. The directory has SwitchMac.mm and SwitchMacUtilities.mm, so the link is dead.
3. HIG color.json was changed on 2025-06-09 ('Updated system color values'). Any HIG colour fetched after that date is a G2 value, not a Sonoma value.

Secondary sources seen (not used as citations): MacRumors, Macworld and apfeltalk coverage of WWDC26 / Golden Gate (Liquid Glass opacity slider), and Techzine and Intego on Apple's patch policy.

</details>

### Step 2 — rows checked

322 rows: 155 same, 30 differs, 137 unverifiable (134 of them a probe or reference app could settle). Scopes: fonts-colours 113, geometry 71, widgets-1 86, widgets-2 52. Every researcher reported its scope covered with no line range left out.

Step 5 verdicts on the *differs* rows: 5 CONFIRMED, 24 PLAUSIBLE, 1 REJECTED.

Step 6 triage: 15 Step 8 (macos-tahoe, D4), 1 drop, 4 fix, 10 maintainer (Step 3 probe).

### Findings — *differs* rows

#### M2a-01 · bug · S4 · docs/platform-facts.md:121
**Claim:** `underPageBackgroundColor` (1.1.2 Colors): the cell says Under-page/sidebar background ✅; G1 (14) 14: 'The background behind a document's content' (no sidebar); G2 (26/27) 26/27: 'area beneath your window's views ... backdrop underneath your app's main content'.
**Evidence:** [G1 (14)] https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS [G2 (26/27)] https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/underpagebackgroundcolor.json; HIG color.json §macOS (2026-08-06) Note: Role text: drop 'sidebar'. API exists (introducedAt 10.8).
**Verdict:** CONFIRMED — I read both quotes as cited, but the live HIG still says 'The background behind a document's content' (its newest change-log entry is December 16, 2025, so the 2026-08-06 date could not be checked); the 'area beneath your window's views' wording comes only from the NSColor page, and no Apple source supports 'sidebar'. Confirmed value: Apple never mentions a sidebar: HIG macOS table, 2024 archive and live page (last change-log entry 2025-12-16): 'The background behind a document's content.'; NSColor doc: 'The color to use in the are….
**Triage:** fix — Apple's own description at the target version contradicts the cell's role text; wording fix, no value change.
**Status:** open

#### M2a-02 · bug · S4 · docs/platform-facts.md:127
**Claim:** `highlightColor` (1.1.2 Colors): the cell says Highlight overlay ✅; G1 (14) 14: 'The virtual light source onscreen'; G2 (26/27) 26/27: 'virtual light source on the screen'; used by highlight(withLevel:).
**Evidence:** [G1 (14)] https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS [G2 (26/27)] https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/highlightcolor.json Note: Role text: it is a bevel light source, not a highlight overlay.
**Verdict:** CONFIRMED — Both cited texts read as claimed, and highlightColor.json does name highlight(withLevel:) as its caller, so Apple describes it as a light source; 'overlay' is at best a paraphrase of how highlight(withLevel:) blends toward it. Confirmed value: HIG macOS (2024 archive): 'The virtual light source onscreen.'; NSColor doc: 'The color to use as a virtual light source on the screen', invoked by highlight(withLevel:).
**Triage:** fix — Apple calls it a virtual light source; wording fix, no value change.
**Status:** open

#### M2a-03 · bug · S2 · docs/platform-facts.md:174
**Claim:** `Control corner radius` (1.1.3 Geometry): the cell says 5px (measured), WebKit baseBorderRadius=5; G1 14 (WebKit legacy path): 5, but only for CSS-styled <select> (menulist-button), truncated as baseBorderRadius + fontScale - 1; AppKit's push-button radius is not published; G2 26 (WebKit vector path; no 27 conditional): push button and <select> 6 (continuous); text field 5.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1351 and #L1448-L1462 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L1814-L1822 ; #L1949 ; #L3085-L3090 Note: The G2 values are WebKit's drawing of Liquid Glass, not AppKit's. The cell's WebKit link is unpinned (Step 1 finding 1). todo.md:995-1003 already covers whether the radius depends on control size.
**Verdict:** PLAUSIBLE — The values hold at de320ba: legacy baseBorderRadius=5 (RenderThemeMac.mm:1351) is used only in adjustMenuListButtonStyle as trunc(5+fontScale-1) (:1460); the vector path's shapeForButton is 6*zoom, continuous, on Mac (RenderThemeCocoa.mm:1821,1850), and paintMenuListForVectorBasedControls draws <select> with it on Mac (:3258); text fields use 5 (:1949, used at :3071/:3073; the cited #L3085-3090 is border-colour code, not the radius). The vector path is compiled only for macOS 26+ (PlatformEnableCocoa.h:1189-1193), but these are shapes WebKit draws itself, not AppKit metrics, so the AppKit radi…
**Triage:** maintainer (Step 3 probe) — WebKit's legacy 5 is a styled-<select> radius, so AppKit's Sonoma push-button radius has no primary source; the probe settles it. The Tahoe 6 (WebKit vector path) feeds Step 8.
**Status:** open

#### M2a-04 · bug · S2 · docs/platform-facts.md:195
**Claim:** `NSButton horizontal padding` (1.1.4 Widget Metrics): the cell says ~8px (WebKit controlPadding); G1 14 (WebKit legacy): 8 left/right, 2 top, 3 bottom (WebKit's own 'Just use 8px'); G2 26 (WebKit vector, mac): 6 left/right, 0 top, 1 bottom.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1690-L1704 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3356-L3359 Note: The cell misquotes Gecko. At firefox dc44717d forms.css#L514, <button> has padding-inline: 8px; the 4px is <select> (#L227). AppKit's own padding is unpublished.
**Verdict:** PLAUSIBLE — Legacy controlPadding returns {top 2, right 8, bottom 3, left 8} (RenderThemeMac.mm:1694-1700); in adjustButtonStyleForVectorBasedControls the Mac branch (#else of IOS_FAMILY at :3331) applies {0, 6, 1, 6} (RenderThemeCocoa.mm:3357-3358) unless the author set padding (:2352-2361). This is WebKit's own CSS padding for its redrawn button; the AppKit bezel padding is still unpublished, so it needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma value 8 equals the cell; the Tahoe 6 is WebKit's vector path, (WebKit)-tagged as the cell already is.
**Status:** open

#### M2a-05 · bug · S2 · docs/platform-facts.md:200
**Claim:** `NSSlider track height` (1.1.4 Widget Metrics): the cell says 5px (WebKit sliderTrackWidth); G1 14 (WebKit legacy): 5 (track radius 2); G2 26 (WebKit vector, mac; no 27 conditional): 8.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SliderTrackMac.mm#L47 , #L78 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3875-L3879 Note: The cell is right for 14 (WebKit) and wrong for 26 (WebKit's rendition). The cell's link is unpinned.
**Verdict:** PLAUSIBLE — SliderTrackMac.mm:47 has sliderTrackWidth=5 and :78 radius 2; RenderThemeCocoa.mm:3876 has trackThicknessForVectorBasedControls=8.0 under PLATFORM(MAC), used at :3997. The 8 is WebKit's own vector track, not AppKit's NSSlider, so it needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma 5 equals the cell; Tahoe 8 is WebKit's vector track, (WebKit)-tagged.
**Status:** open

#### M2a-06 · bug · S2 · docs/platform-facts.md:201
**Claim:** `NSSlider thumb diameter` (1.1.4 Widget Metrics): the cell says 21px (measured); WebKit 17; G1 14 (WebKit legacy): 17 (WebKit FIXME: should come from knobThickness); G2 26 (WebKit vector): 24 x 16 capsule, not a circle.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1650-L1652 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3881 , #L4131-L4141 Note: 21 has no source on either generation. The knobThickness doc gives no value.
**Verdict:** PLAUSIBLE — RenderThemeMac.mm:1650-1652 has sliderThumbThickness=17 with the knobThickness FIXME; the vector path sets a 24x16 thumb (RenderThemeCocoa.mm:3881, :4131-4142) with corner radius min(w,h)/2, a continuous capsule (:1956-1958). These are WebKit-drawn shapes, so AppKit's macOS 26 knob still needs a measurement.
**Triage:** maintainer (Step 3 probe) — The cell's measured 21 and WebKit's 17 disagree at the target version; only a measurement of NSSlider settles it.
**Status:** open

#### M2a-07 · bug · S2 · docs/platform-facts.md:202
**Claim:** `NSSlider tick mark length` (1.1.4 Widget Metrics): the cell says 8px (measured); G1 14 (WebKit legacy): tick 1 x 3; G2 26 (WebKit vector): track/4 = 2.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1640-L1643 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3880 Note: Both are WebKit's own datalist ticks, not AppKit's NSSlider ticks. 8 is unsourced.
**Verdict:** PLAUSIBLE — RenderThemeMac.mm:1642 returns IntSize(1, 3); RenderThemeCocoa.mm:3880 gives tick length = track thickness/4 = 2 on Mac, used at :3901/:4075, and :3885 has a FIXME that sliderTickSize was not updated. Neither is AppKit's tick, which the row says was measured at 8, so it needs a measurement.
**Triage:** maintainer (Step 3 probe) — Measured 8 against WebKit's 1 x 3 tick at the target version; the probe settles it.
**Status:** open

#### M2a-08 · bug · S2 · docs/platform-facts.md:203
**Claim:** `NSProgressIndicator bar height` (1.1.4 Widget Metrics): the cell says 6px visual track; frame 20; G1 14 (WebKit legacy): cell frame 20 regular / 12 small / 12 mini; the visual bar is drawn by AppKit; G2 26 (WebKit vector): bar 8 thick, corner radius 4.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/ProgressBarMac.mm#L49-L57 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3770-L3773 Note: 6 is unsourced on 14.
**Verdict:** PLAUSIBLE — ProgressBarMac.mm:53-56 gives cell heights 20/12/12/20 (regular/small/mini/large), and the legacy bar itself is drawn by AppKit; in the vector path paintProgressBarForVectorBasedControls has barBlockSize=8 and corner radius 4 (RenderThemeCocoa.mm:3770-3773). That is WebKit's own bar, so AppKit's macOS 26 visual track needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma has no source for the visible bar (cell is measured); Tahoe 8 / radius 4 is WebKit's vector bar, (WebKit)-tagged.
**Status:** open

#### M2a-09 · bug · S2 · docs/platform-facts.md:216
**Claim:** `NSSwitch intrinsic size` (1.1.4 Widget Metrics): the cell says 38 x 22 (WebKit); G1 14 (WebKit legacy): 38x22 regular, 32x18 small, 26x15 mini; G2 26 (WebKit vector; no 27 conditional): 18 high x 41.14 wide (18 x 64/28); idealSwitchSize 64x28.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SwitchMacUtilities.mm#L38-L46 ; .../rendering/mac/RenderThemeMac.mm#L1060-L1069 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L216-L221 , #L417-L418 , #L4516-L4540 Note: The G2 numbers are WebKit's rendition. A probe of AppKit's NSSwitch intrinsicContentSize on 26 is needed before changing the cell.
**Verdict:** PLAUSIBLE — Legacy {38,22}/{32,18}/{26,15} hold (SwitchMacUtilities.mm:42-45, RenderThemeMac.mm:1063-1066); in the vector path the Mac logicalSwitchHeight is 18 (RenderThemeCocoa.mm:220), idealSwitchSize is 64x28 (:417), the width is 18*64/28 = 41.14 (:418, set at :4523/:4536), and AX_ZOOM_ADJUSTMENTS is not enabled in PlatformEnableCocoa.h. That is WebKit's HTML switch, not NSSwitch's intrinsic size, so it needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma 38 x 22 equals the cell; the Liquid Glass switch geometry feeds Step 8.
**Status:** open

#### M2a-10 · bug · S2 · docs/platform-facts.md:217
**Claim:** `NSSwitch thumb diameter` (1.1.4 Widget Metrics): the cell says ~18px (WebKit SwitchThumbMac.mm box = track height); G1 14 (WebKit legacy): thumb box = track height (22 regular); visual knob drawn by AppKit; G2 26 (WebKit vector): capsule thumb: width 38/64 of track (≈24.4) and height ≈14.8 on an 18 track (ratio interpolated between 13/16 and 24/28).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SwitchMac.mm#L231 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L420-L450 , #L597-L611 Note: ~18 is unsourced. The cited SwitchThumbMac.mm does not exist (Step 1 finding 2).
**Verdict:** PLAUSIBLE — SwitchMac.mm:231 sets the thumb box to the track height (the row cites SwitchThumbMac.mm instead); in the vector path the Mac thumb width is 38/64 of the track (RenderThemeCocoa.mm:599-603, :609), about 24.4 on a 41.14 track, and the height ratio interpolated from 13/16 to 24/28 (:426-450) gives about 14.76 on an 18 track. WebKit draws this capsule itself, so the AppKit knob needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma unchanged; the Tahoe capsule thumb feeds Step 8 (and the model has only a diameter: Step 8 records what it cannot carry).
**Status:** open

#### M2a-11 · bug · S2 · docs/platform-facts.md:1112
**Claim:** `border.corner_radius` (2.1.6 Global Geometry): the cell says 5px (measured); G1 14 (WebKit legacy): 5 (styled <select> only); G2 26 (WebKit vector): button 6, text field 5.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1351 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L1814-L1822 , #L1949 Note: Mirrors line 174.
**Verdict:** PLAUSIBLE — This is the same source as claim 2: the legacy 5 applies only to styled <select> (RenderThemeMac.mm:1351, :1460), while the macOS 26 vector path uses buttons/<select> 6 (RenderThemeCocoa.mm:1821) and text fields 5 (:1949, :3071). These are WebKit's own shapes, so the mapping to a single macOS border.corner_radius needs a measurement.
**Triage:** maintainer (Step 3 probe) — Same evidence as the chapter 1 corner-radius row: no AppKit source at the target version.
**Status:** open

#### M2a-12 · bug · S2 · docs/platform-facts.md:1170
**Claim:** `min_height` (2.3 Button): the cell says NSButton intrinsic: 22; G1 14/15 (WebKit has no version conditional): 20 for a regular push button cell (small 16, mini 13, large 28), drawn with outsets of 5 top and 7 bottom; G2 26 (WebKit FORM_CONTROL_REFRESH path): 20: adjustButtonStyleForVectorBasedControls calls RenderTheme::adjustButtonStyle, which takes its height from RenderThemeMac::controlSize → buttonSizes().
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1006-L1016 ; .../Source/WebCore/platform/graphics/mac/controls/ButtonMac.mm#L50-L71 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3302-L3309 ; .../Source/WebCore/rendering/RenderTheme.cpp#L1434-L1442 ; .../Source/WebCore/rendering/mac/RenderThemeMac.mm#L1716-L1718 Note: The only source for 22 is the appendix at :1687, which cites 'developer discussions' (secondary). WebKit gives 20. Both numbers are inferences about AppKit, so a probe of NSButton intrinsicContentSize / alignmentRect on 14 and 26 should decide.
**Verdict:** PLAUSIBLE — buttonSizes() is {0,20}/{0,16}/{0,13}/{0,28} (RenderThemeMac.mm:1009ff, used at :1718; ButtonMac.mm:53-58) with outsets {5,7,7,7} top/right/bottom/left (ButtonMac.mm:68); the vector path calls RenderTheme::adjustButtonStyle on Mac (RenderThemeCocoa.mm:3307-3308 → RenderTheme.cpp:1527-1529 → the shared helper's controlSize at :1439), so the height is 20 with no version conditional. That is WebKit's CSS box height, not NSButton's intrinsicContentSize, so 20 against 22 needs a measurement.
**Triage:** maintainer (Step 3 probe) — WebKit gives 20 for a regular push button at both generations, the cell 22 rests on developer discussions; the probe settles it.
**Status:** open

#### M2a-13 · bug · S2 · docs/platform-facts.md:1171
**Claim:** `border.padding_horizontal` (2.3 Button): the cell says NSButton: ~8 (WebKit); G1 14/15: 8 (WebKit controlPadding(PushButton) = {2 top, 8, 3 bottom, 8}); G2 26: 6 (PLATFORM(MAC) branch: PaddingBox {0, 6, 1, 6}).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1690-L1701 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3356-L3359 Note: The cell is right for 14/15 (WebKit). WebKit's 26 rendition uses 6. The cell's WebKit link at :1688 is unpinned (blob/main).
**Verdict:** CONFIRMED — Both cited values read as claimed at de320ba3, and the row already uses WebKit as its source, so the G2 change from 8 to 6 holds for that source. Confirmed value: G1 14/15: 8 (RenderThemeMac.mm:1690-1701, PaddingBox {2, 8, 3, 8}); G2 26: 6 (RenderThemeCocoa.mm:3355-3357 #else-of-IOS_FAMILY branch, PaddingBox {0, 6, 1, 6}, applied after RenderTheme::adjustButton….
**Triage:** Step 8 (macos-tahoe, D4) — CONFIRMED Tahoe change 8 to 6 in the (WebKit) source the cell already uses; Sonoma unchanged.
**Status:** open

#### M2a-14 · bug · S2 · docs/platform-facts.md:1172
**Claim:** `border.padding_vertical` (2.3 Button): the cell says 3 (measured) (22−16)/2; G1 14/15: 2 top / 3 bottom (WebKit); G2 26: 0 top / 1 bottom (WebKit).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1699-L1700 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3358 Note: The derivation is already on todo.md:325-343. The values here are new evidence for that item.
**Verdict:** PLAUSIBLE — WebKit gives 2 top / 3 bottom in G1 (RenderThemeMac.mm:1700) and 0 top / 1 bottom in G2 (RenderThemeCocoa.mm:3356), but the row is a measured AppKit value, (22−16)/2, and WebKit's CSS padding for an HTML button excludes the border and bezel, so a measurement is needed to replace it.
**Triage:** maintainer (Step 3 probe) — Measured (22-16)/2 against WebKit 2/3 at Sonoma; WebKit's CSS padding excludes the bezel, so only a measurement settles it. Tahoe 0/1 feeds Step 8.
**Status:** open

#### M2a-15 · bug · S2 · docs/platform-facts.md:1196
**Claim:** `border.padding_horizontal` (2.4 Text Input): the cell says NSTextField: 4; G1 14/15: no source (WebKit's G1 sets only search-field padding, 1px, to 'match AppKit text positioning'); G2 26: 0.5em, truncated to whole px (6 at a 13 px font).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1539-L1540 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L2787-L2804 , #L2807 , #L2857 Note: The cell's 4 has no source. WebKit's 26 rendition differs.
**Verdict:** PLAUSIBLE — G1 WebKit has no text-field padding override (RenderThemeMac.mm:1539-1540 is inside adjustSearchFieldStyle, 1px). G2 applies 0.5em inline padding truncated to int (RenderThemeCocoa.mm:2787-2807, 2857), which is 6 at 13–13.33px, but only to inputs without a datalist that are not number fields. This is WebKit's HTML input, not AppKit NSTextField, so it needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — No Sonoma source either way; Tahoe 0.5em (6 at 13 px) is WebKit's vector text field, (WebKit)-tagged.
**Status:** open

#### M2a-16 · bug · docs/platform-facts.md:1212
**Claim:** `indicator_width` (2.5 Checkbox): the cell says NSButton checkbox: 14; G1 14/15: 14 regular (12 small, 10 mini, 16 large). Radio: 16 regular, or 14 when large controls are supported.; G2 26: 16 (checkboxRadioSizeForVectorBasedControls). Checkbox corner radius 5.5 at 16..
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/ToggleButtonMac.mm#L49-L71 ; .../Source/WebCore/rendering/mac/RenderThemeMac.mm#L938-L979 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L1589 , #L1679 Note: The cell is right for 14/15. WebKit's 26 rendition uses 16. Line 1222 radio_indicator_width inherits this; WebKit's G1 radio is 16, not 14, unless large controls are supported.
**Verdict:** REJECTED — The G1 table holds (14 regular, ToggleButtonMac.mm:49-71; RenderThemeMac.mm:938-946). In G2, checkboxRadioSizeForVectorBasedControls = 16 (RenderThemeCocoa.mm:1589) is used only as the denominator of the corner-radius ratio 5.5/16 (lines 1679, 1857), not as the box size. The G2 HTML checkbox is still sized by RenderThemeMac::controlSize → checkboxSize → checkboxSizes() (RenderThemeMac.mm:1709-1710, 967), so source does not give 16.
**Triage:** drop — REJECTED: the Tahoe 16 is only the denominator of WebKit's corner-radius ratio, not the drawn size; Sonoma 14 holds.
**Status:** open

#### M2a-17 · bug · S2 · docs/platform-facts.md:1292
**Claim:** `track_height` (2.9 Slider): the cell says NSSlider: 5; G1 14/15: 5 (sliderTrackWidth), radius 2; G2 26: 8 (trackThicknessForVectorBasedControls, PLATFORM(MAC); iOS uses 4).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SliderTrackMac.mm#L47 , #L78 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3875-L3879 Note: The cell is right for 14/15. WebKit's 26 rendition uses 8.
**Verdict:** PLAUSIBLE — G1 sliderTrackWidth = 5 with radius 2 (SliderTrackMac.mm:47, 78) matches the row. G2 trackThicknessForVectorBasedControls = 8 under PLATFORM(MAC) (RenderThemeCocoa.mm:3875-3879) is confirmed, but it is WebKit's own vector HTML slider, so the AppKit NSSlider on 26 still needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Duplicate of the chapter 1 slider track row: Sonoma 5 holds, Tahoe 8 (WebKit).
**Status:** open

#### M2a-18 · bug · S2 · docs/platform-facts.md:1293
**Claim:** `thumb_diameter` (2.9 Slider): the cell says NSSlider knob: 21; G1 14/15: 17 (sliderThumbThickness, with a FIXME that AppKit's knobThickness 'returns an incorrect value'); G2 26: 24 × 16 thumb (not circular).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1650-L1652 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3881 , #L4131-L4132 Note: The cell's 21 is measured and already ❓ in the appendix at :1681. In the 26 rendition the thumb is a capsule, so a single 'diameter' does not fit it.
**Verdict:** PLAUSIBLE — G1 sliderThumbThickness = 17, with the FIXME that AppKit's knobThickness is wrong (RenderThemeMac.mm:1650-1652). G2 kDefaultSliderThumb 24×16 (RenderThemeCocoa.mm:3881, 4131-4142), drawn as a capsule with radius min(w,h)/2 (line 1956). Both are WebKit HTML sizes, so whether AppKit's knob is 21 needs a measurement.
**Triage:** maintainer (Step 3 probe) — Duplicate of the chapter 1 thumb row: 21 measured against WebKit 17.
**Status:** open

#### M2a-19 · bug · S2 · docs/platform-facts.md:1294
**Claim:** `tick_mark_length` (2.9 Slider): the cell says NSSlider: 8; G1 14/15: 3 (sliderTickSize = IntSize(1, 3); offset from track centre −9); G2 26: 2 (= track thickness 8 / 4).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1640-L1648 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3880 Note: No source supports 8.
**Verdict:** PLAUSIBLE — G1 sliderTickSize (1,3) with offset −9 (RenderThemeMac.mm:1640-1648) and G2 tickLength = 8/4 = 2 (RenderThemeCocoa.mm:3880) read as claimed. The G2 ticks are 2×2 dots centred on the track (lines 3901-3909), not marks beside it. Both are WebKit datalist ticks, not NSSlider tick marks, so a measurement is needed.
**Triage:** maintainer (Step 3 probe) — Duplicate of the chapter 1 tick row: 8 measured against WebKit 3.
**Status:** open

#### M2a-20 · bug · S2 · docs/platform-facts.md:1303
**Claim:** `track_height` (2.10 Progress Bar): the cell says NSProgressIndicator: 6; G1 14/15: no source for the visible bar. WebKit's cell (frame) height is 20 regular / 12 small and mini, and CoreUI draws the bar inside it.; G2 26: 8 (barBlockSize), corner radius 4.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/ProgressBarMac.mm#L49-L57 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3770-L3773 Note: WebKit's 26 rendition differs from 6. todo.md:561-562 already notes that NSProgressIndicator is unread (border).
**Verdict:** PLAUSIBLE — G1 cell heights 20/12/12/20 (ProgressBarMac.mm:49-57) and G2 barBlockSize 8, corner radius 4 (RenderThemeCocoa.mm:3770-3773; the paint function is not platform-gated) read as claimed. The G2 value is WebKit's HTML progress bar, not AppKit NSProgressIndicator, so the row needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Duplicate of the chapter 1 progress row.
**Status:** open

#### M2a-21 · bug · S2 · docs/platform-facts.md:1333
**Claim:** `background_color` (2.12 Sidebar): the cell says underPageBackgroundColor; G1 14/15: Apple's sidebar background is NSVisualEffectView.Material.sidebar, 'The material for the background of window sidebars' (10.11+). underPageBackgroundColor is 'the area beneath your window's views'.; G2 26/27: HIG: 'sidebars can float above content in the Liquid Glass layer' (change log June 9, 2025).
**Evidence:** [G1] https://developer.apple.com/tutorials/data/documentation/appkit/nsvisualeffectview/material-swift.enum/sidebar.json (Last-Modified 2026-08-14) ; https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/underpagebackgroundcolor.json (read 2026-10-11) [G2] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/sidebars.json (Last-Modified 2026-08-20, read 2026-10-11) Note: The source named in the cell is not the one Apple documents for sidebars. No colour value is proposed; a probe would measure it.
**Verdict:** CONFIRMED — All three quotes read verbatim on 2026-10-11 (Last-Modified 2026-08-14 / 2026-08-14 / 2026-08-20). The sidebar background is a material, not underPageBackgroundColor, so the row has no single colour value to keep. Confirmed value: Sidebar background is NSVisualEffectView.Material.sidebar ('The material for the background of window sidebars', macOS 10.11+). underPageBackgroundColor is 'The color to use in the area beneath your w….
**Triage:** fix — CONFIRMED: Apple's sidebar background is the NSVisualEffectView sidebar material, not underPageBackgroundColor; the cell names the material and the colour value becomes ❓ unless a source gives one.
**Status:** open

#### M2a-22 · bug · S2 · docs/platform-facts.md:1420
**Claim:** `divider_color` (2.17 Splitter): the cell says separatorColor; G1+G2 (docs unversioned) 14/26: dividerColor: clear for the default thick style (and paneSplitter in textured windows); other thin dividers use an unnamed system colour 'that provides appropriate contrast'. separatorColor docs: 'Do not use this color for split view dividers or window chrome….
**Evidence:** [G1+G2 (docs unversioned)] https://developer.apple.com/tutorials/data/documentation/appkit/nssplitview/dividercolor.json and .../nscolor/separatorcolor.json (both Last-Modified 2026-08-14, read 2026-10-11) Note: Apple's own pages contradict the cell. Docs carry no version; dividerColor is introducedAt 10.5 and not deprecated. Probe: read NSSplitView.dividerColor per style on 14 and 26.
**Verdict:** CONFIRMED — Both pages read verbatim on 2026-10-11 (Last-Modified 2026-08-14). The docs do not say thick is the default dividerStyle (the dividerStyle page states no default), so the claim's word 'default' is unsourced. Confirmed value: NSSplitView.dividerColor: clear when dividerStyle is thick, or paneSplitter in a textured window; other thin dividers get an unnamed system colour 'that provides appropriate contrast between two white….
**Triage:** fix — CONFIRMED: Apple says separatorColor must not be used for split view dividers and dividerColor is clear for the thick style; the cell is corrected to dividerColor with the quote, and the word "default" for the thick style stays ❓ (the dividerStyle page states no default).
**Status:** open

#### M2a-23 · bug · S2 · docs/platform-facts.md:1467
**Claim:** `track_width` (2.21 Switch): the cell says 38px; G1 14: 38 (regular cell 38x22; small 32x18, mini 26x15; paint rect outset 2/2/1/2); G2 26: WebKit Liquid Glass switch: ideal 64x28 (ratio 2.29, against 1.73 for 38x22); default HTML size 18 tall x 41.1 wide (18*64/28).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1060-L1082 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SwitchMacUtilities.mm#L38-L68 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L216-L221 and #L415-L418 and #L4515-L4539 Note: The G2 numbers are WebKit's vector rendition, not NSSwitch's intrinsic size, but its proportions rule out 38 wide at 22 tall. No 27 conditional. Probe: NSSwitch.intrinsicContentSize per controlSize (extraLarge exists from 26.0).
**Verdict:** PLAUSIBLE — G1 38×22 regular, outsets 2/2/1/2 (RenderThemeMac.mm:1060-1082; SwitchMacUtilities.mm:38-68) matches the row. G2 idealSwitchSize 64×28 and HTML default 18 tall × 18·64/28 ≈ 41.1 wide (RenderThemeCocoa.mm:216-221, 417-418, 4516-4537) is confirmed, but this is WebKit's Liquid Glass HTML switch, so AppKit NSSwitch on 26 needs a measurement.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma 38 holds; Liquid Glass switch 64 x 28 ideal / 41.1 at 18 high feeds Step 8.
**Status:** open

#### M2a-24 · bug · S2 · docs/platform-facts.md:1469
**Claim:** `thumb_diameter` (2.21 Switch): the cell says ~18px (measured); G1 14: not stated: the knob is CoreUI art drawn in a square as tall as the track (22), so the visible knob size is not in source; G2 26: not a circle: a capsule 38/64 of the track wide (idealThumbWidth 38 on Mac) and 24/28 of the track tall at height 28 (13/16 at 16, 30/36 at 36).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SwitchMac.mm#L231-L277 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L420-L452 and #L597-L612 Note: A single diameter cannot describe the G2 thumb (width differs from height). G1 ~18 is unverifiable.
**Verdict:** PLAUSIBLE — G1: the knob is CoreUI art in a square as tall as the track (SwitchMac.mm:231-233, 268-277), so its visible size is not in source. G2 Mac thumb width = 38/64 of the track (RenderThemeCocoa.mm:598-603), with heights 13/16, 24/28 and 30/36 interpolated (lines 426-450): about 24.4×14.8 at the 18px HTML default. This is a WebKit HTML switch, not a measurement of NSSwitch.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma knob is CoreUI art (no source, stays ❓); the Tahoe capsule thumb feeds Step 8.
**Status:** open

#### M2a-25 · bug · S2 · docs/platform-facts.md:1473
**Claim:** `thumb_background` (2.21 Switch): the cell says (measured) white; G1 14: CoreUI knob (kCUIWidgetSwitchKnob); no colour in source; G2 26: light: white; dark: white x0.85 alpha (disabled x0.5).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/SwitchMac.mm#L276-L277 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L475-L495 Note: Differs in dark mode on G2 (WebKit). Sonoma is unverifiable.
**Verdict:** PLAUSIBLE — Values read as cited, but G2 is WebKit's own vector HTML-switch painter gated by the formControlRefreshEnabled setting (L740-742), not AppKit NSSwitch, and its light value (white) equals the cell anyway, so a macOS 26 NSSwitch measurement is needed to show a difference. Confirmed value: G1: SwitchMac.mm L276-277 draws kCUIWidgetSwitchKnob via CoreUI, no colour; G2: liquidGlassSwitchThumbColor (RenderThemeCocoa.mm L475-495) = Color::white in light, white alpha x0.85 dark enabled, x sw….
**Triage:** Step 8 (macos-tahoe, D4) — Light value white equals the cell; the Tahoe dark 0.85 alpha thumb is WebKit's Liquid Glass painter, (WebKit)-tagged for Step 8.
**Status:** open

#### M2a-26 · bug · S2 · docs/platform-facts.md:1545
**Claim:** `background_color` (2.24 ComboBox): the cell says controlColor; G1 14: CoreUI bezel (NSPopUpButtonCell); no colour in source. controlColor docs: by default 'a pattern color ... same as windowBackgroundColor'; G2 26: WebKit vector menu list = button fill: secondarySystemFill composited over the canvas (default or submit buttons use the accent).
**Evidence:** [G1] https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controlcolor.json (Last-Modified 2026-08-14) [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L2183-L2195 and #L3247-L3262 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L686-L691 Note: G2 value is WebKit's rendition. Sonoma unverifiable.
**Verdict:** PLAUSIBLE — Doc text and WebKit lines (RenderThemeCocoa.mm L2183-2195, L3247-3262; RenderThemeMac.mm L686-691) say what is claimed, but the G2 fill is WebKit's FormControlRefresh emulation of a select, not NSPopUpButton's CoreUI bezel, so the AppKit 26 fill needs measurement. Confirmed value: controlColor doc (Last-Modified 2026-08-14): 'By default, the control color is a pattern color that will draw the ruled lines for the window background, which is the same as returned by windowBackgrou….
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma: CoreUI bezel, no colour source (cell stays as is); Tahoe fill is WebKit's vector menulist, (WebKit)-tagged for Step 8.
**Status:** open

#### M2a-27 · bug · S2 · docs/platform-facts.md:1547
**Claim:** `font.color` (2.24 ComboBox): the cell says controlTextColor; G1 14: not set for native Menulist (WebKit locks the font, not the colour); G2 26: labelColor enabled, tertiaryLabelColor disabled.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1362-L1395 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3222-L3230 and #L3296-L3300 Note: labelColor and controlTextColor may resolve to the same value; the probe should compare them.
**Verdict:** PLAUSIBLE — G2 holds for WebKit's vector menulist only; G1 (RenderThemeMac.mm L1362-1395 sets no colour) shows only that WebKit leaves text colour to CSS and says nothing about NSPopUpButton's title colour, so neither refutes controlTextColor for AppKit without a measurement. Confirmed value: G2: buttonTextColor = AppleSystemLabel (labelColor) enabled / AppleSystemTertiaryLabel disabled, applied in adjustMenuListStyleForVectorBasedControls when no explicit colour (RenderThemeCocoa.mm L3226….
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma unchanged; Tahoe labelColor / tertiaryLabelColor is WebKit's vector menulist.
**Status:** open

#### M2a-28 · bug · S2 · docs/platform-facts.md:1553
**Claim:** `arrow_icon_size` (2.24 ComboBox): the cell says ~16–18px (measured); G1 14: CoreUI arrows; no size in source; G2 26: SF Symbol chevron.up.chevron.down scaled to 0.55 em wide (14.46x20.17 path), so about 7.2 x 10.0 at 13 pt.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1362-L1395 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3441-L3478 Note: 7.2 x 10 is derived from the 0.55 em scale and the glyph aspect at 13 pt.
**Verdict:** PLAUSIBLE — The source matches (inside paintMenuListButtonDecorationsForVectorBasedControls, L3404, which is FormControlRefresh-only), but it is WebKit's HTML select glyph, the em depends on the select's used font size, and G1 has no value, so the AppKit popup arrow size still needs measurement. Confirmed value: G2: chevron.up.chevron.down path 14.4618 x 20.1723, scaled to 0.55 em wide (RenderThemeCocoa.mm L3443-3473), about 7.15 x 9.97 px at a 13px em.
**Triage:** Step 8 (macos-tahoe, D4) — Sonoma has no source (cell measured); the Tahoe SF Symbol chevron size is WebKit's, (WebKit)-tagged for Step 8.
**Status:** open

#### M2a-29 · bug · S2 · docs/platform-facts.md:1554
**Claim:** `arrow_area_width` (2.24 ComboBox): the cell says ~16–18px (measured); G1 14: 26 regular (23 small, 22 mini, 26 large): right padding holding the arrow; G2 26: same 26 (shared padding function).
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L925-L934 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1401-L1411 Note: The appendix at :1678 already names 26 as the arrow-area right padding, so the cell contradicts its own appendix.
**Verdict:** PLAUSIBLE — The 26 is WebKit's right content inset for text layout in its own box (which also carries popupButtonMargins 3 left/right, L903-912), not the 'arrow area including its padding' defined at platform-facts.md L982, so mapping 26 to arrow_area_width needs a measurement of NSPopUpButton. Confirmed value: popupButtonPadding {top,right,bottom,left} = regular {2,26,3,8}, small {2,23,3,8}, mini {2,22,3,10}, large {2,26,3,8} (RenderThemeMac.mm L925-934, used by platformPopupInternalPaddingBox L1401-1411).
**Triage:** maintainer (Step 3 probe) — WebKit's 26 is a text inset of its own box, not the arrow area as platform-facts.md:982 defines it; the appendix (:1678) already notes 26. A measurement settles it.
**Status:** open

#### M2a-30 · bug · S2 · docs/platform-facts.md:1557
**Claim:** `border.padding_vertical` (2.24 ComboBox): the cell says ~3px (measured); G1 14: 2 top / 3 bottom (all sizes); G2 26: same 2 / 3.
**Evidence:** [G1] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L925-L934 [G2] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1401-L1411 Note: Asymmetric: '2 top / 3 bottom'.
**Verdict:** PLAUSIBLE — The values read as cited, but they are WebKit's text insets measured from its layout box (popup margins 0 top / 1-2 bottom, L903-912), not from the bezel's inner border edge required by platform-facts.md L949, so the per-side mapping to border.padding_vertical needs measurement. Confirmed value: popupButtonPadding top 2 / bottom 3 for all four control sizes (RenderThemeMac.mm L925-934).
**Triage:** maintainer (Step 3 probe) — WebKit 2/3 is measured from its layout box, not the bezel's inner edge (platform-facts.md:949); a measurement settles it.
**Status:** open

### Step 4 — preset-validator

91 findings: 7 mismatch, 2 other, 82 depends-on-differs-row.

<details><summary>Validator summary</summary>

I audited macos-sonoma.toml (731 lines) and macos-sonoma-live.toml (380 lines) against platform-facts.md, property-registry.toml and inheritance-rules.toml in /home/tibi/Rust/native-theme-m2. Line numbers are from that worktree.

Schema check: every key used exists in property-registry.toml. Size conventions hold: `_px` suffixes, and font sizes as `size_pt` (registry :36-39, :74-76). The shorthands `padding_horizontal_px` and `padding_vertical_px` are registered at :59-60. The icon set is `sf-symbols` for a macOS preset, with no Linux rule involved and no mixing. No key drifts in name from the sibling presets.

Rows the researchers flag as changed, and the keys that depend on them:
- **Rows 174 and 1112 (corner radius 5):** `defaults.border.corner_radius_px`. Button, input, checkbox and combo box inherit it through the border-inheritance rule at inheritance-rules.toml:92.
- **Rows 194 and 1170:** `button.min_height_px`.
- **Rows 195 and 1171:** `button.border.padding_horizontal_px`.
- **Row 1172:** `button.border.padding_vertical_px`.
- **Rows 197 and 1196:** `input.border.padding_horizontal_px`.
- **Rows 198 and 1212:** `checkbox.indicator_width_px`.
- **Rows 200/201/202 and 1292/1293/1294:** `slider.track_height_px`, `thumb_diameter_px` and `tick_mark_length_px`.
- **Rows 203 and 1303:** `progress_bar.track_height_px`.
- **Row 216:** `switch.track_width_px`, `track_height_px` and the derived `track_radius_px`; row 1467 covers the width.
- **Rows 217 and 1469:** `switch.thumb_diameter_px`.
- **Row 1473:** `switch.thumb_background` and the derived `disabled_thumb_color`.
- **Rows 121 and 1333:** `sidebar.background_color`. Its value has no citation.
- **Row 1420:** `splitter.hover_color`. `divider_color` itself is not set; it resolves to `defaults.border.color` (inheritance-rules.toml:243).
- **Row 1545:** `combo_box.background_color` is not set; it inherits `defaults.background_color` (inheritance-rules.toml:257), not `controlColor`. The derived `combo_box.disabled_background` also depends on this row.
- **Row 1547:** `combo_box.font.color` is not set. inheritance-rules.toml:479-481 lists it as a wrong safety net, which the preset must provide.
- **Rows 1553, 1554, 1557:** `combo_box.arrow_icon_size_px`, `arrow_area_width_px` (left unset, with a comment citing :1554) and `combo_box.border.padding_vertical_px`.

`arrow_icon_size_px = 17` is the midpoint of the row's range of 16 to 18. The same preset declines to state `arrow_area_width` because its row is also a range, so the two are handled inconsistently. Row 127 (`highlightColor`) has no dependent key in either preset.

Mismatches the researchers did not flag:
- `button.min_width_px` 64 against row 1169, which says none.
- `tab.min_width_px` 64 against row 1316, which says none.
- `progress_bar.min_width_px` 100 against row 1304, which says none.
- `dialog` minimum and maximum width and height 320/560/140/600 against rows 1487-1490, which say AppKit manages them.
- `spinner.stroke_width_px` 2 against row 1535, which says none.
- `combo_box.min_width_px` 120 against row 1551, which says none.
- `expander.header_height_px` 40 against row 1605, which says none.

None of these values cites a source.

The macOS reader also disagrees with the facts: native-theme/src/macos.rs:341 sets `groove_width` 15 (facts :176 and :204 say 16), and :401 sets `divider_width` 9 (facts :215 and :1419 say 6).

All research claims are recorded as reported and were not verified here. I proposed no replacement values.

</details>

#### M2a-P01 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:66
**Claim:** `light.button.min_width_px` (mismatch): preset 64.0, facts (none) — sizes to content (docs/platform-facts.md:1169).
**Evidence:** No source cited. Same in dark :425; live :48 and :234.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2266 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P02 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:163
**Claim:** `light.tab.min_width_px` (mismatch): preset 64.0, facts (none) — sizes to label (docs/platform-facts.md:1316).
**Evidence:** No source cited. Same in dark :522; live :93 and :279.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2277 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P03 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:160
**Claim:** `light.progress_bar.min_width_px` (mismatch): preset 100.0, facts (none) — no minimum (docs/platform-facts.md:1304).
**Evidence:** No source cited. Same in dark :519; live :90 and :276.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2276 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P04 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:263
**Claim:** `light.dialog.min_width_px / max_width_px / min_height_px / max_height_px` (mismatch): preset 320.0 / 560.0 / 140.0 / 600.0, facts (none) — AppKit-managed (all four) (docs/platform-facts.md:1487-1490).
**Evidence:** Lines 263-266; dark :622-625; live :141-144 and :327-330. No source cited.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2284 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P05 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:295
**Claim:** `light.spinner.stroke_width_px` (mismatch): preset 2.0, facts (none) — fin-based (docs/platform-facts.md:1535).
**Evidence:** No source cited. Same in dark :654; live :167 and :353.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2287 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P06 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:299
**Claim:** `light.combo_box.min_width_px` (mismatch): preset 120.0, facts (none) — sizes to content (docs/platform-facts.md:1551).
**Evidence:** No source cited. Same in dark :658; live :171 and :357.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2289 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P07 · bug · S2 · native-theme/src/presets/macos-sonoma.toml:331
**Claim:** `light.expander.header_height_px` (mismatch): preset 40.0, facts (none) — content-sized (docs/platform-facts.md:1605).
**Evidence:** No source cited. Same in dark :690; live :190 and :376.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2294 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2a-P08 · bug · S2 · native-theme/src/macos.rs:341
**Claim:** `scrollbar.groove_width (OS reader)` (other): preset 15.0 (reader); presets state 16.0 (macos-sonoma.toml:134, :493; live :78, :264), facts legacy 16px (docs/platform-facts.md:176 (also :204, :1280)).
**Evidence:** Not a preset file: the macOS reader hardcodes 15 where facts and both presets say 16. Which value wins depends on the merge order.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2272 (the reader's 15 matches neither context); reader code, M4.
**Status:** open

#### M2a-P09 · bug · S2 · native-theme/src/macos.rs:401
**Claim:** `splitter.divider_width (OS reader)` (other): preset 9.0 (reader); presets state 6.0 (macos-sonoma.toml:237, :596; live :131, :317), facts NSSplitView divider: 6 (docs/platform-facts.md:1419 (also :215)).
**Evidence:** Not a preset file: the reader hardcodes 9 against the facts value of 6 that both presets use.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2279 (the reader's 9 against 6); reader code, M4.
**Status:** open

<details><summary>Preset keys that depend on a <i>differs</i> row</summary>

| Preset:line | Key | Preset value | Facts row |
|---|---|---|---|
| macos-sonoma.toml:41 | `light.defaults.border.corner_radius_px` | 5.0 | docs/platform-facts.md:1112 (also :174) |
| macos-sonoma.toml:400 | `dark.defaults.border.corner_radius_px` | 5.0 | docs/platform-facts.md:1112 (also :174) |
| macos-sonoma-live.toml:19 | `light.defaults.border.corner_radius_px` | 5.0 | docs/platform-facts.md:1112 (also :174) |
| macos-sonoma-live.toml:205 | `dark.defaults.border.corner_radius_px` | 5.0 | docs/platform-facts.md:1112 (also :174) |
| macos-sonoma.toml:67 | `light.button.min_height_px` | 22.0 | docs/platform-facts.md:1170 (also :194) |
| macos-sonoma.toml:426 | `dark.button.min_height_px` | 22.0 | docs/platform-facts.md:1170 (also :194) |
| macos-sonoma-live.toml:49 | `light.button.min_height_px` | 22.0 | docs/platform-facts.md:1170 (also :194) |
| macos-sonoma-live.toml:235 | `dark.button.min_height_px` | 22.0 | docs/platform-facts.md:1170 (also :194) |
| macos-sonoma.toml:82 | `light.button.border.padding_horizontal_px` | 8.0 | docs/platform-facts.md:1171 (also :195) |
| macos-sonoma.toml:441 | `dark.button.border.padding_horizontal_px` | 8.0 | docs/platform-facts.md:1171 (also :195) |
| macos-sonoma-live.toml:55 | `light.button.border.padding_horizontal_px` | 8.0 | docs/platform-facts.md:1171 (also :195) |
| macos-sonoma-live.toml:241 | `dark.button.border.padding_horizontal_px` | 8.0 | docs/platform-facts.md:1171 (also :195) |
| macos-sonoma.toml:83 | `light.button.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1172 |
| macos-sonoma.toml:442 | `dark.button.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1172 |
| macos-sonoma-live.toml:56 | `light.button.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1172 |
| macos-sonoma-live.toml:242 | `dark.button.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1172 |
| macos-sonoma.toml:102 | `light.input.border.padding_horizontal_px` | 4.0 | docs/platform-facts.md:1196 (also :197) |
| macos-sonoma.toml:461 | `dark.input.border.padding_horizontal_px` | 4.0 | docs/platform-facts.md:1196 (also :197) |
| macos-sonoma-live.toml:63 | `light.input.border.padding_horizontal_px` | 4.0 | docs/platform-facts.md:1196 (also :197) |
| macos-sonoma-live.toml:249 | `dark.input.border.padding_horizontal_px` | 4.0 | docs/platform-facts.md:1196 (also :197) |
| macos-sonoma.toml:108 | `light.checkbox.indicator_width_px` | 14.0 | docs/platform-facts.md:1212 (also :198) |
| macos-sonoma.toml:467 | `dark.checkbox.indicator_width_px` | 14.0 | docs/platform-facts.md:1212 (also :198) |
| macos-sonoma-live.toml:66 | `light.checkbox.indicator_width_px` | 14.0 | docs/platform-facts.md:1212 (also :198) |
| macos-sonoma-live.toml:252 | `dark.checkbox.indicator_width_px` | 14.0 | docs/platform-facts.md:1212 (also :198) |
| macos-sonoma.toml:145 | `light.slider.track_height_px` | 5.0 | docs/platform-facts.md:1292 (also :200) |
| macos-sonoma.toml:504 | `dark.slider.track_height_px` | 5.0 | docs/platform-facts.md:1292 (also :200) |
| macos-sonoma-live.toml:84 | `light.slider.track_height_px` | 5.0 | docs/platform-facts.md:1292 (also :200) |
| macos-sonoma-live.toml:270 | `dark.slider.track_height_px` | 5.0 | docs/platform-facts.md:1292 (also :200) |
| macos-sonoma.toml:146 | `light.slider.thumb_diameter_px` | 21.0 | docs/platform-facts.md:1293 (also :201) |
| macos-sonoma.toml:505 | `dark.slider.thumb_diameter_px` | 21.0 | docs/platform-facts.md:1293 (also :201) |
| macos-sonoma-live.toml:85 | `light.slider.thumb_diameter_px` | 21.0 | docs/platform-facts.md:1293 (also :201) |
| macos-sonoma-live.toml:271 | `dark.slider.thumb_diameter_px` | 21.0 | docs/platform-facts.md:1293 (also :201) |
| macos-sonoma.toml:147 | `light.slider.tick_mark_length_px` | 8.0 | docs/platform-facts.md:1294 (also :202) |
| macos-sonoma.toml:506 | `dark.slider.tick_mark_length_px` | 8.0 | docs/platform-facts.md:1294 (also :202) |
| macos-sonoma-live.toml:86 | `light.slider.tick_mark_length_px` | 8.0 | docs/platform-facts.md:1294 (also :202) |
| macos-sonoma-live.toml:272 | `dark.slider.tick_mark_length_px` | 8.0 | docs/platform-facts.md:1294 (also :202) |
| macos-sonoma.toml:159 | `light.progress_bar.track_height_px` | 6.0 | docs/platform-facts.md:1303 (also :203) |
| macos-sonoma.toml:518 | `dark.progress_bar.track_height_px` | 6.0 | docs/platform-facts.md:1303 (also :203) |
| macos-sonoma-live.toml:89 | `light.progress_bar.track_height_px` | 6.0 | docs/platform-facts.md:1303 (also :203) |
| macos-sonoma-live.toml:275 | `dark.progress_bar.track_height_px` | 6.0 | docs/platform-facts.md:1303 (also :203) |
| macos-sonoma.toml:219 | `light.sidebar.background_color` | #e8e8ed | docs/platform-facts.md:1333 (also :121) |
| macos-sonoma.toml:578 | `dark.sidebar.background_color` | #262626 | docs/platform-facts.md:1333 (also :121) |
| macos-sonoma.toml:240 | `light.splitter.hover_color` | #d5d5d5 | docs/platform-facts.md:1420 |
| macos-sonoma.toml:599 | `dark.splitter.hover_color` | #3d3d3d | docs/platform-facts.md:1420 |
| macos-sonoma.toml:245 | `light.switch.track_width_px` | 38.0 | docs/platform-facts.md:1467 (also :216) |
| macos-sonoma.toml:604 | `dark.switch.track_width_px` | 38.0 | docs/platform-facts.md:1467 (also :216) |
| macos-sonoma-live.toml:134 | `light.switch.track_width_px` | 38.0 | docs/platform-facts.md:1467 (also :216) |
| macos-sonoma-live.toml:320 | `dark.switch.track_width_px` | 38.0 | docs/platform-facts.md:1467 (also :216) |
| macos-sonoma.toml:246 | `light.switch.track_height_px` | 22.0 | docs/platform-facts.md:216 (the 22 of '38 x 22'; §2 cell :14… |
| macos-sonoma.toml:605 | `dark.switch.track_height_px` | 22.0 | docs/platform-facts.md:216 |
| macos-sonoma-live.toml:135 | `light.switch.track_height_px` | 22.0 | docs/platform-facts.md:216 |
| macos-sonoma-live.toml:321 | `dark.switch.track_height_px` | 22.0 | docs/platform-facts.md:216 |
| macos-sonoma.toml:248 | `light.switch.track_radius_px` | 11.0 | docs/platform-facts.md:1470 (derived from :216 height 22) |
| macos-sonoma.toml:607 | `dark.switch.track_radius_px` | 11.0 | docs/platform-facts.md:1470 (derived from :216) |
| macos-sonoma-live.toml:137 | `light.switch.track_radius_px` | 11.0 | docs/platform-facts.md:1470 (derived from :216) |
| macos-sonoma-live.toml:323 | `dark.switch.track_radius_px` | 11.0 | docs/platform-facts.md:1470 (derived from :216) |
| macos-sonoma.toml:247 | `light.switch.thumb_diameter_px` | 18.0 | docs/platform-facts.md:1469 (also :217) |
| macos-sonoma.toml:606 | `dark.switch.thumb_diameter_px` | 18.0 | docs/platform-facts.md:1469 (also :217) |
| macos-sonoma-live.toml:136 | `light.switch.thumb_diameter_px` | 18.0 | docs/platform-facts.md:1469 (also :217) |
| macos-sonoma-live.toml:322 | `dark.switch.thumb_diameter_px` | 18.0 | docs/platform-facts.md:1469 (also :217) |
| macos-sonoma.toml:244 | `light.switch.thumb_background` | #ffffff | docs/platform-facts.md:1473 |
| macos-sonoma.toml:603 | `dark.switch.thumb_background` | #ffffff | docs/platform-facts.md:1473 |
| macos-sonoma.toml:259 | `light.switch.disabled_thumb_color` | #ffffff80 | docs/platform-facts.md:1473 |
| macos-sonoma.toml:618 | `dark.switch.disabled_thumb_color` | #ffffff80 | docs/platform-facts.md:1473 |
| macos-sonoma.toml:297 | `light.combo_box.background_color` | (absent; resolves to defaults.background_color #f0f0f0 via i… | docs/platform-facts.md:1545 |
| macos-sonoma.toml:656 | `dark.combo_box.background_color` | (absent; resolves to defaults.background_color #1e1e1e) | docs/platform-facts.md:1545 |
| macos-sonoma.toml:307 | `light.combo_box.disabled_background` | #e8e8ed80 | docs/platform-facts.md:1545 |
| macos-sonoma.toml:666 | `dark.combo_box.disabled_background` | #3a3a3c80 | docs/platform-facts.md:1545 |
| macos-sonoma.toml:297 | `light.combo_box.font.color` | (absent; inherits defaults font colour via font_inheritance) | docs/platform-facts.md:1547 |
| macos-sonoma.toml:656 | `dark.combo_box.font.color` | (absent; inherits defaults font colour) | docs/platform-facts.md:1547 |
| macos-sonoma.toml:300 | `light.combo_box.arrow_icon_size_px` | 17.0 | docs/platform-facts.md:1553 |
| macos-sonoma.toml:659 | `dark.combo_box.arrow_icon_size_px` | 17.0 | docs/platform-facts.md:1553 |
| macos-sonoma-live.toml:172 | `light.combo_box.arrow_icon_size_px` | 17.0 | docs/platform-facts.md:1553 |
| macos-sonoma-live.toml:358 | `dark.combo_box.arrow_icon_size_px` | 17.0 | docs/platform-facts.md:1553 |
| macos-sonoma.toml:301 | `light.combo_box.arrow_area_width_px` | (absent; comment cites :1554 range) | docs/platform-facts.md:1554 |
| macos-sonoma.toml:660 | `dark.combo_box.arrow_area_width_px` | (absent; comment cites :1554) | docs/platform-facts.md:1554 |
| macos-sonoma-live.toml:173 | `light.combo_box.arrow_area_width_px` | (absent; comment cites :1554) | docs/platform-facts.md:1554 |
| macos-sonoma-live.toml:359 | `dark.combo_box.arrow_area_width_px` | (absent; comment cites :1554) | docs/platform-facts.md:1554 |
| macos-sonoma.toml:312 | `light.combo_box.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1557 |
| macos-sonoma.toml:671 | `dark.combo_box.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1557 |
| macos-sonoma-live.toml:178 | `light.combo_box.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1557 |
| macos-sonoma-live.toml:364 | `dark.combo_box.border.padding_vertical_px` | 3.0 | docs/platform-facts.md:1557 |

</details>

### Unverifiable rows

No primary source found: the row keeps or gets a ❓ in Step 7; no value is proposed. *Probe* = a probe or the reference app could measure it.

<details><summary>137 unverifiable rows</summary>

| Line | Property | Cell | Probe | Note |
|---|---|---|---|---|
| 61 | `+monospacedSystemFontOfSize:weight:` | SF Mono, 13pt, Regular ✅ | yes | No source says the returned family is 'SF Mono' or that 13/Regular is a default (they are what the reader passes). Keep ❓ instead of ✅. Family-name issue: todo.md:2612. |
| 63 | `+menuFontOfSize:` | SF Pro, 13pt, Regular ✅ | yes | ✅ has no primary source (appendix cites only the NSFont index page). Mark ❓. |
| 64 | `+menuBarFontOfSize:` | SF Pro, 13pt, Regular ❓ | yes | ❓ is correct. |
| 65 | `+toolTipsFontOfSize:` | SF Pro, 11pt, Regular ❓ | yes | Leopard-era HIG (2008) is Apple but predates 14 by 15 years; Cocotron/GNUstep are not Apple. ❓ correct. |
| 66 | `+paletteFontOfSize:` | SF Pro, 12pt, Regular ❓ | yes | Doc says it is the font for 'palette window title bars', not 'tool palettes' generally. ❓ correct. |
| 67 | `+controlContentFontOfSize:` | SF Pro, 13pt, Regular ✅ | yes | ✅ unsupported by a primary source; mark ❓. |
| 74 | `+smallSystemFontSize` | 11pt ✅ | yes | Only Monkeybread (secondary) and 2008 HIG; mark ❓. |
| 108 | `controlColor` | Button/control bg; catalog colour, convert before RGB ✅ | yes | API and role confirmed; the 'NSColorType.catalog' claim has no primary source (doc speaks of a pattern colour). |
| 158 | `NSTextView.insertionPointColor default` | defaults to controlTextColor ✅ | yes | 'defaults to controlTextColor' has no primary source; on 14+ it may follow textInsertionPointColor. |
| 160 | `NSTextInsertionIndicator` | macOS 14+; follows accent by default ✅ | yes | API/version confirmed; 'follows accent colour' unsourced. |
| 162 | `prose: macOS 14 caret matches accent` | macOS 14 changed caret to accent colour | yes | No Apple source opened says accent. |
| 1106 | `focus_ring_offset` | -1px (measured, inset) | yes | No primary source gives -1; keep ❓-level (measured). |
| 173 | `Window corner radius` | 10px (measured) through Sequoia; Tahoe variable per window style (16 / ~20-26) | yes | No Apple doc or source states a window radius for any version; every source the cell cites (macos-corner-fix, Zed, Podfeet, VS Code PR, lapcat, alt-tab) is secondary. WebKit reads the radii at run time (HAVE_NSVIEW_CORNER_CONFIGURATION >= 260100, per Step 1).… |
| 175 | `Frame/border width` | 0.5px (measured) | yes | G1: WebKit leaves the text field to AppKit's NSTextFieldCell and draws its own 1px stroke only in the increased-contrast fallback (TextFieldMac.mm#L55-L91 at de320ba3). So 14 has no source. |
| 176 | `Scrollbar width (legacy)` | 16px (API scrollerWidth) | yes | Apple's scrollerWidth(for:scrollerStyle:) doc gives no number (Last-Modified 2026-08-14). WebKit asks NSScrollerImp at run time (ScrollbarThemeMac.mm#L276-L283 at de320ba3). Gecko at firefox dc44717d uses a 15px non-overlay track (widget/ScrollbarDrawingCocoa.… |
| 177 | `Scrollbar width (overlay)` | ~7px idle thumb | yes | Gecko's 7 idle / 11 hover (firefox dc44717d ScrollbarDrawingCocoa.cpp#L106-L117) and Chromium's minimum thumb width 6 (chromium 7916321c ui/native_theme/native_theme_mac.mm#L332-L338) are both engine constants. G2: WebKit does not paint the scroll corner becau… |
| 179 | `Focus ring offset` | -1px (inset), measured | yes | WebKit's comment 'macOS controls have never honored outline offset' (RenderThemeCocoa.mm#L144-L145 at de320ba3) says only that AppKit takes no offset. The -1 is unsourced for 14 and for 26. |
| 180 | `Disabled opacity` | ≈0.25–0.3 (disabledControlTextColor alpha 0.247) | yes | Apple's disabledControlTextColor doc gives no alpha (Last-Modified 2026-08-14), and the 0.247 comes from a gist (secondary). A probe reading the colour's alpha on 14 and 26 settles it. |
| 181 | `Drop shadows` | yes (compositor-managed) | yes | The NSWindow.hasShadow doc (Last-Modified 2026-08-14) states no default. Trivially observable in a screenshot. |
| 194 | `NSButton intrinsic height` | 22px regular | yes | Gecko's natural height is 22 (firefox dc44717d widget/cocoa/nsNativeThemeCocoa.mm#L828-L843), so the two engines disagree (20 against 22). WebKit's G2 path sets a minimum height only on iOS (RenderThemeCocoa.mm#L3330-L3345), so 26 has no number. The '✅ well-co… |
| 196 | `NSTextField intrinsic height` | 22px (WebKit search field 22/19/17) | yes | The cited evidence is for NSSearchField. WebKit's G2 text field has em-based padding and no fixed height (RenderThemeCocoa.mm#L2835-L2862). |
| 197 | `NSTextField horizontal padding` | 4px (measured) | yes | No source for 14. |
| 198 | `Checkbox indicator` | 14px (WebKit 14 vs Gecko 16) | yes | Gecko gives 16 regular (firefox dc44717d nsNativeThemeCocoa.mm#L675-L686), so the engines still disagree. WebKit G2 uses 16 only as the reference for its corner ratio 5.5/16 (RenderThemeCocoa.mm#L1589, #L1679). |
| 199 | `Checkbox label spacing` | 4px (measured) | yes | No primary source. |
| 204 | `NSScroller track width (legacy)` | 16px | yes | Same as line 176. Gecko uses 15 (ScrollbarDrawingCocoa.cpp#L464 at dc44717d). |
| 205 | `NSScroller thumb width (overlay)` | ~7px idle | yes | Same as line 177. The Gecko numbers check out at dc44717d #L106-L117 but are engine constants. |
| 206 | `NSTabView tab height` | 24px (measured) | yes | No primary source. 27 adds an NSSegmentedControl 'tabs' role with 'a distinct visual appearance' (macOS 27 release notes, AppKit section). |
| 207 | `NSTabView tab horizontal pad` | 12px (measured) | yes | No primary source. |
| 208 | `NSMenuItem item height` | 22px (measured) | yes | No primary source. |
| 209 | `NSMenuItem horizontal padding` | 12px; Chromium item_horizontal_border_padding=12 | yes | Chromium sets 12 in MenuConfig::InitCommon for every platform (chromium 7916321c ui/views/controls/menu/menu_config.cc#L84-L94), and menu_config_mac.mm does not override it, so it is not macOS evidence. On 27, NSMenu hides item symbol images by default (macOS… |
| 210 | `Tooltip padding` | 4px (measured) | yes | No primary source. |
| 211 | `NSTableView row height` | 24px (macOS 11+), ✅ lapcat + AppKit notes 11 | yes | The AppKit Release Notes for macOS 11 (developer.apple.com/tutorials/data/documentation/macos-release-notes/appkit-release-notes-for-macos-11.json, Last-Modified 2026-08-14) say only 'updated metrics (including increased row height)' and give group rows 28pt;… |
| 212 | `NSTableView cell horizontal pad` | 4px (measured) | yes | No primary source. |
| 213 | `NSToolbar bar height` | 38px (measured) | yes | No primary source. The current HIG toolbars.json (Last-Modified 2026-08-20) gives no height. |
| 214 | `NSToolbar item spacing` | 8px ✅ HIG '8 pixels between toolbar controls' | yes | The quote is in neither the current HIG toolbars.json (Last-Modified 2026-08-20) nor the archived OS X HIG WindowTitleBarToolbar page (web.archive.org snapshot 20160306025405, 'Updated: 2015-04-08'). It traces to a pre-2015 HIG that no page opened in this sess… |
| 215 | `NSSplitView thick divider` | 6px (GNUstep) | yes | Apple's docs say only that thick is the default style (NSSplitView.DividerStyle.thick doc) and give no dividerThickness value. GNUstep is not Apple's code. A probe reading dividerThickness settles it. |
| 218 | `NSSwitch track radius` | half height (pill) ✅ | yes | The G2 radius fraction is Apple-internal: the public build sets switchCornerRadiusFraction = 0, and the real value lives in WebKitAdditions (RenderThemeCocoa.mm#L108-L110 at de320ba3). On G1, AppKit draws the track. No source. |
| 219 | `Spinning indicator regular` | 32px (sizeToFit) | yes | Apple's sizeToFit() doc (Last-Modified 2026-08-14) states no size, so the appendix (:1679) attributes 32/16/10 to a doc that does not contain them. Probe. |
| 220 | `Spinning indicator small` | 16px (sizeToFit) | yes | Same as line 219. |
| 221 | `NSPopUpButton intrinsic height` | 21px (WebKit 21/18/15/24) | yes | Right for 14. On 26, WebKit paints <select> as a button with no fixed height (RenderThemeCocoa.mm#L3222-L3263), so 26 needs the probe. |
| 222 | `NSPopUpButton arrow area width` | ~16–18px; WebKit right padding 26 | yes | The 26 in the note checks out. ~16-18 is unsourced. |
| 223 | `NSSegmentedControl segment height` | 24px (measured) | yes | No source. 27 adds the 'tabs' role with a distinct look and fixes NSSegmentedCell drawing under Liquid Glass (macOS 27 release notes). |
| 224 | `NSSegmentedControl separator width` | 1px (measured) | yes | No primary source. |
| 225 | `Disclosure triangle size` | ~13px; Gecko cell 21x21 | yes | Gecko's kDisclosureButtonSize {21,21} checks out (firefox dc44717d nsNativeThemeCocoa.mm#L826), but that is the cell, not the triangle. 13 is unsourced. |
| 235 | `Between unrelated groups` | 20pt ❓ |  | Apple documents 20 only as the IB superview-edge spacing (Auto Layout Guide). There is no group-spacing source, so keep ❓. |
| 236 | `Label to its control` | 8pt ✅ HIG 8/6/5 |  | The legacy HIG it cites was found in no Apple page opened, and the current HIG (layout.json, Last-Modified 2026-09-09) has no macOS spacing values. The ✅ is unsupported. |
| 238 | `Compact spacing (toolbar items)` | 8pt ✅ HIG | yes | Same as line 214: the HIG quote was not found. |
| 252 | `Sidebar (small)` | 16x16px, row 24pt ✅ HIG 2022 archive | yes | Confirmed for G1. The current HIG sidebars.json (Last-Modified 2026-08-20) has no metrics, only that row height, text and glyph size depend on the small/medium/large size, so 26 is unknown. Keep ✅ for 14 and add ❓ for Tahoe. |
| 253 | `Sidebar (medium)` | 20x20px, row 28pt ✅ | yes | Same as line 252. The 'pre-Big Sur 18pt' remark is unsourced. |
| 254 | `Sidebar (large)` | 24x24px, row 32pt ✅ | yes | Same as line 252. |
| 255 | `Menu item icon` | ~13pt ❓ | yes | No source. On 27, NSMenu hides menu item symbol images by default for apps linked on 26.0 or later (macOS 27 release notes, AppKit). |
| 256 | `Menu bar extra` | 16pt ❓ (Bjango) | yes | Apple gives no icon size. HIG the-menu-bar.json (Last-Modified 2026-08-06) says 'The menu bar's height is 24 pt'. The older NSStatusBar.thickness doc says the system status bar is 22 pixels thick, matching the menu bar. Bjango is secondary. |
| 1113 | `border.corner_radius_lg` | 10px (measured) | yes | Mirrors line 173 (window radius). The cell has no Tahoe caveat, though :173 and §2.2 do. |
| 1114 | `border.line_width` | 0.5px (measured) | yes | Mirrors line 175. |
| 1115 | `disabled_opacity` | ≈0.25–0.3 (measured, via text colour alpha) | yes | Mirrors line 180. |
| 1117 | `border.shadow_enabled` | yes | yes | Mirrors line 181. |
| 1133 | `small` | sidebar: 16–20pt | yes | Mirrors lines 252-254. The 16-20 range leaves out the large 24. todo.md:2295. |
| 1147 | `title_bar_background` | (measured) ≈ defaults.surface_color | yes | No primary source gives a colour for 14 or 26. The 26 glass titlebar is likewise unpublished. |
| 1151 | `title_bar_font.style` | Normal | yes | No doc states the slant. A probe reading fontDescriptor.symbolicTraits would settle it. |
| 1155 | `border.corner_radius` | 10px measured through Sequoia; Tahoe variable, no public API | yes | There is no primary source for 10 (14/15) or for the 26 radii. The appendix sources at :1672 are community or third-party ones. Step 1 records that apple.com/os/macos claims 'updated window shapes' for 27, so 27 may differ again. A screenshot probe on macos-14… |
| 1174 | `icon_text_gap` | 4 (measured) AppKit | yes | No primary source. NSButton imageHugsTitle spacing could be measured. |
| 1179 | `hover_background` | (measured) subtle CoreUI highlight | yes | No primary source states a push-button hover state on 14 or 26. Related item: todo.md:79 (AppKit checkbox hover). |
| 1195 | `min_height` | NSTextField intrinsic: 22 | yes | The appendix at :1675 marks this ✅ from search-field sizes, which is a different control (NSSearchField). No source gives an NSTextField height. |
| 1197 | `border.padding_vertical` | 3 (measured) (22−16)/2 | yes | No source for 14/15. The derivation is already on todo.md:325-343. |
| 1206 | `background_color` | (measured) white | yes | G1 is drawn by CoreUI and no source gives a colour. 'white' may hold only in light mode. |
| 1209 | `border.color` | (measured) gray outline | yes | No source for G1. |
| 1211 | `indicator_color` | white (#ffffff) | yes | White holds for a dark accent in an active window. A light accent gets black. |
| 1213 | `label_gap` | AppKit: 4 | yes | No primary source. |
| 1227 | `background_color` | (measured) ≈ defaults.background_color (vibrancy) | yes |  |
| 1228 | `separator_color` | (measured) = separatorColor | yes | separatorColor exists (10.14), but no source says menus use it. |
| 1232 | `font.style` | Normal | yes | Not documented. |
| 1233 | `font.color` | (measured) = labelColor | yes |  |
| 1234 | `row_height` | NSMenuItem: 22 | yes | No primary source for 14 or 26. |
| 1235 | `border.padding_horizontal` | NSMenuItem: 12 | yes |  |
| 1236 | `border.padding_vertical` | 3 (measured) (22−16)/2 | yes | Derivation is already on todo.md:325-343. |
| 1241 | `icon_text_gap` | 4 (measured) AppKit layout | yes |  |
| 1242 | `icon_size` | 16 (cross-project) | yes | No Apple size. The 27 change affects whether menu icons show at all. |
| 1243 | `hover_background` | selectedContentBackgroundColor | yes | No Apple source maps the menu highlight to selectedContentBackgroundColor. Apple points to a material instead. |
| 1251 | `background_color` | (preset) L #2c2c2e D #3a3a3c | yes |  |
| 1255 | `font.style` | Normal | yes | Not documented. |
| 1256 | `font.color` | (preset) #ffffff both variants | yes |  |
| 1257 | `border.padding_horizontal` | NSToolTipManager: 4 | yes | No public source. |
| 1258 | `border.padding_vertical` | NSToolTipManager: 4 | yes |  |
| 1259 | `max_width` | 300 (measured) macOS Sonoma | yes | The HIG (offering-help.json) limits only content, '60 to 75 characters'. It gives no width. |
| 1261 | `border.color` | (measured) subtle frame | yes |  |
| 1263 | `border.shadow_enabled` | yes (system popup shadow) | yes |  |
| 1277 | `track_color` | overlay=transparent, legacy=light gray (measured) | yes | WebKit takes scroller metrics from NSScrollerImp at run time and gives no colour. |
| 1278 | `thumb_color` | #80808080 (measured) Sonoma | yes | The preset gap is already on todo.md:371-380. |
| 1279 | `thumb_hover_color` | #60606080 (measured) Sonoma | yes | See todo.md:371-390. |
| 1280 | `groove_width` | legacy=16, overlay=7 | yes | A probe that calls scrollerWidth for .regular with both styles on 14 and 26 would settle it. |
| 1281 | `min_thumb_length` | 40 (measured) legacy | yes |  |
| 1282 | `thumb_width` | legacy=16, overlay=7 idle / 11 hover | yes | The appendix sources are Gecko and Chromium (cross-project), at unpinned links. |
| 1317 | `min_height` | NSTabView: 24 | yes |  |
| 1318 | `border.padding_horizontal` | NSTabView: 12 | yes |  |
| 1319 | `border.padding_vertical` | 4 (measured) (24−16)/2 | yes | Derivation is already on todo.md:325-343. |
| 1323 | `border.shadow_enabled` | no | yes |  |
| 1342 | `hover_background` | selectedContentBackgroundColor (reduced opacity) | yes | No source states a sidebar hover fill. |
| 1351 | `bar_height` | NSToolbar: 38 | yes | No primary source. Toolbars changed in 26 (Liquid Glass) and 27 ('Uniform toolbars', per Step 1), so this needs a probe for each version. |
| 1352 | `item_gap` | AppKit: 8 | yes |  |
| 1353 | `border.padding_horizontal` | 8 (measured) NSToolbar | yes |  |
| 1354 | `border.padding_vertical` | 0 | yes | No source or tag. |
| 1387 | `header_background` | (measured) ≈ defaults.surface_color | yes |  |
| 1389 | `row_height` | NSTableView row: 24 | yes | The appendix at :1673 says the AppKit 11 notes 'confirm' 24pt. They do not state 24. The only 24 source left is lapcatsoftware (secondary). |
| 1390 | `border.padding_horizontal` | NSTableView: 4 | yes | NSTableView.Style.inset/fullWidth docs mention 'standard horizontal ... insets' but give no number. |
| 1391 | `border.padding_vertical` | 4 (measured) (24−16)/2 | yes | Derivation is already on todo.md:325-343. |
| 1392 | `hover_background` | selectedContentBackgroundColor (reduced opacity) | yes | No source states a hover fill for NSTableView rows. |
| 1411 | `border.shadow_enabled` | yes (system popup shadow) | yes | Neither NSPopover docs nor HIG popovers mention a shadow. Probe: popover window hasShadow / screenshot on 14 and 26. |
| 1419 | `divider_width` | NSSplitView divider: 6 | yes | 6 rests on GNUstep (appendix :1677), not an Apple source. Probe: NSSplitView().dividerThickness for each dividerStyle. |
| 1455 | `window_margin` | 20 (HIG) | yes | No HIG source: HIG layout (2024-05-14 snapshot and current) gives no macOS margin. The only Apple source is the 2016 Auto Layout guide, written around iOS recipes. Probe: standard superview spacing of a visual-format constraint '\|-[v]-\|' on 14 and 26. |
| 1456 | `section_gap` | 20 (HIG) |  | No Apple page opened states a section gap: HIG layout 2024 and 2026, the Auto Layout guide and NSStackView found nothing. Mark ❓ or (none). |
| 1468 | `track_height` | 22px | yes | Sonoma confirmed (WebKit). Tahoe AppKit height unknown: needs the probe. |
| 1470 | `track_radius` | 11 (= track_height/2, pill) | yes | Screenshot probe. |
| 1472 | `unchecked_background` | (measured) track bg | yes | The cell states no value. The G2 value is WebKit's rendition. |
| 1486 | `border.shadow_enabled` | yes (sheet overlay) | yes | HIG alerts, sheets and modality and the NSAlert docs say nothing about a shadow. |
| 1491 | `border.padding_horizontal` | ~20px (measured) | yes | Not documented. NSAlert.layout() docs: 'The standard alert layout is subject to change in future system software versions' (nsalert/layout().json, read 2026-10-11). |
| 1492 | `border.padding_vertical` | ~20px (measured) | yes | As :1491. |
| 1493 | `button_gap` | ~12px (measured) | yes | Not documented. |
| 1496 | `title_font.size` | ✅ 13pt (HIG) emphasized system font | yes | No HIG page opened says 'emphasized system font' for alerts: not the 2024-01 alerts snapshot, not the current alerts or typography pages. Cocotron is a reimplementation, not Apple. The ✅ is unsupported. Probe: font of the NSAlert messageText field. |
| 1497 | `title_font.weight` | ✅ Bold (700) (HIG) | yes | As :1496. |
| 1498 | `title_font.style` | Normal | yes | No source; the probe from :1496 settles it too. |
| 1503 | `icon_size` | 64px (app icon) | yes | NSAlert.icon docs give no size and say 'AppKit may omit the icon from the alert if it's the app icon and the alert's context is clear, such as being presented as a sheet' (nsalert/icon.json, Last-Modified 2026-08-14). |
| 1533 | `diameter` | 32px regular, 16px small | yes | The appendix (:1679) cites the sizeToFit docs, but the page (nsprogressindicator/sizetofit().json, Last-Modified 2026-08-14) gives no sizes, so the ✅ there is unsupported. ControlSize .large (11.0) and .extraLarge (26.0) exist too. Probe: sizeToFit per control… |
| 1534 | `min_diameter` | 10px (mini) | yes | As :1533. |
| 1535 | `stroke_width` | (none) — fin-based | yes | The 'fin-based' look is not documented; HIG progress-indicators says only 'circular appearance'. Whether Tahoe still draws fins needs a screenshot. The preset conflict is already todo.md:345. |
| 1536 | `fill_color` | system gray | yes | No source. |
| 1550 | `min_height` | NSPopUpButton: 21 | yes | Sonoma 21 per WebKit, but Gecko's dropdown regular height is 22 (nsNativeThemeCocoa.mm#L1070-L1086 at dc44717d). Gecko skips cell-margin inflation on 26: 'Controls on macOS 26 fill the entire frame' (#L177-L189). Probe: NSPopUpButton intrinsicContentSize on 14… |
| 1564 | `background_color` | NSSegmentedControl bg | yes | States no value; no source. 27 adds an NSSegmentedControl 'tabs' role with 'a distinct visual appearance' and fixes NSSegmentedCell drawing under Liquid Glass (macos-27-release-notes.json AppKit, Last-Modified 2026-10-03). |
| 1566 | `font.color` | controlTextColor | yes | No segmented-control source. WebKit and Gecko draw no segmented control. |
| 1567 | `active_background` | selectedContentBackgroundColor | yes | HIG (2024 and current) defines selectedContentBackgroundColor as 'The background for selected content in a key window or view' and selectedControlColor as 'The surface of a selected control'. Nothing ties either one to NSSegmentedControl. |
| 1568 | `active_text_color` | alternateSelectedControlTextColor | yes | HIG and the NSColor docs scope this colour to 'text on a selected surface in a list or table'. For controls the counterpart is selectedControlTextColor (nscolor/alternateselectedcontroltextcolor.json). |
| 1569 | `segment_height` | NSSegmentedControl: 24 | yes | No source. |
| 1570 | `separator_width` | 1px | yes | No source. The divider itself is already todo.md:139. |
| 1571 | `border.padding_horizontal` | ~8–10px (measured) | yes | No source. |
| 1573 | `border.color` | NSSegmentedControl border | yes | States no value. |
| 1576 | `border.padding_vertical` | ~3px (measured) | yes | No source. |
| 1606 | `arrow_icon_size` | ~13px (measured) | yes | The appendix at :1683 cites Gecko kDisclosureButtonSize 21x21, but that cell uses NSBezelStyleRoundedDisclosure, the push-disclosure button, not the triangle (nsNativeThemeCocoa.mm#L253-L254, #L827 at dc44717d). There is no source for the triangle. |
| 1624 | `underline_enabled` | yes | yes | The NSTextView.linkTextAttributes docs do not state the default attributes (nstextview/linktextattributes.json, Last-Modified 2026-08-14). Probe: read the default linkTextAttributes. |

</details>

### Same rows

The value at each generation with its source, the input of Step 8 (Tahoe facts: a value unchanged since Sonoma is recorded as unchanged, with this source).

<details><summary>155 same rows</summary>

| Line | Property | Values | Source |
|---|---|---|---|
| 59 | `+systemFontOfSize: default` | G1 (14; 15 same) 14/15: SF Pro is the system font; Body 13pt Regular. 15: same text (Nov 2024 copy identical); G2 (26; 27 same) 26/27: SF Pro; 'Default size macOS 13 pt' | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS, §macOS built-in text styles; https://web.archive.org/web/20241106035514/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typogra… https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §Ensuring legibility table + §macOS (Last-Modified 2026-08-06, read 2026-10-11) |
| 60 | `+boldSystemFontOfSize: default` | G1 (14; 15 same) 14/15: boldface system font; HIG body 13pt; G2 (26; 27 same) 26/27: 'standard system font in boldface'; default size 13pt (HIG) | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS built-in text styles https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/boldsystemfont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11); HIG typography.json §Ensuring legibility (2026-08-06) |
| 62 | `+titleBarFontOfSize:` | G1 (14) 14: listed as 'Title' dynamic font variant; G2 (26/27) 26/27: 'This method is equivalent to boldSystemFont(ofSize:)' → Bold | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS dynamic font variants https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/titlebarfont(ofsize:).json (2026-08-14, read 2026-10-11) |
| 73 | `+systemFontSize` | G1 (14; 15 same) 14/15: 13 (Body text style 13pt; HIG 'minimum 10 pt in macOS'); G2 (26/27) 26/27: 13 ('macOS \| 13 pt \| 10 pt' default/minimum) | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json; https://web.archive.org/web/20241106035514/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §Ensuring legibility (2026-08-06, read 2026-10-11); also https://web.archive.org/web/20250503090058/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json |
| 75 | `+labelFontSize` | G1 (14) 14: 10 ('The label font (Lucida Grande Regular 10 point)'); G2 (26/27) 26/27: 10 (same page, current) | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/labelfontsize.json Discussion (2026-08-14, read 2026-10-11) same page |
| 77 | `NSFont.TextStyle macOS 11+, no Dynamic Type` | G1 (14) 14: 'macOS doesn't support Dynamic Type'; G2 (26/27) 26/27: same sentence; TextStyle introducedAt 11.0 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS (2026-08-06); https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/textstyle.json (2026-08-14) |
| 81 | `.largeTitle` | G1 (14; 15 same) 14/15: Regular 26/32, emphasized Bold; G2 (26/27) 26/27: Regular 26/32, emphasized Bold | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS built-in text styles (identical in 20241106035514 copy) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS built-in text styles (2026-08-06, read 2026-10-11) |
| 82 | `.title1` | G1 (14; 15 same) 14/15: Regular 22/26; G2 (26/27) 26/27: Regular 22/26 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS built-in text styles https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 83 | `.title2` | G1 (14; 15 same) 14/15: Regular 17/22; G2 (26/27) 26/27: Regular 17/22 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 84 | `.title3` | G1 (14; 15 same) 14/15: Regular 15/20; G2 (26/27) 26/27: Regular 15/20 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 85 | `.headline` | G1 (14; 15 same) 14/15: Bold 13/16, emphasized Heavy; G2 (26/27) 26/27: Bold 13/16, emphasized Heavy | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 86 | `.subheadline` | G1 (14; 15 same) 14/15: Regular 11/14; G2 (26/27) 26/27: Regular 11/14 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 87 | `.body` | G1 (14; 15 same) 14/15: Regular 13/16, emphasized Semibold; G2 (26/27) 26/27: Regular 13/16 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 88 | `.callout` | G1 (14; 15 same) 14/15: Regular 12/15; G2 (26/27) 26/27: Regular 12/15 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 89 | `.footnote` | G1 (14; 15 same) 14/15: Regular 10/13; G2 (26/27) 26/27: Regular 10/13 | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 90 | `.caption1` | G1 (14; 15 same) 14/15: Regular 10/13, emphasized Medium; G2 (26/27) 26/27: Regular 10/13, emphasized Medium | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 91 | `.caption2` | G1 (14; 15 same) 14/15: Medium 10/13, emphasized Semibold; G2 (26/27) 26/27: Medium 10/13, emphasized Semibold | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (2026-08-06) |
| 93 | `weight from NSFontDescriptor traits` | G1 (14) all: traits attribute: 'A dictionary that fully describes the font traits'; G2 (26/27) 26/27: same, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsfontdescriptor/attributename/traits.json (read 2026-10-11) https://developer.apple.com/tutorials/data/documentation/appkit/nsfontdescriptor/traitkey.json (read 2026-10-11) |
| 102 | `controlAccentColor` | G1 (14) 14: exists (introducedAt 10.14); 'accent color people select in System Settings'; G2 (26/27) 26/27: same, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controlaccentcolor.json; https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS (2026-08-06, read 2026-10-11) |
| 103 | `windowBackgroundColor` | G1 (14) 14: 'The background of a window'; G2 (26/27) 26/27: same; not deprecated | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/windowbackgroundcolor.json (read 2026-10-11); HIG color.json §macOS (2026-08-06) |
| 104 | `labelColor` | G1 (14) 14: introducedAt 10.10; 'text of a label containing primary content'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json; HIG color.json §macOS (2026-08-06) |
| 105 | `secondaryLabelColor` | G1 (14) 14: introducedAt 10.10; secondary label text; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/secondarylabelcolor.json; HIG color.json §macOS (2026-08-06) |
| 106 | `tertiaryLabelColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/tertiarylabelcolor.json |
| 107 | `quaternaryLabelColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/quaternarylabelcolor.json |
| 109 | `controlBackgroundColor` | G1 (14) 14: 'background of a large interface element, such as a browser or table'; G2 (26/27) 26/27: same; doc adds 'Do not use this color for drawing. Instead, use an NSVisualEffectView' | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controlbackgroundcolor.json |
| 110 | `controlTextColor` | G1 (14) 14: 'text of a control that is available'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controltextcolor.json |
| 111 | `disabledControlTextColor` | G1 (14) 14: 'text of a control that's unavailable'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/disabledcontroltextcolor.json |
| 112 | `selectedContentBackgroundColor` | G1 (14) 14: introducedAt 10.14; 'selected content in a key window or view'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/selectedcontentbackgroundcolor.json |
| 113 | `unemphasizedSelectedContentBackgroundColor` | G1 (14) 14: introducedAt 10.14; non-key window or view; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/unemphasizedselectedcontentbackgroundcolor.json |
| 114 | `selectedTextColor` | G1 (14) 14: 'The color for selected text'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/selectedtextcolor.json |
| 115 | `alternateSelectedControlTextColor` | G1 (14) 14: 'text on a selected surface in a list or table'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/alternateselectedcontroltextcolor.json |
| 116 | `separatorColor` | G1 (14) 14: introducedAt 10.14; separator between sections of content; G2 (26/27) 26/27: same; doc: 'Do not use this color for split view dividers or window chrome dividers' | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/separatorcolor.json |
| 117 | `gridColor` | G1 (14) 14: gridlines of a table; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/gridcolor.json |
| 118 | `linkColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/linkcolor.json |
| 119 | `placeholderTextColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/placeholdertextcolor.json |
| 120 | `keyboardFocusIndicatorColor` | G1 (14) 14: focus ring around the focused control; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/keyboardfocusindicatorcolor.json |
| 122 | `windowFrameTextColor` | G1 (14) 14: 'text in the window's title bar area'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/windowframetextcolor.json |
| 123 | `textBackgroundColor` | G1 (14) 14: 'background color behind text'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textbackgroundcolor.json |
| 124 | `textColor` | G1 (14) 14: 'The text in a document'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textcolor.json |
| 125 | `headerTextColor` | G1 (14) 14: text of a header cell in a table; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/headertextcolor.json |
| 126 | `shadowColor` | G1 (14) 14: 'virtual shadow cast by a raised object onscreen'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/shadowcolor.json |
| 128 | `findHighlightColor` | G1 (14) 14: introducedAt 10.13; colour of a find indicator; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/findhighlightcolor.json |
| 129 | `systemRedColor` | G1 (14) 14: exists (10.10); HIG macOS default L #ff3b30 D #ff453a; G2 (26/27) 26/27: exists; HIG unified red L #ff383c D #ff4245 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json image alts macos-default-systemredcolor(.dark) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §System colors image alts colors-unified-red-light/dark (2026-08-06); https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemred.json |
| 130 | `systemOrangeColor` | G1 (14) 14: exists; HIG L #ff9500 D #ff9f0a; G2 (26/27) 26/27: exists; HIG L #ff8d28 D #ff9230 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (macos-default-systemorangecolor alts) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (colors-unified-orange alts); nscolor/systemorange.json |
| 131 | `systemGreenColor` | G1 (14) 14: exists; HIG L #28cd41 D #32d74b; G2 (26/27) 26/27: exists; HIG L #34c759 D #30d158 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (macos-default-systemgreencolor alts) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (colors-unified-green alts); nscolor/systemgreen.json |
| 132 | `systemBlueColor` | G1 (14) 14: exists; HIG L #007aff D #0a84ff; G2 (26/27) 26/27: exists; HIG L #0088ff D #0091ff | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (macos-default-systembluecolor alts) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (colors-unified-blue alts); nscolor/systemblue.json |
| 133 | `systemYellowColor` | G1 (14) 14: exists; HIG L #ffcc00 D #ffd60a; G2 (26/27) 26/27: exists; HIG L #ffcc00 D #ffd600 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (macos-default-systemyellowcolor alts) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (colors-unified-yellow alts); nscolor/systemyellow.json |
| 134 | `systemPurpleColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systempurple.json (read 2026-10-11) same; https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 135 | `systemPinkColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systempink.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 136 | `systemTealColor` | G1 (14) 14: introducedAt 10.12; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemteal.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 137 | `systemIndigoColor` | G1 (14) 14: introducedAt 10.15; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemindigo.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 138 | `systemCyanColor` | G1 (14) 14: introducedAt 12.0; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemcyan.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 139 | `systemMintColor` | G1 (14) 14: introducedAt 10.12; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemmint.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 140 | `systemBrownColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systembrown.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 141 | `systemGrayColor` | G1 (14) 14: introducedAt 10.10; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemgray.json https://developer.apple.com/tutorials/data/documentation/appkit/standard-colors.json |
| 142 | `alternatingContentBackgroundColors` | G1 (14) 14: introducedAt 10.14; alternating rows/columns; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/alternatingcontentbackgroundcolors.json |
| 145 | `performAsCurrentDrawingAppearance` | G1 (14) 14: introducedAt 11.0; G2 (26/27) 26/27: not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsappearance/performascurrentdrawingappearance(_:).json (read 2026-10-11) same |
| 146 | `colorUsingColorSpace:` | G1 (14) 14: converts to equivalent colour in a space; nil if impossible; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/usingcolorspace(_:).json same |
| 148 | `system colours date to 10.10+` | G1 (14) 14: labelColor/linkColor/systemRed…Gray introducedAt 10.10; G2 (26/27) 26/27: unchanged | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json; .../systemred.json same |
| 149 | `systemTealColor availability` | G1 (14) 14: introducedAt 10.12; G2 (26/27) 26/27: 10.12 | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemteal.json same |
| 150 | `systemMintColor availability` | G1 (14) 14: introducedAt 10.12; G2 (26/27) 26/27: 10.12 | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemmint.json same |
| 151 | `systemIndigoColor availability` | G1 (14) 14: introducedAt 10.15; G2 (26/27) 26/27: 10.15 | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemindigo.json same |
| 152 | `systemCyanColor availability` | G1 (14) 14: introducedAt 12.0; G2 (26/27) 26/27: 12.0 | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/systemcyan.json same |
| 159 | `NSColor.textInsertionPointColor` | G1 (14) 14: introducedAt 14.0, roleHeading 'Type Property'; G2 (26/27) 26/27: same, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textinsertionpointcolor.json; appkit-release-notes-for-macos-14.json §NSColor https://developer.apple.com/tutorials/data/documentation/appkit/ui-element-colors.json (2026-08-14) |
| 1025 | `family` | G1 (14) 14: API exists; HIG: SF Pro is the system font; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124163151/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/systemfont(ofsize:).json (2026-08-14); HIG typography.json §macOS (2026-08-06) |
| 1026 | `size` | G1 (14) 14: API mapping; 13 per line 73; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/systemfontsize.json same |
| 1027 | `weight` | G1 (14) 14: traits dictionary; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfontdescriptor/attributename/traits.json same |
| 1028 | `style` | G1 (14) 14: traits dictionary (symbolic traits); G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfontdescriptor/traitkey.json same |
| 1029 | `color` | G1 (14) 14: labelColor (10.10); G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json |
| 1030 | `line_height` | G1 (14) SF Pro 19.0d8e1 (pkg…: UPM 2048, sTypo 1950/-494/0 (hhea and win equal) → 1.193; G2 (26/27) SF Pro 22.0d5e4 (dmg…: same 2048 / 1950 / -494 / 0 → 1.193 | https://web.archive.org/web/20240111181629/https://devimages-cdn.apple.com/design/resources/download/SF-Pro.dmg (sha256 9e4b8781…5e7d), SF-Pro.ttf OS/2+hhea read with fontTools https://devimages-cdn.apple.com/design/resources/download/SF-Pro.dmg (sha256 968ab3b8…2918, read 2026-10-11), SF-Pro.ttf |
| 1036 | `family` | G1 (14) 14: API introducedAt 10.15; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/monospacedsystemfont(ofsize:weight:).json same (2026-08-14) |
| 1037 | `size` | G1 (14) 14: size is the caller's argument; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/monospacedsystemfont(ofsize:weight:).json same |
| 1038 | `weight` | G1 (14) 14: traits; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfontdescriptor/attributename/traits.json same |
| 1039 | `style` | G1 (14) 14: traits; G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nsfontdescriptor/traitkey.json same |
| 1046 | `background_color` | G1 (14) 14: windowBackgroundColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/windowbackgroundcolor.json |
| 1047 | `text_color` | G1 (14) 14: labelColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json |
| 1048 | `accent_color` | G1 (14) 14: controlAccentColor (10.14); G2 (26/27) 26/27: same | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controlaccentcolor.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS (2026-08-06) |
| 1049 | `accent_text_color` | G1 (14) 14: text on a selected surface in a list/table; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/alternateselectedcontroltextcolor.json |
| 1050 | `surface_color` | G1 (14) 14: background of large elements (browser/table); G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controlbackgroundcolor.json |
| 1051 | `border.color` | G1 (14) 14: separatorColor (10.14); G2 (26/27) 26/27: same; Apple: content separators only | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/separatorcolor.json |
| 1052 | `muted_color` | G1 (14) 14: secondaryLabelColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/secondarylabelcolor.json |
| 1053 | `shadow_color` | G1 (14) 14: shadowColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/shadowcolor.json |
| 1054 | `link_color` | G1 (14) 14: linkColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/linkcolor.json |
| 1055 | `selection_background` | G1 (14) 14: selectedContentBackgroundColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/selectedcontentbackgroundcolor.json |
| 1056 | `selection_text_color` | G1 (14) 14: alternateSelectedControlTextColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/alternateselectedcontroltextcolor.json |
| 1057 | `selection_inactive_background` | G1 (14) 14: same API; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/unemphasizedselectedcontentbackgroundcolor.json |
| 1058 | `text_selection_background` | G1 (14) 14: 'background of selected text'; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/selectedtextbackgroundcolor.json |
| 1059 | `text_selection_color` | G1 (14) 14: selectedTextColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/selectedtextcolor.json |
| 1060 | `disabled_text_color` | G1 (14) 14: disabledControlTextColor; G2 (26/27) 26/27: same | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json §macOS https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/disabledcontroltextcolor.json |
| 1080 | `danger_color` | G1 (14) 14: systemRed; HIG L #ff3b30 D #ff453a; G2 (26/27) 26/27: systemRed; HIG L #ff383c D #ff4245 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (2026-08-06) |
| 1081 | `danger_text_color` | G1 (14) 14: labelColor; G2 (26/27) 26/27: labelColor | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json same |
| 1082 | `warning_color` | G1 (14) 14: systemOrange; HIG L #ff9500 D #ff9f0a; G2 (26/27) 26/27: systemOrange; HIG L #ff8d28 D #ff9230 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (2026-08-06) |
| 1083 | `warning_text_color` | G1 (14) 14: labelColor; G2 (26/27) 26/27: labelColor | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json same |
| 1084 | `success_color` | G1 (14) 14: systemGreen; HIG L #28cd41 D #32d74b; G2 (26/27) 26/27: systemGreen; HIG L #34c759 D #30d158 | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (2026-08-06) |
| 1085 | `success_text_color` | G1 (14) 14: labelColor; G2 (26/27) 26/27: labelColor | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json same |
| 1086 | `info_color` | G1 (14) 14: systemBlue; HIG L #007aff D #0a84ff; G2 (26/27) 26/27: systemBlue; HIG L #0088ff D #0091ff | https://web.archive.org/web/20240124114733/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (2026-08-06) |
| 1087 | `info_text_color` | G1 (14) 14: labelColor; G2 (26/27) 26/27: labelColor | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/labelcolor.json same |
| 1104 | `focus_ring_color` | G1 (14) 14: keyboardFocusIndicatorColor (AppKit cell path, opaque); G2 (26/27) 26/27: keyboardFocusIndicatorColor via platformFocusRingColor, used by the vector-control focus ring | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/ControlMac.mm#L331-L334 https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L387-L396 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L140-L164 |
| 1105 | `focus_ring_width` | G1 (14) 14: 3 ('on macOS this matches AppKit'); cell path pads by focusRingOutlineWidth 3; G2 (26/27) 26/27: 3 (same unconditional constant); the vector path passes 0 and lets AppKit's NSInitializeCGFocusRingStyleForTime pick the… | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/RenderTheme.h#L207-L208 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/mac/controls/ControlMac.mm#L366-L367 https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/cocoa/GraphicsContextCocoa.mm#L103-L110 ; RenderThemeCocoa.mm#L161-L163 at the same commit |
| 178 | `Focus ring width` | G1+G2 14, 26 (no version c…: 3. WebKit: 'on macOS this matches AppKit'. On macOS the ring style comes from AppKit's NSInitializeCGFocusRingStyleForTi… | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/RenderTheme.h#L207-L208 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/platform/graphics/cocoa/GraphicsContextCocoa.mm#L103-L117 ; https://github.c… |
| 234 | `Between related controls` | unversioned current doc: 8: IB standard spacing between sibling views; NSStackView default spacing | https://developer.apple.com/library/archive/documentation/UserExperience/Conceptual/AutolayoutPG/WorkingwithSimpleConstraints.html (Updated 2016-03-21, read 2026-10-11) ; https://developer.apple.com/tutorials/data/documentation/appkit/nsstackview/spacing.json (Last-Modified 2026-08-14) |
| 237 | `Content margin (window edge)` | unversioned archive doc: 20.0 points between a view and the edge of its superview (IB standard) | https://developer.apple.com/library/archive/documentation/UserExperience/Conceptual/AutolayoutPG/WorkingwithSimpleConstraints.html (Updated 2016-03-21, read 2026-10-11) |
| 240 | `IB standard spacing` | unversioned archive doc: 8.0 points between sibling views | https://developer.apple.com/library/archive/documentation/UserExperience/Conceptual/AutolayoutPG/WorkingwithSimpleConstraints.html (Updated 2016-03-21, read 2026-10-11) |
| 241 | `NSStackView default spacing` | unversioned (current) doc read 2026-10-11: 'The default value for the spacing property is 8.0 points' (introducedAt 10.9) | https://developer.apple.com/tutorials/data/documentation/appkit/nsstackview/spacing.json (Last-Modified 2026-08-14) |
| 250 | `Toolbar (regular mode)` | G1+G2 14, 26 (doc text unv…: 'regular-sized controls and 32 by 32 pixel icons'; deprecatedAt macOS 27.0 | https://developer.apple.com/tutorials/data/documentation/appkit/nstoolbar/sizemode-swift.enum/regular.json (Last-Modified 2026-09-14, read 2026-10-11) |
| 251 | `Toolbar (small mode)` | G1+G2 14, 26: 'small-sized controls and 24 by 24 pixel icons'; deprecatedAt macOS 27.0 | https://developer.apple.com/tutorials/data/documentation/appkit/nstoolbar/sizemode-swift.enum/small.json (Last-Modified 2026-09-14, read 2026-10-11) |
| 266 | `Text styles` | G1+G2 current docs: preferredFont(forTextStyle:options:) introducedAt 11.0; 'macOS doesn't support Dynamic Type' | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/preferredfont(fortextstyle:options:).json (Last-Modified 2026-09-11) ; https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Last-Modified 2026-08-06) |
| 267 | `Reduce motion` | all current doc: introducedAt 10.12, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducemotion.json (Last-Modified 2026-08-14) |
| 268 | `Reduce transparency` | all current doc: introducedAt 10.10, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducetransparency.json (Last-Modified 2026-08-14) |
| 269 | `Increase contrast` | all current doc: introducedAt 10.10, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshouldincreasecontrast.json (Last-Modified 2026-08-14) |
| 270 | `Differentiate without color` | all current doc: introducedAt 10.10, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshoulddifferentiatewithoutcolor.json (Last-Modified 2026-08-14) |
| 1123 | `text_scaling_factor` | G1 14.0: Accessibility > Display > Text size slider; the 13.0 page has no such slider; G2 26: same slider: 'desktop, in app sidebars, and in the apps listed' | https://support.apple.com/guide/mac-help/text-items-screen-bigger-mchld786f2cd/14.0/mac/14.0 and .../13.0/mac/13.0 (read 2026-10-11) https://support.apple.com/guide/mac-help/increase-font-size-and-icons-mchld786f2cd/26/mac/26 (read 2026-10-11) |
| 1124 | `reduce_motion` | all current doc: introducedAt 10.12 | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducemotion.json (Last-Modified 2026-08-14) |
| 1125 | `high_contrast` | all current doc: introducedAt 10.10 | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshouldincreasecontrast.json (Last-Modified 2026-08-14) |
| 1126 | `reduce_transparency` | all current doc: introducedAt 10.10 | https://developer.apple.com/tutorials/data/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducetransparency.json (Last-Modified 2026-08-14) |
| 1132 | `toolbar` | G1+G2 14, 26: 32x32 regular, 24x24 small; deprecated at macOS 27.0 ('will be ignored in the future') | https://developer.apple.com/tutorials/data/documentation/appkit/nstoolbar/sizemode-swift.enum/regular.json and .../small.json (Last-Modified 2026-09-14) |
| 1148 | `title_bar_font.family` | G1+G2 14,15,26,27: titleBarFont(ofSize:) exists, is not deprecated, and is documented as 'equivalent to boldSystemFont(ofSize:)' | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/titlebarfont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1149 | `title_bar_font.size` | G1+G2 14-27: fontSize <= 0 'returns the title bar font at the default size'. The default size is not stated. | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/titlebarfont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1150 | `title_bar_font.weight` | G1+G2 14-27: 'This method is equivalent to boldSystemFont(ofSize:)' | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/titlebarfont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1152 | `title_bar_font.color` | G1+G2 14-27: windowFrameTextColor: 'The color to use for text in a window's frame', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/windowframetextcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1164 | `background_color` | G1+G2 14-27: controlColor: 'The color to use for the flat surfaces of a control', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controlcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1166 | `font.color` | G1+G2 14-27: controlTextColor: 'text on enabled controls', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/controltextcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1186 | `background_color` | G1+G2 14-27: textBackgroundColor: 'background area behind text', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textbackgroundcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1188 | `font.color` | G1+G2 14-27: textColor: 'The color to use for text', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1191 | `placeholder_color` | G1+G2 14-27: placeholderTextColor, introducedAt 10.10, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/placeholdertextcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1192 | `caret_color` | G1+G2 14-27: textInsertionPointColor introducedAt 14.0, not deprecated. NSTextView.insertionPointColor is present and not deprecated. | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textinsertionpointcolor.json ; https://developer.apple.com/tutorials/data/documentation/appkit/nstextview/insertionpointcolor.json (both Last-Modified 2026-08-14, read 2026-10-11) |
| 1220 | `radio_dot_diameter` | G2 HIG current (26/27): HIG Toggles: 'selected (a filled circle)'; no metric | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/toggles.json (Last-Modified 2026-08-20, read 2026-10-11) |
| 1221 | `check_mark_stroke_width` | G2 HIG current: quote present, no metric | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/toggles.json (Last-Modified 2026-08-20, read 2026-10-11) |
| 1229 | `font.family` | G1+G2 14-27: menuFont(ofSize:) 'font used for menu items', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/menufont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1230 | `font.size` | G1+G2 14-27: fontSize <= 0 returns 'the menu items font with the default size' | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/menufont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1231 | `font.weight` | G1+G2 14-27: API as above; weight is read at run time | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/menufont(ofsize:).json (read 2026-10-11) |
| 1244 | `hover_text_color` | G1+G2 14-27: selectedMenuItemTextColor present, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/selectedmenuitemtextcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1245 | `disabled_text_color` | G1+G2 14-27: disabledControlTextColor 'text on disabled controls', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/disabledcontroltextcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1252 | `font.family` | G1+G2 14-27: toolTipsFont(ofSize:) present, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/tooltipsfont(ofsize:).json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1253 | `font.size` | G1+G2 14-27: fontSize <= 0 returns the tool tips font 'at the default size' (size unstated) | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/tooltipsfont(ofsize:).json (read 2026-10-11) |
| 1254 | `font.weight` | G1+G2 14-27: API as above | https://developer.apple.com/tutorials/data/documentation/appkit/nsfont/tooltipsfont(ofsize:).json (read 2026-10-11) |
| 1283 | `overlay_mode` | G1+G2 14-27: preferredScrollerStyle introducedAt 10.7, not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nsscroller/preferredscrollerstyle.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1326 | `item_gap` | HIG current 26/27: quote 'The tabbed control appears on the top edge of the content area' present; no spacing (last change log entry June 5… | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/tab-views.json (Last-Modified 2026-08-06, read 2026-10-11) |
| 1356 | `icon_size` | G1+G2 14,15,26: .regular: 'regular-sized controls and 32 by 32 pixel icons'; .small: '24 by 24 pixel icons'; .default = regular; G2 refinement 27: deprecatedAt macOS 27.0: 'no longer recommended and will be ignored in the future'; summary 'The toolbar no longer uses… | https://developer.apple.com/tutorials/data/documentation/appkit/nstoolbar/sizemode-swift.enum/regular.json , .../small.json , .../default.json (Last-Modified 2026-09-14, read 2026-10-11) https://developer.apple.com/tutorials/data/documentation/appkit/nstoolbar/sizemode-swift.enum.json (Last-Modified 2026-09-14, read 2026-10-11) |
| 1372 | `border.padding_horizontal` | current 14-27: NSStatusBar: 'status items displayed within the system-wide menu bar'; HIG Status bars: 'Not supported in macOS' | https://developer.apple.com/tutorials/data/documentation/appkit/nsstatusbar.json (Last-Modified 2026-08-14) ; https://developer.apple.com/tutorials/data/design/human-interface-guidelines/status-bars.json (Last-Modified 2026-08-06), read 2026-10-11 |
| 1382 | `alternate_row_background` | G1+G2 14-27: introducedAt 10.14, not deprecated; 'alternating content, typically found in table views' | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/alternatingcontentbackgroundcolors.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1386 | `header_font.color` | G1+G2 14-27: headerTextColor 'text in header cells in table views and outline views', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/headertextcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1388 | `grid_color` | G1+G2 14-27: gridColor 'optional gridlines, such as those in a table view', not deprecated | https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/gridcolor.json (Last-Modified 2026-08-14, read 2026-10-11) |
| 1426 | `line_color` | G1 14: separatorColor: 'A separator between different sections of content'; G2 26: separatorColor, same wording | HIG color snapshot 2024-05-15 http://web.archive.org/web/20240515183231/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (macOS dynamic colors table) https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (Last-Modified 2026-08-06, read 2026-10-11); nscolor/separatorcolor.json introducedAt 10.14 |
| 1435 | `caption` | G1 14: Caption 1 Regular 10; G2 26: Caption 1 Regular 10 (line height 13) | http://web.archive.org/web/20240320022751/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json ('macOS built-in text styles') https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Last-Modified 2026-08-06, read 2026-10-11) |
| 1436 | `section_heading` | G1 14: Headline Bold 13; G2 26: Headline Bold 13 (emphasized: Heavy) | http://web.archive.org/web/20240320022751/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Last-Modified 2026-08-06) |
| 1437 | `dialog_title` | G1 14: Title 1 Regular 22; G2 26: Title 1 Regular 22 | http://web.archive.org/web/20240320022751/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Last-Modified 2026-08-06) |
| 1438 | `display` | G1 14: Large Title Regular 26; G2 26: Large Title Regular 26 | http://web.archive.org/web/20240320022751/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Last-Modified 2026-08-06) |
| 1453 | `widget_gap` | G1+G2 (docs unversioned) 14/26: NSStackView.spacing default 8.0 pt | https://developer.apple.com/tutorials/data/documentation/appkit/nsstackview/spacing.json (Last-Modified 2026-08-14, introducedAt 10.9, read 2026-10-11) |
| 1494 | `button_order` | G1 14: 'Always place the default button on the trailing side of a row or at the top of a stack'; G2 26: same text; NSAlert.buttons: 'The button displayed rightmost ... (left-to-right language) ... is considered the default b… | http://web.archive.org/web/20240101235353/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/alerts.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/alerts.json (Last-Modified 2026-08-06; change log latest 2024-02-02); https://developer.apple.com/tutorials/data/documentation/appkit/nsalert/buttons.json (read 2026-10-11) |
| 1552 | `border.padding_horizontal` | G1 14: left 8 (regular/small/large), 10 (mini); right 26/23/22/26 holds the arrow; G2 26: same: platformPopupInternalPaddingBox has no refresh gate, and the G2 glyph is placed from it | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L925-L934 and #L1401-L1411 https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3485-L3490 |
| 1613 | `arrow_side` | G1 14: 'points inward from the leading edge when its content is hidden'; G2 26: same sentence | http://web.archive.org/web/20240906234423/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/disclosure-controls.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/disclosure-controls.json (Last-Modified 2026-08-06); nsbutton/bezelstyle-swift.enum/disclosure.json: triangle points right when closed |
| 1622 | `font.color` | G1 14: linkColor 'A link to other content'; G2 26: linkColor | http://web.archive.org/web/20240515183231/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (Last-Modified 2026-08-06); nscolor/linkcolor.json introducedAt 10.10 |
| 1639 | `border.padding_horizontal` | G1+G2 (docs unversioned) 14/26: lineFragmentPadding default 5.0; textContainerInset page states no default | https://developer.apple.com/tutorials/data/documentation/appkit/nstextcontainer/linefragmentpadding.json (introducedAt 10.0) and .../nstextview/textcontainerinset.json (both Last-Modified 2026-08-14, read 2026-10-11) |
| 1640 | `border.padding_vertical` | G1+G2 (docs unversioned) 14/26: no default stated | https://developer.apple.com/tutorials/data/documentation/appkit/nstextview/textcontainerinset.json (Last-Modified 2026-08-14) |

</details>

### Researcher notes

Each researcher's own notes, verbatim: coverage, method caveats and errors found outside the *differs* rows (wrong citations on ✅ rows, unpinned links). Step 7 corrects the citation errors (S4) as part of the row fixes.

<details><summary>fonts-colours</summary>

Scope done: every row in lines 51-164 and the macOS column of lines 1019-1107. Nothing is left uncovered.

Fonts and colours do not change across versions. Between 14, 15, 26 and 27 the API names, their availability, the HIG macOS text-style table (identical in the 2024-01, 2024-11, 2025-05 and current copies) and SF Pro's vertical metrics (19.0d8e1 in 2023 and 22.0d5e4 in 2026: UPM 2048, 1950/-494/0) are all the same. The 27 release notes have no NSFont or NSColor change.

New findings (not in todo.md):
1. Some ✅ rows have no primary source: menuFont (:63), controlContentFont (:67), smallSystemFontSize (:74), and the default family, size and weight of monospacedSystemFont (:61). Apple's API pages give no default size. The appendix's Monkeybread citation (third party) is the only source for 11pt. Suggest ❓, or a probe.
2. Two role descriptions are wrong. :121 underPageBackgroundColor: Apple says 'background behind a document's content', with no 'sidebar'. :127 highlightColor is a 'virtual light source', not a 'highlight overlay'.
3. Three insertion-point claims are unsourced: :158 'defaults to controlTextColor', the 'follows accent' part of :160, and the prose at :162. The AppKit 14 notes do not support them.
4. :108 controlColor: the doc calls it a pattern colour equal to windowBackgroundColor by default. The 'NSColorType.catalog' claim has no source.
5. HIG system colour RGB changed on 2025-06-09, between G1 and G2. G1 values (light/dark): red #ff3b30/#ff453a, orange #ff9500/#ff9f0a, green #28cd41/#32d74b, blue #007aff/#0a84ff, yellow #ffcc00/#ffd60a. G2 unified values: red #ff383c/#ff4245, orange #ff8d28/#ff9230, green #34c759/#30d158, blue #0088ff/#0091ff, yellow #ffcc00/#ffd600. The table cells name only the APIs, so this matters for the preset (M2c). macos-sonoma's success colour #34c759 (todo.md:1240) is neither Sonoma value: it is iOS's green, which is G2's light green.
6. :1105 focus ring width 3: WebKit's unconditional constant (RenderTheme.h:207-208, 'on macOS this matches AppKit') supports it, so the tag could be (WebKit). On 26, WebKit lets AppKit choose the thickness at run time. :1106 offset -1 has no source; WebKit notes 'macOS controls have never honored outline offset'.
7. selectedTextBackgroundColor (used at :1058) and quinaryLabelColor (macOS 11) are missing from §1.1.2's table.

Already in todo.md: the 'SF Pro' family name versus what the API reports (todo.md:2612) and the unemphasised selection text colour (todo.md:579).

Step 1 version map: no correction needed. The Wayback 429 noted in Step 1 has cleared, so archived HIG JSON (typography, color) and SF-Pro.dmg captures are reachable through the CDX API.

Other agents share the m2a-scripts directory under the scratchpad: one overwrote doc.py mid-run. This agent used its own m2a-fontcolor-scripts.

</details>

<details><summary>geometry</summary>

I checked all 60 rows in scope (lines 165-273 and 1108-1139); none is left out. Result: 17 same, 9 differs, 34 unverifiable.

Caveats:
- WebKit is the only primary source for Tahoe metrics. Its G2 values (the 'ForVectorBasedControls' path, gated on __MAC_OS_X_VERSION_MIN_REQUIRED >= 260000) are WebKit's own drawing of Liquid Glass, not AppKit's. Every 'differs' row rests on them, so a probe should confirm before any cell changes.
- WebKit's G1 tables are hardcoded copies of AppKit sizes with no version conditional, so they stand for 14 and 15. No source separates 15 from 14.
- At the pinned WebKit commit, no theme file has a macOS-27 conditional, so WebKit predicts 27 = 26. The current HIG has no macOS control metrics at all.

New findings (none of them on docs/todo.md):
1. Line 178 cites the wrong evidence for the focus ring. UIFocusRingStyle.borderThickness is the iOS (non-AppKit) branch of GraphicsContextCocoa.mm. The macOS evidence is RenderTheme.h:207-208 ('on macOS this matches AppKit', width 3). The appendix (:1686) has the same error.
2. Line 195 misquotes Gecko. At firefox dc44717d, forms.css:514 gives <button> padding-inline: 8px; the 4px it quotes belongs to <select> (:227).
3. Line 196 cites WebKit's search-field table (searchFieldSizes) as NSTextField evidence. The appendix (:1675) repeats it.
4. Lines 211 and 1673: the AppKit Release Notes for macOS 11 do not state 24pt; they say only 'increased row height' (group rows 28pt). The 24 rests on lapcat (secondary).
5. Lines 219-220 and appendix 1679: Apple's sizeToFit() doc states no 32/16/10.
6. Lines 214 and 238: the 'HIG: 8 pixels between toolbar controls' quote is in neither the current HIG nor the 2015 OS X HIG toolbar page. The same holds for the 8/6/5 label spacing at line 236.
7. NSToolbar.SizeMode is deprecatedAt macOS 27.0 ('will be ignored in the future'; the sizeMode property says 'The toolbar ignores this property'). It was not deprecated on 14 or 26, so the '(deprecated)' at :250-251 holds only from 27.
8. Line 209: Chromium's item_horizontal_border_padding = 12 is set in MenuConfig::InitCommon for every platform, so it is not macOS evidence.
9. Gecko's legacy (non-overlay) scrollbar track is 15, not 16 (ScrollbarDrawingCocoa.cpp:464). That is an engine constant; a probe settles the AppKit value.
10. G2 WebKit has macOS values the cells lack: switch 41.14x18 with a capsule thumb, slider track 8 with a 24x16 capsule thumb, progress bar 8 (radius 4), push button radius 6 with padding 6/0/6/1, text-field border 0.5 and radius 5, disabled controls faded by 0.4.

Already known, not re-reported: Step 1 findings 1 (unpinned WebKit links) and 2 (dead SwitchThumbMac.mm link); todo.md:995-1003 (radius per control size), :2272-2273 (scrollbar groove/thumb), :2295 and :2299 (icon_sizes small and toolbar).

Probe priorities, macos-14 against macos-26 (and xcode-27): NSButton/NSTextField/NSPopUpButton/NSSwitch/NSSlider intrinsicContentSize; scrollerWidth(.regular, .legacy); NSTableView sidebar row height and icon size per size class; NSSplitView dividerThickness; NSProgressIndicator sizeToFit; disabledControlTextColor alpha; window corner radius from screenshots.

</details>

<details><summary>widgets-1</summary>

Covered: every row with a macOS claim in lines 1140-1400. Nothing in scope was left out.

Overall:
- Almost every metric cell is either unsourced (measured, or an AppKit intrinsic) or rests on WebKit.
- WebKit's G1 tables (RenderThemeMac.mm, mac/controls/*.mm) have no 14/15 conditionals, so they give the same value for Sonoma and Sequoia. Whether 15 differs from 14 needs the probe.
- WebKit's macOS-26 path (RenderThemeCocoa '*ForVectorBasedControls', gated by FORM_CONTROL_REFRESH) uses different constants from the G1 path. That accounts for every 'differs' row:
  - button padding 6 / 0 top, 1 bottom; radius 6
  - checkbox 16, radius 5.5, border secondaryLabelColor 1.5
  - slider track 8, thumb 24×16, tick 2
  - progress bar 8, radius 4
  - text field padding 0.5em / 0.25em, border 0.5, radius 5
- These G2 values are WebKit's own vector rendition, not AppKit's. Only a probe can show whether native AppKit 26 matches them.
- Neither WebKit nor any Apple page has a 27-specific control metric. 27 differs from 26 only in:
  - NSToolbar.SizeMode deprecated at 27.0
  - menu symbol images hidden by default
  - the segmented control's .tabs role

New findings (not on todo.md):
1. :1170 button min_height 22 has no primary source. The appendix at :1687 cites 'developer discussions'. WebKit's regular push button is 20 on both its G1 and G2 paths.
2. Slider rows contradict WebKit G1:
   - :1294 tick_mark_length 8: WebKit sliderTickSize is 1×3.
   - :1293 thumb 21: WebKit says 17, and the row is already ❓.
3. :1333 sidebar background: Apple documents NSVisualEffectView.Material.sidebar (G1) and a Liquid Glass sidebar (G2, HIG), not underPageBackgroundColor.
4. The appendix at :1673 says the AppKit 11 release notes 'confirm' NSTableView row 24pt. They do not state 24; they say only 'increased row height' and 28pt group rows. The rowHeight doc still says 'default row height is 16.0'.
5. The appendix at :1675 (✅ for NSTextField 22) actually rests on WebKit's searchFieldSizes, which belong to a different control. TextFieldMac.mm has no size.
6. :1356 should say NSToolbar.SizeMode is deprecated at macOS 27.0 ('The toolbar no longer uses this type'). It is not deprecated on 14/15/26.
7. :1243 menu hover: Apple's deprecation of selectedMenuItemColor (11.0) points to NSVisualEffectMaterialSelection. No Apple source maps the highlight to selectedContentBackgroundColor.
8. :1220: WebKit's G2 radio dot is 0.3× the box (4.8 at 16). That is a citable (WebKit) value if one is wanted.
9. Already known, cited rather than re-reported:
   - todo.md:325-343 covers the vertical-padding derivation at :1172/:1197/:1236/:1319/:1391.
   - todo.md:371-390 covers the scrollbar thumb colours.
   - todo.md:79 covers checkbox hover.
   - todo.md:561 covers the unread NSProgressIndicator border.

Step 1 sources: confirmed as usable for these rows. I opened the WebKit files at de320ba3 myself. The Step 1 WebKit line references I re-checked (ScrollbarThemeMac.mm:660-665, RenderThemeCocoa vector functions) match.

Probe priorities, each on macos-14, macos-15, macos-26 and xcode-27:
- NSButton push intrinsicContentSize / alignmentRect
- NSTextField height
- NSSlider knob and track
- NSProgressIndicator bar
- NSScroller.scrollerWidth for both styles
- NSTableView default rowHeight (.inset)
- NSMenuItem row height
- NSToolbar height
- tooltip max width
- window corner radius
- font traits of titleBar/menu/toolTips

</details>

<details><summary>widgets-2</summary>

I covered the whole scope (1401-1647); nothing was left out. Step 1's pinned sources hold. One correction: the Wayback availability and CDX APIs answered this time, with no 429. That gave Sonoma-era HIG snapshots for typography, alerts, popovers, layout, color, split-views and disclosure-controls.

New findings (none of these is in docs/todo.md):
1. :1420 contradicts Apple. NSSplitView.dividerColor is clear for the default thick style, and the separatorColor page says 'Do not use this color for split view dividers'.
2. :1419 rests on GNUstep (appendix :1677), not on Apple. Apple's HIG gives only the thin divider (1 pt) and recommends it; the default thick width is unstated.
3. Appendix :1679 is marked ✅ on the strength of the sizeToFit docs, but that page gives no 32/16/10. So :1533-1534 are unsourced.
4. Rows :1496-1497 carry '✅ (HIG) emphasized system font'. No HIG page says this, neither the 2024 snapshot nor the current one, so the ✅ is unsupported.
5. :1453/:1455/:1456 are tagged (HIG), but HIG layout states no macOS spacing (2024 or 2026).
   - 8 comes from the NSStackView.spacing API.
   - 20 comes from the 2016 Auto Layout Guide (IB standard spacing to the superview).
   - section_gap has no source at all.
6. Popup button (WebKit RenderThemeMac.mm:925-934, used by both the G1 and G2 paths):
   - :1554 arrow_area_width is 26 regular, not 16-18; the appendix at :1678 already says so.
   - :1557 padding is 2 top / 3 bottom.
   - :1552 left padding is exactly 8.
   - Gecko gives the regular dropdown a height of 22, where WebKit gives 21.
   - Gecko says 'Controls on macOS 26 fill the entire frame' (no cell-margin inflation on 26).
7. Appendix :1683: Gecko's 21x21 cell is NSBezelStyleRoundedDisclosure (the push-disclosure button), not the disclosure triangle.
8. Switch on G2 (WebKit's Liquid Glass rendition):
   - track ratio is 64:28 (against 38:22 on G1);
   - the thumb is a capsule, 38/64 of the track wide;
   - dark thumb is white at alpha 0.85;
   - off track is quaternaryLabelColor;
   - disabled is opacity 0.5;
   - the track radius fraction lives only in Apple-internal WebKitAdditions.
9. G2 popup (WebKit):
   - fill is secondarySystemFill composited over the canvas;
   - text is labelColor (tertiaryLabelColor when disabled);
   - chevron is 0.55 em wide;
   - corner radius depends on control size. This bears on the open item todo.md:995.
10. NSControl.ControlSize.extraLarge is introducedAt macOS 26.0. Rows that list size variants (switch, popup, spinner) do not cover it.
11. Segmented control rows :1567-1568: per HIG, alternateSelectedControlTextColor is for lists and tables. HIG also defines selectedControlColor and selectedControlTextColor for 'a selected control'. No source maps any colour to NSSegmentedControl.

Version notes:
- Sequoia 15: no source separates it from 14 for any row checked here. The HIG change logs show no macOS control changes in 2024-2025 before 2025-06-09.
- macOS 27: the AppKit release notes add an NSSegmentedControl 'tabs' role with a distinct look and fix NSSegmentedCell drawing under Liquid Glass. There are no metrics, and WebKit has no 27 conditional. 27 is unknown for every other row.
- AppKit and HIG pages are unversioned. Where I marked a row 'same' on them, the evidence is a 2024 snapshot plus the current page, or an introducedAt with no deprecation.
- Known items cited, not re-reported: todo.md:345 (spinner stroke), :748 (text area padding), :139 (segmented divider), :995 (per-size radius).

Probe list for Step 3, on macos-14 and macos-26 runners:
- NSSwitch.intrinsicContentSize for every controlSize;
- NSPopUpButton intrinsic height;
- NSSplitView.dividerThickness and dividerColor for each style;
- NSProgressIndicator spinning sizeToFit for every controlSize;
- NSSegmentedControl height and colours;
- NSAlert messageText font, icon size and margins;
- NSTextView linkTextAttributes and textContainerInset;
- popover shadow;
- standard superview spacing.

</details>

## M2b — Windows

**Status:** Steps 1, 2, 4, 5 and 6 done 2026-10-11, beside M1 (D7), in the `m2-research` worktree; Step 3 awaits the maintainer; the Windows 10 gap waits for M1's stop point A; Step 7 not started. One Workflow ran all five parts (41 Opus agents at high effort, 5.41M subagent tokens, 36.5 min). Triage values: *fix* (Step 7), *Step 8* / *Step 13* (the part's build steps, already decided), *maintainer (Step 3 probe)*, *maintainer (version gap)*, *M4* (reader code), *defer* (an existing docs/todo.md entry), *drop*.

The preset targets Windows 11 with no version stated (`windows-11.toml:1-3`). Every WinUI 3 file the facts cite is
byte-identical at `8463f45`, the latest release `winui3/release/2.5.1` (`ba3a8d59`, 2026-09-19), `1.8.12` and `main`, and
WinUI 3 draws the same on Windows 10 (Windows App SDK runs on 10 1809+); WinUI 2.8.7 also defaults to the Windows 11 styles.
So the WinUI-sourced cells do not vary by feature update; only OS-drawn values (Win32 metrics, `GetSysColor`, UISettings,
`NONCLIENTMETRICSW` fonts) can, and Microsoft's documentation states none of them per version: the probe's question.
Windows 10 22H2 ended 2025-10-14 (ESU to 2026-10-13 for consumers, LTSC 2021 to 2027-01-12): the version gap for M1 T1.
Current Windows 11: 25H2 and 26H2 (GA 2026-09-29); 26H1 is new-device only; no client after 11 is announced.
Almost all 36 *differs* rows are CONFIRMED errors at the target version: status text colours, the focus visual, seven
`GetSysColor` indices Microsoft documents as unsupported since Windows 10 (the reader reads them: M4), the toolbar icon
(16 in the shipping template, 20 only in the docs), slider, progress, tab, list and flyout resources.

### Step 1 — version map

| Version | Released | Support on 2026-10-11 | Generation |
|---|---|---|---|
| Windows 10 22H2 (OS build 19045) Windows 10 2022 Update | 2022-10-18 (https://learn.microsoft.com/en-us/lifecycle/products/windows-10-home-and-pro, page updated 2024-09-17, read 2026-10-11; build 19045 from https://lea… | END OF SUPPORT 2025-10-14 for all GA editions (lifecycle pages above: 'Windows 10 will reach end of support on October 14, 2025. The current version, 22H2, will be the final version of Windows 10'; tables show retirement '10/15/2025 6:59:59 AM'; same on https:… | G1 Windows 10 (MDL2 / WinUI 2 Version1 styles) |
| Windows 10 Enterprise LTSC 2021 (21H2, build 19044); LTSC 2019 (1809, build 17763) Windows 10 LTSC | LTSC 2021: 2021-11-16; LTSC 2019: 2018-11-13 (https://learn.microsoft.com/en-us/windows/release-health/release-information, ms.date 2026-09-29) | LTSC 2021 mainstream end 2027-01-12 (IoT Enterprise extended to 2032-01-13); LTSC 2019 extended end 2029-01-09; LTSB 2016 ends 2026-10-13 (same release-information page; LTSC 2021 also https://learn.microsoft.com/en-us/lifecycle/products/windows-10-enterprise-… | G1 Windows 10 |
| Windows 11 23H2 (OS build 22631) Windows 11 2023 Update | 2023-10-31 (https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information, ms.date 2026-09-29, read 2026-10-11) | Home/Pro: END OF UPDATES (ended 2025-11-11; lifecycle table '11/12/2025 6:59:59 AM', https://learn.microsoft.com/en-us/lifecycle/products/windows-11-home-and-pro, updated 2026-09-29). Enterprise/Education: until 2026-11-10 (lifecycle '11/11/2026 6:59:59 AM', h… | G2 Windows 11 (Fluent / WinUI 3, Mica) |
| Windows 11 24H2 (OS build 26100) Windows 11 2024 Update | 2024-10-01 (release-information page above) | Home/Pro until 2026-10-13 (two days after this map; lifecycle '10/14/2026 6:59:59 AM'); Enterprise/Education until 2027-10-12; Enterprise LTSC 2024 until 2029-10-09, IoT Enterprise LTSC 2024 extended to 2034-10-10 (release-information page; lifecycle pages abo… | G2 Windows 11 |
| Windows 11 25H2 (OS build 26200) Windows 11 2025 Update | 2025-09-30 (release-information page) | Home/Pro until 2027-10-12; Enterprise/Education until 2028-10-10 (release-information page; lifecycle pages). Delivered as enablement package on 24H2: 'Windows 11, versions 24H2 and 25H2 share a common core operating system with an identical set of system file… | G2 Windows 11 |
| Windows 11 26H1 (OS build 28000) Windows 11, version 26H1 (new devices only) | 2026-02-10 (release-information page) | Home/Pro until 2028-03-14; Enterprise/Education until 2029-03-13 (release-information page; lifecycle '3/15/2028' / '3/14/2029 6:59:59 AM'). Page note: '26H1 is scoped to support new devices that started coming to market in early 2026 and is not designed as a… | G2 Windows 11 |
| Windows 11 26H2 (OS build 26300) Windows 11 2026 Update | 2026-09-29 (release-information page; lifecycle 'Version 26H2 9/29/2026') | Home/Pro until 2028-10-10; Enterprise/Education until 2029-10-09 (release-information page). Enablement package KB5121794 for devices on 24H2 or 25H2, 'a small, quick-to-install master switch that activates the Windows 11, version 26H2 features' (https://suppo… | G2 Windows 11 |
| Any Windows client release after Windows 11 none | none announced by 2026-10-11 | Microsoft lifecycle and release-health pages read 2026-10-11 list no client product or version after Windows 11 26H2. Only secondary press (e.g. PC-WELT 2026-06-03 relaying a Microsoft executive's denial of a 'Windows 12' at Build 2026) — not a primary source. | none |
| Windows Server 2022 (10.0.20348) / Windows Server 2025 (10.0.26100) Server builds behind GitHub runners windows-2022 / w… | Server 2022: 2021-08-18; Server 2025: 2024-11-01 (https://learn.microsoft.com/en-us/lifecycle/products/windows-server-2022, https://learn.microsoft.com/en-us/li… | Server 2022 mainstream end '10/14/2026 6:59:59 AM', extended '10/15/2031 6:59:59 AM'; Server 2025 mainstream '11/14/2029 6:59:59 AM', extended '11/15/2034 6:59:59 AM' (pages above). Recorded as Server builds, not Windows 10/11. | Server (not a client generation) |

Generations:

- **G1 — Windows 10 (MDL2 / WinUI 2 'Version1' legacy styles)** (Windows 10 1507–22H2 (22H2 the only one left, ESU only), Windows 10 LTSC 2021/2019): Baseline: Segoe MDL2 Assets icon font (Segoe Fluent Icons not included on Windows 10); no Mica (Mica falls back to SolidBackgroundFillColorBase when 'The Windows version is below 22000'); no automatic window corner rounding (a Windows 11 Geometry feature). UWP/WinUI 2 legacy control styles = ControlsResourcesVersion.Version1 ('Legacy style'), shipped as *_themeresources_v1.xaml in WinUI 2.8.7 (dev/CommonStyles/, e.g. Button_themeresources_v1.xaml, Common_themeresources_v1.xaml); selected in dev/dll/XamlControlsResources.cpp:34-36,79 at v2.8.7. Win32 metrics/colours via the same APIs as Windows 11 (GetSystemMetricsForDpi needs Windows 10 1607+), but runtime values may differ (facts :286 claim… Source: https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font (ms.date 2026-09-03); https://learn.microsoft.com/en-us/windows/apps/design/style/mica (ms.date 2026-07-22); https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-rounded-corners (ms.date 2026-07-07); https://learn.microsoft.com/en-us/windows/winui/api/microsoft.ui.xaml.controls.controlsresource…
- **G2 — Windows 11 (Fluent / WinUI 3, Mica, rounded corners)** (Windows 11 21H2–26H2 incl. 26H1; in support: 23H2 (Ent/Edu only), 24H2, 25H2, 26H1, 26H2, LTSC 2024): Segoe Fluent Icons replaced Segoe MDL2 Assets as recommended icon font 'with the release of Windows 11'; Mica / Mica Alt on build 22000+; system rounds top-level window corners; WinUI 2.6 (June 2021) 'Most WinUI controls now support the latest Windows 11 styles' (Version2 = unsuffixed *_themeresources.xaml); WinUI 3 styles in microsoft-ui-xaml controls/dev/**/*_themeresources.xaml, versioned by Windows App SDK release ('WinUI 3 in WinAppSDK x.y.z' tags), not by Windows feature update. Within G2: 24H2/25H2/26H2 share one core OS (25H2 and 26H2 are enablement packages on 24H2); 23H2 (22631) and 26H1 (28000) are separate build lines. Whether any visual value differs between 23H2…26H2 is NOT est… Source: https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font; https://learn.microsoft.com/en-us/windows/apps/design/style/mica; https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-rounded-corners; https://learn.microsoft.com/en-us/windows/apps/winui/winui2/release-notes/winui-2.6 (ms.date 2025-04-22); https://support.microsoft.com/help/5054156; https://…

Pinned sources:

- microsoft-ui-xaml (WinUI 3) main HEAD: https://github.com/microsoft/microsoft-ui-xaml/tree/48df672d5192726c5c0efae2a020ed9444c2f8bd — main @ 48df672d5192726c5c0efae2a020ed9444c2f8bd (2026-10-10, 'Reduce XAML compiler metadata lookup overhead (#12062)'); root layout controls/dev/…; covers G2. All 27 microsoft-ui-xaml files the facts doc/todo cite (24 *_themeresources.xaml + DropDownButton.xaml, Expander.xaml, TabView.xaml, ListViewBaseItemChrome.h, generic.xaml) have IDENTICAL blobs at…
- microsoft-ui-xaml current facts pin: https://github.com/microsoft/microsoft-ui-xaml/tree/8463f45162149de0ec3ad7df752596893fe3e13e — 8463f45162149de0ec3ad7df752596893fe3e13e (2026-05-30, 'Switch winui3/main source mirror to root layout'); ancestor of main and of tag 1.8.12, NOT of the 2.x tags; covers G2; cited 31× in platform-facts.md. Still valid: cited files unchanged up to main HEAD. 2b8c7757ef2dd57234d5d7c3016d136c59a24bad (2026-09-23, facts :1552, todo :321) is on main; same blobs.
- microsoft-ui-xaml latest stable release (GitHub 'latest'): https://github.com/microsoft/microsoft-ui-xaml/releases/tag/winui3/release/2.5.1 — tag winui3/release/2.5.1 @ ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd, published 2026-09-19 ('WinUI 3 in WinAppSDK 2.5.1'); src/-prefixed layout (src/controls/dev/…), separate history from main; covers G2, WinAppSDK 2.x line. Newer: 2.5.4-experimental (7b127093…, 2026-09-30, prerelease). vs 8463f45 only CommandBarFlyout (2.5.1's unsuffixed file = 8463's _perf2026 blob), CommandBar_perf2026 (absent),…
- microsoft-ui-xaml 1.8 servicing release: https://github.com/microsoft/microsoft-ui-xaml/releases/tag/winui3/release/1.8.12 — tag winui3/release/1.8.12 @ 91da8f5303651791acbb96e63a8ac4c38851e678, published 2026-09-24; on main, root layout; covers G2, WinAppSDK 1.8 line; the release tag closest to the current 8463f45 pin (same paths, same cited blobs) — candidate pin if a release tag is preferred.
- microsoft-ui-xaml WinUI 2 (UWP): https://github.com/microsoft/microsoft-ui-xaml/tree/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/CommonStyles — tag v2.8.7 @ 232a16e5ddfc22c9a1b79a2c51abeb9a39a94494 (2025-01-31, released 2025-02-05); branch release/2.8 HEAD 771c6af668646e9df50a7c9c6d31710d5e8d3f5f (2025-11-27) is 5 commits later with no dev/CommonStyles change; covers G1 (Windows 10-era *_themeresources_v1.xaml, ControlsResourcesVersion.Version1) and G2 WinUI 2 Version2 styles.
- Fluent UI tokens (Fluent 2 code tokens): https://github.com/microsoft/fluentui/tree/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src — tag @fluentui/tokens_v1.0.0-alpha.24 @ b6351802032e9af61bf03033d22cb354b2e2822c (2026-08-11); master 6e52576d457f0ca13225ea2712e0db7bd1c27e28 (2026-10-09); covers G2 cross-check. Facts :1736, :1746, :1756 cite blob/master (unpinned); cited spacings.ts last changed 97c1c1ab (2022-12-20), lightColor.ts/darkColor.ts 3e77c7d3 (2026-01-22), both contained in the alp…
- Fluent 2 design spec: https://fluent2.microsoft.design/ — unversioned site, reachable (HTTP 200 for /layout, /elevation) 2026-10-11; content not re-read in Step 1; covers G2 design guidance (elevation, layout); cite as a dated page.
- Win32 / WinRT API docs: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi — read 2026-10-11: GetSystemMetricsForDpi git fa536415 (updated 2025-07-01; min client Windows 10 1607); GetSystemMetrics git fa536415 (2025-07-01); GetSysColor git 92672624 (2024-02-22); NONCLIENTMETRICSW git a9040b7b (2024-11-20); UISettings git 3abc0b43 (2025…; covers G1 and G2 (same APIs on both; runtime values per version need the probe).
- Microsoft lifecycle / release health: https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information — Win11 release info ms.date 2026-09-29 (git 13ff9d05); Win10 release info ms.date 2026-09-29; lifecycle Win11 Home/Pro git b2a94db7 (updated 2026-09-29); Win11 Ent/Edu git 096e2c61 (2026-10-06); Win10 Home/Pro git 8bc0bd31; Win10 Ent/Edu git ad2d1131; ESU page…; covers All version, date and support claims above.
- actions/runner-images: https://github.com/actions/runner-images/tree/8197087fc536320d1441203fdb5da9ae1b44b863 — main @ 8197087fc536320d1441203fdb5da9ae1b44b863 (2026-10-09); covers CI runner images (see ci_runner_images).

Preset target: No OS version or source revision stated. /home/tibi/Rust/native-theme-m2/native-theme/src/presets/windows-11.toml:1-3 "# Windows 11 Fluent Design theme / # Source: WinUI3 XAML theme resources, Fluent Design System / name = \"Windows 11\"" (values then cite platform-facts.md lines, e.g. :33, :100, :121, :158). windows-11-live.toml:1-2,6 "Windows 11 live preset - geometry/metrics only … name = \"Windows 11\"". presets/README.md:27, :43, :67-68 "Windows 11 standalone theme" / "Windows 11 base for the Windows reader". The reader picks it for every Windows version: native-theme/src/pipeline.rs:390, :418 (PlatformPreset name "windows-11"), :641 ("windows-11-live"); windows.rs:1406. docs/new-os-version-guide.md:44-82 suggests a new preset (e.g. windows-12.toml) only 'if the visual language changes significantly'. Effective target: G2 Windows 11, any feature update; WinUI 3 values pinned via fac…

CI runner images: From github.com/actions/runner-images @ 8197087fc536320d1441203fdb5da9ae1b44b863 (2026-10-09), README.md:37-40 and images/windows/*-Readme.md lines 6-8: (1) windows-2022 = Windows Server 2022, OS 10.0.20348 Build 5622, image 20261004.326.1 (Server build). (2) windows-2025 / windows-latest / windows-2025-vs2026 = Windows Server 2025, OS 10.0.26100 Build 33438, images 20261004.281.1 (Windows2025-Readme.md) and 20260925.250.1 (Windows2025-VS2026-Readme.md); README row 37 maps all three labels to the VS2026 image (Server build; 26100 is the same build number as Windows 11 24H2, but it is Server). (3) windows-11-arm = 'Windows 11 Enterprise', OS 10.0.26200 Build 9457 (= Windows 11 25H2 build line), arm64, image 20261004.186.1; (4) windows-11-vs2026-arm = same OS, image 20261004.176.1; announcement (issue #14602) says windows-11-arm moves to the VS2026 image in September 2026. windows-11-arm is a real Windows 11 CLIENT image — not listed in the review plan's Step 3 (which names only windows-2022/windows-2025). No Windows 10 image and no x64 Windows 11 image is offered.

<details><summary>Version-map notes</summary>

New, not on docs/todo.md: (a) platform-facts.md lines 1711, 1723-1735, 1742, 1748-1755 cite microsoft-ui-xaml/blob/main/src/… — unpinned AND dead (main dropped the src/ prefix at 8463f45 on 2026-05-30; raw URL returns 404, checked 2026-10-11); the same files exist at 8463f45 under controls/dev/… with identical blobs. (b) fluentui citations at :1736, :1746, :1756 use blob/master (unpinned); pin to tokens alpha.24 b6351802. (c) The Windows chapter's rows (274-547) name no OS version except :286 (Win10 400 / Win11 700 caption weight), whose citation is a Microsoft Q&A forum answer = secondary. Version gap: G1 Windows 10 has no preset; the reader maps every Windows to windows-11(-live) (pipeline.rs:390,418,641); 22H2 lives on in ESU to 2027-10-12 (consumer) and LTSC 2021 to 2027-01-12 (IoT 2032) — a T1 decision (plan :506 already asks 'is windows-10 worth adding'). Lifecycle edges: 24H2 Home/Pro ends 2026-10-13; 23H2 Home/Pro already ended 2025-11-11; Ent/Edu 23H2 ends 2026-11-10. 26H1 (build 28000) is new-device-only. 26H2 GA 2026-09-29 as eKB on 24H2/25H2. WinUI 3 control styles ship per WinAppSDK release (1.8.x and 2.x lines), so they do not by themselves vary by Windows feature update; OS-drawn parts (Win32 metrics, UISettings colours, NONCLIENTMETRICS fonts) need the probe per version. Lifecycle-page timestamps are 'M/D/YYYY 6:59:59 AM' of the day after the release-health ISO end date; dates above use the release-health ISO dates unless quoted. Usage figures are StatCounter (secondary). Clone/scratch: /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/win-map/ (muxc partial clone, runner readmes, web/).

</details>

### Step 2 — rows checked

358 rows: 283 same, 36 differs, 39 unverifiable (36 of them a probe or reference app could settle). Scopes: fonts-colours 73, geometry 113, widgets-1 104, widgets-2 68. Every researcher reported its scope covered with no line range left out.

Step 5 verdicts on the *differs* rows: 34 CONFIRMED, 2 PLAUSIBLE.

Step 6 triage: 34 fix, 2 maintainer (Step 3 probe).

### Findings — *differs* rows

#### M2b-01 · bug · S2 · docs/platform-facts.md:1081
**Claim:** `danger_text_color` (2.1.4 Status Colors): the cell says L #ffffff D #1a1a1a — no dedicated WinUI3 resource; G2 Windows 11 WinAppSDK 2.5.1 (= 8463f45 blob): InfoBarErrorSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000; G1 Windows 10 WinUI 2.8.7: same mapping: L #FFFFFF, D #E4000000.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/InfoBar/InfoBar_themeresources.xaml#L13 , #L32 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L213 , #L9 [G1 Windows 10] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/InfoBar/InfoBar_themeresources.xaml#L21 , #L45 ; Common_themeresources_any.xaml#L272 , #L17 at the same tag Note: NEW, not on todo.md. WinUI does have a resource for the glyph drawn on the status fill: InfoBar's severity icon (background SystemFillColorCriticalBrush, InfoBar_themeresources.xaml:9) is drawn in TextFillColorInverse. The dark #1a1a1a matches no brush, and the appendix row :1745 needs the same correction.
**Verdict:** CONFIRMED — Read at both tags: the InfoBar severity-icon glyph, which sits on SystemFillColorCritical (the danger fill), maps to TextFillColorInverse L #FFFFFF / D #E4000000, so the row's 'no dedicated WinUI3 resource' and its opaque D #1a1a1a are wrong (HighContrast uses SystemColorHighlightTextColor). Confirmed value: InfoBarErrorSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000 (WinAppSDK 2.5.1 tag winui3/release/2.5.1 = ba3a8d59, InfoBar_themeresources.xaml:13 Light, :32 Default; Common_t….
**Triage:** fix — Read at both tags: the InfoBar severity-icon glyph, which sits on SystemFillColorCritical (the danger fill), maps to TextFillColorInverse L #FFFFFF / D #E4000000, so the row's 'no dedicated WinUI3 resource' and its opaque D #1a1a1a are wrong (HighContrast uses SystemColorHighlightTextColor).
**Status:** open

#### M2b-02 · bug · S2 · docs/platform-facts.md:1083
**Claim:** `warning_text_color` (2.1.4 Status Colors): the cell says L #1a1a1a D #1a1a1a — no dedicated WinUI3 resource; G2 Windows 11 WinAppSDK 2.5.1: InfoBarWarningSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000; G1 Windows 10 WinUI 2.8.7: same.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/InfoBar/InfoBar_themeresources.xaml#L14 , #L33 ; Common_themeresources_any.xaml#L213 , #L9 (same tag) [G1 Windows 10] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/InfoBar/InfoBar_themeresources.xaml#L22 , #L46 Note: NEW. The light value differs too: WinUI draws white on the #9D5D00 fill, where the cell says #1a1a1a.
**Verdict:** CONFIRMED — Read at both tags; the light value is #FFFFFF, not the row's #1a1a1a, and a dedicated resource exists. Confirmed value: InfoBarWarningSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000 (2.5.1 InfoBar_themeresources.xaml:14, :33; v2.8.7 :22, :46).
**Triage:** fix — Read at both tags; the light value is #FFFFFF, not the row's #1a1a1a, and a dedicated resource exists.
**Status:** open

#### M2b-03 · bug · S2 · docs/platform-facts.md:1085
**Claim:** `success_text_color` (2.1.4 Status Colors): the cell says L #ffffff D #1a1a1a; G2 Windows 11 WinAppSDK 2.5.1: InfoBarSuccessSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000; G1 Windows 10 WinUI 2.8.7: same.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/InfoBar/InfoBar_themeresources.xaml#L15 , #L34 [G1 Windows 10] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/InfoBar/InfoBar_themeresources.xaml#L23 , #L47 Note: NEW. Only the dark value differs.
**Verdict:** CONFIRMED — Read at both tags; dark is #E4000000 (black, alpha 0xE4), not opaque #1a1a1a, and a dedicated resource exists. Confirmed value: InfoBarSuccessSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000 (2.5.1 :15, :34; v2.8.7 :23, :47).
**Triage:** fix — Read at both tags; dark is #E4000000 (black, alpha 0xE4), not opaque #1a1a1a, and a dedicated resource exists.
**Status:** open

#### M2b-04 · bug · S2 · docs/platform-facts.md:1087
**Claim:** `info_text_color` (2.1.4 Status Colors): the cell says L #ffffff D #1a1a1a; G2 Windows 11 WinAppSDK 2.5.1: InfoBarInformationalSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000; G1 Windows 10 WinUI 2.8.7: same.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/InfoBar/InfoBar_themeresources.xaml#L16 , #L35 [G1 Windows 10] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/InfoBar/InfoBar_themeresources.xaml#L24 , #L48 Note: NEW. Only the dark value differs.
**Verdict:** CONFIRMED — Read at both tags; dark is #E4000000, not #1a1a1a, and a dedicated resource exists. Confirmed value: InfoBarInformationalSeverityIconForeground = TextFillColorInverseBrush: L #FFFFFF, D #E4000000 (2.5.1 :16, :35; v2.8.7 :24, :48).
**Triage:** fix — Read at both tags; dark is #E4000000, not #1a1a1a, and a dedicated resource exists.
**Status:** open

#### M2b-05 · bug · S2 · docs/platform-facts.md:1104
**Claim:** `focus_ring_color` (2.1.5 Focus Ring): the cell says UISettings(Accent) (same as accent); G2 Windows 11 WinAppSDK 2.5.1 (= 8463f45): SystemControlFocusVisualPrimaryBrush = FocusStrokeColorOuter: L #E4000000, D #FFFFFF (inner/secondary FocusStrokeColorInner L #B3FFFFFF, D #B3000000). Accent is used only when Reveal focus is enabled; G1 Windows 10 WinUI 2.8.7: same: FocusStrokeColorOuter L #E4000000, D #FFFFFF.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources.xaml#L15-L16 , #L29-L30 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L54-L55 , #L258-L259 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/core/core/elements/DependencyProperty.cpp#L27… [G1 Windows 10] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/CommonStyles/Common_themeresources.xaml#L21 , #L35 ; Common_themeresources_any.xaml#L76-L77 , #L333-L334 at the same tag Note: NEW, not on todo.md. windows-11.toml:29 and :437 state focus_ring_color #0078d4 (accent). WinUI's focus visual is black/white, not the accent.
**Verdict:** CONFIRMED — Read at both tags: in DependencyProperty.cpp:278-301 the primary brush uses SystemAccentColor only when AreRevealFocusRectsEnabled(), otherwise SystemControlFocusVisualPrimaryBrush; the WinAppSDK FocusVisualPrimaryBrush page (updated 2026-09-30) says the outer colour is typically solid black, so the ring is not the accent colour. Confirmed value: SystemControlFocusVisualPrimaryBrush = FocusStrokeColorOuterBrush: L #E4000000, D #FFFFFF; secondary = FocusStrokeColorInner: L #B3FFFFFF, D #B3000000 (2.5.1 Common_themeresources.xaml:15-16, :29-30;….
**Triage:** fix — Read at both tags: in DependencyProperty.cpp:278-301 the primary brush uses SystemAccentColor only when AreRevealFocusRectsEnabled(), otherwise SystemControlFocusVisualPrimaryBrush; the WinAppSDK FocusVisualPrimaryBrush page (updated 2026-09-30) says the outer colour is typically solid black, so the ring is not the accent colour.
**Status:** open

#### M2b-06 · bug · S2 · docs/platform-facts.md:1106
**Claim:** `focus_ring_offset` (2.1.5 Focus Ring): the cell says Fluent: 0px default margin (outset); G2 Windows 11 WinAppSDK 2.5.1 (= 8463f45): FrameworkElement default 0, but 'A different runtime default is often provided by individual control templates'. DefaultButtonStyle and AccentButtonStyle set FocusVisualMargin -3 (ring 3px outside); CheckBox -7,-3,-7,-3; G1 Windows 10 UWP docs: same default 0 and the same template-override remark.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.frameworkelement.focusvisualmargin (updated 2026-09-30); https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Button_themeresources.xaml#L167 , #L310 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L275 [G1 Windows 10] https://learn.microsoft.com/en-us/uwp/api/windows.ui.xaml.frameworkelement.focusvisualmargin (updated 2025-11-21) Note: The doc says 'a negative margin will push the border away from the center'. The cell gives the bare FrameworkElement default, but the controls the model maps (button, checkbox) draw the ring 3px outset, or 7/3. The presets state 0 (windows-11.toml:37, -live:16).
**Verdict:** CONFIRMED — Both quotes are on the WinAppSDK page (updated 2026-09-30), and the Button and CheckBox values hold at 2.5.1; correction: #L310 is SubtleButtonStyle, not AccentButtonStyle, which (line 235) has no BasedOn and no FocusVisualMargin setter; I did not open the UWP page; the value differs by control (ListViewItem 1, others -2), so a single value needs a choice. Confirmed value: FrameworkElement default 0 ('A different runtime default is often provided by individual control templates'; 'A negative margin will push the border away from the center'); DefaultButtonStyle FocusVis….
**Triage:** fix — CONFIRMED with a correction: line 310 is SubtleButtonStyle, not AccentButtonStyle.
**Status:** open

#### M2b-07 · bug · S2 · docs/platform-facts.md:436
**Claim:** `ListView horizontal padding` (1.2.4 Widget Metrics): the cell says 12px ✅ 'Padding=12,0,12,0 in Grid-based style (first style uses 16/12)'; G2 WinUI 3 pin 8463f45: 16 left / 12 right — the implicit default style (TargetType=ListViewItem BasedOn DefaultListViewItemStyle) sets Padding 16,0,12,0; 12,0,12,0 is only the named, unreferenced style ListViewItemExpanded; G2 WinUI 3 latest release 2.5.1: 16 left / 12 right (identical blob).
**Evidence:** [G2 WinUI 3 pin] https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L233-L241 ; …#L261-L270 [G2 WinUI 3 latest release] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L241 Note: Cell picks the non-default style. Per the per-side rule the cell should read '16 left / 12 right'. ListViewItemExpanded is referenced nowhere else in controls/ at 8463f45 (git grep).
**Verdict:** CONFIRMED — Grepped the whole 2.5.1 tree: ListViewItemExpanded appears only where it is defined (generic.xaml, the themeresources files) and in an API-test baseline list, so no control uses the 12/12 style. Confirmed value: 16 left / 12 right: the implicit style (ListViewItem_themeresources.xaml:233, BasedOn DefaultListViewItemStyle :234) sets Padding 16,0,12,0 at :241; 12,0,12,0 (:270) is only in x:Key=ListViewItemExpan….
**Triage:** fix — Grepped the whole 2.5.1 tree: ListViewItemExpanded appears only where it is defined (generic.xaml, the themeresources files) and in an API-test baseline list, so no control uses the 12/12 style.
**Status:** open

#### M2b-08 · bug · S2 · docs/platform-facts.md:530
**Claim:** `AppBarButton icon` (1.2.6 Icon Sizes): the cell says 20px ✅ 'MS docs 20×20; AppBarButtonContentHeight=16 is legacy v1 (Reveal) only'; G2 WinUI 3 pin 8463f45: 16 — AppBarButton template ContentViewbox Height={ThemeResource AppBarButtonContentHeight}; the key is defined at root level of WinUI 3 generic.xaml = 16 (and in RevealBrush_themeresources = 16); no controls/ themeresources defines 20 (only TestUI CommonStyles…; G2 WinUI 3 latest release 2.5.1: 16 (generic.xaml:19462 identical blob); G2 docs Command bar, ms.date 2025-02-26: 'icons when shown in the primary command area is 20x20px; in the overflow menu … 16x16px'.
**Evidence:** [G2 WinUI 3 pin] https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/AppBarButton_themeresources.xaml#L366 ; https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/dxaml/xcp/dxaml/themes/generic.xaml#L19462 [G2 WinUI 3 latest release] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/themes/generic.xaml#L19462 [G2 docs] https://learn.microsoft.com/en-us/windows/apps/design/controls/command-bar (git 35e19e6d, updated 2026-06-25) Note: Source contradicts both the doc and the cell's 'legacy only' claim: 16 is the WinUI 3 framework default. Overflow states set 16x16 explicitly (:215-233). Screenshot of a default CommandBar settles 16 vs 20. Not on todo.md.
**Verdict:** CONFIRMED — Read at both 8463f45 and 2.5.1 (the files are byte-identical): the current template reads the token, the token is 16 everywhere it ships, and the row's claim that 16 is 'legacy Reveal only' is false; the docs' 20x20 (command-bar page, ms.date 2025-02-26, git 35e19e6d) conflicts with the source. Confirmed value: 16: the DefaultAppBarButtonStyle template's ContentViewbox Height={ThemeResource AppBarButtonContentHeight} (AppBarButton_themeresources.xaml:366), and the overflow states set Width and Height 16 expl….
**Triage:** fix — CONFIRMED: the shipping template renders the icon through a 16-high Viewbox; the docs' 20 x 20 is recorded beside it.
**Status:** open

#### M2b-09 · bug · S2 · docs/platform-facts.md:1126
**Claim:** `reduce_transparency (Windows)` (2.1.7 Accessibility): the cell says (none) — high contrast disables it; G1+G2 UISettings docs read 2026-10-11: UISettings.AdvancedEffectsEnabled — 'whether the system Transparency effects setting is enabled' (Windows 10 1703 / 15063+; change event AdvancedEffectsEnabledChanged 1903+).
**Evidence:** [G1+G2] https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.advancedeffectsenabled (git 3abc0b43, updated 2025-11-21); https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings (version table) Note: Windows does expose this setting; native-theme/src/windows.rs:536 also says 'Windows does not expose this setting'. Not on todo.md.
**Verdict:** CONFIRMED — Read on the page (updated 2025-11-21, git 3abc0b43), so the row's '(none)' is wrong; correction: AdvancedEffectsEnabledChanged is also listed as introduced in 10.0.15063.0, not 1903. Confirmed value: UISettings.AdvancedEffectsEnabled: 'Gets a value that indicates whether the system Transparency effects setting is enabled' (introduced 10.0.15063.0, UniversalApiContract v4).
**Triage:** fix — Read on the page (updated 2025-11-21, git 3abc0b43), so the row's '(none)' is wrong; correction: AdvancedEffectsEnabledChanged is also listed as introduced in 10.0.15063.0, not 1903.
**Status:** open

#### M2b-10 · bug · S2 · docs/platform-facts.md:1132
**Claim:** `toolbar (Windows)` (2.1.8 Icon Sizes): the cell says Fluent AppBarButton: 20; G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 16 (AppBarButtonContentHeight, generic.xaml:19462); G2 docs ms.date 2025-02-26: 20x20 (doc).
**Evidence:** [G2 WinUI 3 pin / 2.5.1] https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/dxaml/xcp/dxaml/themes/generic.xaml#L19462 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/themes/generic.xaml#L19462 [G2 docs] https://learn.microsoft.com/en-us/windows/apps/design/controls/command-bar (git 35e19e6d) Note: Same conflict as :530; source vs doc, screenshot settles.
**Verdict:** CONFIRMED — Same evidence as index 7: the shipping template renders the primary-area icon through a 16-high Viewbox, and only the docs say 20. Confirmed value: 16 (AppBarButtonContentHeight, generic.xaml:19462 at 8463f45 = 2.5.1); docs say 20x20 for the primary area and 16x16 for overflow.
**Triage:** fix — Same evidence as the :530 row.
**Status:** open

#### M2b-11 · bug · S4 · docs/platform-facts.md:1152
**Claim:** `title_bar_font.color` (2.2 Window chrome): the cell says COLOR_CAPTIONTEXT; G2 Windows 11 25H2: COLOR_CAPTIONTEXT: 'Windows 10 or greater: This value is not supported.' Supported set: WINDOW, WINDOWTEXT, HOTLIGHT, GRAYTEXT, HIGHLIGHTTEXT, HIGHLIGHT, BTNTEXT, 3DFACE; G1 Windows 10 22H2: same: unsupported on Windows 10 or greater.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (git 92672624, updated 2024-02-22, read 2026-10-11) [G1 Windows 10] same URL Note: Chapter 1 (:355-365) already says these constants are unsupported, but §2.2 cites them bare; the reader reads it (native-theme/src/windows.rs:312). No supported replacement named by the doc; what the caption text really is needs the probe. Not on todo.md.
**Verdict:** CONFIRMED — Read on the GetSysColor page (updated 2024-02-22, git 92672624, read 2026-10-11); the page has no per-release split, so Windows 10 22H2 and Windows 11 are the same. Confirmed value: COLOR_CAPTIONTEXT (9): 'Windows 10 or greater: This value is not supported.'; supported on Windows 10/11: WINDOW, WINDOWTEXT, HOTLIGHT, GRAYTEXT, HIGHLIGHTTEXT, HIGHLIGHT, BTNTEXT, 3DFACE.
**Triage:** fix — CONFIRMED; chapter 1 (:355-365) already marks the index unsupported, the chapter 2 cell lacks the caveat. The reader reads it (windows.rs:304-313): M4.
**Status:** open

#### M2b-12 · bug · S4 · docs/platform-facts.md:1153
**Claim:** `inactive_title_bar_background` (2.2 Window chrome): the cell says COLOR_INACTIVECAPTION; G2 Windows 11 25H2: 'Windows 10 or greater: This value is not supported.'; G1 Windows 10 22H2: same.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10] same URL Note: Reader reads it (windows.rs:359-360). Not on todo.md.
**Verdict:** CONFIRMED — Read on the same GetSysColor page, read 2026-10-11. Confirmed value: COLOR_INACTIVECAPTION (3): 'Windows 10 or greater: This value is not supported.' (COLOR_INACTIVECAPTIONTEXT on row 1154 carries the same note).
**Triage:** fix — As :1152.
**Status:** open

#### M2b-13 · bug · S4 · docs/platform-facts.md:1154
**Claim:** `inactive_title_bar_text_color` (2.2 Window chrome): the cell says COLOR_INACTIVECAPTIONTEXT; G2 Windows 11 25H2: 'Windows 10 or greater: This value is not supported.'; G1 Windows 10 22H2: same.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10] same URL Note: Reader reads it (windows.rs:313).
**Verdict:** CONFIRMED — The live page marks COLOR_INACTIVECAPTIONTEXT unsupported on Windows 10 or greater (covers both 22H2 and 25H2), but platform-facts.md:358-364 already says so; the §2.2 row just doesn't carry the caveat, so this is not a new discovery. Confirmed value: COLOR_INACTIVECAPTIONTEXT: 'Windows 10 or greater: This value is not supported.' (GetSysColor page, git 92672624, updated 2024-02-22, read 2026-10-11; also raw sdk-api nf-winuser-getsyscolor.md:385-38….
**Triage:** fix — As :1152 (verifier: chapter 1 already says so).
**Status:** open

#### M2b-14 · bug · S2 · docs/platform-facts.md:1179
**Claim:** `hover_background` (2.3 Button): the cell says (Fluent) SubtleFillColorSecondary overlay; G2 Windows 11 WinUI 3 2.5.1: ButtonBackgroundPointerOver = ControlFillColorSecondaryBrush (L #80F9F9F9, D #15FFFFFF), a replacement fill, not an overlay; G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: SolidColorBrush SystemBaseHighColor Opacity 0.1.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Button_themeresources.xaml#L31 (identical at 8463f45 #L31; Light dictionary in same file); colours Common_themeresources_any.xaml#L16 (D), #L220 (L) at both revisions [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/CommonStyles/Button_themeresources_v1.xaml#L23 Note: SubtleFillColorSecondary is the menu/list/nav hover, not Button's. Preset states #0000000a / #ffffff0f (windows-11.toml:86, :494). Not on todo.md.
**Verdict:** CONFIRMED — Both cited values hold at the given lines; the row's SubtleFillColorSecondary belongs to SubtleButtonStyle (:18, :116), not to the default Button. Confirmed value: G2: ButtonBackgroundPointerOver = ControlFillColorSecondaryBrush (Default dict Button_themeresources.xaml:31, Light :129, used by DefaultButtonStyle PointerOver :182 as a ContentPresenter Background r….
**Triage:** fix — Both cited values hold at the given lines; the row's SubtleFillColorSecondary belongs to SubtleButtonStyle (:18, :116), not to the default Button.
**Status:** open

#### M2b-15 · bug · S2 · docs/platform-facts.md:1191
**Claim:** `placeholder_color` (2.4 Text Input): the cell says (Fluent) TextPlaceholderColor; G2 Windows 11 WinUI 3 2.5.1: TextControlPlaceholderForeground = TextFillColorSecondaryBrush (L #9E000000, D #C5FFFFFF); no resource named TextPlaceholderColor.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBox_themeresources.xaml#L35 , #L142; Common_themeresources_any.xaml#L6 (D), #L210 (L) (identical at 8463f45) Note: Name not found in TextBox/Common/generic resources. Not on todo.md.
**Verdict:** CONFIRMED — Read as cited, and Common_themeresources_any.xaml contains no Placeholder resource at all. Confirmed value: TextControlPlaceholderForeground = TextFillColorSecondaryBrush (TextBox_themeresources.xaml:35 Default, :142 Light, used at :338); TextFillColorSecondary D #C5FFFFFF (Common_themeresources_any.xaml:6)….
**Triage:** fix — Read as cited, and Common_themeresources_any.xaml contains no Placeholder resource at all.
**Status:** open

#### M2b-16 · bug · S4 · docs/platform-facts.md:1227
**Claim:** `background_color` (2.6 Menu): the cell says COLOR_MENU; G2 Windows 11 25H2: COLOR_MENU: 'Windows 10 or greater: This value is not supported.'; G1 Windows 10 22H2: same.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (git 92672624, read 2026-10-11) [G1 Windows 10] same URL Note: Reader reads it (windows.rs:304). WinUI 3 menu: MenuFlyoutPresenterBackground = DesktopAcrylicTransparentBrush (MenuFlyout_themeresources.xaml:40, :202). Not on todo.md.
**Verdict:** CONFIRMED — The live page says this word for word, but platform-facts.md:358-364 already lists COLOR_MENU as 'not supported' (still returns values), so this is not new. Confirmed value: COLOR_MENU: 'Windows 10 or greater: This value is not supported.' (GetSysColor, git 92672624, read 2026-10-11).
**Triage:** fix — As :1152.
**Status:** open

#### M2b-17 · bug · S4 · docs/platform-facts.md:1233
**Claim:** `font.color` (2.6 Menu): the cell says COLOR_MENUTEXT; G2 Windows 11 25H2: COLOR_MENUTEXT: 'Windows 10 or greater: This value is not supported.'; G1 Windows 10 22H2: same.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10] same URL Note: Reader reads it (windows.rs:305). WinUI 3: MenuFlyoutItemForeground = TextFillColorPrimaryBrush (MenuFlyout_themeresources.xaml:11, :177).
**Verdict:** CONFIRMED — The live page says this word for word, but platform-facts.md:358-364 already records it, so this is not new. Confirmed value: COLOR_MENUTEXT: 'Windows 10 or greater: This value is not supported.' (GetSysColor, git 92672624, read 2026-10-11).
**Triage:** fix — As :1152.
**Status:** open

#### M2b-18 · bug · S4 · docs/platform-facts.md:1251
**Claim:** `background_color` (2.7 Tooltip): the cell says COLOR_INFOBK; G2 Windows 11 25H2: COLOR_INFOBK: 'Windows 10 or greater: This value is not supported.'; G1 Windows 10 22H2: same.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10] same URL Note: Reader reads it (windows.rs:306). WinUI 3: ToolTip Background = ToolTipBackgroundBrush; ToolTipBackground = SystemControlBackgroundChromeMediumLowBrush (ToolTip_themeresources.xaml:8, :32, :44).
**Verdict:** CONFIRMED — The live page says this word for word, but platform-facts.md:358-364 already records it, so this is not new. Confirmed value: COLOR_INFOBK: 'Windows 10 or greater: This value is not supported.' (GetSysColor, git 92672624, read 2026-10-11).
**Triage:** fix — As :1152.
**Status:** open

#### M2b-19 · bug · S2 · docs/platform-facts.md:1253
**Claim:** `font.size` (2.7 Tooltip): the cell says ← defaults.font; G2 Windows 11 WinUI 3 2.5.1: ToolTipContentThemeFontSize 12 (not the 14 body size); Win32 tooltips use lfStatusFont ('font used in status bars and tooltips'); G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: ToolTipContentThemeFontSize 12.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L5 , #L29 , #L49 (identical at 8463f45); https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (read 2026-10-11) [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/CommonStyles/ToolTip_rs1_themeresources_v1.xaml#L9 Note: Inheritance cell contradicted by both WinUI and Win32 sources (1252 family likewise → lfStatusFont for Win32). Not on todo.md.
**Verdict:** PLAUSIBLE — ToolTipContentThemeFontSize is 12 at ToolTip_themeresources.xaml:5/:29 and used at :49 (ba3a8d59 = 8463f451; v2.8.7 ToolTip_rs1_themeresources_v1.xaml:9), and NONCLIENTMETRICSW says lfStatusFont is 'the font used in status bars and tooltips', but 12 epx equals Windows defaults.font (lfMessageFont 9pt = 12 epx at 96 DPI, platform-facts.md:285, :291-292, :1026), so the 'not 14' contrast is wrong and only the row's source field (lfStatusFont rather than defaults.font) differs.
**Triage:** maintainer (Step 3 probe) — PLAUSIBLE: WinUI 3 ToolTipContentThemeFontSize is 12 and Win32 tooltips use lfStatusFont; a probe shows lfStatusFont's size at 96 DPI.
**Status:** open

#### M2b-20 · bug · S4 · docs/platform-facts.md:1256
**Claim:** `font.color` (2.7 Tooltip): the cell says COLOR_INFOTEXT; G2 Windows 11 25H2: COLOR_INFOTEXT: 'Windows 10 or greater: This value is not supported.'; G1 Windows 10 22H2: same.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10] same URL Note: Reader reads it (windows.rs:307). WinUI 3 ToolTipForeground = TextFillColorPrimaryBrush (ToolTip_themeresources.xaml:7, :31).
**Verdict:** CONFIRMED — The live page says this word for word, but platform-facts.md:358-364 already records it, so this is not new. Confirmed value: COLOR_INFOTEXT: 'Windows 10 or greater: This value is not supported.' (GetSysColor, git 92672624, read 2026-10-11).
**Triage:** fix — As :1152.
**Status:** open

#### M2b-21 · bug · S2 · docs/platform-facts.md:1283
**Claim:** `overlay_mode` (2.8 Scrollbar): the cell says (none) — always persistent; G2 Windows 11 25H2: UISettings.AutoHideScrollBars: true if 'Automatically hide scroll bars in Windows' is set; WinUI 3 ScrollBar collapses to a panning indicator by default; G1 Windows 10 22H2: same API, 'Windows 10, version 1903 (introduced in 10.0.18362.0)'.
**Evidence:** [G2 Windows 11] https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.autohidescrollbars (git 3abc0b43, updated 2025-11-21, read 2026-10-11) [G1 Windows 10] same URL Note: Not on todo.md or in facts/reader. The setting is queryable on both generations.
**Verdict:** PLAUSIBLE — The UISettings.AutoHideScrollBars page (git 3abc0b43, updated 2025-11-21) confirms the setting ('Automatically hide scroll bars in Windows') from 10.0.18362 onwards, but its Remarks say it 'returns the specified user setting, not whether a specific scroll bar is visible', and the WinUI 3 'collapses to a panning indicator' part has no cited source; Win32 scroll bars are not shown to auto-hide, so whether the row applies depends on which scroll bar the Windows column models.
**Triage:** maintainer (Step 3 probe) — PLAUSIBLE: UISettings.AutoHideScrollBars exists (10.0.18362+), but it reports the user setting, not a scroll bar's behaviour; the cell's "(none)" is wrong either way, the value needs the probe.
**Status:** open

#### M2b-22 · bug · S2 · docs/platform-facts.md:1290
**Claim:** `track_color` (2.9 Slider): the cell says ← defaults.muted_color (TextFillColorSecondary); G2 Windows 11 WinUI 3 2.5.1: SliderTrackFill = ControlStrongFillColorDefaultBrush (L #72000000, D #8BFFFFFF).
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Slider_themeresources.xaml#L20 , #L124 ; Common_themeresources_any.xaml#L22 , #L226 (identical at 8463f45) Note: Inheritance cell contradicted by source. Not on todo.md.
**Verdict:** CONFIRMED — Read as cited; the track rectangle draws this brush through TemplateBinding Background. Confirmed value: SliderTrackFill = ControlStrongFillColorDefaultBrush (Slider_themeresources.xaml:20 Default, :124 Light; Slider Background setter :176 → HorizontalTrackRect Fill :411); ControlStrongFillColorDefault D….
**Triage:** fix — Read as cited; the track rectangle draws this brush through TemplateBinding Background.
**Status:** open

#### M2b-23 · bug · S2 · docs/platform-facts.md:1291
**Claim:** `thumb_color` (2.9 Slider): the cell says ← defaults.surface_color (CardBackgroundFillColorDefault); G2 Windows 11 WinUI 3 2.5.1: outer thumb SliderOuterThumbBackground = ControlSolidFillColorDefaultBrush (L #FFFFFF, D #454545); inner dot SliderThumbBackground = AccentFillColorDefaultBrush.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Slider_themeresources.xaml#L14 , #L19 , #L118 , #L123 , #L198-L199 ; Common_themeresources_any.xaml#L24 , #L228 (identical at 8463f45) Note: CardBackgroundFillColorDefault is L #B3FFFFFF / D #0DFFFFFF (translucent), not the thumb's solid fill.
**Verdict:** CONFIRMED — Read as cited; the outer-thumb/inner-dot split matches the template at :198-199. Confirmed value: Outer thumb Border Background = SliderOuterThumbBackground = ControlSolidFillColorDefaultBrush (Slider_themeresources.xaml:19, :123, :198); ControlSolidFillColorDefault D #454545 (Common_themeresource….
**Triage:** fix — Read as cited; the outer-thumb/inner-dot split matches the template at :198-199.
**Status:** open

#### M2b-24 · bug · S2 · docs/platform-facts.md:1302
**Claim:** `track_color` (2.10 Progress Bar): the cell says ← defaults.muted_color; G2 Windows 11 WinUI 3 2.5.1: track = ControlStrongStrokeColorDefault (per todo.md:779, ProgressBar_themeresources.xaml:7).
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressBar/ProgressBar_themeresources.xaml#L7 (identical at 8463f45) Note: Already on todo.md:769-783.
**Verdict:** CONFIRMED — The value holds at ba3a8d59 and 8463f451, but it is already on docs/todo.md:779, so this is not new. Confirmed value: ProgressBarBackground = ControlStrongStrokeColorDefault (ProgressBar_themeresources.xaml:7 Default, :23 Light; ProgressBar.xaml:5 setter → ProgressBarTrack Fill :158); ControlStrongStrokeColorDefault….
**Triage:** fix — CONFIRMED, already docs/todo.md:779.
**Status:** open

#### M2b-25 · bug · S2 · docs/platform-facts.md:1305
**Claim:** `border.corner_radius` (2.10 Progress Bar): the cell says ← defaults.border.corner_radius (4); G2 Windows 11 WinUI 3 2.5.1: ProgressBarCornerRadius 1.5, ProgressBarTrackCornerRadius 0.5.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressBar/ProgressBar_themeresources.xaml#L31-L32 (identical at 8463f45) Note: Already on todo.md:778-781. The Geometry page says bars use 4px (https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/geometry), conflicting with the source.
**Verdict:** CONFIRMED — ProgressBar_themeresources.xaml:31-32 at ba3a8d5 (identical file at 8463f45) defines them, and ProgressBar.xaml:12 (style CornerRadius) and :158/:163 (track and indicator RadiusX/Y) use them, so the row's inherited value of 4 is wrong. Confirmed value: WinUI 3: ProgressBarCornerRadius 1.5 (indicator), ProgressBarTrackCornerRadius 0.5 (track).
**Triage:** fix — ProgressBar_themeresources.xaml:31-32 at ba3a8d5 (identical file at 8463f45) defines them, and ProgressBar.xaml:12 (style CornerRadius) and :158/:163 (track and indicator RadiusX/Y) use them, so the row's inherited value of 4 is wrong.
**Status:** open

#### M2b-26 · bug · S2 · docs/platform-facts.md:1316
**Claim:** `min_width` (2.11 Tab Bar): the cell says (none) — sizes to label; G2 Windows 11 WinUI 3 2.5.1: default TabWidthMode is Equal, which clamps tabs to TabViewItemMinWidth 100 (max TabViewItemMaxWidth 240); G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: TabViewItemMinWidth 100.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView.idl#L105-L107 ; .../src/controls/dev/TabView/TabView.cpp#L1182 ; .../src/controls/dev/TabView/TabView_themeresources.xaml#L243-L244 (same lines at 8463f45 controls/dev/TabView/…) [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/TabView/TabView_themeresources_v1.xaml#L154 Note: todo.md:2277 lists tab.min_width as Windows (none) vs preset 64; the source gives 100 in the default mode (SizeToContent/Compact differ).
**Verdict:** CONFIRMED — TabView.idl:105-107 sets MUX_DEFAULT_VALUE Equal, TabView.cpp:1182 reads TabViewItemMinWidth in the Equal branch and std::clamp(…, minTabWidth, maxTabWidth) follows, TabView_themeresources.xaml:243-244 gives 240/100 (identical at 8463f45), and TabView_themeresources_v1.xaml:154 gives 100. Confirmed value: WinUI 3: TabWidthMode defaults to Equal; tab width = clamp(available/count, TabViewItemMinWidth 100, TabViewItemMaxWidth 240); WinUI 2.8.7 v1: TabViewItemMinWidth 100.
**Triage:** fix — TabView.idl:105-107 sets MUX_DEFAULT_VALUE Equal, TabView.cpp:1182 reads TabViewItemMinWidth in the Equal branch and std::clamp(…, minTabWidth, maxTabWidth) follows, TabView_themeresources.xaml:243-244 gives 240/100 (identical at 8463f45), and TabView_themeresources_v1.xaml:154 gives 100.
**Status:** open

#### M2b-27 · bug · S2 · docs/platform-facts.md:1336
**Claim:** `selection_background` (2.12 Sidebar): the cell says ← defaults.selection_background (COLOR_HIGHLIGHT); G2 Windows 11 WinUI 3 2.5.1: NavigationViewItemBackgroundSelected = SubtleFillColorSecondaryBrush, plus accent pill NavigationViewSelectionIndicatorForeground = AccentFillColorDefaultBrush.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/NavigationView/NavigationView_themeresources.xaml#L17 , #L83 , #L48 , #L114 (identical at 8463f45) Note: Inheritance contradicted. Not on todo.md.
**Verdict:** CONFIRMED — NavigationView_themeresources.xaml:17/:83 and :48/:114 hold these values, and the item template uses them (LayoutRoot.Background at :475, the SelectionIndicator Rectangle Fill at :602). Confirmed value: NavigationViewItemBackgroundSelected = SubtleFillColorSecondaryBrush; selection pill NavigationViewSelectionIndicatorForeground = AccentFillColorDefaultBrush.
**Triage:** fix — NavigationView_themeresources.xaml:17/:83 and :48/:114 hold these values, and the item template uses them (LayoutRoot.Background at :475, the SelectionIndicator Rectangle Fill at :602).
**Status:** open

#### M2b-28 · bug · S2 · docs/platform-facts.md:1337
**Claim:** `selection_text_color` (2.12 Sidebar): the cell says ← defaults.selection_text_color (COLOR_HIGHLIGHTTEXT); G2 Windows 11 WinUI 3 2.5.1: NavigationViewItemForegroundSelected = TextFillColorPrimaryBrush.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/NavigationView/NavigationView_themeresources.xaml#L29 , #L95 (identical at 8463f45)
**Verdict:** CONFIRMED — NavigationView_themeresources.xaml:29/:95 hold this value, and the Selected state applies it to the Icon and ContentPresenter foreground (:476-477). Confirmed value: NavigationViewItemForegroundSelected = TextFillColorPrimaryBrush.
**Triage:** fix — NavigationView_themeresources.xaml:29/:95 hold this value, and the Selected state applies it to the Icon and ContentPresenter foreground (:476-477).
**Status:** open

#### M2b-29 · bug · S2 · docs/platform-facts.md:1356
**Claim:** `icon_size` (2.13 Toolbar): the cell says ↕ 20 — see §2.1.8 (Fluent AppBarButton: 20); G2 Windows 11 WinUI 3 2.5.1: 16: AppBarButton ContentViewbox Height = AppBarButtonContentHeight = 16.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/AppBarButton_themeresources.xaml#L366 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/themes/generic.xaml#L19462 (identical blobs at 8463f45: controls/dev/CommonStyles/AppBarButton_themeresources.xaml#L366, dxaml/xcp/dxaml/themes/generic.xaml#L19462) Note: Also §2.1.8 :1132 ('Fluent AppBarButton: 20', out of scope). WinUI 2 takes the value from the OS generic.xaml (not in repo), so the Win10 value is not public. Not on todo.md.
**Verdict:** CONFIRMED — In WinUI 3's DefaultAppBarButtonStyle (the implicit style, AppBarButton_themeresources.xaml:120/:125), line 366 sets Viewbox Height to AppBarButtonContentHeight, which generic.xaml:19462 defines as 16 outside any theme dictionary; this refutes platform-facts.md:530's claim that 16 comes from legacy Reveal styles only (RevealBrush also says 16, at :563). Confirmed value: 16 (AppBarButton ContentViewbox Height = AppBarButtonContentHeight = 16).
**Triage:** fix — In WinUI 3's DefaultAppBarButtonStyle (the implicit style, AppBarButton_themeresources.xaml:120/:125), line 366 sets Viewbox Height to AppBarButtonContentHeight, which generic.xaml:19462 defines as 16 outside any theme dictionary; this refutes platform-facts.md:530's claim that 16 comes from legacy Reveal styles only (RevealBrush also says 16, at :563).
**Status:** open

#### M2b-30 · bug · S2 · docs/platform-facts.md:1383
**Claim:** `selection_background` (2.15 List/Table): the cell says ← defaults.selection_background (COLOR_HIGHLIGHT); G2 Windows 11 WinUI 3 2.5.1: ListViewItemBackgroundSelected = SubtleFillColorSecondaryBrush, plus ListViewItemSelectionIndicatorBrush = AccentFillColorDefaultBrush.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L20 , #L172 , #L75 , #L227 (identical at 8463f45) Note: The section otherwise cites WinUI 3 ListView (row height, padding, hover). Not on todo.md.
**Verdict:** CONFIRMED — ListViewItem_themeresources.xaml:20/:172 and :75/:227 hold these values, and ListViewItemPresenter at :256 binds SelectedBackground and SelectionIndicatorBrush to them, with the indicator enabled at :57/:209. Confirmed value: ListViewItemBackgroundSelected = SubtleFillColorSecondaryBrush; ListViewItemSelectionIndicatorBrush = AccentFillColorDefaultBrush (SelectionIndicatorVisualEnabled True).
**Triage:** fix — ListViewItem_themeresources.xaml:20/:172 and :75/:227 hold these values, and ListViewItemPresenter at :256 binds SelectedBackground and SelectionIndicatorBrush to them, with the indicator enabled at :57/:209.
**Status:** open

#### M2b-31 · bug · S2 · docs/platform-facts.md:1384
**Claim:** `selection_text_color` (2.15 List/Table): the cell says ← defaults.selection_text_color (COLOR_HIGHLIGHTTEXT); G2 Windows 11 WinUI 3 2.5.1: ListViewItemForegroundSelected = TextFillColorPrimaryBrush.
**Evidence:** [G2 Windows 11] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L26 , #L178 (identical at 8463f45)
**Verdict:** CONFIRMED — ListViewItem_themeresources.xaml:26/:178 hold this value, and the presenter binds SelectedForeground to it (:256). Confirmed value: ListViewItemForegroundSelected = TextFillColorPrimaryBrush.
**Triage:** fix — ListViewItem_themeresources.xaml:26/:178 hold this value, and the presenter binds SelectedForeground to it (:256).
**Status:** open

#### M2b-32 · bug · S2 · docs/platform-facts.md:1405
**Claim:** `background_color` (2.16 Popover): the cell says (Fluent) Flyout bg = defaults.surface_color; G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8463f45 = main 48df672, same blobs): FlyoutPresenterBackground = AcrylicInAppFillColorDefaultBrush: Light AcrylicBrush TintColor #FCFCFC, TintOpacity 0, FallbackColor #F9F9F9, TintLuminosityOpacity 0.85; Dark TintColor #2C2C2C, TintOpacity 0.15, FallbackColor #2C2C2C, TintLuminosityOpacity 0.96; G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: FlyoutPresenterBackground = AcrylicBackgroundFillColorDefaultBrush (contract7) / SystemControlBackgroundChromeMediumLowBrush (pre-contract7).
**Evidence:** [G2 Windows 11 (WinUI 3)] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/FlyoutPresenter_themeresources.xaml#L5-L15 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Materials/Acrylic/AcrylicBrush_themeresources.xaml#L44 (Dark), #L96 (Light); same at https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/FlyoutPresenter_th… [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/CommonStyles/FlyoutPresenter_themeresources_v1.xaml#L10-L11 Note: defaults.surface_color for Windows is CardBackgroundFillColorDefault (facts :1050; L #B3FFFFFF, D #0DFFFFFF), not the flyout's acrylic brush. Not on todo.md (todo :919-924 is a gpui menu-surface item, not this).
**Verdict:** CONFIRMED — FlyoutPresenter_themeresources.xaml:5/:15 (byte-identical at 8463f45), AcrylicBrush_themeresources.xaml:44 (in the Default/Dark dictionary) and :96 (Light), and FlyoutPresenter_themeresources_v1.xaml:10-11 match the claim. Confirmed value: FlyoutPresenterBackground = AcrylicInAppFillColorDefaultBrush (Light: Tint #FCFCFC, TintOpacity 0, Fallback #F9F9F9, TintLuminosityOpacity 0.85; Dark: Tint #2C2C2C, TintOpacity 0.15, Fallback #2C2C2C,….
**Triage:** fix — FlyoutPresenter_themeresources.xaml:5/:15 (byte-identical at 8463f45), AcrylicBrush_themeresources.xaml:44 (in the Default/Dark dictionary) and :96 (Light), and FlyoutPresenter_themeresources_v1.xaml:10-11 match the claim.
**Status:** open

#### M2b-33 · bug · S2 · docs/platform-facts.md:1473
**Claim:** `thumb_background` (2.21 Switch): the cell says (Fluent) ToggleSwitchKnob; G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: no resource named ToggleSwitchKnob; checked knob = ToggleSwitchKnobFillOn = TextOnAccentFillColorPrimaryBrush (L #FFFFFF, D #000000); G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: ToggleSwitchKnobFillOn = SystemControlHighlightAltChromeWhiteBrush.
**Evidence:** [G2 Windows 11 (WinUI 3)] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L35 , #L155 ; Common_themeresources_any.xaml #L12 (D), #L216 (L) at the same revision [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/CommonStyles/ToggleSwitch_themeresources_v1.xaml#L40 Note: Naming only: cell names a non-existent key; :1476 already states the right one.
**Verdict:** CONFIRMED — ToggleSwitch_themeresources.xaml:35/:155 and Common_themeresources_any.xaml:12 (Default=Dark) and :216 (Light) match the claim, no x:Key "ToggleSwitchKnob" exists, ToggleSwitch_themeresources_v1.xaml:40 matches, and the row is the checked knob because unchecked_thumb_background is a separate row. Confirmed value: No resource named ToggleSwitchKnob; checked knob ToggleSwitchKnobFillOn = TextOnAccentFillColorPrimaryBrush (L #FFFFFF, D #000000); WinUI 2.8.7: SystemControlHighlightAltChromeWhiteBrush.
**Triage:** fix — ToggleSwitch_themeresources.xaml:35/:155 and Common_themeresources_any.xaml:12 (Default=Dark) and :216 (Light) match the claim, no x:Key "ToggleSwitchKnob" exists, ToggleSwitch_themeresources_v1.xaml:40 matches, and the row is the checked knob because unchecked_thumb_background is a separate row.
**Status:** open

#### M2b-34 · bug · S2 · docs/platform-facts.md:1495
**Claim:** `title_font.family` (2.22 Dialog): the cell says ← defaults.font (Segoe UI); G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: FontFamily = ContentControlThemeFontFamily = XamlAutoFontFamily; XAML common controls select Segoe UI Variable by default.
**Evidence:** [G2 Windows 11 (WinUI 3)] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L238 ; https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/themes/generic.xaml#L14 ; https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/typography (git 7814a981, updated 2026-07-14) Note: Only the parenthetical differs (Segoe UI vs Segoe UI Variable); systemic to every WinUI text cell that inherits defaults.font.
**Verdict:** CONFIRMED — ContentDialog_themeresources.xaml:238 and generic.xaml:14 hold these values, and the Learn typography page (git 7814a981, updated 2026-07-14, read 2026-10-11) says 'When using XAML common controls, the Segoe UI Variable font will be selected by default for supported languages'. Confirmed value: ContentDialog Title FontFamily = ContentControlThemeFontFamily = XamlAutoFontFamily; XAML common controls select Segoe UI Variable by default (for supported languages).
**Triage:** fix — ContentDialog_themeresources.xaml:238 and generic.xaml:14 hold these values, and the Learn typography page (git 7814a981, updated 2026-07-14, read 2026-10-11) says 'When using XAML common controls, the Segoe UI Variable font will be selected by default for supported languages'.
**Status:** open

#### M2b-35 · bug · S2 · docs/platform-facts.md:1545
**Claim:** `background_color` (2.24 ComboBox): the cell says COLOR_BTNFACE; G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8463f45, blob 4578f4c9): ComboBoxBackground = ControlFillColorDefaultBrush: L #B3FFFFFF, D #0FFFFFFF; Win32 (cell's source) GetSysColor doc, updated 2024-02-22 (git 92672624): COLOR_BTNFACE (15) marked 'Windows 10 or greater: This value is not supported'; alias COLOR_3DFACE (15) listed as supported; G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: SystemControlBackgroundAltMediumLowBrush.
**Evidence:** [G2 Windows 11 (WinUI 3)] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L32 , #L242 ; Common_themeresources_any.xaml #L15 (D), #L219 (L) same revision [Win32 (cell's source)] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/ComboBox/ComboBox_themeresources_v1.xaml#L54 Note: Every other geometry row of §2.24 is WinUI 3; this one names a Win32 colour no public source ties to the themed combobox (uxtheme drawing is not public). The BTNFACE/3DFACE doc inconsistency is already recorded (facts :357-358, :1714).
**Verdict:** CONFIRMED — ComboBox_themeresources.xaml:32/:242 (identical at 8463f45), Common_themeresources_any.xaml:15/:219 and ComboBox_themeresources_v1.xaml:54 match, and the GetSysColor page (updated 2024-02-22) reads as claimed; because both names are index 15, the Win32 point is a renaming to COLOR_3DFACE, not a different colour. Confirmed value: WinUI 3 ComboBoxBackground = ControlFillColorDefaultBrush (L #B3FFFFFF, D #0FFFFFFF); WinUI 2.8.7: SystemControlBackgroundAltMediumLowBrush; Win32 doc marks COLOR_BTNFACE 'Windows 10 or greater: not s….
**Triage:** fix — CONFIRMED: WinUI ComboBox uses ControlFillColorDefault, and COLOR_BTNFACE is the unsupported alias of COLOR_3DFACE (index 15).
**Status:** open

#### M2b-36 · bug · S2 · docs/platform-facts.md:1547
**Claim:** `font.color` (2.24 ComboBox): the cell says COLOR_BTNTEXT; G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: ComboBoxForeground = TextFillColorPrimaryBrush: L #E4000000, D #FFFFFF; Win32 (cell's source) GetSysColor doc, updated 2024-02-22: COLOR_BTNTEXT (18), supported on Windows 10+: 'Text on push buttons'; G1 Windows 10 (WinUI 2 Version1) WinUI 2.8.7: SystemControlForegroundBaseHighBrush.
**Evidence:** [G2 Windows 11 (WinUI 3)] https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L40 , #L250 ; Common_themeresources_any.xaml #L5 (D), #L209 (L) same revision [Win32 (cell's source)] https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) [G1 Windows 10 (WinUI 2 Version1)] https://github.com/microsoft/microsoft-ui-xaml/blob/232a16e5ddfc22c9a1b79a2c51abeb9a39a94494/dev/ComboBox/ComboBox_themeresources_v1.xaml#L62 Note: Same Win32-vs-WinUI split as :1545.
**Verdict:** CONFIRMED — ComboBox_themeresources.xaml:40/:250, Common_themeresources_any.xaml:5 (Default=Dark) and :209 (Light), and ComboBox_themeresources_v1.xaml:62 match, and the GetSysColor page lists COLOR_BTNTEXT without the 'not supported' note. Confirmed value: WinUI 3 ComboBoxForeground = TextFillColorPrimaryBrush (L #E4000000, D #FFFFFF); WinUI 2.8.7: SystemControlForegroundBaseHighBrush; COLOR_BTNTEXT (18) supported on Win10+.
**Triage:** fix — ComboBox_themeresources.xaml:40/:250, Common_themeresources_any.xaml:5 (Default=Dark) and :209 (Light), and ComboBox_themeresources_v1.xaml:62 match, and the GetSysColor page lists COLOR_BTNTEXT without the 'not supported' note.
**Status:** open

### Step 4 — preset-validator

67 findings: 5 mismatch, 1 no-citation, 6 other, 55 depends-on-differs-row.

<details><summary>Validator summary</summary>

I checked native-theme/src/presets/windows-11.toml and windows-11-live.toml against platform-facts.md, property-registry.toml and inheritance-rules.toml, read-only, in /home/tibi/Rust/native-theme-m2. Their geometry values agree with each other line for line. Every key used is in the registry, and the icon set (segoe-fluent) suits a Windows preset.

All 34 facts rows that Step 2 reported as differing reach at least one preset value. In most cases the preset matches the facts row as it stands now, so if a researcher's claim is confirmed, the preset value goes stale along with the row. Some values are set directly in the preset: the status text colours, focus ring colour and offset, toolbar icon size, inactive title bar colours, button hover, tooltip colours, overlay_mode, tab min_width, popover background and switch thumb. Others come in through inheritance-rules.toml: placeholder, menu background and text colour, title bar text colour, tooltip font size, slider and progress-bar track and thumb, progress-bar corner radius, sidebar and list selection, toolbar icon size, combo_box background and text colour, and dialog title font family. For those I cite the preset line of the source key that feeds the value.

The live preset carries no colours. Its colours come from native-theme/src/windows.rs, which reads seven GetSysColor indices that the researchers report as unsupported on Windows 10 and later: CAPTIONTEXT, INACTIVECAPTION, INACTIVECAPTIONTEXT, MENU, MENUTEXT, INFOBK and INFOTEXT. It also reads BTNFACE (index 15), which the researchers report unsupported too, though its alias COLOR_3DFACE is listed as supported. I report these reader lines as kind "other".

Problems found that are not differing rows, all blocking under the "match the facts and carry a source" rule:
- `border.opacity` = 0.14 is a value platform-facts.md:1116 says has no source.
- `icon_sizes.dialog_px` (22) and `icon_sizes.panel_px` (20) have no source; the Windows cells are "(none)" (platform-facts.md:1135-1136).
- `progress_bar.min_width_px` = 100 and `button.min_width_px` = 64 are set where the facts say there is no minimum.
- `button.min_height_px` = 32, where the facts row says 27.
- `scrollbar.thumb_width_px` = 6, where the facts say SM_CXVSCROLL, the same metric as the 17px groove.
- `menu.icon_size` resolves to 20 by inheriting the toolbar size, where the facts say 16.
- `tab.min_width_px` = 64, which matches neither the facts row ("none") nor the researcher's 100.
- The dark `switch.thumb_background` (#ffffff) contradicts platform-facts.md:1476, which gives the checked knob as #000000 in dark.
- `combo_box.background_color` inherits `defaults.background_color` (inheritance-rules.toml:257), not the COLOR_BTNFACE the facts row names.
- platform-facts.md:436 (12px) contradicts platform-facts.md:1390 (16/12). The preset follows :1390, which is also what the researcher found.

I did not suggest replacement values anywhere. Each one needs the maintainer to confirm the source first.

</details>

#### M2b-P01 · bug · S2 · native-theme/src/windows.rs:466
**Claim:** `variant.defaults.focus_ring_color (reader, feeds windows-11-live)` (other): preset accent, facts UISettings(Accent) (docs/platform-facts.md:1104).
**Evidence:** windows-11-live.toml declares no focus_ring_color; the OS reader sets it to the accent. That follows the differing row, and would need to change with it if the researcher's FocusStrokeColorOuter claim is confirmed.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** M4 — the reader follows the facts row M2b-05 corrects; changes with it.
**Status:** open

#### M2b-P02 · bug · S2 · native-theme/src/windows.rs:331
**Claim:** `window.title_bar_font.color (reader; caption_text read at :312)` (other): preset GetSysColor(COLOR_CAPTIONTEXT), facts COLOR_CAPTIONTEXT (docs/platform-facts.md:1152).
**Evidence:** The live merge base (windows-11-live.toml) takes this colour from the reader, which reads an index the researcher reports as 'Windows 10 or greater: This value is not supported.'
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** M4 — the reader reads GetSysColor indices Microsoft marks unsupported on Windows 10+ (M2b-11…20); what it reads instead is M4's.
**Status:** open

#### M2b-P03 · bug · S2 · native-theme/src/windows.rs:328
**Claim:** `input.placeholder_color (reader, feeds windows-11-live)` (other): preset GetSysColor(COLOR_GRAYTEXT), facts (Fluent) TextPlaceholderColor (docs/platform-facts.md:1191).
**Evidence:** The reader's source matches neither the facts row (TextPlaceholderColor) nor the researcher (TextFillColorSecondaryBrush). This is a mismatch already, regardless of the differing claim.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — the reader's COLOR_GRAYTEXT matches neither the facts nor WinUI (TextFillColorSecondary, M2b-15); with M2b-15 in Step 7, the reader in M4.
**Status:** open

#### M2b-P04 · bug · S2 · native-theme/src/windows.rs:322
**Claim:** `menu.background_color / menu.font.color (reader :322-323; read at :304-305)` (other): preset GetSysColor(COLOR_MENU) / GetSysColor(COLOR_MENUTEXT), facts COLOR_MENU / COLOR_MENUTEXT (docs/platform-facts.md:1227, :1233).
**Evidence:** The live merge base gets its menu colours from indices the researcher reports unsupported on Windows 10+.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** M4 — as M2b-P02.
**Status:** open

#### M2b-P05 · bug · S2 · native-theme/src/windows.rs:324
**Claim:** `tooltip.background_color / tooltip.font.color (reader :324-325; read at :306-307)` (other): preset GetSysColor(COLOR_INFOBK) / GetSysColor(COLOR_INFOTEXT), facts COLOR_INFOBK / COLOR_INFOTEXT (docs/platform-facts.md:1251, :1256).
**Evidence:** The live merge base gets its tooltip colours from indices the researcher reports unsupported on Windows 10+.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** M4 — as M2b-P02.
**Status:** open

#### M2b-P06 · bug · S2 · native-theme/src/presets/windows-11.toml:80
**Claim:** `light.button.background_color` (other): preset #fdfdfd, facts COLOR_BTNFACE (docs/platform-facts.md:1164).
**Evidence:** Not in the differing list, but it is the same COLOR_BTNFACE cell the researcher flagged at :1545: BTNFACE is marked unsupported on Windows 10+, while its alias COLOR_3DFACE (also index 15) is listed as supported. The reader reads COLOR_BTNFACE (windows.rs:302, applied at :320). The same applies to dark line 488 ('#333333'). Neither hex value has a source comment.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — the §2.3 cell names COLOR_BTNFACE, the unsupported alias of the supported COLOR_3DFACE (same index 15); rename in Step 7 (S4).
**Status:** open

#### M2b-P07 · bug · S2 · native-theme/src/presets/windows-11.toml:45
**Claim:** `light.defaults.border.opacity` (mismatch): preset 0.14, facts (none) — no multiplier … the preset's 0.14 has no source (docs/platform-facts.md:1116).
**Evidence:** The facts row states outright that this value has no source. The same key is at dark line 453 and at windows-11-live.toml lines 22 and 242.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — docs/todo.md:605 (*Border opacity*) owns `defaults.border.opacity`; the uncited 0.14 is added to that entry.
**Status:** open

#### M2b-P08 · bug · S2 · native-theme/src/presets/windows-11.toml:61
**Claim:** `light.defaults.icon_sizes.dialog_px / panel_px (lines 61-62)` (no-citation): preset 22.0 / 20.0, facts dialog: (none); panel: (none) (docs/platform-facts.md:1135-1136).
**Evidence:** The Windows cells are '(none)'; the values have no source. The same keys are at dark lines 469-470 and at windows-11-live.toml lines 28-29 and 248-249.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2297 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2b-P09 · bug · S2 · native-theme/src/presets/windows-11.toml:192
**Claim:** `light.progress_bar.min_width_px` (mismatch): preset 100.0, facts (none) — no minimum (docs/platform-facts.md:1304).
**Evidence:** Uncited minimum where the facts say there is none. The same key is at dark line 600 and at windows-11-live.toml lines 101 and 321.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2276 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2b-P10 · bug · S2 · native-theme/src/presets/windows-11.toml:81
**Claim:** `light.button.min_width_px / min_height_px (lines 81-82)` (mismatch): preset 64.0 / 32.0, facts min_width (none) — sizes to content; min_height WinUI3: 27 (derived) (docs/platform-facts.md:1169-1170).
**Evidence:** Both values differ from the facts rows and have no citation. The same keys are at dark lines 489-490 and at windows-11-live.toml lines 47-48 and 267-268.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2266 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2b-P11 · bug · S2 · native-theme/src/presets/windows-11.toml:168
**Claim:** `light.scrollbar.thumb_width_px` (mismatch): preset 6.0, facts ↕ SM_CXVSCROLL (same) (docs/platform-facts.md:1282).
**Evidence:** The facts make the thumb width equal to SM_CXVSCROLL, the same metric as groove_width_px (17 in this preset); 6 has no source. The same key is at dark line 576 and at windows-11-live.toml lines 91 and 311.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — already docs/todo.md:2273 (windows-11 6 against the API value).
**Status:** open

#### M2b-P12 · bug · S2 · native-theme/src/presets/windows-11.toml:215
**Claim:** `light.menu.icon_size_px (undeclared; inherits defaults.icon_sizes.toolbar_px = 20)` (mismatch): preset (inherited) 20.0, facts ↕ SM_CXSMICON: 16 (docs/platform-facts.md:1242).
**Evidence:** inheritance-rules.toml:188 makes menu.icon_size inherit the toolbar size, so the Windows menu icon resolves to 20 where the facts say 16. Any change to toolbar_px (differing rows :530, :1132) also moves this value. The same applies to [dark.menu] (line 623) and both live variants.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — the menu icon resolves to the toolbar size (20) where §2.6 states SM_CXSMICON 16; state `menu.icon_size_px = 16` in windows-11 and its -live twin, citing platform-facts.md:1242 (S2).
**Status:** open

<details><summary>Preset keys that depend on a <i>differs</i> row</summary>

| Preset:line | Key | Preset value | Facts row |
|---|---|---|---|
| windows-11.toml:18 | `light.defaults.danger_text_color` | #ffffff | docs/platform-facts.md:1081 |
| windows-11.toml:426 | `dark.defaults.danger_text_color` | #1a1a1a | docs/platform-facts.md:1081 |
| windows-11.toml:20 | `light.defaults.warning_text_color` | #1a1a1a | docs/platform-facts.md:1083 |
| windows-11.toml:428 | `dark.defaults.warning_text_color` | #1a1a1a | docs/platform-facts.md:1083 |
| windows-11.toml:22 | `light.defaults.success_text_color` | #ffffff | docs/platform-facts.md:1085 |
| windows-11.toml:430 | `dark.defaults.success_text_color` | #1a1a1a | docs/platform-facts.md:1085 |
| windows-11.toml:24 | `light.defaults.info_text_color` | #ffffff | docs/platform-facts.md:1087 |
| windows-11.toml:432 | `dark.defaults.info_text_color` | #1a1a1a | docs/platform-facts.md:1087 |
| windows-11.toml:29 | `light.defaults.focus_ring_color` | #0078d4 | docs/platform-facts.md:1104 |
| windows-11.toml:437 | `dark.defaults.focus_ring_color` | #0078d4 | docs/platform-facts.md:1104 |
| windows-11.toml:37 | `light.defaults.focus_ring_offset_px` | 0.0 | docs/platform-facts.md:1106 |
| windows-11.toml:445 | `dark.defaults.focus_ring_offset_px` | 0.0 | docs/platform-facts.md:1106 |
| windows-11-live.toml:16 | `light.defaults.focus_ring_offset_px` | 0.0 | docs/platform-facts.md:1106 |
| windows-11-live.toml:236 | `dark.defaults.focus_ring_offset_px` | 0.0 | docs/platform-facts.md:1106 |
| windows-11.toml:244 | `light.list.border.padding_left_px / padding_right_px (lines 244-245)` | 16.0 / 12.0 | docs/platform-facts.md:436 |
| windows-11.toml:652 | `dark.list.border.padding_left_px / padding_right_px (lines 652-653)` | 16.0 / 12.0 | docs/platform-facts.md:436 |
| windows-11-live.toml:131 | `light.list.border.padding_left_px / padding_right_px (lines 131-132)` | 16.0 / 12.0 | docs/platform-facts.md:436 |
| windows-11-live.toml:351 | `dark.list.border.padding_left_px / padding_right_px (lines 351-352)` | 16.0 / 12.0 | docs/platform-facts.md:436 |
| windows-11.toml:58 | `light.defaults.icon_sizes.toolbar_px` | 20.0 | docs/platform-facts.md:530, :1132, :1356 |
| windows-11.toml:466 | `dark.defaults.icon_sizes.toolbar_px` | 20.0 | docs/platform-facts.md:530, :1132, :1356 |
| windows-11-live.toml:25 | `light.defaults.icon_sizes.toolbar_px` | 20.0 | docs/platform-facts.md:530, :1132, :1356 |
| windows-11-live.toml:245 | `dark.defaults.icon_sizes.toolbar_px` | 20.0 | docs/platform-facts.md:530, :1132, :1356 |
| windows.rs:536 | `AccessibilityPreferences.reduce_transparency (reader, both presets)` | default (not read): '// reduce_transparency keeps default --… | docs/platform-facts.md:1126 |
| windows-11.toml:12 | `light.defaults.text_color (feeds window.title_bar_font.color, menu.font.color, c…` | #1a1a1a | docs/platform-facts.md:1152, :1233, :1547 |
| windows-11.toml:420 | `dark.defaults.text_color (feeds window.title_bar_font.color, menu.font.color, co…` | #ffffff | docs/platform-facts.md:1152, :1233, :1547 |
| windows-11.toml:411 | `light.window.inactive_title_bar_background` | #f1f1f1 | docs/platform-facts.md:1153 |
| windows-11.toml:819 | `dark.window.inactive_title_bar_background` | #2b2b2b | docs/platform-facts.md:1153 |
| windows-11.toml:413 | `light.window.inactive_title_bar_text_color` | #1a1a1a99 | docs/platform-facts.md:1154 |
| windows-11.toml:821 | `dark.window.inactive_title_bar_text_color` | #ffffff99 | docs/platform-facts.md:1154 |
| windows-11.toml:86 | `light.button.hover_background` | #0000000a | docs/platform-facts.md:1179 |
| windows-11.toml:494 | `dark.button.hover_background` | #ffffff0f | docs/platform-facts.md:1179 |
| windows-11.toml:375 | `light.segmented_control.hover_background` | #0000000a | docs/platform-facts.md:1179 |
| windows-11.toml:783 | `dark.segmented_control.hover_background` | #ffffff0f | docs/platform-facts.md:1179 |
| windows-11.toml:14 | `light.defaults.muted_color (feeds input.placeholder_color, slider.track_color, p…` | #717171 | docs/platform-facts.md:1191, :1290, :1302 |
| windows-11.toml:422 | `dark.defaults.muted_color (feeds input.placeholder_color, slider.track_color, pr…` | #9a9a9a | docs/platform-facts.md:1191, :1290, :1302 |
| windows-11.toml:11 | `light.defaults.background_color (feeds menu.background_color, combo_box.backgrou…` | #f3f3f3 | docs/platform-facts.md:1227, :1545 |
| windows-11.toml:419 | `dark.defaults.background_color (feeds menu.background_color, combo_box.backgroun…` | #202020 | docs/platform-facts.md:1227, :1545 |
| windows-11.toml:153 | `light.tooltip.background_color` | #2c2c2c | docs/platform-facts.md:1251 |
| windows-11.toml:561 | `dark.tooltip.background_color` | #3a3a3a | docs/platform-facts.md:1251 |
| windows-11.toml:164 | `light.tooltip.font.color` | #ffffff | docs/platform-facts.md:1256 |
| windows-11.toml:572 | `dark.tooltip.font.color` | #ffffff | docs/platform-facts.md:1256 |
| windows-11.toml:49 | `light.defaults.font.size_pt (feeds tooltip.font.size)` | 10.5 | docs/platform-facts.md:1253 |
| windows-11.toml:169 | `light.scrollbar.overlay_mode` | false | docs/platform-facts.md:1283 |
| windows-11-live.toml:92 | `light.scrollbar.overlay_mode` | false | docs/platform-facts.md:1283 |
| windows-11.toml:13 | `light.defaults.surface_color (feeds slider.thumb_color, popover via per_platform…` | #ffffff | docs/platform-facts.md:1291 |
| windows-11.toml:421 | `dark.defaults.surface_color (feeds slider.thumb_color)` | #2d2d2d | docs/platform-facts.md:1291 |
| windows-11.toml:42 | `light.defaults.border.corner_radius_px (feeds progress_bar.border.corner_radius)` | 4.0 | docs/platform-facts.md:1305 |
| windows-11.toml:199 | `light.tab.min_width_px` | 64.0 | docs/platform-facts.md:1316 |
| windows-11.toml:26 | `light.defaults.selection_background (feeds sidebar.selection_background, list.se…` | #0078d4 | docs/platform-facts.md:1336, :1383 |
| windows-11.toml:27 | `light.defaults.selection_text_color (feeds sidebar.selection_text_color, list.se…` | #ffffff | docs/platform-facts.md:1337, :1384 |
| windows-11.toml:270 | `light.popover.background_color` | #ffffff | docs/platform-facts.md:1405 |
| windows-11.toml:678 | `dark.popover.background_color` | #2d2d2d | docs/platform-facts.md:1405 |
| windows-11.toml:293 | `light.switch.thumb_background` | #ffffff | docs/platform-facts.md:1473 |
| windows-11.toml:701 | `dark.switch.thumb_background` | #ffffff | docs/platform-facts.md:1473 |
| windows-11.toml:48 | `light.defaults.font.family (feeds dialog.title_font.family)` | Segoe UI | docs/platform-facts.md:1495 |

</details>

### Unverifiable rows

No primary source found: the row keeps or gets a ❓ in Step 7; no value is proposed. *Probe* = a probe or the reference app could measure it.

<details><summary>39 unverifiable rows</summary>

| Line | Property | Cell | Probe | Note |
|---|---|---|---|---|
| 285 | `lfMessageFont typical default` | Segoe UI, lfHeight=-12, 400; ✅ face+size via Win32 UX Guide | yes | The cited UX Guide says 'This design guide was created for Windows 7 and has not been updated for newer versions of Windows' and its table is introduced as 'defaults for Windows Vista and Windows 7' (https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-… |
| 286 | `lfCaptionFont typical default` | Segoe UI, -12, 400 (Win10) / 700 (Win11); Microsoft Q&A | yes | The Q&A thread (learn.microsoft.com/answers/questions/5489781, 2025-07-23) is a forum answer, so secondary. It says only that the title bar text 'appears bold' in Windows 11. It does not say that lfCaptionFont.lfWeight returns 700, and a bold title-bar renderi… |
| 287 | `lfSmCaptionFont typical default` | Segoe UI, -12, 400 | yes | Same as line 285: the only cited source is the Vista/7 UX Guide. |
| 288 | `lfMenuFont typical default` | Segoe UI, -12, 400 | yes | Same as line 285. |
| 289 | `lfStatusFont typical default` | Segoe UI, -12, 400; role 'Status bar text' | yes | Same as line 285. NONCLIENTMETRICSW says lfStatusFont is the font 'used in status bars and tooltips', so the role column could add tooltips. |
| 291 | `Segoe UI Variable in XAML; Win32 always returns Segoe UI` | Win11 XAML uses Segoe UI Variable; SystemParametersInfoW always returns 'Segoe UI' | yes | The first half is confirmed by the doc. The second half ('always returns Segoe UI') rests only on Mozilla bug 1732404 and VS Code #156766 (appendix :1709), which are secondary; no primary source was found. |
| 370 | `DwmGetColorizationColor` | window chrome / title bar color ✅ | yes | The API exists on Windows 10 and 11. The doc names the glass composition colour, not the title bar colour, so the role claim (the reader uses it for the title bar, windows.rs:335-342) has no primary source. |
| 1057 | `selection_inactive_background` | (none) — reduced emphasis / COLOR_BTNFACE | yes | The cell's fallback, COLOR_BTNFACE, is marked 'Windows 10 or greater: This value is not supported' (getsyscolor doc, updated 2024-02-22); the alias COLOR_3DFACE is not marked. No primary source states an unfocused-selection colour. The ❓ should stay. |
| 378 | `SM_CXBORDER` | ↕ 1px ✅ citing NONCLIENTMETRICSW iBorderWidth 'default is 1 pixel' | yes | Citation is to the wrong metric: NONCLIENTMETRICSW iBorderWidth is 'the thickness of the sizing border' (ns-winuser-nonclientmetricsw, git a9040b7b, 2024-11-20), i.e. SM_CXSIZEFRAME ('thickness of the sizing border around the perimeter of a window that can be… |
| 379 | `SM_CYBORDER` | ↕ 1px ✅ same source | yes | Same mis-citation as :378 (iBorderWidth = sizing border). |
| 380 | `SM_CXVSCROLL` | ↕ 17px ✅ via .NET SystemParameters.VerticalScrollBarWidth docs + measurements | yes | The cited .NET doc does not contain 17. Registry ScrollWidth=-255 appears only in community guides (secondary). Keep ❓; probe GetSystemMetricsForDpi(SM_CXVSCROLL,96) on 10 and 11. |
| 381 | `SM_CYHSCROLL` | ↕ 17px ✅ same | yes | As :380. |
| 382 | `SM_CYVTHUMB` | ↕ 17px ❓ (registry ScrollHeight derivation) | yes | ❓ is right. Related, already on todo: docs/todo.md:2274 (windows-11 17 vs reader's testable build 40). |
| 383 | `SM_CYMENU` | ↕ 20px ✅ registry MenuHeight -285 + Wine iMenuHeight+1 | yes | Wine (a reimplementation) and Winaero/flylib are secondary; the meaning quote is confirmed, the 20 is not. Downgrade ✅ to ❓. |
| 384 | `SM_CXFOCUSBORDER` | ↕ 1px ✅ ReactOS and Wine defaults | yes | ReactOS/Wine are secondary for Windows' own value. Downgrade to ❓. |
| 385 | `SM_CYFOCUSBORDER` | ↕ 1px ✅ same | yes | As :384. |
| 409 | `Button min height` | ~27px ❓ (14 text + 5+6 + 2 border) | yes | Arithmetic uses font size 14 as content height; the content box is the text line height, so 27 is not derivable from source. Rendered height needs a probe/screenshot. Already on todo: docs/todo.md:2267 (27 vs windows-11/reader 32). |
| 412 | `Button icon spacing` | 8px ❓ DropDownButton Margin=8,0,0,0 |  | The quoted margin is confirmed; WinUI Button has no icon+text layout, so the mapping stays ❓. |
| 418 | `Scrollbar (Win32) width` | ↕ 17px ✅ see §1.2.3 | yes | Inherits :380's missing primary source. |
| 419 | `Scrollbar (Win32) thumb height` | ↕ 17px ❓ | yes | As :382; todo :2274. |
| 420 | `Scrollbar (WinUI3) collapsed width` | ~2px ❓ 'XAML template inline value' | yes | The cell's 'no named theme resource' is wrong (resources above). 2px visible follows only if the 6px transparent stroke insets the fill by 3 per side — a rendering rule not cited; a screenshot settles it. Only ScrollBar_themeresources_perf2026 (opt-in) changed… |
| 421 | `Scrollbar (WinUI3) expanded width` | ~6px ❓ | yes | As :420: 6 = 12 − 2×3 is a derivation needing the stroke rule; ScrollBarSize=12 is the named resource the cell says doesn't exist. |
| 430 | `Menu item height` | ↕ ~20px (Win32) ❓; WinUI3 touch ~31 / narrow ~23 | yes | Win32 formula is from Wine/ReactOS (secondary; links unpinned, ReactOS one is a personal fork nicknisi/reactos). WinUI totals add font size 14, not the line height. Keep ❓. |
| 1147 | `title_bar_background` | DwmGetColorizationColor | yes | Whether the visible caption uses the colorization colour (user 'accent on title bars' setting) has no primary source; Win11 adds DWMWA_CAPTION_COLOR ('supported starting with Windows 11 Build 22000', https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/n… |
| 1150 | `title_bar_font.weight` | lfCaptionFont.lfWeight (varies; see §1.2.1: Win10 400 / Win11 700) | yes | Field is right; the per-version weights :286 relies on are not primary-sourced. Keep ❓ on the weights. |
| 1170 | `min_height` | WinUI3: 27 (derived: 14 text + 5+6 pad + 2 border) | yes | Derivation adds the font size (14) where the content height is the text line box (> 14), so 27 undercounts; no source states the total. Already on todo.md:2267 (27 vs preset/reader 32). |
| 1174 | `icon_text_gap` | WinUI3: 8 |  | 8 is a content-to-chevron gap of DropDownButton (appendix :1747), not an icon-label gap of Button; no WinUI resource for the latter. |
| 1192 | `caret_color` | foreground (system default) | yes | No primary source for the caret colour. |
| 1234 | `row_height` | WinUI3: touch 31 (14+8+9), mouse 23 (14+4+5), derived | yes | Same font-size-for-line-height derivation flaw as :1170; totals not source-stated. Preset/reader 23 recorded at todo.md:2270. |
| 1277 | `track_color` | transparent | yes | Unsourced; keep ❓. |
| 1278 | `thumb_color` | #c2c2c2 (measured) | yes | Measurement names no OS version. Preset gap on todo.md:372-377. |
| 1279 | `thumb_hover_color` | #a0a0a0 (measured) | yes |  |
| 1282 | `thumb_width` | ↕ SM_CXVSCROLL (same) | yes | Preset 6 vs API recorded at todo.md:2273. |
| 1382 | `alternate_row_background` | (Fluent) preset L #f9f9f9 D #262626 |  | Tagged (Fluent) but the values are the preset's own; no primary source. Retag (preset) ❓. |
| 1387 | `header_background` | (Fluent) ≈ defaults.background_color | yes | '≈' with no source. Note main has a new TableView/TabularSurfaces set (Step 1 map) not in a release yet. |
| 1590 | `border.corner_radius` | 8px (OverlayCornerRadius) | yes | No Card control; geometry doc (git dea23108) gives 8px to windows, flyouts, dialogs and 4px to in-page elements, and names no card radius. Keep with a ❓. |
| 1591 | `border.shadow_enabled` | (none) — border only | yes | Layering doc lists 'Card Elevation value: 8, Stroke width: 1' and says elevation is expressed 'with shadow and contour', but no Card control or template exists to show whether a shadow is drawn. Keep with a ❓. |
| 1592 | `border.padding_horizontal` | 12px (convention) | yes | No primary source states a card padding; 'convention' is not a source. Keep with a ❓. |
| 1593 | `border.padding_vertical` | 12px (convention) | yes | As :1592. |

</details>

### Same rows

Rows whose cell holds at the versions checked; the first citation is given.

<details><summary>283 same rows</summary>

| Line | Property | Values | Source |
|---|---|---|---|
| 278 | `NONCLIENTMETRICSW via SystemParametersInfoW(SPI_GETNONCLIENT…` | G2 Windows 11 Win32 API (Windows 1…: SPI_GETNONCLIENTMETRICS 0x0029 fills a NONCLIENTMETRICS struct; G1 Windows 10 Win32 API (Windows 1…: same API | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow (updated 2025-07-01, read 2026-10-11) |
| 280 | `five LOGFONTW entries (lfFaceName, lfHeight, lfWeight 0-1000…` | G2 Windows 11 Win32 API: lfCaptionFont, lfSmCaptionFont, lfMenuFont, lfStatusFont, lfMessageFon…; G1 Windows 10 Win32 API: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (updated 2024-11-20, read 2026-10-11); https://learn.microsoft.com/en-u… |
| 294 | `size conversion points = abs(lfHeight)*72/dpi` | G2 Windows 11 Win32 API: lfHeight = -MulDiv(PointSize, GetDeviceCaps(hDC, LOGPIXELSY), 72), so…; G1 Windows 10 Win32 API: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (updated 2025-08-15, read 2026-10-11) |
| 295 | `reader reads fonts at 96 DPI (SystemParametersInfoForDpi), f…` | G2 Windows 11 Win32 API: SystemParametersInfoForDpi is 'only intended for use with SPI_GETICONT…; G1 Windows 10 Windows 10 1607+: same (22H2 is above the 1607 minimum) | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfofordpi (updated 2024-02-22, read 2026-10-11) |
| 301 | `Caption` | G2 Windows 11 WinUI 3 in WinAppSDK…: CaptionTextBlockFontSize 12, FontWeight Normal; doc 12/16 epx; G1 Windows 10 WinUI 2.8.7: 12, Normal | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L3-L22… |
| 302 | `Body` | G2 Windows 11 WinAppSDK 2.5.1: BodyTextBlockFontSize 14, Normal; doc 14/20; G1 Windows 10 WinUI 2.8.7: 14, Normal | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L4-L25… |
| 303 | `BodyStrong` | G2 Windows 11 WinAppSDK 2.5.1: BodyStrong inherits Base: 14, SemiBold; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L10-L26 |
| 304 | `BodyLarge` | G2 Windows 11 WinAppSDK 2.5.1: BodyLargeTextBlockFontSize 18, Normal; doc 18/24; G1 Windows 10 WinUI 2.8.7: no BodyLarge style in WinUI 2.8.7 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L5-L31 |
| 305 | `BodyLargeStrong` | G2 Windows 11 WinAppSDK 2.5.1: 18, SemiBold (inherited from Base) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L32-L35 |
| 306 | `Subtitle` | G2 Windows 11 WinAppSDK 2.5.1: 20, SemiBold; doc 20/28; G1 Windows 10 WinUI 2.8.7: 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L6-L39 |
| 307 | `Title` | G2 Windows 11 WinAppSDK 2.5.1: 28, SemiBold; doc 28/36; G1 Windows 10 WinUI 2.8.7: 28 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L7-L43 |
| 308 | `TitleLarge` | G2 Windows 11 WinAppSDK 2.5.1: 40, SemiBold; doc 40/52; G1 Windows 10 WinUI 2.8.7: 40 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L8-L47 |
| 309 | `Display` | G2 Windows 11 WinAppSDK 2.5.1: 68, SemiBold; doc 68/92 epx ('All sizes are in effective pixels'); G1 Windows 10 WinUI 2.8.7: 68 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L9-L51… |
| 314 | `no system monospace setting; apps use Consolas or Cascadia M…` | G2 Windows 11 Windows 11 font list: NONCLIENTMETRICSW has no monospace LOGFONT. Windows 11 ships Cascadia…; G1 Windows 10 Windows 10 2004 font…: Consolas ships; no Cascadia font in the list | https://learn.microsoft.com/en-us/typography/fonts/windows_11_font_list (updated 2022-06-16, read 2026-10-11); https://learn.microsoft.com/en-us/windows/win32/a… |
| 323 | `UISettings GetColorValue(Foreground)` | G2 Windows 11 WinRT: UIColorType.Foreground = 1 'The foreground color'; G1 Windows 10 WinRT 10.0.10240+: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21, read 2026-10-11) |
| 324 | `GetColorValue(Background)` | G2 Windows 11 WinRT: Background = 0 'The background color'; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 325 | `GetColorValue(Accent)` | G2 Windows 11 WinRT: Accent = 5; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 326 | `GetColorValue(AccentDark1/2/3)` | G2 Windows 11 WinRT: AccentDark3=2, AccentDark2=3, AccentDark1=4; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 327 | `GetColorValue(AccentLight1/2/3)` | G2 Windows 11 WinRT: AccentLight1=6, AccentLight2=7, AccentLight3=8; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 328 | `GetColorValue(Complement)` | G2 Windows 11 WinRT: Complement = 9 'Not supported. Do not use.'; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 334 | `COLOR_WINDOW` | G2 Windows 11 Win32: 5, 'Window background'; supported on Windows 10 or greater; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22, read 2026-10-11) |
| 335 | `COLOR_WINDOWTEXT` | G2 Windows 11 Win32: 8, supported; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22) |
| 336 | `COLOR_BTNFACE` | G2 Windows 11 Win32: 15; 'Windows 10 or greater: This value is not supported.' (COLOR_3DFAC…; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22) |
| 337 | `COLOR_BTNTEXT` | G2 Windows 11 Win32: 18, supported; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 338 | `COLOR_HIGHLIGHT` | G2 Windows 11 Win32: 13 'Item(s) selected in a control', supported; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22) |
| 339 | `COLOR_HIGHLIGHTTEXT` | G2 Windows 11 Win32: 14, supported; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 340 | `COLOR_GRAYTEXT` | G2 Windows 11 Win32: 17 'Grayed (disabled) text', supported; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 341 | `COLOR_MENU / COLOR_MENUTEXT / COLOR_SCROLLBAR / COLOR_INFOBK…` | G2 Windows 11 Win32: MENU 4, MENUTEXT 7, SCROLLBAR 0 ('Scroll bar gray area'), INFOBK 24, I…; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22) |
| 346 | `COLOR_ACTIVECAPTION / CAPTIONTEXT / INACTIVECAPTION / INACTI…` | G2 Windows 11 Win32: 2, 9, 3, 19; each 'Windows 10 or greater: This value is not supported.…; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 350 | `COLOR_3DSHADOW / COLOR_3DHIGHLIGHT (lines 350-351)` | G2 Windows 11 Win32: 16, 20; both unsupported on Windows 10 or greater; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 352 | `COLOR_HOTLIGHT` | G2 Windows 11 Win32: 26 'Color for a hyperlink or hot-tracked item', supported; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 354 | `Win10+ supported constant set (prose 354-364)` | G2 Windows 11 Win32: Exactly those 8 carry no 'Windows 10 or greater: not supported' note;…; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22) |
| 1025 | `family` | G2 Windows 11 Win32: lfMessageFont = font 'used in message boxes'; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (updated 2024-11-20) |
| 1026 | `size` | G2 Windows 11 Win32: inverse of lfHeight = -MulDiv(pt, LOGPIXELSY, 72); G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (updated 2025-08-15) |
| 1027 | `weight` | G2 Windows 11 Win32: lfWeight 0-1000, 0 = default; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (updated 2025-08-15) |
| 1028 | `style` | G2 Windows 11 Win32: lfItalic 'An italic font if set to TRUE'; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (updated 2025-08-15) |
| 1029 | `color` | G2 Windows 11 WinRT: UIColorType.Foreground; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 1030 | `line_height` | G2 Windows 11 Typography doc: Body 14/20 epx, so 20/14 = 1.4286 | https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/typography (updated 2026-07-14, read 2026-10-11) |
| 1036 | `family` | G2 Windows 11 Windows 11 font list: no OS monospace setting; Cascadia Mono shipped, '* Added in Windows 11…; G1 Windows 10 Windows 10 2004 font…: Cascadia Mono not shipped (Consolas is) | https://learn.microsoft.com/en-us/typography/fonts/windows_11_font_list (updated 2022-06-16) |
| 1037 | `size` | G2 Windows 11 preset: no OS value; windows-11.toml:54 size_pt 10.5 (= 14 px) | https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (no monospace entry) |
| 1038 | `weight` | G2 Windows 11 preset: windows-11.toml:55 weight 400 | NONCLIENTMETRICSW doc (no monospace entry) |
| 1039 | `style` | G2 Windows 11 preset: no OS value | NONCLIENTMETRICSW doc |
| 1046 | `background_color` | G2 Windows 11 WinRT: UIColorType.Background; G1 Windows 10 WinRT: same | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype (updated 2025-11-21) |
| 1047 | `text_color` | G2 Windows 11 WinRT: UIColorType.Foreground; G1 Windows 10 WinRT: same | uicolortype doc (updated 2025-11-21) |
| 1048 | `accent_color` | G2 Windows 11 WinRT: UIColorType.Accent; G1 Windows 10 WinRT: same | uicolortype doc (updated 2025-11-21) |
| 1049 | `accent_text_color` | G2 Windows 11 WinAppSDK 2.5.1 (= 8…: L #FFFFFF, D #000000; G1 Windows 10 WinUI 2.8.7: L #FFFFFF, D #000000 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L216 (… |
| 1050 | `surface_color` | G2 Windows 11 WinAppSDK 2.5.1: L #B3FFFFFF, D #0DFFFFFF; G1 Windows 10 WinUI 2.8.7: L #B3FFFFFF, D #0DFFFFFF | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L260 ,… |
| 1051 | `border.color` | G2 Windows 11 WinAppSDK 2.5.1: L #0F000000, D #19000000; G1 Windows 10 WinUI 2.8.7: L #0F000000, D #19000000 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L250 ,… |
| 1052 | `muted_color` | G2 Windows 11 WinAppSDK 2.5.1: L #9E000000, D #C5FFFFFF; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L210 ,… |
| 1053 | `shadow_color` | G2 Windows 11 Fluent 2 site (unver…: low: L 14%/14%, D 28%/14%; high: L 24%/20%, D 28%/20% | https://fluent2.microsoft.design/elevation (read 2026-10-11, page '© Microsoft 2026') |
| 1054 | `link_color` | G2 Windows 11 WinAppSDK 2.5.1: HyperlinkForeground = AccentTextFillColorPrimaryBrush: L SystemAccentC…; G1 Windows 10 WinUI 2.8.7: same mapping | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Hyperlink_themeresources.xaml#L5-L15… |
| 1055 | `selection_background` | G2 Windows 11 Win32: COLOR_HIGHLIGHT 13, supported on Win10+; G1 Windows 10 Win32: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (updated 2024-02-22) |
| 1056 | `selection_text_color` | G2 Windows 11 Win32: 14, supported; G1 Windows 10 Win32: same | getsyscolor doc (updated 2024-02-22) |
| 1060 | `disabled_text_color` | G2 Windows 11 WinAppSDK 2.5.1: L #5C000000, D #5DFFFFFF; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L212 ,… |
| 1080 | `danger_color` | G2 Windows 11 WinAppSDK 2.5.1: L #C42B1C, D #FF99A4; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L282 ,… |
| 1082 | `warning_color` | G2 Windows 11 WinAppSDK 2.5.1: L #9D5D00, D #FCE100; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L281 ,… |
| 1084 | `success_color` | G2 Windows 11 WinAppSDK 2.5.1: L #0F7B0F, D #6CCB5F; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L280 ,… |
| 1086 | `info_color` | G2 Windows 11 WinAppSDK 2.5.1: SystemFillColorAttentionBrush: L SystemAccentColor, D SystemAccentColo…; G1 Windows 10 WinUI 2.8.7: same | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L369 ,… |
| 1105 | `focus_ring_width` | G2 Windows 11 WinAppSDK docs: FocusVisualPrimaryThickness default 2; FocusVisualSecondaryThickness d…; G1 Windows 10 UWP 10.0.14393+: FocusVisualPrimaryThickness default 2 | https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.frameworkelement.focusvisualprimarythickness and .../microsoft.ui.xaml.fra… |
| 386 | `SM_CXSMICON` | G1+G2 (Win32, not version-spec… About Icons read 202…: 16x16 at 96 DPI (24 at 144, 32 at 192) | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6, updated 2026-09-21), 'System icon sizes' table |
| 387 | `SM_CYSMICON` | G1+G2 read 2026-10-11: 16x16 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6) |
| 389 | `GetDpiForSystem` | G1+G2 read 2026-10-11: returns system DPI (96 when DPI-unaware); 'fixed DPI value of 96 (100%… | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforsystem (git fa536415); https://learn.microsoft.com/en-us/windows/win32/hidpi/hig… |
| 395 | `Control radius` | G2 WinUI 3 pin 8463f45: ControlCornerRadius 4,4,4,4; G2 WinUI 3 latest release winui3/release/2.5.1…: 4 (identical blob e1d58260); G2 docs Geometry, updated 20…: ControlCornerRadius (default 4px); G1 WinUI 2 opt-in Version1 v2.8.7: 2 (only if ControlsResourcesVersion=Version1; default is Version2 = 4) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CornerRadius_themeresources.xaml#L5 |
| 396 | `Overlay radius` | G2 WinUI 3 pin 8463f45: OverlayCornerRadius 8; G2 WinUI 3 latest release 2.5.1: 8; G1 WinUI 2 opt-in Version1 v2.8.7: 4 (Version1 only) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CornerRadius_themeresources.xaml#L6 |
| 397 | `Shadow` | G2 Layering and elevati…: 'Standard controls like flyouts, dialogs, and tooltips already include… | https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/layering (git ff6bf107), read 2026-10-11 |
| 398 | `Disabled opacity` | G2 WinUI 3 pin 8463f45: 0.3; C++ default 0.3 rounded / 0.55 legacy; G2 WinUI 3 latest release 2.5.1: 0.3 (identical blobs) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L6 ; htt… |
| 400 | `WinUI3 focus visual primary/secondary border` | G2 docs read 2026-10-11: primary 2px, secondary 1px inside; FocusVisualPrimaryThickness 'The de… | https://learn.microsoft.com/en-us/windows/apps/design/input/guidelines-for-visualfeedback (git 48a0f7b0, ms.date 2026-09-27); https://learn.microsoft.com/en-us/… |
| 410 | `Button horizontal padding` | G2 WinUI 3 pin 8463f45: 11,5,11,6; G2 WinUI 3 latest release 2.5.1: 11,5,11,6; G1 WinUI 2 opt-in Version1 v2.8.7: 8,5,8,6 (Version1 only; default Version2 = 11,5,11,6 at Button_themere… | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Button_themeresources.xaml#L152 |
| 411 | `Button vertical padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 5 / 6 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Button_themeresources.xaml#L152 ; https:/… |
| 413 | `CheckBox indicator size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: CheckBoxSize 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L270 ; https… |
| 414 | `CheckBox label spacing` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: CheckBoxPadding 8,5,0,0 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L274 ; https… |
| 415 | `TextBox min height` | G2 WinUI 3 pin 8463f45: 32 (generic.xaml:96), used by TextBox style MinHeight; G2 WinUI 3 latest release 2.5.1: 32 (identical blobs) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/dxaml/xcp/dxaml/themes/generic.xaml#L96 ; https://github.com/micros… |
| 416 | `TextBox horizontal padding` | G2 WinUI 3 pin 8463f45: TextControlThemePadding 10,5,6,6; G2 WinUI 3 latest release 2.5.1: 10,5,6,6; G1 WinUI 2 opt-in Version1 v2.8.7: 10,6,6,5 (Version1 only) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Common_themeresources.xaml#L12 |
| 417 | `TextBox vertical padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 5 / 6 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Common_themeresources.xaml#L12 ; https://… |
| 422 | `Slider track height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: SliderTrackThemeHeight 4 (Default/Light); 2 in HighContrast (:81) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Slider_themeresources.xaml#L6 ; https://g… |
| 423 | `Slider thumb size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: SliderHorizontalThumbWidth/Height 18; G1 WinUI 2 opt-in Version1 v2.8.7: 20 (Version1 only) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Slider_themeresources.xaml#L169-L170 ; ht… |
| 424 | `Slider tick length` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: SliderOutsideTickBarThemeHeight 4 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Slider_themeresources.xaml#L5 ; https://g… |
| 425 | `ProgressBar min height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ProgressBarMinHeight 3 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ProgressBar/ProgressBar_themeresources.xaml#L29 ; http… |
| 426 | `ProgressBar track height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ProgressBarTrackHeight 1 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ProgressBar/ProgressBar_themeresources.xaml#L30 ; http… |
| 427 | `TabView min height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: TabViewItemMinHeight 32 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/TabView_themeresources.xaml#L242 ; https://git… |
| 428 | `TabView horizontal padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: TabViewItemHeaderPadding 8,3,4,3; WithoutCloseButton 8,3,8,3; G1 WinUI 2 opt-in Version1 v2.8.7: 12,8,10,8 (Version1 only) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/TabView_themeresources.xaml#L240 ; …#L253-L254… |
| 429 | `TabView vertical padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 3 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/TabView_themeresources.xaml#L240 |
| 431 | `Menu item horizontal padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 11 (padding), MenuFlyoutItemMargin 4,2,4,2; G1 WinUI 2 opt-in Version1 v2.8.7: 11,9,11,10 (Version1 only) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L259-L260… |
| 432 | `Menu item icon spacing` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: MenuFlyoutItemPlaceholderThemeThickness 28,0,0,0; IconRoot 16x16 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L46 ; …#L3… |
| 433 | `ToolTip padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ToolTipBorderPadding 9,6,9,8 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L76 ; https:/… |
| 434 | `ToolTip max width` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ToolTipMaxWidth 320 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L77 |
| 435 | `ListView item height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ListViewItemMinHeight 40 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L14 ; ht… |
| 437 | `ListView vertical padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 0 (both styles) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L241 |
| 438 | `CommandBar height (default)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ClosedDisplayMode Compact (:86), ContentRoot MinHeight AppBarThemeComp… | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L72 ; http… |
| 439 | `CommandBar height (compact)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 48 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L72 |
| 440 | `CommandBar item spacing` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: StackPanel Orientation=Horizontal, no Spacing; AppBarButtonInnerBorder… | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L823-L825… |
| 441 | `CommandBar padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Padding 4,0,0,0 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L80 |
| 442 | `ToggleSwitch track width` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: OuterBorder Width 40 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L507 ; h… |
| 443 | `ToggleSwitch track height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Height 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L507 |
| 444 | `ToggleSwitch thumb (rest)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 12x12 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L510-L51… |
| 445 | `ToggleSwitch thumb (hover)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: PointerOver (:245) animates knob to 14x14 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L268-L27… |
| 446 | `ToggleSwitch track radius` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: RadiusX/RadiusY 10 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L507 |
| 447 | `ContentDialog min/max width` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 320 / 548 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L12-L13… |
| 448 | `ContentDialog min/max height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 184 / 756 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L14-L15 |
| 449 | `ContentDialog content padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ContentDialogPadding 24; G1 WinUI 2 opt-in Version1 v2.8.7: 24,18,24,24 (Version1 only) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L18 |
| 450 | `ContentDialog button spacing` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ContentDialogButtonSpacing 8 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L16 |
| 451 | `ContentDialog title font` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Title FontSize 20, FontWeight SemiBold | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L238 |
| 452 | `ContentDialog corner radius` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: CornerRadius = OverlayCornerRadius (8) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L66 |
| 453 | `ProgressRing default size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Width/Height 32 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ProgressRing/ProgressRing.xaml#L12-L13 ; https://githu… |
| 454 | `ProgressRing min size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: MinHeight/MinWidth 16 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ProgressRing/ProgressRing.xaml#L9-L10 |
| 455 | `ProgressRing stroke width` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ProgressRingStrokeThickness 4 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ProgressRing/ProgressRing_themeresources.xaml#L17 |
| 456 | `ComboBox min height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ComboBoxMinHeight 32 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ComboBox/ComboBox_themeresources.xaml#L327 ; https://g… |
| 457 | `ComboBox min width` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ComboBoxThemeMinWidth 64, used by Background border MinWidth (:571) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ComboBox/ComboBox_themeresources.xaml#L321 |
| 458 | `ComboBox padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ComboBoxPadding 12,5,0,7 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ComboBox/ComboBox_themeresources.xaml#L341 |
| 459 | `ComboBox arrow glyph size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: DropDownGlyph Width/Height 12, fallback FontSize 12 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ComboBox/ComboBox_themeresources.xaml#L582-L585 |
| 460 | `ComboBox arrow area width` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ColumnDefinition Width 38 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/ComboBox/ComboBox_themeresources.xaml#L567 |
| 461 | `Expander header min height` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ExpanderMinHeight 48 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/Expander/Expander_themeresources.xaml#L77 ; https://gi… |
| 462 | `Expander chevron button size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ExpanderChevronButtonSize 32 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/Expander/Expander_themeresources.xaml#L84 |
| 463 | `Expander chevron glyph size` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ExpanderChevronGlyphSize 12 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/Expander/Expander_themeresources.xaml#L85 |
| 464 | `Expander content padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: ExpanderContentPadding 16 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/Expander/Expander_themeresources.xaml#L86 |
| 465 | `HyperlinkButton padding` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Padding = ButtonPadding (11,5,11,6) | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml#L57 |
| 466 | `HyperlinkButton background` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: HyperlinkButtonBackground = SubtleFillColorTransparentBrush (#00FFFFFF… | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml#L9 ;… |
| 471 | `Fluent spacing ramp naming (sizeNone..size320 code subset; f…` | G2 Fluent 2 design page read 2026-10-11: 17 tokens sizeNone, size20…size560 (0–56); G2 Fluent UI tokens @fluentui/tokens_v1.…: 11 code tokens none…xxxl (0–32px) | https://fluent2.microsoft.design/layout (read 2026-10-11) |
| 475 | `None` | G2 Fluent UI tokens alpha.24 b6351802 (=…: '0' | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L5 |
| 476 | `XXSmall` | G2 Fluent UI tokens b6351802: 2px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L6 |
| 477 | `XSmall` | G2 Fluent UI tokens b6351802: 4px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L7 |
| 478 | `sNudge` | G2 Fluent UI tokens b6351802: 6px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L8 |
| 479 | `Small` | G2 Fluent UI tokens b6351802: 8px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L9 |
| 480 | `mNudge` | G2 Fluent UI tokens b6351802: 10px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L10 |
| 481 | `Medium` | G2 Fluent UI tokens b6351802: 12px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L11 |
| 482 | `Large` | G2 Fluent UI tokens b6351802: 16px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L12 |
| 483 | `XLarge` | G2 Fluent UI tokens b6351802: 20px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L13 |
| 484 | `XXLarge` | G2 Fluent UI tokens b6351802: 24px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L14 |
| 485 | `XXXLarge` | G2 Fluent UI tokens b6351802: 32px | https://github.com/microsoft/fluentui/blob/b6351802032e9af61bf03033d22cb354b2e2822c/packages/tokens/src/global/spacings.ts#L15 |
| 512 | `SM_CXICON` | G1+G2 About Icons, updated…: 32x32 at 96 DPI (48 at 144, 64 at 192) | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6), 'System icon sizes' |
| 513 | `SM_CYICON` | G1+G2 read 2026-10-11: 32x32 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6) |
| 514 | `SM_CXSMICON` | G1+G2 read 2026-10-11: 16x16 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6) |
| 515 | `SM_CYSMICON` | G1+G2 read 2026-10-11: 16x16 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6) |
| 521 | `SHIL_SMALL` | G1+G2 read 2026-10-11: 16x16 'can be customized by the user'; = SM_CXSMICON, 16 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetimagelist (git 540c1f2d, updated 2025-10-31); https://learn.microsoft.com/en-us/wi… |
| 522 | `SHIL_LARGE` | G1+G2 read 2026-10-11: normally 32x32 (48 with 'Use large icons') | https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetimagelist (git 540c1f2d) |
| 523 | `SHIL_EXTRALARGE` | G1+G2 read 2026-10-11: typically 48x48 | https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetimagelist (git 540c1f2d) |
| 524 | `SHIL_JUMBO` | G1+G2 read 2026-10-11: normally 256x256, fixed regardless of DPI | https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetimagelist (git 540c1f2d) |
| 531 | `NavigationViewItem icon` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: NavigationViewItemOnLeftIconBoxHeight 16; G1 WinUI 2 v2.8.7: 16 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/NavigationView/NavigationView_themeresources.xaml#L219… |
| 537 | `Display scale factor` | G1+G2 read 2026-10-11: 96 = 100% | https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows (git 55e845e9); https://learn.microsoft.com/en-us/wind… |
| 538 | `Text scale factor` | G1+G2 read 2026-10-11: 'The value ranges from 1 to 2.25' | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactor (git 3abc0b43, updated 2025-11-21) |
| 539 | `High contrast mode` | G1+G2 read 2026-10-11: SPI_GETHIGHCONTRAST 0x0042 fills HIGHCONTRAST | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow (git fa536415) |
| 540 | `Reduce motion` | G1+G2 read 2026-10-11: SPI_GETCLIENTAREAANIMATION 0x1042, BOOL | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow (git fa536415) |
| 1112 | `border.corner_radius (Windows)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 4 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CornerRadius_themeresources.xaml#L5 ; htt… |
| 1113 | `border.corner_radius_lg (Windows)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: 8 | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/CornerRadius_themeresources.xaml#L6 |
| 1114 | `border.line_width (Windows)` | G1+G2 read 2026-10-11: SM_CXBORDER = window border width (no value documented); G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Fluent controls draw ButtonBorderThemeThickness 1 (not from SM_CXBORDE… | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics (git fa536415) |
| 1115 | `disabled_opacity (Windows)` | G2 WinUI 3 pin 8463f45: Button Disabled swaps ButtonBackgroundDisabled/BorderBrushDisabled/For…; G2 WinUI 3 latest release 2.5.1: identical blobs | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Button_themeresources.xaml#L131-L139 ; …#… |
| 1116 | `border.opacity (Windows)` | G2 WinUI 3 pin / 2.5.1 8463f45 = 2.5.1: Light dict (from :208): ControlStrokeColorDefault, CardStrokeColorDefa… | https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Common_themeresources_any.xaml#L243-L257… |
| 1117 | `border.shadow_enabled (Windows)` | G2 updated 2026-02-19: standard flyouts/dialogs/tooltips include shadows | https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/layering (git ff6bf107) |
| 1123 | `text_scaling_factor (Windows)` | G1+G2 read 2026-10-11: TextScaleFactor 1–2.25 ('system text size setting'), Windows 10 10240+ | https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactor (git 3abc0b43); https://learn.microsoft.com/en-us/uwp/api/windows… |
| 1124 | `reduce_motion (Windows)` | G1+G2 read 2026-10-11: BOOL, TRUE if animations enabled | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow (git fa536415) |
| 1125 | `high_contrast (Windows)` | G1+G2 read 2026-10-11: HIGHCONTRAST structure | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow (git fa536415); https://learn.microsoft.com/en-us/windows/win32/api… |
| 1133 | `small (Windows)` | G1+G2 read 2026-10-11: 16 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6) |
| 1134 | `large (Windows)` | G1+G2 read 2026-10-11: 32 at 96 DPI | https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons (git 4955b1e6) |
| 1145 | `border.color` | G2 Windows 11 25H2 (doc, all Win10…: COLOR_ACTIVEBORDER 'Windows 10 or greater: This value is not supported…; G1 Windows 10 22H2: same doc statement covers Windows 10 | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (git 92672624, updated 2024-02-22, read 2026-10-11) |
| 1148 | `title_bar_font.family` | G2 Windows 11 25H2: NONCLIENTMETRICSW.lfCaptionFont 'information about the caption font'; G1 Windows 10 22H2: same struct | https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (git a9040b7b, updated 2024-11-20, read 2026-10-11) |
| 1149 | `title_bar_font.size` | G2 Windows 11 25H2: lfHeight = -MulDiv(PointSize, LOGPIXELSY, 72) (inverse of the cell's f…; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (git a7f1adeb, updated 2025-08-15, read 2026-10-11) |
| 1151 | `title_bar_font.style` | G2 Windows 11 25H2: LOGFONTW.lfItalic; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1155 | `border.corner_radius` | G2 Windows 11 25H2: 8px: 'Top-level containers such as app windows, flyouts and dialogs ar…; G1 Windows 10 22H2: no system window rounding: rounding is a Windows 11 feature; DWMWA_WIN… | https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/geometry (git dea23108, updated 2026-02-19, read 2026-10-11) |
| 1164 | `background_color` | G2 Windows 11 25H2: index 15: COLOR_BTNFACE marked 'not supported' on Win10+, but its alia…; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) |
| 1166 | `font.color` | G2 Windows 11 25H2: supported on Win10/11 ('Foreground color for buttons'); G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) |
| 1171 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: ButtonPadding 11,5,11,6; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: ButtonPadding 8,5,8,6 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Button_themeresources.xaml#L152 (iden… |
| 1172 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: 5 top / 6 bottom (ButtonPadding 11,5,11,6); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 5 / 6 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Button_themeresources.xaml#L152 (iden… |
| 1181 | `checked_background` | G2 Windows 11 WinUI 3 2.5.1: AccentFillColorDefaultBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleButton_themeresources.xaml#L11… |
| 1182 | `checked_text_color` | G2 Windows 11 WinUI 3 2.5.1: TextOnAccentFillColorPrimaryBrush; L #FFFFFF, D #000000 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleButton_themeresources.xaml#L23… |
| 1186 | `background_color` | G2 Windows 11 25H2: supported on Win10/11; G1 Windows 10 22H2: supported | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) |
| 1188 | `font.color` | G2 Windows 11 25H2: supported; G1 Windows 10 22H2: supported | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor (read 2026-10-11) |
| 1195 | `min_height` | G2 Windows 11 WinUI 3 2.5.1: TextControlThemeMinHeight 32; TextBox MinHeight setter | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/themes/generic.xaml#L96 ; .../src/controls/dev/… |
| 1196 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: TextControlThemePadding 10,5,6,6; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 10,6,6,5 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources.xaml#L12 (ident… |
| 1197 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: 5 / 6; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 6 top / 5 bottom | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources.xaml#L12 (ident… |
| 1206 | `background_color` | G2 Windows 11 WinUI 3 2.5.1: CheckBoxCheckBackgroundFillUnchecked = ControlAltFillColorSecondaryBru… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L53 , #L… |
| 1209 | `border.color` | G2 Windows 11 WinUI 3 2.5.1: CheckBoxCheckBackgroundStrokeUnchecked = ControlStrongStrokeColorDefau… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L41 , #L… |
| 1211 | `indicator_color` | G2 Windows 11 WinUI 3 2.5.1: CheckBoxCheckGlyphForegroundChecked = TextOnAccentFillColorPrimaryBrus… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L69 , #L… |
| 1212 | `indicator_width` | G2 Windows 11 WinUI 3 2.5.1: CheckBoxSize 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L270 , #… |
| 1213 | `label_gap` | G2 Windows 11 WinUI 3 2.5.1: CheckBoxPadding 8,5,0,0 applied as the label ContentPresenter Margin | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L274 , #… |
| 1220 | `radio_dot_diameter` | G2 Windows 11 WinUI 3 2.5.1: 12 (:179, :374); PointerOver 14 (:256, :259), Pressed 10 (:293, :296),… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/RadioButton_themeresources.xaml#L179… |
| 1221 | `check_mark_stroke_width` | G2 Windows 11 WinUI 3 2.5.1: CheckBoxCheckedGlyph &#xE73E; (:276), CheckBoxGlyphSize 12 (:271), Fon… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L271-L27… |
| 1222 | `radio_indicator_width` | G2 Windows 11 WinUI 3 2.5.1: OuterEllipse Width/Height 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/RadioButton_themeresources.xaml#L371… |
| 1229 | `font.family` | G2 Windows 11 25H2: lfMenuFont 'font used in menu bars'; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (git a9040b7b, read 2026-10-11) |
| 1230 | `font.size` | G2 Windows 11 25H2: LOGFONTW lfHeight formula; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1231 | `font.weight` | G2 Windows 11 25H2: LOGFONTW.lfWeight; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1232 | `font.style` | G2 Windows 11 25H2: LOGFONTW.lfItalic; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1235 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: 11 (MenuFlyoutItemThemePadding / Narrow); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 11 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L260-L… |
| 1236 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: default 8/9; NarrowPadding 4/5 when opened by Mouse, Pen or Keyboard; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 9/10, narrow 4/7 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L260-L… |
| 1241 | `icon_text_gap` | G2 Windows 11 WinUI 3 2.5.1: text Margin 28 (MenuFlyoutItemPlaceholderThemeThickness) − IconRoot 16… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L46 ,… |
| 1242 | `icon_size` | G2 Windows 11 25H2 / WinUI 3 2.5.1: SM_CXSMICON 'system small width of an icon'; WinUI 3 menu IconRoot Vie…; G1 Windows 10 22H2: same API | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics (git fa536415, read 2026-10-11); https://github.com/microsoft/microsoft-… |
| 1243 | `hover_background` | G2 Windows 11 WinUI 3 2.5.1: MenuFlyoutItemBackgroundPointerOver = SubtleFillColorSecondaryBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L7 , #… |
| 1245 | `disabled_text_color` | G2 Windows 11 WinUI 3 2.5.1: MenuFlyoutItemForegroundDisabled = TextFillColorDisabledBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml#L14 ,… |
| 1257 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: ToolTipBorderPadding 9,6,9,8; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: ToolTipBorderThemePadding 8,5,8,7 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L76 (iden… |
| 1258 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: 6 / 8; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 5 / 7 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L76 (iden… |
| 1259 | `max_width` | G2 Windows 11 WinUI 3 2.5.1: ToolTipMaxWidth 320; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: MaxWidth="320" | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L51 , #L7… |
| 1261 | `border.color` | G2 Windows 11 WinUI 3 2.5.1: ToolTipBorderBrush = SurfaceStrokeColorFlyoutBrush (L #0F000000, D #33… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToolTip_themeresources.xaml#L9 , #L33… |
| 1263 | `border.shadow_enabled` | G2 Windows 11 WinUI 3 2.5.1: ToolTip applies ApplyElevationEffect to LayoutRoot (baseElevation 16 i… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/lib/ToolTip_Partial.cpp#L644-L651 (identical bl… |
| 1280 | `groove_width` | G2 Windows 11 25H2: SM_CXVSCROLL 'The width of a vertical scroll bar, in pixels.'; G1 Windows 10 22H2: same API | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics (git fa536415, updated 2025-07-01, read 2026-10-11) |
| 1281 | `min_thumb_length` | G2 Windows 11 25H2: SM_CYVTHUMB 'The height of the thumb box in a vertical scroll bar, in…; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics (read 2026-10-11) |
| 1292 | `track_height` | G2 Windows 11 WinUI 3 2.5.1: SliderTrackThemeHeight 4 (2 in HighContrast); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 2 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Slider_themeresources.xaml#L6 , #L110… |
| 1293 | `thumb_diameter` | G2 Windows 11 WinUI 3 2.5.1: SliderHorizontalThumbWidth/Height 18; template Border Margin="-2" draw…; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Slider_themeresources.xaml#L169-L170… |
| 1294 | `tick_mark_length` | G2 Windows 11 WinUI 3 2.5.1: SliderOutsideTickBarThemeHeight 4; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 4 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Slider_themeresources.xaml#L5 , #L109… |
| 1303 | `track_height` | G2 Windows 11 WinUI 3 2.5.1: ProgressBarMinHeight 3, ProgressBarTrackHeight 1; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: ProgressBarThemeMinHeight 4 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressBar/ProgressBar_themeresources.xaml#L29-L3… |
| 1308 | `border.line_width` | G2 Windows 11 WinUI 3 2.5.1: 0 / 0 / 1 (HC) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressBar/ProgressBar_themeresources.xaml#L5-L21… |
| 1317 | `min_height` | G2 Windows 11 WinUI 3 2.5.1: TabViewItemMinHeight 32 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView_themeresources.xaml#L242 ; TabView… |
| 1318 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: 8,3,8,3 / 8,3,4,3 picked by CloseIconStates; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: TabViewItemHeaderPadding 12,8,10,8 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView_themeresources.xaml#L253-L254 ; Ta… |
| 1319 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: 3; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 8 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView_themeresources.xaml#L253-L254 (ide… |
| 1320 | `border.color` | G2 Windows 11 WinUI 3 2.5.1: TabViewSelectedItemBorderBrush LinearGradientBrush with CardStrokeColo… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView_themeresources.xaml#L72-L79 ; TabV… |
| 1321 | `border.line_width` | G2 Windows 11 WinUI 3 2.5.1: TabViewItemBorderThickness 1; TabViewSelectedItemBorderThickness 1,1,1… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView_themeresources.xaml#L267-L268 ; Ta… |
| 1322 | `border.corner_radius` | G2 Windows 11 WinUI 3 2.5.1: CornerRadius OverlayCornerRadius, filtered to top corners on TabContai… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView.xaml#L297 , #L562 (identical at 84… |
| 1323 | `border.shadow_enabled` | G2 Windows 11 WinUI 3 2.5.1: TabViewShadowDepth 16 defined; no 'shadow' reference in TabView.xaml,… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView_themeresources.xaml#L265 |
| 1326 | `item_gap` | G2 Windows 11 WinUI 3 2.5.1: ItemsStackPanel Horizontal; TabSeparator Width 1 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView.xaml#L76-L82 , #L292-L300 , #L553… |
| 1327 | `active_indicator_color` | G2 Windows 11 WinUI 3 2.5.1: Selected state shows SelectedBackgroundPath filled TabViewItemHeaderBa… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/TabView/TabView.xaml#L335-L347 (identical at 8463f… |
| 1333 | `background_color` | G2 Windows 11 WinUI 3 2.5.1: expanded (Left) pane NavigationViewExpandedPaneBackground = SolidBackg… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/NavigationView/NavigationView_themeresources.xaml#… |
| 1342 | `hover_background` | G2 Windows 11 WinUI 3 2.5.1: NavigationViewItemBackgroundPointerOver = SubtleFillColorSecondaryBrus… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/NavigationView/NavigationView_themeresources.xaml#… |
| 1351 | `bar_height` | G2 Windows 11 WinUI 3 2.5.1: 48 (:72), ClosedDisplayMode Compact (:86), ContentRoot MinHeight (:795… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L72 ,… |
| 1352 | `item_gap` | G2 Windows 11 WinUI 3 2.5.1: PrimaryItemsControl uses StackPanel Horizontal (no Spacing); AppBarBut… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L821-L… |
| 1353 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: CommandBar Padding 4,0,0,0 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L80 ,… |
| 1354 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: 0 (Padding 4,0,0,0) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/CommandBar_themeresources.xaml#L80 (i… |
| 1362 | `font.family` | G2 Windows 11 25H2: lfStatusFont 'font used in status bars and tooltips'; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw (git a9040b7b, read 2026-10-11) |
| 1363 | `font.size` | G2 Windows 11 25H2: LOGFONTW formula; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1364 | `font.weight` | G2 Windows 11 25H2: LOGFONTW.lfWeight; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1365 | `font.style` | G2 Windows 11 25H2: LOGFONTW.lfItalic; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw (read 2026-10-11) |
| 1372 | `border.padding_horizontal` | G2 Windows 11 25H2: SB_GETBORDERS returns horizontal, vertical and between-rectangle borde…; G1 Windows 10 22H2: same | https://learn.microsoft.com/en-us/windows/win32/controls/sb-getborders (git 2ec0df65, updated 2020-12-11, read 2026-10-11) |
| 1373 | `border.padding_vertical` | G2 Windows 11 25H2: same as :1372 | https://learn.microsoft.com/en-us/windows/win32/controls/sb-getborders (read 2026-10-11) |
| 1389 | `row_height` | G2 Windows 11 WinUI 3 2.5.1: ListViewItemMinHeight 40; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 40 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L14… |
| 1390 | `border.padding_horizontal` | G2 Windows 11 WinUI 3 2.5.1: DefaultListViewItemStyle Padding 16,0,12,0; ListViewItemExpanded 12,0,… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L241… |
| 1391 | `border.padding_vertical` | G2 Windows 11 WinUI 3 2.5.1: 0 (Padding 16,0,12,0) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L241… |
| 1392 | `hover_background` | G2 Windows 11 WinUI 3 2.5.1: ListViewItemBackgroundPointerOver = SubtleFillColorSecondaryBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml#L18… |
| 1411 | `border.shadow_enabled` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 / 84…: yes: FlyoutPresenter applies ApplyElevationEffect when IsDefaultShadow…; G1 Windows 10 Windows 10 1903+ (UW…: yes: FlyoutPresenter.IsDefaultShadowEnabled default true, introduced 1… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/dxaml/lib/FlyoutPresenter_partial.cpp#L85-L98 ; https… |
| 1412 | `border.padding_horizontal` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 16 / 16 (FlyoutContentPadding 16,15,16,17, set as Padding); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7 v1 style: Padding = FlyoutContentThemePadding (resource not in WinUI 2; legacy b… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/FlyoutPresenter_themeresources.xaml#L… |
| 1413 | `border.padding_vertical` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 15 top / 17 bottom; G1 Windows 10 (legacy) WinUI 2.8.7 v1 style: 11 top / 12 bottom (FlyoutContentThemePadding 12,11,12,12) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/FlyoutPresenter_themeresources.xaml#L… |
| 1419 | `divider_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 1 (SplitViewLeftBorderThemeThickness 0,0,1,0), but BorderBrush = Syste…; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: no border bound on PaneRoot; only a 1px HCPaneBorder rectangle (Transp… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/SplitView/SplitView_themeresources.xaml#L7 and #L4… |
| 1435 | `caption` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 12, Normal (400); G1 Windows 10 (legacy base sty… legacy generic.xaml: 12, Normal (400) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L3 and… |
| 1436 | `section_heading` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 20, SemiBold (600, inherited from BaseTextBlockStyle); G1 Windows 10 (legacy base sty… legacy generic.xaml: 20, Normal (400) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L6 , #L… |
| 1437 | `dialog_title` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 28, SemiBold (600); G1 Windows 10 (legacy base sty… legacy generic.xaml: 24, SemiLight (350) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L7 and… |
| 1438 | `display` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 68, SemiBold (600); G1 Windows 10 (legacy base sty… legacy generic.xaml: no Display style; largest is HeaderTextBlockStyle 46 Light (300) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml#L9 and… |
| 1467 | `track_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 40 (OuterBorder Width); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 40 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L507 |
| 1468 | `track_height` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 20; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L507 |
| 1469 | `thumb_diameter` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 12 rest (Normal state), 14 PointerOver; Pressed animates to 17 wide ×…; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 10 (Ellipse 10×10, no hover growth) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L231… |
| 1470 | `track_radius` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: RadiusX/RadiusY 10; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 10 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L507 |
| 1472 | `unchecked_background` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: ToggleSwitchFillOff = ControlAltFillColorSecondaryBrush: L #06000000,…; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: SystemControlTransparentBrush (transparent track, outlined) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L15… |
| 1475 | `unchecked_thumb_diameter` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 12 × 12 both; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 10 × 10 both | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L510… |
| 1476 | `unchecked_thumb_background` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: as cell; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: ToggleSwitchKnobFillOff = SystemControlHighlightBaseHighBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml#L31… |
| 1483 | `background_color` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: ContentDialogBackground = SolidBackgroundFillColorBaseBrush: L #F3F3F3…; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: SystemControlPageBackgroundAltHighBrush (contract7) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L6… |
| 1486 | `border.shadow_enabled` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: yes: SmokeLayerBackground rectangle; ApplyElevationEffect on the backg… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L22… |
| 1487 | `min_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 320; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 320 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L12 |
| 1488 | `max_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 548; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 548 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L13 |
| 1489 | `min_height` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 184; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 184 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L14 |
| 1490 | `max_height` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 756; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 756 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L15 |
| 1491 | `border.padding_horizontal` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 24 (ContentDialogPadding 24, on both content and command grids); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 24 (ContentDialogPadding 24,18,24,24) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L18… |
| 1492 | `border.padding_vertical` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 24 top / 24 bottom; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 18 top / 24 bottom | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L18 |
| 1493 | `button_gap` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 8 (ContentDialogButtonSpacing spacer columns); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 4 (button margins 0,0,2,0 / 2,0,2,0 / 2,0,0,0) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L16… |
| 1494 | `button_order` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: PrimaryButton column 0, CloseButton column 4 (rightmost); a lone prima…; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: PrimaryButton column 0, CloseButton column 3 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L25… |
| 1496 | `title_font.size` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 20; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 20 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L23… |
| 1497 | `title_font.weight` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: SemiBold (600); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: Normal (400) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L23… |
| 1498 | `title_font.style` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: Normal (no FontStyle set on Title) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L23… |
| 1500 | `border.corner_radius` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (Cor…: 8 (OverlayCornerRadius 8,8,8,8); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 4 (OverlayCornerRadius 4,4,4,4) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L66… |
| 1503 | `icon_size` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: no icon element in the ContentDialog template | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml#L22… |
| 1533 | `diameter` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 32 × 32; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: Width 32 (no Height setter) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressRing/ProgressRing.xaml#L12-L13 |
| 1534 | `min_diameter` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 16; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 16 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressRing/ProgressRing.xaml#L9-L10 |
| 1535 | `stroke_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (blo…: 4 (ProgressRingStrokeThickness); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 4 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ProgressRing/ProgressRing_themeresources.xaml#L17 |
| 1550 | `min_height` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 32 (ComboBoxMinHeight on the DropDownGlyph); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 32 (declared as Thickness) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L327 , #L582 |
| 1551 | `min_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 64 (ComboBoxThemeMinWidth on the Background border); G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 64 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L321 , #L571 |
| 1552 | `border.padding_horizontal` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 2…: as cell; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 12 left / 0 right (same ComboBoxPadding) but 32px arrow column, glyph… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L341 , #L566… |
| 1553 | `arrow_icon_size` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 12 × 12, fallback glyph E70D FontSize 12; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: FontIcon E70D, FontSize 12 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L582-L587 |
| 1554 | `arrow_area_width` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 38; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 32 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L567 |
| 1556 | `border.shadow_enabled` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: no ThemeShadow in the template (Background border only has Translation… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L54 , #L571 |
| 1557 | `border.padding_vertical` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 5 top / 7 bottom; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 5 top / 7 bottom | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/ComboBox/ComboBox_themeresources.xaml#L341 |
| 1587 | `background_color` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: L #B3FFFFFF, D #0DFFFFFF | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L56 (D… |
| 1588 | `border.color` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: L #0F000000, D #19000000 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources_any.xaml#L46 (D… |
| 1589 | `border.line_width` | G2 Windows 11 Layering doc git ff6…: Card: stroke width 1 | https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/layering (read 2026-10-11) |
| 1605 | `header_height` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 48 (ExpanderMinHeight, bound to header MinHeight); G1 Windows 10 (WinUI 2) WinUI 2.8.7 (Expande…: 48 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L77 ; https:… |
| 1606 | `arrow_icon_size` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 12 (ExpanderChevronGlyphSize); G1 Windows 10 (WinUI 2) WinUI 2.8.7: 12 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L85 |
| 1607 | `border.padding_horizontal` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: as cell; G1 Windows 10 (WinUI 2) WinUI 2.8.7: 16,0,0,0 / content 16 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L80 , #L86 |
| 1608 | `border.padding_vertical` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 0 | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L80 |
| 1612 | `border.shadow_enabled` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: no shadow/Translation in Expander.xaml or Expander_themeresources.xaml | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander.xaml |
| 1613 | `arrow_side` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: as cell | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L83 , #L280 |
| 1614 | `arrow_gap` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: ExpanderChevronMargin 20,0,8,0; button 32; glyph 12 centred → 30; G1 Windows 10 (WinUI 2) WinUI 2.8.7: same resources (20,0,8,0; 32; 12) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L81 , #L84-L… |
| 1615 | `content_indent` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 16 (ExpanderContentPadding, Padding of the content Border) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L86-L87 ; ht… |
| 1616 | `frame_enabled` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: header border CardStrokeColorDefaultBrush, thickness 1; content 1,0,1,… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/Expander/Expander_themeresources.xaml#L4-L16 , #L8… |
| 1622 | `font.color` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: HyperlinkButtonForeground = AccentTextFillColorPrimaryBrush: L SystemA…; G1 Windows 10 (legacy base sty… legacy generic.xaml…: SystemControlHyperlinkTextBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml#L… |
| 1623 | `visited_text_color` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: no visited state or brush in HyperlinkButton resources | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml |
| 1624 | `underline_enabled` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: HyperlinkButton template has no underline; inline Hyperlink text eleme… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml ;… |
| 1625 | `background_color` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: SubtleFillColorTransparentBrush #00FFFFFF; G1 Windows 10 (legacy base sty… legacy generic.xaml: SystemControlPageBackgroundTransparentBrush | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml#L… |
| 1626 | `hover_background` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: L #09000000, D #0FFFFFFF; G1 Windows 10 (legacy base sty… legacy generic.xaml: transparent (SystemControlPageBackgroundTransparentBrush); no hover fi… | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml#L… |
| 1639 | `border.padding_horizontal` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1 (= 8…: 10 left / 6 right; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 10 left / 6 right (TextControlThemePadding 10,6,6,5) | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources.xaml#L12 ; http… |
| 1640 | `border.padding_vertical` | G2 Windows 11 (WinUI 3) WinAppSDK 2.5.1: 5 top / 6 bottom; G1 Windows 10 (WinUI 2 Version… WinUI 2.8.7: 6 top / 5 bottom | https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Common_themeresources.xaml#L12 |

</details>

### Researcher notes

Each researcher's own notes, verbatim: coverage, method caveats and errors found outside the *differs* rows (wrong citations on ✅ rows, unpinned links). Step 7 corrects the citation errors (S4) as part of the row fixes.

<details><summary>fonts-colours</summary>

I covered every Windows row in scope (lines 274-371 and the Windows column of 1019-1107); no range is left uncovered. Local copies are in scratchpad/m2-src/m2b-fontcol/ (p8463, r251, r1812, w287, web/).

The Step 1 version map is correct as I used it. 8463f45 and 2.5.1 have identical blobs for every file I cited, so all WinUI-cited cells hold at the latest release. WinUI 2.8.7 has the same Fluent colour values, and it uses the Version2 (Windows 11) styles by default on Windows 10 too (dev/dll/XamlControlsResources.cpp:23).

New findings, not on docs/todo.md:
(1) Status text colours :1081/:1083/:1085/:1087. WinUI does have a resource for content on a status fill: InfoBar's *SeverityIconForeground maps to TextFillColorInverse, L #FFFFFF / D #E4000000, for all four severities. So every dark value (#1a1a1a) is wrong, and warning light #1a1a1a is wrong too (WinUI draws #FFFFFF). The appendix :1745 and windows-11.toml:18-24 / :426-432 carry the same values.
(2) focus_ring_color :1104. WinUI's focus visual is not the accent: it is FocusStrokeColorOuter (L #E4000000, D #FFFFFF) with an inner FocusStrokeColorInner. The accent is used only when Reveal focus is enabled. windows-11.toml:29/:437 state #0078d4.
(3) focus_ring_offset :1106. The FrameworkElement default is 0, but the Button styles set FocusVisualMargin -3 (a 3px outset) and CheckBox sets -7,-3.
(4) The ring is a 2px primary border plus a 1px inner secondary border.

Unverifiable (should be ❓, and the probe can settle them):
- the Windows 10/11 NONCLIENTMETRICS defaults at :285-289. The only cited source, the UX Guide, says it covers Vista/7 and has not been updated.
- the Win10 400 / Win11 700 caption weight at :286. The Q&A answer is secondary and talks about how the title bar looks, not about lfWeight.
- 'Win32 always returns Segoe UI' at :291-292.
- SM_CXFOCUSBORDER = 1 at :1105.
- DwmGetColorizationColor as the title bar colour at :370.
- the COLOR_BTNFACE fallback at :1057, which is marked unsupported on Windows 10+.

Windows 10 gap: Cascadia Mono is not in the Windows 10 font list, but the windows-11 preset, which the reader also uses on Windows 10, names it as the mono font. Already known: todo.md:675-678 records that the windows-11 dark border and disabled-text colour #454545 match no WinUI brush.

The appendix rows :1709/:1711/:1742/:1748 cite unpinned or dead links (blob/main/src/…), which Step 1 already noted.

</details>

<details><summary>geometry</summary>

Coverage: every value row in 372-547 and the Windows column of 1108-1139 checked; none left. Totals: 101 rows; same 82, differs 4, unverifiable 15.

Corrections to Step 1's version map:
(1) WinUI 2.8.7 defaults to ControlsResourcesVersion.Version2, the Windows 11 styles (dev/Generated/XamlControlsResources.properties.cpp:35 @232a16e5). The *_v1.xaml styles are opt-in. A Windows 10 app using WinUI 2 does not draw them by default. The G1 'Version1' entries above are given for reference only.
(2) Windows App SDK runs on Windows 10 1809+ (system-requirements page, line 'Consistent across Windows versions'). Every WinUI 3 value is therefore the same on Windows 10 22H2 and Windows 11. Only Win32 and OS values could vary by version, and the Microsoft docs state none of those values.
(3) AppBarButton_themeresources.xaml differs between 8463f45 and both 1.8.12 and main. The only change is that the label FontSize=12 moved into a resource; no geometry changed.

All cited WinUI 3 files are identical between 8463f45 and 2.5.1, 1.8.12 and main, except two opt-in sets: the ScrollBar and CommandBar *_perf2026.xaml files.

New findings, none on docs/todo.md:
(a) :530 and :1132: the AppBarButton icon is 16 in source. AppBarButton_themeresources.xaml:366 binds AppBarButtonContentHeight, which WinUI 3 generic.xaml:19462 sets to 16. The Microsoft Learn Command bar page says 20x20. The cell's claim that 16 is 'legacy v1 only' is disproved; a screenshot settles 16 vs 20.
(b) :436: the default ListViewItem padding is 16 left / 12 right (DefaultListViewItemStyle :241). 12/12 belongs to the unreferenced ListViewItemExpanded style.
(c) :1126: Windows does expose reduce-transparency through UISettings.AdvancedEffectsEnabled (Windows 10 1703+). windows.rs:536's comment is wrong too.
(d) :378-379: the cited iBorderWidth is the sizing border (SM_CXSIZEFRAME), not SM_CXBORDER.
(e) :380-385 marked ✅ have no primary source: .NET, Wine, ReactOS and registry guides only. They should be ❓ pending the probe.
(f) :420-421 say there is no named resource, but there are three: ScrollBarSize 12, ScrollBarVerticalThumbMinWidth 8 and ScrollBarThumbStrokeThickness 6.
(g) §1.2.5 is headed 'WinUI3' but its tokens are Fluent UI web tokens.
(h) Outside this scope: the focus-visuals doc says 'default margin is 1px away from the control bounds', and DefaultListViewItemStyle sets FocusVisualMargin=1. This bears on §2.1.5 :1106, which says 'Fluent: 0px'.

Already on todo: button min height (:409) is todo.md:2267; SM_CYVTHUMB (:382) is todo.md:2274.

Probe items, Windows 10 22H2 and 11 at 96 DPI: GetSystemMetricsForDpi SM_CXBORDER, SM_CXVSCROLL, SM_CYHSCROLL, SM_CYVTHUMB, SM_CYMENU, SM_CXFOCUSBORDER; Win32 menu item height. Screenshots: WinUI 3 Button height, ScrollBar collapsed and expanded thumb, CommandBar icon size.

Scratch: /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/m2b-geom/ (muxc shallow blobless fetch of 5 revisions, x/<rev>/ extracted files, web/ docs).

</details>

<details><summary>widgets-1</summary>

All in-scope rows 1140–1400 covered; nothing left. Step 1 map is right: every WinUI 3 file cited in scope is byte-identical at 8463f45 and tag 2.5.1 (and 1.8.12, apart from an AppBarButton label-size resource). So WinUI-sourced cells do not vary by Windows 11 feature update. WinUI 3 also draws the same on Windows 10. G1 values given are the WinUI 2 Version1 legacy styles (v2.8.7) and are for reference only.

New findings (not on todo.md):
1. Seven cells name GetSysColor constants that MS documents as "Windows 10 or greater: not supported": COLOR_CAPTIONTEXT :1152, COLOR_INACTIVECAPTION :1153, COLOR_INACTIVECAPTIONTEXT :1154, COLOR_MENU :1227, COLOR_MENUTEXT :1233, COLOR_INFOBK :1251, COLOR_INFOTEXT :1256. Chapter 1 :355-365 already says so, but chapter 2 cites them bare, and the reader reads all of them (windows.rs:304-313, :359-360). :1164 COLOR_BTNFACE is the same index as the supported COLOR_3DFACE, so the value is the same; rename it.
2. Button hover :1179 is ControlFillColorSecondary (Button_themeresources.xaml:31), not SubtleFillColorSecondary.
3. Placeholder :1191: no resource "TextPlaceholderColor" exists. The real one is TextFillColorSecondary (TextBox_themeresources.xaml:35, :142).
4. overlay_mode :1283 is queryable: UISettings.AutoHideScrollBars (Windows 10 1903+).
5. Toolbar icon :1356 (and §2.1.8 :1132) is 16 (AppBarButtonContentHeight, generic.xaml:19462), not 20.
6. Tab min_width :1316: the default TabWidthMode is Equal, with a minimum of 100 and a maximum of 240. todo.md:2277 assumes (none).
7. Inheritance cells the source contradicts: tooltip font size 12 (:1253); slider track ControlStrongFillColorDefault and thumb ControlSolidFillColorDefault (:1290, :1291); NavigationView and ListView selection SubtleFillColorSecondary with TextFillColorPrimary text plus an accent pill (:1336-1337, :1383-1384).
8. :1263 tooltip shadow now has a source (ToolTip_Partial.cpp:644-651).
9. :1382 is tagged (Fluent) but the values are the preset's own.

Already on todo.md: :1170 (todo :2267); :1234 (:2270); :1281-1282 (:2273-2274); :1278 (:372-377); progress track and radius :1302/:1305 (:769-783).

Derivation caveats (probe):
- :1170 and :1234 add the 14 px font size where the text line box belongs, so they undercount.
- :1293: the thumb template's Margin=-2 likely draws 22, not 18.
- :1174's 8 is DropDownButton's chevron gap, not a Button icon-to-label gap.

Probe items: caption/menu/status font weights per version; what the unsupported COLOR_* return; the caption colour vs DwmGetColorizationColor; the Win32 scrollbar track and thumb colours and thumb width; caret colour.

Scratch: /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/m2b-widgets/ (muxc blobless clone, p8463/, r251/, r1812/, w2/, web/).

</details>

<details><summary>widgets-2</summary>

I checked the whole scope (lines 1401–1647), with no gaps. Step 1's version map and pins are correct: all 17 WinUI 3 files used are identical at 8463f45, 2.5.1, 1.8.12 and main. New findings, none of them on todo.md: (1) :1405, the popover background differs. WinUI's FlyoutPresenter uses AcrylicInAppFillColorDefaultBrush (L #FCFCFC tint / #F9F9F9 fallback, D #2C2C2C), not defaults.surface_color (CardBackgroundFillColorDefault, :1050). (2) :1545/:1547, the ComboBox background and text use Win32 COLOR_BTNFACE/BTNTEXT, while the rest of §2.24 is WinUI 3. WinUI uses ControlFillColorDefault and TextFillColorPrimary. (3) :1473 names a key, 'ToggleSwitchKnob', that does not exist. (4) :1419 is right that the divider is 1 px, but its brush is Transparent in WinUI 3. Windows 10 v1 binds no pane border at all. So ← border.color at :1420 does not match WinUI. The reader/preset mismatch is already on todo :2279. (5) :1495: the dialog title font is XamlAutoFontFamily, which is Segoe UI Variable on Windows 11, not Segoe UI. (6) Card :1590-1593 is unverifiable: WinUI has no Card control. The layering doc gives Card 'Elevation 8, Stroke 1', which supports :1589 and raises doubt about :1591's 'no shadow'. Windows 10 (WinUI 2 v1) values often differ, and the cells name no version: dialog padding top 18, button gap 4, title weight 400, overlay radius 4; switch knob 10 with no hover growth; ComboBox arrow column 32; text area padding 6 top / 5 bottom; legacy type ramp Subtitle 20/400 and Title 24/350 with no Display style. Several Windows 10 values (flyout padding 12,11,12,12, type ramp, HyperlinkButton) come from the legacy generic.xaml copy inside WinUI 3, because Windows 10's own generic.xaml is not public. Inline Hyperlink is underlined by default (:1624 is right only for HyperlinkButton). Scratch clone: /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/m2b-216/ (w3/ = WinUI 3 files, w2/ = WinUI 2.8.7 files).

</details>

## M2c — iOS

**Status:** Steps 1, 2, 4, 5 and 6 done 2026-10-11, beside M1 (D7), in the `m2-research` worktree; Step 3 awaits the maintainer; the iOS 26 gap waits for M1's stop point A; Step 7 not started. One Workflow ran all five parts (41 Opus agents at high effort, 5.41M subagent tokens, 36.5 min). Triage values: *fix* (Step 7), *Step 8* / *Step 13* (the part's build steps, already decided), *maintainer (Step 3 probe)*, *maintainer (version gap)*, *M4* (reader code), *defer* (an existing docs/todo.md entry), *drop*.

`ios.toml` names no iOS version; its colours are the flat-generation (iOS 13–18) palette exactly (HIG `color.json` as captured
2025-06-07, WebKit's `platform/ios-18` baselines), and its fonts and Dynamic Type sizes are unchanged in iOS 26. iOS 26 (Liquid
Glass) changed the system colours (blue #0088ff, red #ff383c, orange #ff8d28 light), the separator alpha (0.29 → 0.12) and,
in WebKit's rendering, the switch and progress bar; no source separates 27 from 26. iOS 18 still gets security fixes only for
devices that cannot run 26. Which version `ios` names is M1 T1/T2's question (D5). The preset has no facts column, so 225
of its 330 keys have no primary source (desktop-only keys and UIKit metrics Apple does not publish); the research used the
HIG JSON, UIKit's documentation and WebKit's iOS rendering (the **(WebKit)** tag `platform-facts.md` accepts).

### Step 1 — version map

| Version | Released | Support on 2026-10-11 | Generation |
|---|---|---|---|
| iOS 17 iOS 17 / iPadOS 17 | 18 Sep 2023 (iOS 17 and iPadOS 17 row, https://support.apple.com/en-us/121012 'Apple security updates (2022 to 2023)', read 2026-10-11) | Ended for iPhone: last iPhone release iOS 17.7.2 and iPadOS 17.7.2, 19 Nov 2024. iPad only: iPadOS 17.7.11 on 11 May 2026 for iPad Pro 12.9-inch 2nd gen, iPad Pro 10.5-inch and iPad 6th gen (https://support.apple.com/en-us/100100, read 2026-10-11). Apple publi… | flat (iOS 7–18) |
| iOS 18 iOS 18 / iPadOS 18 | 16 Sep 2024 (https://support.apple.com/en-us/100100) | Security fixes for devices that cannot run iOS 26 (iPhone XS, XS Max, XR, iPad 7th gen): iOS 18.7.10, 17 Aug 2026. The last iOS 18 release for iPhone 11 and later was 18.7.8 on 22 Apr 2026. No iOS 18 release followed iOS 27 (14 Sep 2026) as of 2026-10-11 (http… | flat (iOS 7–18) |
| iOS 26 iOS 26 / iPadOS 26 | 15 Sep 2025 (https://support.apple.com/en-us/100100); announced 9 Jun 2025 (https://www.apple.com/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new… | In support: iOS 26.7.1 and iPadOS 26.7.1 shipped 28 Sep 2026 for iPhone 11 and later and iPad models down to iPad 8th gen, iPad mini 5th gen, iPad Air 3rd gen and iPad Pro 3rd gen. Some of those iPads cannot run iPadOS 27 (https://support.apple.com/en-us/10010… | Liquid Glass (iOS 26+) |
| iOS 27 iOS 27 / iPadOS 27 (current) | 14 Sep 2026; 27.0.1 on 28 Sep 2026 (https://support.apple.com/en-us/100100). The page says 'The latest version of iOS and iPadOS is 27.0.1.' Announced 8 Jun 202… | Current version, in support. Runs on iPhone 11 and later and iPad 9th gen and later, iPad mini 6th gen, iPad Air 4th gen and iPad Pro 4th gen (12.9-inch) / 2nd gen (11-inch) (iOS 27 row, https://support.apple.com/en-us/100100). | Liquid Glass (iOS 26+), refined |

Generations:

- **Flat redesign (iOS 7–18)** (iOS 7 (2013) to iOS 18 (2024)): Apple's iOS 7 press release (June 10, 2013) says iOS 7 is 'completely redesigned with an entirely new user interface … subtle motion, an elegant color palette and distinct, functional layers'. 'Flat' is the common name for the look, not Apple's word. The system colour values the HIG listed before 9 Jun 2025 belong to this generation (iOS default light/dark): systemBlue 0,122,255 / 10,132,255; systemRed 255,59,48 / 255,69,58; systemOrange 255,149,0 / 255,159,10; systemGreen 52,199,89 / 48,209,88. Source: HIG color.json, Wayback capture 20250607225442, image alt text ios-default-system*.png; its change log ends at February 2, 2024. In WebKit, iOS form controls use the pre-refresh code: ENABLE_… Source: https://www.apple.com/newsroom/2013/06/10Apple-Unveils-iOS-7/ ; https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json ; https://github.com/WebKit/WebKit/blob/1f67101144076ec393082cf141c18924a0823db8/Source/WTF/wtf/PlatformEnableCocoa.h#L1103-L1110
- **Liquid Glass (iOS 26+)** (iOS 26 (Sep 2025), iOS 27 (Sep 2026)): iOS 26: a new material, Liquid Glass. 'Controls, toolbars, and navigation within apps have been redesigned … concentric with the rounded corners', and tab bars shrink on scroll (newsroom, 9 Jun 2025). The HIG color page change log, entry June 9, 2025, says 'Updated system color values, and added guidance for Liquid Glass'. Current iOS values (light/dark): Blue 0,136,255 / 0,145,255; Red light 255,56,60; Orange light 255,141,40; Green light 52,199,89 (unchanged). Source: HIG color.json read 2026-10-11, alt text colors-unified-*.png. WebKit: the form-control refresh (LiquidGlass switch etc., RenderThemeCocoa.mm:411+) is on for iOS >= 26. iOS 27 is a refinement, not a new generation. Apple: 'Re… Source: https://www.apple.com/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/ ; https://www.apple.com/newsroom/2026/06/apple-unveils-next-generation-of-apple-intelligence-siri-ai-and-more/ ; https://www.apple.com/newsroom/2026/09/major-updates-for-apples-software-platforms-are-now-available/ ; https://developer.apple.com/tutorials/data/design/human-interface-guidelines/colo…

Pinned sources:

- Apple security releases (current list): https://support.apple.com/en-us/100100 — read 2026-10-11; newest entry iOS 27.0.1 / 26.7.1, 28 Sep 2026; covers release dates and security-update status for iOS 17, 18, 26 and 27 (2024 onward)
- Apple security updates (2022 to 2023): https://support.apple.com/en-us/121012 — read 2026-10-11; covers iOS 17.0 release date (18 Sep 2023) and 17.0.x–17.2.1
- Apple App Store iOS/iPadOS usage: https://developer.apple.com/support/app-store/ — read 2026-10-11; data 'measured by devices that transacted on the App Store on June 7, 2026'; covers usage share: iOS 26 / 18 / earlier
- Apple Newsroom: Liquid Glass announcement: https://www.apple.com/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/ — datePublished 2025-06-09; covers the iOS 26 generation boundary
- Apple Newsroom: WWDC 2026 (2027 releases): https://www.apple.com/newsroom/2026/06/apple-unveils-next-generation-of-apple-intelligence-siri-ai-and-more/ — datePublished 2026-06-08; covers iOS 27 design: Liquid Glass refinements, ultra-clear to fully tinted slider
- Apple Newsroom: iOS 27 release: https://www.apple.com/newsroom/2026/09/major-updates-for-apples-software-platforms-are-now-available/ — datePublished 2026-09-14; covers iOS 27 release, design refinements
- Apple Newsroom: iOS 7: https://www.apple.com/newsroom/2013/06/10Apple-Unveils-iOS-7/ — dated June 10, 2013; covers the iOS 7 redesign that starts the flat generation
- HIG Color (JSON), current: https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json — read 2026-10-11; change log newest December 16, 2025; sha256 prefix 262ce33cb88d41ac; covers iOS 26+ system colours (alt text of colors-unified-*.png) and the iOS system grays
- HIG Color (JSON), pre-iOS-26 capture: https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json — Wayback capture 2025-06-07 22:54:42 UTC; change log newest February 2, 2024; covers iOS 7–18-era system colours (ios-default-*/ios-accessible-*.png alt text); archived copy of the vendor page
- HIG Typography (JSON): https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json — read 2026-10-11; change log newest December 16, 2025 (emphasized weights added); sha256 prefix 85bf7dc78d41a6e5; covers iOS/iPadOS Dynamic Type sizes, 'Large (default)' tab (Large Title 34/41 … Caption 2 11/13), plus the AX sizes and tracking
- HIG Layout (JSON): https://developer.apple.com/tutorials/data/design/human-interface-guidelines/layout.json — read 2026-10-11; change log newest September 9, 2026 ('Updated guidance to reflect current best practices'); sha256 prefix d41f32ddf76c597f; covers iOS layout and device specifications
- HIG Materials (JSON): https://developer.apple.com/tutorials/data/design/human-interface-guidelines/materials.json — read 2026-10-11; change log newest September 9, 2025; covers Liquid Glass material guidance
- UIKit UIColor documentation (JSON): https://developer.apple.com/tutorials/data/documentation/uikit/uicolor/systemblue.json — read 2026-10-11; covers API names and availability only (systemBlue iOS 7.0+). It gives no RGB values, so the colour values come from the HIG
- WebKit main: https://github.com/WebKit/WebKit/tree/de320ba3ff90229f983988d42d4b0f83315459da — commit de320ba3ff90229f983988d42d4b0f83315459da (2026-10-11T00:07:54Z); RenderThemeIOS.mm 2009 lines, last touched by a2b7a1b2 (2026-09-18); RenderThemeCocoa.mm 5406 lines, last touched by 6cb05bb8 (2026-10-08); covers iOS 26+ form-control constants: Source/WebCore/rendering/ios/RenderThemeIOS.mm (e.g. :259, :382-383, :660-662, :1454-1455, :1573-1574) and rendering/cocoa/RenderThemeCocoa.mm (LiquidGlass switch from…
- WebKit tag WebKit-7622.2.11.14.6: https://github.com/WebKit/WebKit/blob/1f67101144076ec393082cf141c18924a0823db8/Source/WTF/wtf/PlatformEnableCocoa.h#L1103-L1110 — tag WebKit-7622.2.11.14.6 = 1f67101144076ec393082cf141c18924a0823db8 (2025-10-23); covers shows the refresh is gated on iOS >= 26.0 (__IPHONE_OS_VERSION_MIN_REQUIRED >= 260000), the WebKit-side 18→26 boundary
- WebKit tag WebKit-7621.2.5.14.9 (pre-refresh): https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm — tag WebKit-7621.2.5.14.9 = 31adf9d1605fad1fb548e86d36384161a2a060b4 (2025-05-03); RenderThemeIOS.mm 1868 lines; no FORM_CONTROL_REFRESH; covers pre-Liquid-Glass iOS form-control constants. No primary source found here that maps this tag to a specific iOS 18.x release

Preset target: The preset names no iOS version. /home/tibi/Rust/native-theme-m2/native-theme/src/presets/ios.toml:1-2 reads '# iOS system theme' / '# Source: Apple UIColor system colors, Human Interface Guidelines', with name = "iOS" at :3. presets/README.md:57 calls it 'iOS/iPadOS standalone theme' and :20-21 notes it has no platform-facts column. Its colours match the pre-iOS-26 HIG palette (flat generation, iOS ≤18), not the current one. Light: accent #007aff (:10) = 0,122,255; danger #ff3b30 (:17) = 255,59,48; warning #ff9500 (:19) = 255,149,0; success #34c759 (:21). Dark: #0a84ff (:258), #ff453a (:265), #ff9f0a (:267), #30d158 (:269). These equal the HIG ios-default-* values in the 2025-06-07 capture. The current HIG (iOS 26+) gives Blue 0,136,255 / 0,145,255, Red 255,56,60 and Orange 255,141,40. In effect the preset targets iOS 13–18, though nothing in the file says so. ios.toml has 26 commits; t…

<details><summary>Version-map notes</summary>

The iOS 27 design is a refinement of Liquid Glass (user opacity slider, sharper icons). Neither the HIG change logs nor WebKit gates show colour, type or control changes for 27, so 26 and 27 are one generation. Apple publishes no iOS lifecycle policy: support status here comes only from the release rows on the security page. One correction: the WebFetch summary gave iOS 26.5.1 as 16 Jun 2026; the raw page says 01 Jun 2026, and the raw page was used. Secondary only (third-party WWDC 2026 coverage, e.g. macmyths.com, feedbagel.com) agrees that iOS 27 keeps Liquid Glass. It is not cited. Working files are in /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/ios-map/ (sec.html, hig/*.json incl. color-20250607.json, webkit/*.mm). Not checked: individual facts rows. Not mapped: WebKit tags to specific iOS point releases (no primary source).

</details>

### Step 2 — rows checked

330 rows: 51 same, 54 differs, 225 unverifiable (133 of them a probe or reference app could settle). Scopes: preset 330. Every researcher reported its scope covered with no line range left out.

Step 5 verdicts on the *differs* rows: 29 CONFIRMED, 21 PLAUSIBLE, 4 REJECTED.

Step 6 triage: 8 drop, 27 fix, 19 maintainer (version gap, after M1 stop point A).

### Findings — *differs* rows

#### M2c-01 · bug · S2 · native-theme/src/presets/ios.toml:10
**Claim:** `defaults.accent_color` (): the cell says #007aff; flat iOS 18: #007aff; Liquid Glass iOS 26: #0088ff.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Blue, Light) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Blue, light) Note: systemBlue. Matches iOS 13-18. iOS 26 changed it to #0088ff. WebKit baselines agree (LT18/LT26 L12-16).
**Verdict:** CONFIRMED — Archived HIG color.json (2025-06-07) iOS Blue Light alt R-0,G-122,B-255; current color.json 'System colors' Blue light R-0,G-136,B-255, corroborated by WebKit de320ba3 LayoutTests/platform/ios/.../system-color-for-css-value-expected.txt:13 rgb(0,136,255) vs platform/ios-18 :13 rgb(0,122,255); only the iOS 26 value differs from the cell. Confirmed value: iOS 18 #007aff (= cell); iOS 26 #0088ff.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-02 · bug · S2 · native-theme/src/presets/ios.toml:14
**Claim:** `defaults.muted_color` (): the cell says #8e8e93; flat iOS 18: rgba(60,60,67,0.6); on white #8a8a8e; Liquid Glass iOS 26: rgba(60,60,67,0.6); on white #8a8a8e.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L144-L148 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L144-L148 Note: The macOS column maps muted to secondaryLabelColor, and iOS secondaryLabelColor is unchanged from 18 to 26. The preset uses systemGray instead (HIG gray table), which is a different role.
**Verdict:** CONFIRMED — Both expected.txt files :144-148 read -apple-system-secondary-label rgba(60,60,67,0.6) light (dark rgba(235,235,245,0.6)); compositing on #ffffff gives #8a8a8e, and secondaryLabel is the macOS-column mapping for muted_color (platform-facts.md:1052), so the cell's #8e8e93 (systemGray) is the wrong role. Confirmed value: secondaryLabel rgba(60,60,67,0.6) → #8a8a8e on white, iOS 18 and iOS 26.
**Triage:** fix — muted_color maps to secondaryLabel (platform-facts.md:1052, the macOS mapping); the cell is systemGray. Same at iOS 18 and 26.
**Status:** open

#### M2c-03 · bug · S2 · native-theme/src/presets/ios.toml:17
**Claim:** `defaults.danger_color` (): the cell says #ff3b30; flat iOS 18: #ff3b30; Liquid Glass iOS 26: #ff383c.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Red, Light) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Red, light) Note: systemRed. Matches iOS 13-18. iOS 26 changed it to #ff383c (WebKit L126-130 agrees).
**Verdict:** CONFIRMED — Archived HIG iOS Red Light R-255,G-59,B-48; current HIG Red light R-255,G-56,B-60, matching WebKit platform/ios expected.txt:127 rgb(255,56,60); systemRed is the mapping at platform-facts.md:1080. Confirmed value: iOS 18 #ff3b30 (= cell); iOS 26 #ff383c.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-04 · bug · S2 · native-theme/src/presets/ios.toml:19
**Claim:** `defaults.warning_color` (): the cell says #ff9500; flat iOS 18: #ff9500; Liquid Glass iOS 26: #ff8d28.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Orange, Light) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Orange, light) Note: systemOrange. Matches iOS 13-18. iOS 26 changed it to #ff8d28 (WebKit L96-100 agrees).
**Verdict:** CONFIRMED — Archived HIG iOS Orange Light R-255,G-149,B-0; current HIG Orange light R-255,G-141,B-40, matching WebKit platform/ios expected.txt:97 rgb(255,141,40); systemOrange is the mapping at platform-facts.md:1082. Confirmed value: iOS 18 #ff9500 (= cell); iOS 26 #ff8d28.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-05 · bug · S2 · native-theme/src/presets/ios.toml:23
**Claim:** `defaults.info_color` (): the cell says #007aff; flat iOS 18: #007aff; Liquid Glass iOS 26: #0088ff.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Blue, Light) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Blue, light) Note: systemBlue, as in the macOS column. iOS 26 changed it to #0088ff.
**Verdict:** CONFIRMED — Same Blue values as index 0 (HIG archived/current, WebKit expected.txt:13); systemBlue is the mapping at platform-facts.md:1086. Confirmed value: iOS 18 #007aff (= cell); iOS 26 #0088ff.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-06 · bug · S2 · native-theme/src/presets/ios.toml:29
**Claim:** `defaults.focus_ring_color` (): the cell says #007aff; flat iOS 18: #007aff (systemBlue); Liquid Glass iOS 26: #0088ff (systemBlue).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L936-L943 ; https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Blue, Light) [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L1027-L1034 ; https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Blue, light) Note: WebKit's iOS focus ring is systemBlueColor. The preset matches iOS 18 but not iOS 26.
**Verdict:** CONFIRMED — RenderThemeIOS.mm 31adf9d1:936-943 and de320ba3:1027-1034 set the focus ring to UIColor systemBlueColor (with a FIXME that it should be keyboardFocusIndicatorColor), and systemBlue is #0088ff in iOS 26 per HIG and WebKit expected.txt:13. Confirmed value: systemBlue: iOS 18 #007aff (= cell); iOS 26 #0088ff.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-07 · bug · S2 · native-theme/src/presets/ios.toml:30
**Claim:** `defaults.disabled_text_color` (): the cell says #c7c7cc; flat iOS 18: tertiaryLabel rgba(60,60,67,0.298); on white #c5c5c7; Liquid Glass iOS 26: tertiaryLabel rgba(60,60,67,0.298); on white #c5c5c7.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L996-L1003 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L983-L984 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 Note: UIKit has no disabled-text colour. The only primary source is WebKit's iOS disabled button text, which is tertiaryLabelColor (WebKit tag). The preset uses systemGray3.
**Verdict:** PLAUSIBLE — html.css 31adf9d1:996-1003 and de320ba3:979-986 (inside the iOS-family #else) set disabled button/select color to -apple-system-tertiary-label, read as rgba(60,60,67,0.298) at expected.txt:180-184 in both versions, but this is WebKit's form-control styling and UIKit has no disabledControlTextColor equivalent, so mapping it to the row needs a UIKit measurement; note also the WebKit button sits on opaque-secondary-fill-disabled, not white. Confirmed value: WebKit iOS disabled button/select text = tertiaryLabel rgba(60,60,67,0.298) → #c5c5c7 on white.
**Triage:** fix — PLAUSIBLE: WebKit's iOS html.css draws disabled controls in tertiaryLabel (a UIKit colour), identical at iOS 18 and 26; the cell #c7c7cc / #3a3a3c is systemGray3 with no source. Fixed with the (WebKit) tag platform-facts.md accepts.
**Status:** open

#### M2c-08 · bug · native-theme/src/presets/ios.toml:34
**Claim:** `defaults.line_height` (): the cell says 1.2; flat iOS 18: 1.294 (Body 17 pt / leading 22); Liquid Glass iOS 26: 1.294 (Body 17 pt / leading 22).
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Body) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Body) Note: HIG Body leading is unchanged. The preset's 1.2 matches neither. The macOS column uses SF Pro font metrics (1.19), a different method.
**Verdict:** PLAUSIBLE — Both typography.json versions' 'iOS, iPadOS Dynamic Type sizes / Large (default)' table read Body Regular 17 / 22, but platform-facts.md:1030 and :997 define line_height from font metrics for Apple (SF Pro 1.19, treating HIG per-style leading as a guideline), so 1.2 vs 1.294 depends on the HIG leading vs the UIFont lineHeight that UIKit lays out with, which needs a measurement. Confirmed value: HIG Body 17 pt / leading 22 (ratio 1.294), unchanged iOS 18 → 26.
**Triage:** drop — platform-facts.md:1030 defines line_height from font metrics for Apple, not from the HIG leading; the cell follows that rule.
**Status:** open

#### M2c-09 · bug · S2 · native-theme/src/presets/ios.toml:40
**Claim:** `defaults.border.color` (): the cell says #c6c6c8; flat iOS 18: separator rgba(60,60,67,0.29); on white #c6c6c8; Liquid Glass iOS 26: separator rgba(60,60,67,0.12); on white #e8e8e8.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 Note: The preset is exactly separatorColor composited on white at iOS 18. iOS 26 lowered the alpha to 0.12. UIKit opaqueSeparatorColor has no published RGB; WebKit composites separatorColor itself (RenderThemeIOS.mm:1162).
**Verdict:** CONFIRMED — expected.txt:156-160 reads separator rgba(60,60,67,0.29) in platform/ios-18 and rgba(60,60,67,0.12) in platform/ios (dark 0.6 → 0.5), separatorColor is the mapping at platform-facts.md:1051, and the white composites compute to #c6c6c8 and #e8e8e8; only iOS 26 differs from the cell. Confirmed value: iOS 18 separator rgba(60,60,67,0.29) → #c6c6c8 (= cell); iOS 26 rgba(60,60,67,0.12) → #e8e8e8 on white.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 separator on white is the cell's #c6c6c8; iOS 26 lowered the alpha to 0.12. iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-10 · bug · native-theme/src/presets/ios.toml:44
**Claim:** `defaults.border.opacity` (): the cell says 0.15; flat iOS 18: 0.29 (separator alpha); Liquid Glass iOS 26: 0.12 (separator alpha).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 Note: iOS line opacity is the separatorColor alpha, over 60,60,67 rather than the label colour. The preset's 0.15 matches neither version.
**Verdict:** REJECTED — The separator alphas 0.29 / 0.12 (expected.txt:157 in each file) are read correctly, but they are the alpha of the separator's own base rgb(60,60,67), while platform-facts.md:946 and :997 define border.opacity as the share of the text colour (#000000, ios.toml:12) folded into border.color; as that share, #c6c6c8 on white is about 0.22 (iOS 18) and #e8e8e8 about 0.09 (iOS 26), so 0.29/0.12 do not belong in this field as claimed.
**Triage:** drop — REJECTED: border.opacity is the text colour's share in a line (platform-facts.md:946, :997), not the separator's alpha. The preset's 0.15 is wrong on its own grounds (validator finding below).
**Status:** open

#### M2c-11 · bug · S2 · native-theme/src/presets/ios.toml:66
**Claim:** `text_scale.caption.line_height_pt` (): the cell says 14.4; flat iOS 18: 16; Liquid Glass iOS 26: 16.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Caption 1 leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Caption 1 leading) Note: The preset uses size x 1.2.
**Verdict:** CONFIRMED — Archived (2025-05-23) and current typography.json, iOS Large (default) table: Caption 1 Regular 12 / 16; the cell's 14.4 is just 12 × 1.2. Confirmed value: 16 (Caption 1, 12 pt / leading 16), iOS 18 and iOS 26.
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-12 · bug · S2 · native-theme/src/presets/ios.toml:69
**Claim:** `text_scale.section_heading.size_pt` (): the cell says 15.0; flat iOS 18: 17; Liquid Glass iOS 26: 17.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Headline) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Headline) Note: Mapped as in the macOS column: .headline becomes iOS Headline. Subhead (15, Regular, leading 20) is the alternative.
**Verdict:** CONFIRMED — Both typography.json versions read Headline Semibold 17 / 22 in iOS Large (default), and .headline is the style platform-facts.md:1436 maps section_heading to on macOS; the same rows also contradict the cell's weight 700 (Semibold = 600) and line_height_pt 18 (ios.toml:70-71). Confirmed value: 17 (Headline, Semibold, leading 22), iOS 18 and iOS 26.
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-13 · bug · S2 · native-theme/src/presets/ios.toml:70
**Claim:** `text_scale.section_heading.weight` (): the cell says 700; flat iOS 18: 600 (Semibold); Liquid Glass iOS 26: 600 (Semibold).
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Headline) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Headline) Note: No iOS text style is Bold (700) at the default size. Subhead is Regular.
**Verdict:** CONFIRMED — Both the archived (2025-05-23) and the live (read 2026-10-11) HIG typography.json give Headline = Semibold, 17/22, in the iOS Large (default) table; the role mapping follows §2.19's macOS column (section_heading = .headline), since platform-facts has no iOS column. Confirmed value: 600 (Semibold), iOS 18 and iOS 26.
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-14 · bug · S2 · native-theme/src/presets/ios.toml:71
**Claim:** `text_scale.section_heading.line_height_pt` (): the cell says 18.0; flat iOS 18: 22; Liquid Glass iOS 26: 22.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Headline leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Headline leading) Note: The preset uses size x 1.2. Subhead's leading would be 20.
**Verdict:** CONFIRMED — HIG JSON Large (default): Headline leading 22 in both snapshots; note that ios.toml:69 size_pt 15 also disagrees with Headline's 17, which the claim does not cover. Confirmed value: 22 (both).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-15 · bug · S2 · native-theme/src/presets/ios.toml:74
**Claim:** `text_scale.dialog_title.size_pt` (): the cell says 22.0; flat iOS 18: 28; Liquid Glass iOS 26: 28.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Title 1) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Title 1) Note: Mapped as in the macOS column: .title1 becomes iOS Title 1. The preset's 22 equals iOS Title 2 (22, Regular, leading 28).
**Verdict:** CONFIRMED — HIG JSON Large (default): Title 1 is Regular 28/34 in both snapshots; dialog_title = .title1 per §2.19's macOS column, and ios.toml's 22 is the macOS title1 size. Confirmed value: 28 (both).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-16 · bug · S2 · native-theme/src/presets/ios.toml:76
**Claim:** `text_scale.dialog_title.line_height_pt` (): the cell says 26.4; flat iOS 18: 34; Liquid Glass iOS 26: 34.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Title 1 leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Title 1 leading) Note: The preset uses size x 1.2. Title 2's leading would be 28.
**Verdict:** CONFIRMED — HIG JSON Large (default): Title 1 leading 34 in both snapshots; the cell 26.4 is 22×1.2, computed rather than sourced. Confirmed value: 34 (both).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-17 · bug · S2 · native-theme/src/presets/ios.toml:81
**Claim:** `text_scale.display.line_height_pt` (): the cell says 40.8; flat iOS 18: 41; Liquid Glass iOS 26: 41.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Large Title leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Large Title leading) Note: The preset uses size x 1.2.
**Verdict:** CONFIRMED — HIG JSON Large (default): Large Title is 34/41 in both snapshots; the cell 40.8 is 34×1.2, computed. Confirmed value: 41 (both).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-18 · bug · S2 · native-theme/src/presets/ios.toml:84
**Claim:** `button.background_color` (): the cell says #e5e5ea; flat iOS 18: opaque-secondary-fill #e9e9ea; Liquid Glass iOS 26: opaque-secondary-fill #e9e9ea.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L895 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L84-L88 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L885 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L84-L88 Note: The value is WebKit's iOS HTML button fill (WebKit tag); UIKit publishes no button fill RGB. The preset uses systemGray5.
**Verdict:** PLAUSIBLE — The value is read correctly: html.css 31adf9d:895 and de320ba:885 set the iOS HTML button background to -apple-system-opaque-secondary-fill, and expected.txt:85 gives rgb(233,233,234) for both iOS 18 and iOS 26; but this is WebKit's own styling of web-content buttons (11px system-ui at :890/:880), not UIButton, so the mapping needs a UIKit source or a measurement.
**Triage:** fix — PLAUSIBLE: WebKit's iOS button fill is opaque-secondary-fill (iOS 18 = iOS 26); the cell has no source. (WebKit)-tagged.
**Status:** open

#### M2c-19 · bug · S2 · native-theme/src/presets/ios.toml:93
**Claim:** `button.disabled_text_color` (): the cell says #c7c7cc; flat iOS 18: tertiaryLabel rgba(60,60,67,0.298); on white #c5c5c7; Liquid Glass iOS 26: tertiaryLabel rgba(60,60,67,0.298); on white #c5c5c7.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L996-L1003 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L983-L984 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 Note: Same source as line 30 (WebKit tag).
**Verdict:** PLAUSIBLE — The value is read correctly: html.css 31adf9d:1001 and de320ba:984 set disabled HTML buttons to -apple-system-tertiary-label, which is rgba(60,60,67,0.298) at expected.txt:181 in both baselines, ≈#c5c5c7 on white; but it is WebKit's web-button styling, not UIButton's disabled state.
**Triage:** fix — PLAUSIBLE: WebKit's iOS html.css draws disabled controls in tertiaryLabel (a UIKit colour), identical at iOS 18 and 26; the cell #c7c7cc / #3a3a3c is systemGray3 with no source. Fixed with the (WebKit) tag platform-facts.md accepts.
**Status:** open

#### M2c-20 · bug · S2 · native-theme/src/presets/ios.toml:97
**Claim:** `button.font.color` (): the cell says #000000; flat iOS 18: #007aff (systemBlue); Liquid Glass iOS 26: #0088ff (systemBlue).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L891 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L881 Note: WebKit draws iOS button text in the tint colour (systemBlue), not the label colour (WebKit tag). No UIKit document gives this.
**Verdict:** PLAUSIBLE — html.css 31adf9d:891 and de320ba:881 set the iOS HTML button colour to -apple-system-blue, which is rgb(0,122,255) in ios-18 expected.txt:13 and rgb(0,136,255) in ios expected.txt:13 (the iOS 26 hex comes from there, not from html.css:881); this is a proxy from WebKit form controls, not a UIButton source.
**Triage:** fix — PLAUSIBLE: an iOS button's label is the tint (systemBlue) in WebKit's iOS html.css; the cell's black / white has no source. (WebKit)-tagged; the iOS 26 blue follows the version-gap decision.
**Status:** open

#### M2c-21 · bug · native-theme/src/presets/ios.toml:135
**Claim:** `slider.thumb_diameter_px` (): the cell says 28.0; flat iOS 18: 16; Liquid Glass iOS 26: 24 x 16 (not round).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L562 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3881 Note: These are WebKit's HTML range thumb sizes only. UISlider's thumb size is not published, so a probe should decide.
**Verdict:** REJECTED — RenderThemeIOS.mm 31adf9d:562 is kDefaultSliderThumbSize=16, applied at :659-660 to WebKit's HTML <input type=range> thumb; RenderThemeCocoa.mm de320ba:3881 sliderThumbSize {24,16} is WebKit's form-control-refresh metric, which has no iOS/Mac conditional (the track at :3875-3879 has one); both are web-content control sizes, not UISlider's thumb.
**Triage:** drop — REJECTED: 16 and 24 x 16 are WebKit's HTML range thumb, not UISlider; the cell 28 has no source either (stays unverifiable).
**Status:** open

#### M2c-22 · bug · S2 · native-theme/src/presets/ios.toml:143
**Claim:** `progress_bar.track_height_px` (): the cell says 4.0; flat iOS 18: 4; Liquid Glass iOS 26: 8.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L718 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3770 Note: WebKit iOS progress bar thickness (WebKit tag). The preset matches iOS 18; the iOS 26 refreshed painter uses 8.
**Verdict:** REJECTED — The iOS 18 value, barBlockSize 4 at RenderThemeIOS.mm 31adf9d:718, equals the cell; the iOS 26 value, 8.f at RenderThemeCocoa.mm de320ba:3770, belongs to paintProgressBarForVectorBasedControls, WebKit's <progress> refresh shared by macOS and iOS (gated by the internal FormControlRefreshEnabled setting, UnifiedWebPreferences.yaml:2489-2497), not UIProgressView.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 value 4 equals the cell; the iOS 26 8 is WebKit's form-control-refresh <progress> (verdicts split: REJECTED at index 21 as not UIKit, CONFIRMED at 48 as WebKit), so the iOS 26 value is (WebKit)-only.
**Status:** open

#### M2c-23 · bug · S2 · native-theme/src/presets/ios.toml:185
**Claim:** `separator.line_color` (): the cell says #c6c6c8; flat iOS 18: separator rgba(60,60,67,0.29); on white #c6c6c8; Liquid Glass iOS 26: separator rgba(60,60,67,0.12); on white #e8e8e8.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 Note: Same as line 40: matches iOS 18, not iOS 26.
**Verdict:** CONFIRMED — -apple-system-separator is ios-18 expected.txt:157 rgba(60,60,67,0.29) and ios expected.txt:157 rgba(60,60,67,0.12), the UIColor.separator test baselines; only iOS 26 differs from the cell, and opaqueSeparator is not in the file. Confirmed value: iOS 18: rgba(60,60,67,0.29) ≈ #c6c6c8 (equals the cell); iOS 26: rgba(60,60,67,0.12) ≈ #e8e8e8 on white.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-24 · bug · S2 · native-theme/src/presets/ios.toml:192
**Claim:** `switch.unchecked_background` (): the cell says #c6c6c8; flat iOS 18: rgba(120,120,128,0.16); on white #e9e9eb; Liquid Glass iOS 26: tertiaryLabel rgba(60,60,67,0.298) over canvas; #c5c5c7.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L276-L279 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L281-L294 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 Note: WebKit switch off-track colour (WebKit tag). The iOS 18 value is cited from main's non-refreshed path, because the iOS-18-era tag painted switches only in the internal SDK. The preset matches neither version.
**Verdict:** PLAUSIBLE — RenderThemeCocoa.mm de320ba:277 hardcodes the off colour as rgba(120,120,128,41/255) ≈ #e9e9eb, and under form-control refresh (:282-295) it is tertiaryLabel composited over the canvas ≈ #c5c5c7, which is 1 unit from the cell #c6c6c8; :269's FIXME says UIKit does not expose _switchOffColor, so these are WebKit's approximations of UISwitch and need a measurement.
**Triage:** fix — PLAUSIBLE: WebKit's classic iOS switch off-track is rgba(120,120,128,0.16 / 0.32), not the cell's separator grey; iOS 26 replaces it (version gap).
**Status:** open

#### M2c-25 · bug · S2 · native-theme/src/presets/ios.toml:199
**Claim:** `switch.track_width_px` (): the cell says 51.0; flat iOS 18: 51; Liquid Glass iOS 26: 70.86 (31 x 64/28).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L668-L669 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L417-L418 Note: WebKit tag. 51x31 before the refresh, which matches the preset. The Liquid Glass switch is an ideal 64x28 shape scaled to a height of 31 (applied at :4515-4537).
**Verdict:** PLAUSIBLE — RenderThemeIOS.mm@31adf9d L668-669 gives 51x31 (so iOS 18 matches the cell), and RenderThemeCocoa.mm@de320ba L417-418 gives logicalRefreshedSwitchWidth = 31*64/28, but that only holds under ENABLE(FORM_CONTROL_REFRESH) with formControlRefreshEnabled(); the same source's own native proportion is idealSwitchSize {64, 28}, and 70.86 is WebKit keeping its old 31 pt height, so the UISwitch width needs a measurement. Confirmed value: iOS 18: 51 (same as the cell); iOS 26 (WebKit refreshed switch): 31*64/28 = 70.86.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 value equals the cell; the iOS 26 Liquid Glass switch is WebKit's rescaling, (WebKit)-only.
**Status:** open

#### M2c-26 · bug · S2 · native-theme/src/presets/ios.toml:201
**Claim:** `switch.thumb_diameter_px` (): the cell says 27.0; flat iOS 18: 27 (31 minus a 2 pt margin each side); Liquid Glass iOS 26: 39.86 x 26.57 capsule.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L746-L771 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L423 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L598-L612 Note: WebKit tag. The pre-refresh value matches (the source comment says 'Values chosen for consistency with UIKit'). The iOS 26 thumb is not round.
**Verdict:** PLAUSIBLE — RenderThemeCocoa.mm@de320ba L766-771 gives thumbLength = 31 - 2*2 = 27 for the classic switch, and L423 (6:7 height ratio on iOS) plus L600-609 (idealThumbWidth 36 / 64) give 39.86 x 26.57 at WebKit's 31 pt height; at the source's ideal 64x28 track that is 36 x 24, and it is a capsule, not a circle, so a 'diameter' row needs a measurement and a mapping decision. Confirmed value: iOS 18: 27 (same as the cell); iOS 26 WebKit refreshed thumb: 39.86 x 26.57 capsule.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 value equals the cell; the iOS 26 Liquid Glass switch is WebKit's rescaling, (WebKit)-only.
**Status:** open

#### M2c-27 · bug · S2 · native-theme/src/presets/ios.toml:258
**Claim:** `defaults.accent_color` (): the cell says #0a84ff; flat iOS 18: #0a84ff; Liquid Glass iOS 26: #0091ff.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Blue, Dark) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Blue, dark) Note: systemBlue in dark mode. Matches iOS 13-18. iOS 26 changed it to #0091ff.
**Verdict:** CONFIRMED — The archived HIG color.json (20250607) has alt 'R-10,G-132,B-255' for ios-default-systembluedark, and the live color.json (read 2026-10-11, change log 'June 9, 2025: Updated system color values') has 'R-0,G-145,B-255' for colors-unified-blue-dark; WebKit's ios/ and ios-18/ system-color expectations agree (rgb(0,145,255) vs rgb(10,132,255)). Confirmed value: iOS 18 dark #0a84ff (same as the cell); iOS 26 dark #0091ff.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-28 · bug · S2 · native-theme/src/presets/ios.toml:262
**Claim:** `defaults.muted_color` (): the cell says #8e8e93; flat iOS 18: rgba(235,235,245,0.6); on black #8d8d93; Liquid Glass iOS 26: rgba(235,235,245,0.6); on black #8d8d93.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L144-L148 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L144-L148 Note: Same as line 14.
**Verdict:** PLAUSIBLE — Both expectation files L144-148 read rgba(235,235,245,0.6) dark, and the macOS column maps muted_color to secondaryLabelColor (platform-facts.md:1052), but the cell #8e8e93 (systemGray dark, rgb(142,142,147) in both files L36-40) is only 1/255 away from the composite, and that composite holds only over pure black, not over the dark surface #1c1c1e. Confirmed value: secondaryLabel dark rgba(235,235,245,0.6) on iOS 18 and 26; over #000000 = #8d8d93.
**Triage:** fix — muted_color maps to secondaryLabel (platform-facts.md:1052, the macOS mapping); the cell is systemGray. Same at iOS 18 and 26.
**Status:** open

#### M2c-29 · bug · S2 · native-theme/src/presets/ios.toml:265
**Claim:** `defaults.danger_color` (): the cell says #ff453a; flat iOS 18: #ff453a; Liquid Glass iOS 26: #ff4245.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Red, Dark) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Red, dark) Note: systemRed in dark mode. iOS 26 changed it to #ff4245.
**Verdict:** CONFIRMED — The archived HIG JSON alt for ios-default-systemreddark is 'R-255,G-69,B-58', and the live JSON's colors-unified-red-dark is 'R-255,G-66,B-69'; WebKit's ios/ expectation also reads rgb(255,66,69). Confirmed value: iOS 18 dark #ff453a (same as the cell); iOS 26 dark #ff4245.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-30 · bug · S2 · native-theme/src/presets/ios.toml:267
**Claim:** `defaults.warning_color` (): the cell says #ff9f0a; flat iOS 18: #ff9f0a; Liquid Glass iOS 26: #ff9230.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Orange, Dark) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Orange, dark) Note: systemOrange in dark mode. iOS 26 changed it to #ff9230.
**Verdict:** CONFIRMED — The archived HIG JSON alt for ios-default-systemorangedark is 'R-255,G-159,B-10', and the live JSON's colors-unified-orange-dark is 'R-255,G-146,B-48'; WebKit's ios/ expectation also reads rgb(255,146,48). Confirmed value: iOS 18 dark #ff9f0a (same as the cell); iOS 26 dark #ff9230.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-31 · bug · S2 · native-theme/src/presets/ios.toml:271
**Claim:** `defaults.info_color` (): the cell says #0a84ff; flat iOS 18: #0a84ff; Liquid Glass iOS 26: #0091ff.
**Evidence:** [flat] https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Blue, Dark) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Blue, dark) Note: systemBlue in dark mode.
**Verdict:** CONFIRMED — info_color maps to systemBlue (platform-facts.md:1086, macOS column), and the HIG JSON alts are 'R-10,G-132,B-255' (archive 20250607) and 'R-0,G-145,B-255' (live, 2026-10-11). Confirmed value: iOS 18 dark #0a84ff (same as the cell); iOS 26 dark #0091ff.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-32 · bug · S2 · native-theme/src/presets/ios.toml:277
**Claim:** `defaults.focus_ring_color` (): the cell says #0a84ff; flat iOS 18: #0a84ff (systemBlue); Liquid Glass iOS 26: #0091ff (systemBlue).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L936-L943 ; https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Blue, Dark) [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L1027-L1034 ; https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (read 2026-10-11; System colors, Blue, dark) Note: Same as line 29.
**Verdict:** CONFIRMED — RenderThemeIOS::systemFocusRingColor uses [UIColor systemBlueColor] at 31adf9d L936-943 and at de320ba L1027-1034, with a FIXME saying it should use keyboardFocusIndicatorColor, which is WebKit's choice rather than UIKit's; the blue values are as in index 26. Confirmed value: systemBlue: iOS 18 dark #0a84ff (same as the cell); iOS 26 dark #0091ff.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-33 · bug · S2 · native-theme/src/presets/ios.toml:278
**Claim:** `defaults.disabled_text_color` (): the cell says #3a3a3c; flat iOS 18: tertiaryLabel rgba(235,235,245,0.298); on black #464649; Liquid Glass iOS 26: tertiaryLabel rgba(235,235,245,0.298); on black #464649.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L996-L1003 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L983-L984 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 Note: Same as line 30. The preset uses systemGray4 dark.
**Verdict:** PLAUSIBLE — html.css (31adf9d L996-1003, de320ba L979-986, inside the WTF_PLATFORM_IOS_FAMILY #else branch) colours disabled buttons and selects -apple-system-tertiary-label, and both expectation files L180-184 read rgba(235,235,245,0.298), but this is WebKit's form-control styling, not a UIKit disabled-text API, so the mapping needs confirmation. Confirmed value: tertiaryLabel dark rgba(235,235,245,0.298); over #000000 = #464649 (iOS 18 and 26).
**Triage:** fix — PLAUSIBLE: WebKit's iOS html.css draws disabled controls in tertiaryLabel (a UIKit colour), identical at iOS 18 and 26; the cell #c7c7cc / #3a3a3c is systemGray3 with no source. Fixed with the (WebKit) tag platform-facts.md accepts.
**Status:** open

#### M2c-34 · bug · native-theme/src/presets/ios.toml:282
**Claim:** `defaults.line_height` (): the cell says 1.2; flat iOS 18: 1.294 (Body 17 pt / leading 22); Liquid Glass iOS 26: 1.294 (Body 17 pt / leading 22).
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Body) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Body) Note: Same as line 34.
**Verdict:** PLAUSIBLE — Both typography.json files list Body Regular 17/22 in the iOS Large (default) table, but platform-facts.md:1030 rejects the HIG per-style leadings for macOS as 'design guidelines, not API values' and uses SF Pro font metrics (1.19), while :996 accepts a type ramp for Windows, so which convention iOS follows is a decision (the cell's 1.2 matches neither). Confirmed value: HIG iOS Large (default) Body 17 pt / leading 22 = 1.294 (archive 20250523 and live 2026-10-11).
**Triage:** drop — platform-facts.md:1030 defines line_height from font metrics for Apple, not from the HIG leading; the cell follows that rule.
**Status:** open

#### M2c-35 · bug · native-theme/src/presets/ios.toml:288
**Claim:** `defaults.border.color` (): the cell says #38383a; flat iOS 18: separator rgba(84,84,88,0.6); on black #323235; Liquid Glass iOS 26: separator rgba(84,84,88,0.5); on black #2a2a2c.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 Note: The preset's #38383a matches neither composite. It may be UIKit opaqueSeparatorColor in dark mode, which has no primary RGB.
**Verdict:** PLAUSIBLE — Both expectation files L156-160 read these alphas, and the arithmetic over #000000 is right, but the composite depends on the background (not the #1c1c1e surface), and the HIG also lists an 'Opaque separator' role, so separator vs opaqueSeparator for border.color is a mapping decision. Confirmed value: separator dark: iOS 18 rgba(84,84,88,0.6) = #323235 over black; iOS 26 rgba(84,84,88,0.5) = #2a2a2c over black.
**Triage:** drop — The cell #38383a is UIKit's opaqueSeparator dark, as the light #c6c6c8 is opaqueSeparator light; the claim compared it with the translucent separator.
**Status:** open

#### M2c-36 · bug · native-theme/src/presets/ios.toml:292
**Claim:** `defaults.border.opacity` (): the cell says 0.15; flat iOS 18: 0.6 (separator alpha); Liquid Glass iOS 26: 0.5 (separator alpha).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 Note: Dark-mode separator alpha over 84,84,88.
**Verdict:** REJECTED — The expectation files do give the separator alphas 0.6 (ios-18) and 0.5 (ios), but platform-facts.md:946 and :997 define border.opacity as the text colour's share in a line, and iOS's separator is rgba(84,84,88,a), not the white label colour at alpha a, so its alpha is not that share (by that definition, #323235 over black is about 0.2 white).
**Triage:** drop — REJECTED: border.opacity is the text colour's share in a line (platform-facts.md:946, :997), not the separator's alpha. The preset's 0.15 is wrong on its own grounds (validator finding below).
**Status:** open

#### M2c-37 · bug · S2 · native-theme/src/presets/ios.toml:314
**Claim:** `text_scale.caption.line_height_pt` (): the cell says 14.4; flat iOS 18: 16; Liquid Glass iOS 26: 16.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Caption 1 leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Caption 1 leading) Note: Same as line 66.
**Verdict:** CONFIRMED — Both the archived (2025-05-23) and live (read 2026-10-11) HIG typography.json, section 'iOS, iPadOS Dynamic Type sizes', tab 'Large (default)', give Caption 1 as Regular, 12, leading 16; ios.toml:314 has 14.4 (12 × 1.2). Confirmed value: 16 (Caption 1, 12 pt Regular, Large (default)), iOS 18 and iOS 26.
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-38 · bug · S2 · native-theme/src/presets/ios.toml:317
**Claim:** `text_scale.section_heading.size_pt` (): the cell says 15.0; flat iOS 18: 17; Liquid Glass iOS 26: 17.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Headline) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Headline) Note: Same as line 69.
**Verdict:** CONFIRMED — Both JSON versions give Headline 17 pt in the iOS Large (default) tab; this assumes section_heading maps to .headline, as the macOS column of platform-facts §2.19 (line 1436) does. Confirmed value: 17 (Headline, Large (default)), iOS 18 and iOS 26.
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-39 · bug · S2 · native-theme/src/presets/ios.toml:318
**Claim:** `text_scale.section_heading.weight` (): the cell says 700; flat iOS 18: 600 (Semibold); Liquid Glass iOS 26: 600 (Semibold).
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Headline) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Headline) Note: Same as line 70.
**Verdict:** CONFIRMED — Both JSON versions list the iOS Headline weight as Semibold; platform-facts.md line 85 already says 'iOS .headline is SemiBold', while ios.toml:318 has 700. Confirmed value: 600 (Semibold).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-40 · bug · S2 · native-theme/src/presets/ios.toml:319
**Claim:** `text_scale.section_heading.line_height_pt` (): the cell says 18.0; flat iOS 18: 22; Liquid Glass iOS 26: 22.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Headline leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Headline leading) Note: Same as line 71.
**Verdict:** CONFIRMED — Both JSON versions give iOS Headline leading 22 at Large (default); ios.toml:319 has 18.0. Confirmed value: 22.
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-41 · bug · S2 · native-theme/src/presets/ios.toml:322
**Claim:** `text_scale.dialog_title.size_pt` (): the cell says 22.0; flat iOS 18: 28; Liquid Glass iOS 26: 28.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Title 1) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Title 1) Note: Same as line 74.
**Verdict:** CONFIRMED — Both JSON versions give iOS Title 1 as 28 pt Regular, leading 34; this holds under the macOS §2.19 mapping dialog_title = .title1 (line 1437). ios.toml's 22 equals iOS Title 2 (22/28), so if iOS meant Title 2 instead, only the line height (28) would differ. Confirmed value: 28 (Title 1).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-42 · bug · S2 · native-theme/src/presets/ios.toml:324
**Claim:** `text_scale.dialog_title.line_height_pt` (): the cell says 26.4; flat iOS 18: 34; Liquid Glass iOS 26: 34.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Title 1 leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Title 1 leading) Note: Same as line 76.
**Verdict:** CONFIRMED — Both JSON versions give iOS Title 1 leading 34 at Large (default), under the same .title1 mapping as index 40. Confirmed value: 34 (Title 1 leading).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-43 · bug · S2 · native-theme/src/presets/ios.toml:329
**Claim:** `text_scale.display.line_height_pt` (): the cell says 40.8; flat iOS 18: 41; Liquid Glass iOS 26: 41.
**Evidence:** [flat] https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Large Title leading) [Liquid Glass] https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (read 2026-10-11; Large (default); Large Title leading) Note: Same as line 81.
**Verdict:** CONFIRMED — Both JSON versions give iOS Large Title 34 pt Regular, leading 41 at Large (default); the live JSON adds an emphasized weight of Bold. Confirmed value: 41 (Large Title leading).
**Triage:** fix — CONFIRMED against the HIG Dynamic Type table, Large (default), identical in the 2025-05 and 2026-10 copies; the cell is the macOS size or size x 1.2.
**Status:** open

#### M2c-44 · bug · S2 · native-theme/src/presets/ios.toml:332
**Claim:** `button.background_color` (): the cell says #2c2c2e; flat iOS 18: opaque-secondary-fill #262629; Liquid Glass iOS 26: opaque-secondary-fill #262629.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L895 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L84-L88 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L885 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L84-L88 Note: Same as line 84. The preset uses systemGray5 dark.
**Verdict:** PLAUSIBLE — Verified: html.css@31adf9d1 L895 and @de320ba3 L885 set `background-color: -apple-system-opaque-secondary-fill` for iOS, and both expected.txt files (L84-88) give dark rgb(38,38,41) = #262629. But this is WebKit's stylesheet for HTML `<button>` in Safari, not UIKit's UIButton, so mapping it to the native iOS button row needs a UIKit source or a measurement.
**Triage:** fix — PLAUSIBLE: WebKit's iOS button fill is opaque-secondary-fill (iOS 18 = iOS 26); the cell has no source. (WebKit)-tagged.
**Status:** open

#### M2c-45 · bug · S2 · native-theme/src/presets/ios.toml:341
**Claim:** `button.disabled_text_color` (): the cell says #3a3a3c; flat iOS 18: tertiaryLabel rgba(235,235,245,0.298); on black #464649; Liquid Glass iOS 26: tertiaryLabel rgba(235,235,245,0.298); on black #464649.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L996-L1003 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L983-L984 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 Note: Same as line 30.
**Verdict:** PLAUSIBLE — Verified: html.css@31adf9d1 L1001 and @de320ba3 L984 give the disabled button `color: -apple-system-tertiary-label`, and the expected.txt files (L183-184) give dark rgba(235,235,245,0.298). Two problems: this is WebKit's HTML-control styling, not UIKit, and the '#464649 on black' composite uses the wrong backdrop, because the same rule paints the disabled button with -apple-system-opaque-secondary-fill-disabled (RenderThemeIOS.mm@de320ba3 L1148: secondarySystemFillColor, opaque, 0.75), not black.
**Triage:** fix — PLAUSIBLE: WebKit's iOS html.css draws disabled controls in tertiaryLabel (a UIKit colour), identical at iOS 18 and 26; the cell #c7c7cc / #3a3a3c is systemGray3 with no source. Fixed with the (WebKit) tag platform-facts.md accepts.
**Status:** open

#### M2c-46 · bug · S2 · native-theme/src/presets/ios.toml:345
**Claim:** `button.font.color` (): the cell says #ffffff; flat iOS 18: #0a84ff (systemBlue); Liquid Glass iOS 26: #0091ff (systemBlue).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L891 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/css/html.css#L881 Note: Same as line 97.
**Verdict:** PLAUSIBLE — Verified: html.css@31adf9d1 L891 and @de320ba3 L881 give the iOS `<button>` `color: -apple-system-blue`, which is dark rgb(10,132,255) = #0a84ff in ios-18 expected.txt L15-16 and rgb(0,145,255) = #0091ff in ios expected.txt L15-16. These are UIKit systemBlue values that WebKit reads through UIColor, but assigning them as button text comes from WebKit's HTML stylesheet, not a UIKit source.
**Triage:** fix — PLAUSIBLE: an iOS button's label is the tint (systemBlue) in WebKit's iOS html.css; the cell's black / white has no source. (WebKit)-tagged; the iOS 26 blue follows the version-gap decision.
**Status:** open

#### M2c-47 · bug · S2 · native-theme/src/presets/ios.toml:347
**Claim:** `input.background_color` (): the cell says #1c1c1e; flat iOS 18: #000000 (Canvas = systemBackgroundColor); Liquid Glass iOS 26: #000000 (Canvas = systemBackgroundColor).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L463 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L6-L10 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L1127 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L6-L10 Note: WebKit tag. The preset's #1c1c1e is the elevated-level or secondary background, not the base level.
**Verdict:** PLAUSIBLE — Verified: html.css@31adf9d1 L463 (and @de320ba3 L414) set `input { background-color: Canvas }`, RenderThemeIOS.mm@de320ba3 L1127 maps Canvas to systemBackgroundColor, and expected.txt L9 gives dark rgb(0,0,0). This is Safari's HTML `<input>`, not UITextField, so it does not establish the native text field fill.
**Triage:** fix — PLAUSIBLE: an iOS input's background is Canvas = systemBackground (#000000 dark), not #1c1c1e.
**Status:** open

#### M2c-48 · bug · native-theme/src/presets/ios.toml:383
**Claim:** `slider.thumb_diameter_px` (): the cell says 28.0; flat iOS 18: 16; Liquid Glass iOS 26: 24 x 16 (not round).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L562 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3881 Note: Same as line 135.
**Verdict:** PLAUSIBLE — Verified: RenderThemeIOS.mm@31adf9d1 L562 has kDefaultSliderThumbSize = 16, and RenderThemeCocoa.mm@de320ba3 L3881 has sliderThumbSize {24,16}, used when FormControlRefreshEnabled is on (UnifiedWebPreferences.yaml@de320ba3 L2489-2497: default true, WebCore false). Both are WebKit's own drawing of HTML `<input type=range>`, not UISlider's thumb, and the iOS 26 thumb is not round, so it cannot fill thumb_diameter as it stands.
**Triage:** drop — As index 20 (dark twin): WebKit's HTML range thumb, not UISlider.
**Status:** open

#### M2c-49 · bug · S2 · native-theme/src/presets/ios.toml:391
**Claim:** `progress_bar.track_height_px` (): the cell says 4.0; flat iOS 18: 4; Liquid Glass iOS 26: 8.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L718 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L3770 Note: Same as line 143.
**Verdict:** CONFIRMED — Both constants read as cited; the 8 is WebKit's Liquid Glass path, drawn only when formControlRefreshEnabled (L3757), so the cell 4.0 holds for iOS 18 and differs from WebKit's iOS 26 bar. Confirmed value: iOS 18: 4 (RenderThemeIOS.mm@31adf9d1 L718 barBlockSize = 4.0f); iOS 26: 8 (RenderThemeCocoa.mm@de320ba3 L3770 barBlockSize = 8.f, radii 4).
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 value 4 equals the cell; the iOS 26 8 is WebKit's form-control-refresh <progress> (verdicts split: REJECTED at index 21 as not UIKit, CONFIRMED at 48 as WebKit), so the iOS 26 value is (WebKit)-only.
**Status:** open

#### M2c-50 · bug · native-theme/src/presets/ios.toml:433
**Claim:** `separator.line_color` (): the cell says #38383a; flat iOS 18: separator rgba(84,84,88,0.6); on black #323235; Liquid Glass iOS 26: separator rgba(84,84,88,0.5); on black #2a2a2c.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L156-L160 Note: Same as line 288.
**Verdict:** PLAUSIBLE — The token values hold (ios-18 expected.txt L159 rgba(84,84,88,0.6), ios expected.txt L159 rgba(84,84,88,0.5); over black #323235 and #2a2a2c), but the cell #38383a looks like the opaque separator token, which WebKit does not dump, and the translucent separator's on-screen colour depends on the background, so mapping it to the row needs a decision or a measurement.
**Triage:** drop — The cell #38383a is UIKit's opaqueSeparator dark, as the light #c6c6c8 is opaqueSeparator light; the claim compared it with the translucent separator.
**Status:** open

#### M2c-51 · bug · S2 · native-theme/src/presets/ios.toml:440
**Claim:** `switch.unchecked_background` (): the cell says #38383a; flat iOS 18: rgba(120,120,128,0.32); on black #262629; Liquid Glass iOS 26: tertiaryLabel rgba(235,235,245,0.298) over canvas; #464649.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L276-L279 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L281-L294 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios/fast/css/ios/system-color-for-css-value-expected.txt#L180-L184 Note: Same as line 192.
**Verdict:** PLAUSIBLE — RenderThemeCocoa.mm@de320ba3 L279 gives offDarkColor SRGBA{120,120,128,82} (over black #272729 with alpha 82/255, not #262629), and L283-294 replace it on iOS with tertiaryLabel composited over Canvas (expected.txt L183 rgba(235,235,245,0.298), about #464649 over black); the colours are WebKit's approximation, since L269 says UIKit does not expose _switchOffColor, and the iOS 26 one applies only when formControlRefreshEnabled.
**Triage:** fix — PLAUSIBLE: WebKit's classic iOS switch off-track is rgba(120,120,128,0.16 / 0.32), not the cell's separator grey; iOS 26 replaces it (version gap).
**Status:** open

#### M2c-52 · bug · S2 · native-theme/src/presets/ios.toml:445
**Claim:** `switch.disabled_thumb_color` (): the cell says #ffffff; flat iOS 18: white; Liquid Glass iOS 26: Color::darkGray (disabled, dark mode).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L763 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L483-L488 Note: WebKit tag. The iOS 26 painter greys a disabled thumb in dark mode. The RGB of Color::darkGray was not read.
**Verdict:** CONFIRMED — liquidGlassSwitchThumbColor returns white in light mode and, on IOS_FAMILY in dark mode, darkGray when disabled, read as cited; the Liquid Glass path is gated by formControlRefreshEnabled. Confirmed value: iOS 18: white (L763 thumbColor = Color::white, no disabled change to the thumb); iOS 26 dark disabled: Color::darkGray = SRGBA{128,128,128} #808080 (L483-488; Color.h@de320ba3 L178).
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 26 changed it; the preset matches the iOS 18 (flat) generation. Which iOS version `ios` names is M1 T1/T2's question.
**Status:** open

#### M2c-53 · bug · S2 · native-theme/src/presets/ios.toml:447
**Claim:** `switch.track_width_px` (): the cell says 51.0; flat iOS 18: 51; Liquid Glass iOS 26: 70.86 (31 x 64/28).
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L668-L669 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L417-L418 Note: Same as line 199.
**Verdict:** PLAUSIBLE — 51 holds (RenderThemeIOS.mm@31adf9d1 L669 is in the !USE(APPLE_INTERNAL_SDK) fallback; RenderThemeCocoa.mm@de320ba3 L216 has logicalSwitchWidth = 51.f), but the iOS 26 70.86 is WebKit rescaling idealSwitchSize {64,28} (L417) to keep its 31pt CSS height (L418), so the switch's own ideal track is 64x28, and which width is right for the row needs a measurement.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 value equals the cell; the iOS 26 Liquid Glass switch is WebKit's rescaling, (WebKit)-only.
**Status:** open

#### M2c-54 · bug · S2 · native-theme/src/presets/ios.toml:449
**Claim:** `switch.thumb_diameter_px` (): the cell says 27.0; flat iOS 18: 27; Liquid Glass iOS 26: 39.86 x 26.57 capsule.
**Evidence:** [flat] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L746-L771 [Liquid Glass] https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L423 ; https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L598-L612 Note: Same as line 201.
**Verdict:** PLAUSIBLE — Flat 27 holds (L776-777: 31 - 2x2 margin); the iOS 26 39.86x26.57 comes from the same height-31 rescale (L598-612: idealThumbWidth 36/64 of the track width, L423: height 6/7 of the track height), so at the ideal 64x28 size the thumb is 36x24, and the row's value needs a measurement.
**Triage:** maintainer (version gap, after M1 stop point A) — iOS 18 value equals the cell; the iOS 26 Liquid Glass switch is WebKit's rescaling, (WebKit)-only.
**Status:** open

### Step 4 — preset-validator

122 findings: 1 no-citation, 3 other, 118 depends-on-differs-row.

<details><summary>Validator summary</summary>

Audit of /home/tibi/Rust/native-theme-m2/native-theme/src/presets/ios.toml (read-only). Status: FAIL.

1. No source rows to check against. /home/tibi/Rust/native-theme-m2/docs/platform-facts.md has no iOS chapter. Chapter 1 covers only macOS (:51), Windows (:274), KDE (:548) and GNOME (:719). The two places "iOS" appears (:85, :1661) are notes about macOS fonts. None of the preset's values traces to a facts row. Its only source note is the general comment at ios.toml:2 ("Apple UIColor system colors, Human Interface Guidelines"). Each "line" in the Step 2 differs-rows list is therefore a line in ios.toml, not in platform-facts.md; the line numbers match the preset exactly. They are reported here that way.

2. Keys and icons are fine.
   - Every key in the preset exists in /home/tibi/Rust/native-theme-m2/docs/property-registry.toml, with the right type and suffix: _px, size_pt and line_height_pt (TextScaleEntry, :72-80), u16 weights, colours.
   - The icon setup is a single set, `icon_set = icon_theme = "sf-symbols"` (lines 4-5). It matches macos-sonoma.toml:4-5. iOS is not a Linux preset, so the freedesktop rule does not apply.

3. Values that depend on the differs-rows (54 rows). Each finding below names the key, its line and the row it depends on. There are three kinds of dependence:
   - **The row's own key.**
   - **A value copied by hex or alpha from a row:**
     - accent: #007aff / #0a84ff
     - disabled text: #c7c7cc / #3a3a3c
     - border: #c6c6c8 / #38383a
     - button background: #e5e5ea / #2c2c2e
     - button text: #000000 / #ffffff
   - **Inheritance from a row.** These keys are not set in the preset, so they take the row's value through /home/tibi/Rust/native-theme-m2/docs/inheritance-rules.toml. This covers the accent, muted and border.color rows.

4. Other problems.
   - **switch.checked_background is never set.** It inherits accent blue (inheritance-rules.toml:250), while the hover and disabled checked tracks in the same preset are green.
   - **line_height = 1.2 matches a known invented value.** It is the value inheritance-rules.toml:518 lists as an invented (hardcoded) safety net. The text_scale line heights are simply size × 1.2.
   - **border.opacity = 0.15 does not reproduce border.color = #c6c6c8** under the definition at platform-facts.md:997.
   - **disabled_opacity = 0.3 is set alongside full disabled colours.** platform-facts.md:992 and inheritance-rules.toml:352-359 say a preset dims either by colour or by opacity, not both.

No value has been changed, and none is suggested. Each gap needs an authoritative Apple source, either a new iOS section in platform-facts.md or a cited HIG/UIKit reference, before the preset is edited.

</details>

#### M2c-P01 · bug · S2 · native-theme/src/presets/ios.toml:2
**Claim:** `(whole preset)` (no-citation): preset # Source: Apple UIColor system colors, Human Interface Guidelines, facts no iOS rows (docs/platform-facts.md:46-856 (Chapter 1 has only 1.1 macOS :51, 1.2 Windows :27…).
**Evidence:** None of the ~400 values in ios.toml traces to a platform-facts row; the only source is the general comment on line 2. All researcher differs-row 'line' numbers below are ios.toml lines, because no facts row exists. Remediation: add an iOS section to platform-facts.md with cited HIG/UIKit sources (or cite per value) before changing values.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** maintainer (version gap, after M1 stop point A) — `ios` has no facts column (D5 keeps it a look, not a platform). Whether it gets one, or its values cite their sources inline as material.toml's do, is part of the T1/T2 question.
**Status:** open

#### M2c-P02 · bug · S2 · native-theme/src/presets/ios.toml:191
**Claim:** `light/dark switch.checked_background (unset)` (other): preset (unset; resolves to accent #007aff / #0a84ff), facts  (inheritance-rules.toml:250).
**Evidence:** Inconsistent within the preset. The on-track is not stated, so it inherits blue accent, while hover_checked_background (193 #2d8a47 / 441 #25813c) and disabled_checked_background (195 #34c75980 / 443 #30d15880) are green shades of success_color. The preset should state checked_background with a cited source; no value is suggested here.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** maintainer, with M2c-P01 — the on-track inherits the blue accent while its hover and disabled states are green; Step 2 found no source for any checked switch colour (ios.toml:193, :195 unverifiable), so nothing can be stated from a source yet.
**Status:** open

#### M2c-P03 · bug · S2 · native-theme/src/presets/ios.toml:33
**Claim:** `light/dark defaults.disabled_opacity` (other): preset 0.3 (lines 33, 281), facts one disabled mechanism per platform; the other is an identity (platform-facts.md:992; inheritance-rules.toml:352-359).
**Evidence:** The preset sets both disabled_opacity = 0.3 and full disabled_* colours, so consumers apply both. Without an iOS facts row it is unclear which mechanism iOS uses.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** defer — the preset states both a disabled opacity and disabled colours; docs/platform-facts.md:992 says one mechanism per platform; with no iOS column, which one iOS uses is the M2c-P01 question.
**Status:** open

#### M2c-P04 · bug · S2 · native-theme/src/presets/ios.toml:44
**Claim:** `light/dark defaults.border.opacity` (other): preset 0.15 (lines 44, 292), facts opacity is the text colour's share, already folded into border.color (platform-facts.md:997).
**Evidence:** 0.15 does not reproduce border.color = #c6c6c8 (light) or #38383a (dark) under this definition. The preset contradicts itself, separately from the researcher-reported alpha change.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — 0.15 does not reproduce the preset's own border colours under platform-facts.md:997; with M2c-09 dropped, the value is set to what the colours give or left to the M2c-P01 decision (S2).
**Status:** open

<details><summary>Preset keys that depend on a <i>differs</i> row</summary>

| Preset:line | Key | Preset value | Facts row |
|---|---|---|---|
| ios.toml:10 | `light.defaults.accent_color` | #007aff | researcher row line 10 (no iOS row in docs/platform-facts.md… |
| ios.toml:26 | `light.defaults.selection_background` | #007aff | researcher row line 10 (accent_color); inheritance-rules.tom… |
| ios.toml:28 | `light.defaults.link_color` | #007aff | researcher row line 10 (accent_color) |
| ios.toml:101 | `light.input.hover_border_color` | #007aff | researcher row line 10 (accent_color) |
| ios.toml:102 | `light.input.focus_border_color` | #007aff | researcher row line 10 (accent_color) |
| ios.toml:138 | `light.slider.disabled_fill_color` | #007aff80 | researcher row line 10 (accent_color) |
| ios.toml:152 | `light.tab.active_text_color` | #007aff | researcher row line 10 (accent_color) |
| ios.toml:157 | `light.menu.hover_background` | #007aff | researcher row line 10 (accent_color); inheritance-rules.tom… |
| ios.toml:189 | `light.splitter.hover_color` | #007aff | researcher row line 10 (accent_color) |
| ios.toml:216 | `light.link.hover_background` | #007aff18 | researcher row line 10 (accent_color) |
| ios.toml:217 | `light.link.hover_text_color` | #007aff | researcher row line 10 (accent_color) |
| ios.toml:218 | `light.link.active_text_color` | #0056b3 | researcher row line 10 (accent_color) |
| ios.toml:239 | `light.segmented_control.active_background` | #007aff | researcher row line 10 (accent_color); inheritance-rules.tom… |
| ios.toml:10 | `light (inherited, unset): button.primary_background, checkbox.checked_background…` | (unset; resolves to #007aff) | researcher row line 10; inheritance-rules.toml:156,180,201,2… |
| ios.toml:14 | `light.defaults.muted_color` | #8e8e93 | researcher row line 14 |
| ios.toml:131 | `light.scrollbar.thumb_active_color` | #8e8e93 | researcher row line 14 (muted_color) |
| ios.toml:249 | `light.expander.arrow_color` | #8e8e93 | researcher row line 14 (muted_color) |
| ios.toml:14 | `light (inherited, unset): input.placeholder_color, scrollbar.thumb_color, slider…` | (unset; resolves to #8e8e93) | researcher row line 14; inheritance-rules.toml:166,197,202,2… |
| ios.toml:17 | `light.defaults.danger_color` | #ff3b30 | researcher row line 17 |
| ios.toml:19 | `light.defaults.warning_color` | #ff9500 | researcher row line 19 |
| ios.toml:23 | `light.defaults.info_color` | #007aff | researcher row line 23 |
| ios.toml:29 | `light.defaults.focus_ring_color` | #007aff | researcher row line 29; inheritance-rules.toml:52 |
| ios.toml:30 | `light.defaults.disabled_text_color` | #c7c7cc | researcher row line 30 |
| ios.toml:93 | `light.button.disabled_text_color` | #c7c7cc | researcher row line 93 (and line 30 via inheritance-rules.to… |
| ios.toml:104 | `light.input.disabled_text_color` | #c7c7cc | researcher row line 30; inheritance-rules.toml:170 |
| ios.toml:114 | `light.checkbox.disabled_text_color` | #c7c7cc | researcher row line 30; inheritance-rules.toml:183 |
| ios.toml:159 | `light.menu.disabled_text_color` | #c7c7cc | researcher row line 30; inheritance-rules.toml:191 |
| ios.toml:166 | `light.list.disabled_text_color` | #c7c7cc | researcher row line 30; inheritance-rules.toml:275 |
| ios.toml:219 | `light.link.disabled_text_color` | #c7c7cc | researcher row line 30; inheritance-rules.toml:293 |
| ios.toml:232 | `light.combo_box.disabled_text_color` | #c7c7cc | researcher row line 30; inheritance-rules.toml:284 |
| ios.toml:34 | `light.defaults.line_height` | 1.2 | researcher row line 34; inheritance-rules.toml:518 |
| ios.toml:40 | `light.defaults.border.color` | #c6c6c8 | researcher row line 40 |
| ios.toml:116 | `light.checkbox.unchecked_border_color` | #c6c6c8 | researcher row line 40 (border.color) |
| ios.toml:40 | `light (inherited, unset): border.color of window/button/input/checkbox/tooltip/p…` | (unset; resolves to #c6c6c8) | researcher row line 40; inheritance-rules.toml:86-91,173,187… |
| ios.toml:44 | `light.defaults.border.opacity` | 0.15 | researcher row line 44; platform-facts.md:997 |
| ios.toml:66 | `light.text_scale.caption.line_height_pt` | 14.4 | researcher row line 66 |
| ios.toml:69 | `light.text_scale.section_heading.size_pt` | 15.0 | researcher row line 69 |
| ios.toml:70 | `light.text_scale.section_heading.weight` | 700 | researcher row line 70 |
| ios.toml:71 | `light.text_scale.section_heading.line_height_pt` | 18.0 | researcher row line 71 |
| ios.toml:74 | `light.text_scale.dialog_title.size_pt` | 22.0 | researcher row line 74 |
| ios.toml:76 | `light.text_scale.dialog_title.line_height_pt` | 26.4 | researcher row line 76 |
| ios.toml:81 | `light.text_scale.display.line_height_pt` | 40.8 | researcher row line 81 |
| ios.toml:84 | `light.button.background_color` | #e5e5ea | researcher row line 84 |
| ios.toml:92 | `light.button.disabled_background` | #e5e5ea80 | researcher row line 84 (button.background_color) |
| ios.toml:113 | `light.checkbox.disabled_background` | #e5e5ea80 | researcher row line 84 (button.background_color) |
| ios.toml:231 | `light.combo_box.disabled_background` | #e5e5ea80 | researcher row line 84 (button.background_color) |
| ios.toml:97 | `light.button.font.color` | #000000 | researcher row line 97 |
| ios.toml:89 | `light.button.hover_text_color` | #000000 | researcher row line 97 (button.font.color); inheritance-rule… |
| ios.toml:91 | `light.button.active_text_color` | #000000 | researcher row line 97 (button.font.color); inheritance-rule… |
| ios.toml:135 | `light.slider.thumb_diameter_px` | 28.0 | researcher row line 135 |
| ios.toml:143 | `light.progress_bar.track_height_px` | 4.0 | researcher row line 143 |
| ios.toml:185 | `light.separator.line_color` | #c6c6c8 | researcher row line 185 (and line 40 via inheritance-rules.t… |
| ios.toml:192 | `light.switch.unchecked_background` | #c6c6c8 | researcher row line 192 (also the hex of row 40) |
| ios.toml:194 | `light.switch.hover_unchecked_background` | #b0b0b5 | researcher row line 192 (switch.unchecked_background) |
| ios.toml:196 | `light.switch.disabled_unchecked_background` | #c6c6c880 | researcher rows line 192 and line 40 |
| ios.toml:199 | `light.switch.track_width_px` | 51.0 | researcher row line 199 |
| ios.toml:201 | `light.switch.thumb_diameter_px` | 27.0 | researcher row line 201 |
| ios.toml:258 | `dark.defaults.accent_color` | #0a84ff | researcher row line 258 |
| ios.toml:274 | `dark.defaults.selection_background` | #0a84ff | researcher row line 258; inheritance-rules.toml:51 |
| ios.toml:276 | `dark.defaults.link_color` | #0a84ff | researcher row line 258 |
| ios.toml:349 | `dark.input.hover_border_color` | #0a84ff | researcher row line 258 |
| ios.toml:350 | `dark.input.focus_border_color` | #0a84ff | researcher row line 258 |
| ios.toml:386 | `dark.slider.disabled_fill_color` | #0a84ff80 | researcher row line 258 |
| ios.toml:400 | `dark.tab.active_text_color` | #0a84ff | researcher row line 258 |
| ios.toml:405 | `dark.menu.hover_background` | #0a84ff | researcher row line 258; inheritance-rules.toml:189 |
| ios.toml:437 | `dark.splitter.hover_color` | #0a84ff | researcher row line 258 |
| ios.toml:464 | `dark.link.hover_background` | #0a84ff18 | researcher row line 258 |
| ios.toml:465 | `dark.link.hover_text_color` | #0a84ff | researcher row line 258 |
| ios.toml:466 | `dark.link.active_text_color` | #409cff | researcher row line 258 |
| ios.toml:487 | `dark.segmented_control.active_background` | #0a84ff | researcher row line 258; inheritance-rules.toml:262 |
| ios.toml:258 | `dark (inherited, unset): button.primary_background, checkbox.checked_background,…` | (unset; resolves to #0a84ff) | researcher row line 258; inheritance-rules.toml:156,180,201,… |
| ios.toml:262 | `dark.defaults.muted_color` | #8e8e93 | researcher row line 262 |
| ios.toml:379 | `dark.scrollbar.thumb_active_color` | #8e8e93 | researcher row line 262 |
| ios.toml:497 | `dark.expander.arrow_color` | #8e8e93 | researcher row line 262 |
| ios.toml:262 | `dark (inherited, unset): input.placeholder_color, scrollbar.thumb_color, slider.…` | (unset; resolves to #8e8e93) | researcher row line 262; inheritance-rules.toml:166,197,202,… |
| ios.toml:265 | `dark.defaults.danger_color` | #ff453a | researcher row line 265 |
| ios.toml:267 | `dark.defaults.warning_color` | #ff9f0a | researcher row line 267 |
| ios.toml:271 | `dark.defaults.info_color` | #0a84ff | researcher row line 271 |
| ios.toml:277 | `dark.defaults.focus_ring_color` | #0a84ff | researcher row line 277; inheritance-rules.toml:52 |
| ios.toml:278 | `dark.defaults.disabled_text_color` | #3a3a3c | researcher row line 278 |
| ios.toml:341 | `dark.button.disabled_text_color` | #3a3a3c | researcher row line 341 (and 278) |
| ios.toml:352 | `dark.input.disabled_text_color` | #3a3a3c | researcher row line 278; inheritance-rules.toml:170 |
| ios.toml:362 | `dark.checkbox.disabled_text_color` | #3a3a3c | researcher row line 278; inheritance-rules.toml:183 |
| ios.toml:407 | `dark.menu.disabled_text_color` | #3a3a3c | researcher row line 278; inheritance-rules.toml:191 |
| ios.toml:414 | `dark.list.disabled_text_color` | #3a3a3c | researcher row line 278; inheritance-rules.toml:275 |
| ios.toml:467 | `dark.link.disabled_text_color` | #3a3a3c | researcher row line 278; inheritance-rules.toml:293 |
| ios.toml:480 | `dark.combo_box.disabled_text_color` | #3a3a3c | researcher row line 278; inheritance-rules.toml:284 |
| ios.toml:282 | `dark.defaults.line_height` | 1.2 | researcher row line 282; inheritance-rules.toml:518 |
| ios.toml:288 | `dark.defaults.border.color` | #38383a | researcher row line 288 |
| ios.toml:364 | `dark.checkbox.unchecked_border_color` | #38383a | researcher row line 288 |
| ios.toml:387 | `dark.slider.disabled_track_color` | #38383a | researcher row line 288 |
| ios.toml:288 | `dark (inherited, unset): border.color of window/button/input/checkbox/tooltip/pr…` | (unset; resolves to #38383a) | researcher row line 288; inheritance-rules.toml:86-91,173,18… |
| ios.toml:292 | `dark.defaults.border.opacity` | 0.15 | researcher row line 292; platform-facts.md:997 |
| ios.toml:314 | `dark.text_scale.caption.line_height_pt` | 14.4 | researcher row line 314 |
| ios.toml:317 | `dark.text_scale.section_heading.size_pt` | 15.0 | researcher row line 317 |
| ios.toml:318 | `dark.text_scale.section_heading.weight` | 700 | researcher row line 318 |
| ios.toml:319 | `dark.text_scale.section_heading.line_height_pt` | 18.0 | researcher row line 319 |
| ios.toml:322 | `dark.text_scale.dialog_title.size_pt` | 22.0 | researcher row line 322 |
| ios.toml:324 | `dark.text_scale.dialog_title.line_height_pt` | 26.4 | researcher row line 324 |
| ios.toml:329 | `dark.text_scale.display.line_height_pt` | 40.8 | researcher row line 329 |
| ios.toml:332 | `dark.button.background_color` | #2c2c2e | researcher row line 332 |
| ios.toml:340 | `dark.button.disabled_background` | #2c2c2e80 | researcher row line 332 |
| ios.toml:361 | `dark.checkbox.disabled_background` | #2c2c2e80 | researcher row line 332 |
| ios.toml:479 | `dark.combo_box.disabled_background` | #2c2c2e80 | researcher row line 332 |
| ios.toml:345 | `dark.button.font.color` | #ffffff | researcher row line 345 |
| ios.toml:337 | `dark.button.hover_text_color` | #ffffff | researcher row line 345; inheritance-rules.toml:160 |
| ios.toml:339 | `dark.button.active_text_color` | #ffffff | researcher row line 345; inheritance-rules.toml:162 |
| ios.toml:347 | `dark.input.background_color` | #1c1c1e | researcher row line 347 |
| ios.toml:363 | `dark.checkbox.unchecked_background` | #1c1c1e | researcher row line 347 (input.background_color) |
| ios.toml:383 | `dark.slider.thumb_diameter_px` | 28.0 | researcher row line 383 |
| ios.toml:391 | `dark.progress_bar.track_height_px` | 4.0 | researcher row line 391 |
| ios.toml:433 | `dark.separator.line_color` | #38383a | researcher row line 433 (and 288 via inheritance-rules.toml:… |
| ios.toml:440 | `dark.switch.unchecked_background` | #38383a | researcher row line 440 |
| ios.toml:442 | `dark.switch.hover_unchecked_background` | #48484a | researcher row line 440 |
| ios.toml:444 | `dark.switch.disabled_unchecked_background` | #38383a80 | researcher rows line 440 and 288 |
| ios.toml:445 | `dark.switch.disabled_thumb_color` | #ffffff | researcher row line 445 |
| ios.toml:447 | `dark.switch.track_width_px` | 51.0 | researcher row line 447 |
| ios.toml:449 | `dark.switch.thumb_diameter_px` | 27.0 | researcher row line 449 |

</details>

### Unverifiable rows

No primary source found: the row keeps or gets a ❓ in Step 7; no value is proposed. *Probe* = a probe or the reference app could measure it.

<details><summary>225 unverifiable rows</summary>

| Line | Property | Cell | Probe | Note |
|---|---|---|---|---|
| 15 | `defaults.shadow_color` | #00000018 |  | U1 |
| 18 | `defaults.danger_text_color` | #ffffff |  | U2 |
| 20 | `defaults.warning_text_color` | #000000 |  | U2 |
| 22 | `defaults.success_text_color` | #ffffff |  | U2 |
| 24 | `defaults.info_text_color` | #ffffff |  | U2 |
| 26 | `defaults.selection_background` | #007aff | yes | U3 |
| 27 | `defaults.selection_text_color` | #ffffff | yes | U3 |
| 28 | `defaults.link_color` | #007aff | yes | U4 |
| 31 | `defaults.accent_text_color` | #ffffff |  | U2 |
| 33 | `defaults.disabled_opacity` | 0.3 |  | U5 |
| 35 | `defaults.focus_ring_width_px` | 2.0 | yes | U6 |
| 36 | `defaults.focus_ring_offset_px` | 1.0 | yes | U6 |
| 41 | `defaults.border.corner_radius_px` | 10.0 | yes | U7 |
| 42 | `defaults.border.corner_radius_lg_px` | 13.0 | yes | U7 |
| 43 | `defaults.border.line_width_px` | 0.5 | yes | U8 |
| 45 | `defaults.border.shadow_enabled` | true |  | U9 |
| 52 | `defaults.mono_font.family` | SF Mono | yes | U10 |
| 53 | `defaults.mono_font.size_pt` | 17.0 |  | U11 |
| 54 | `defaults.mono_font.weight` | 400 |  | U11 |
| 57 | `defaults.icon_sizes.toolbar_px` | 24.0 | yes | U12 |
| 58 | `defaults.icon_sizes.small_px` | 16.0 | yes | U12 |
| 59 | `defaults.icon_sizes.large_px` | 32.0 | yes | U12 |
| 60 | `defaults.icon_sizes.dialog_px` | 22.0 | yes | U12 |
| 61 | `defaults.icon_sizes.panel_px` | 20.0 | yes | U12 |
| 85 | `button.min_width_px` | 64.0 | yes | U13 |
| 87 | `button.icon_text_gap_px` | 6.0 | yes | U14 |
| 88 | `button.hover_background` | #d1d1d6 |  | U15 |
| 89 | `button.hover_text_color` | #000000 |  | U15 |
| 90 | `button.active_background` | #c7c7cc | yes | U16 |
| 91 | `button.active_text_color` | #000000 | yes | U16 |
| 92 | `button.disabled_background` | #e5e5ea80 | yes | U17 |
| 101 | `input.hover_border_color` | #007aff |  | U15 |
| 102 | `input.focus_border_color` | #007aff | yes | U18 |
| 103 | `input.disabled_background` | #f2f2f7 | yes | U19 |
| 104 | `input.disabled_text_color` | #c7c7cc | yes | U19 |
| 110 | `checkbox.indicator_width_px` | 22.0 |  | U20 |
| 111 | `checkbox.label_gap_px` | 8.0 |  | U20 |
| 112 | `checkbox.hover_background` | #d1d1d6 |  | U20 |
| 113 | `checkbox.disabled_background` | #e5e5ea80 |  | U20 |
| 114 | `checkbox.disabled_text_color` | #c7c7cc |  | U20 |
| 115 | `checkbox.unchecked_background` | #ffffff |  | U20 |
| 116 | `checkbox.unchecked_border_color` | #c6c6c8 |  | U20 |
| 119 | `tooltip.background_color` | #2c2c2e | yes | U21 |
| 120 | `tooltip.max_width_px` | 300.0 | yes | U21 |
| 124 | `tooltip.font.color` | #ffffff | yes | U21 |
| 126 | `scrollbar.groove_width_px` | 6.0 | yes | U22 |
| 127 | `scrollbar.thumb_width_px` | 3.0 | yes | U22 |
| 129 | `scrollbar.min_thumb_length_px` | 30.0 | yes | U22 |
| 130 | `scrollbar.thumb_hover_color` | #a0a0a5 | yes | U22 |
| 131 | `scrollbar.thumb_active_color` | #8e8e93 | yes | U22 |
| 136 | `slider.tick_mark_length_px` | 4.0 |  | U23 |
| 137 | `slider.thumb_hover_color` | #f0f0f0 |  | U15 |
| 138 | `slider.disabled_fill_color` | #007aff80 | yes | U24 |
| 139 | `slider.disabled_track_color` | #c7c7cc | yes | U24 |
| 140 | `slider.disabled_thumb_color` | #e5e5ea | yes | U24 |
| 144 | `progress_bar.min_width_px` | 100.0 |  | U25 |
| 147 | `tab.min_width_px` | 64.0 | yes | U26 |
| 148 | `tab.min_height_px` | 49.0 | yes | U26 |
| 149 | `tab.hover_background` | #e5e5ea |  | U15 |
| 150 | `tab.hover_text_color` | #000000 |  | U15 |
| 151 | `tab.active_background` | #ffffff | yes | U27 |
| 152 | `tab.active_text_color` | #007aff | yes | U27 |
| 156 | `menu.icon_text_gap_px` | 8.0 | yes | U28 |
| 157 | `menu.hover_background` | #007aff | yes | U28 |
| 158 | `menu.hover_text_color` | #ffffff | yes | U28 |
| 159 | `menu.disabled_text_color` | #c7c7cc | yes | U28 |
| 163 | `list.alternate_row_background` | #f2f2f7 |  | U29 |
| 164 | `list.hover_background` | #e5e5ea |  | U30 |
| 165 | `list.hover_text_color` | #000000 |  | U30 |
| 166 | `list.disabled_text_color` | #c7c7cc |  | U31 |
| 172 | `sidebar.background_color` | #f2f2f7 | yes | U32 |
| 173 | `sidebar.hover_background` | #e5e5ea |  | U15 |
| 179 | `popover.background_color` | #ffffff | yes | U33 |
| 188 | `splitter.divider_width_px` | 1.0 |  | U34 |
| 189 | `splitter.hover_color` | #007aff |  | U34 |
| 193 | `switch.hover_checked_background` | #2d8a47 |  | U15 |
| 194 | `switch.hover_unchecked_background` | #b0b0b5 |  | U15 |
| 195 | `switch.disabled_checked_background` | #34c75980 | yes | U35 |
| 196 | `switch.disabled_unchecked_background` | #c6c6c880 | yes | U35 |
| 202 | `switch.track_radius_px` | 15.5 | yes | U36 |
| 206 | `dialog.min_width_px` | 270.0 | yes | U37 |
| 207 | `dialog.max_width_px` | 560.0 | yes | U37 |
| 208 | `dialog.min_height_px` | 140.0 | yes | U37 |
| 209 | `dialog.max_height_px` | 600.0 | yes | U37 |
| 210 | `dialog.button_gap_px` | 8.0 | yes | U31 |
| 211 | `dialog.icon_size_px` | 32.0 | yes | U31 |
| 214 | `link.underline_enabled` | false |  | U38 |
| 215 | `link.background_color` | #00000000 |  | U39 |
| 216 | `link.hover_background` | #007aff18 |  | U39 |
| 217 | `link.hover_text_color` | #007aff |  | U39 |
| 218 | `link.active_text_color` | #0056b3 |  | U39 |
| 219 | `link.disabled_text_color` | #c7c7cc |  | U39 |
| 222 | `spinner.diameter_px` | 24.0 | yes | U40 |
| 223 | `spinner.min_diameter_px` | 16.0 | yes | U40 |
| 224 | `spinner.stroke_width_px` | 2.0 | yes | U40 |
| 228 | `combo_box.min_width_px` | 120.0 | yes | U41 |
| 229 | `combo_box.arrow_icon_size_px` | 12.0 | yes | U41 |
| 230 | `combo_box.hover_background` | #d1d1d6 |  | U15 |
| 231 | `combo_box.disabled_background` | #e5e5ea80 | yes | U31 |
| 232 | `combo_box.disabled_text_color` | #c7c7cc | yes | U31 |
| 236 | `segmented_control.segment_height_px` | 32.0 | yes | U42 |
| 237 | `segmented_control.separator_width_px` | 1.0 | yes | U42 |
| 238 | `segmented_control.hover_background` | #d1d1d6 |  | U15 |
| 239 | `segmented_control.active_background` | #007aff | yes | U42 |
| 240 | `segmented_control.active_text_color` | #ffffff | yes | U42 |
| 246 | `expander.header_height_px` | 44.0 | yes | U43 |
| 247 | `expander.arrow_icon_size_px` | 12.0 | yes | U43 |
| 248 | `expander.hover_background` | #e5e5ea |  | U15 |
| 249 | `expander.arrow_color` | #8e8e93 | yes | U43 |
| 252 | `window.inactive_title_bar_background` | #f2f2f7 |  | U44 |
| 253 | `window.inactive_title_bar_text_color` | #000000 |  | U44 |
| 263 | `defaults.shadow_color` | #00000040 |  | U1 |
| 266 | `defaults.danger_text_color` | #ffffff |  | U2 |
| 268 | `defaults.warning_text_color` | #000000 |  | U2 |
| 270 | `defaults.success_text_color` | #ffffff |  | U2 |
| 272 | `defaults.info_text_color` | #ffffff |  | U2 |
| 274 | `defaults.selection_background` | #0a84ff | yes | U3 |
| 275 | `defaults.selection_text_color` | #ffffff | yes | U3 |
| 276 | `defaults.link_color` | #0a84ff | yes | U4 |
| 279 | `defaults.accent_text_color` | #ffffff |  | U2 |
| 281 | `defaults.disabled_opacity` | 0.3 |  | U5 |
| 283 | `defaults.focus_ring_width_px` | 2.0 | yes | U6 |
| 284 | `defaults.focus_ring_offset_px` | 1.0 | yes | U6 |
| 289 | `defaults.border.corner_radius_px` | 10.0 | yes | U7 |
| 290 | `defaults.border.corner_radius_lg_px` | 13.0 | yes | U7 |
| 291 | `defaults.border.line_width_px` | 0.5 | yes | U8 |
| 293 | `defaults.border.shadow_enabled` | true |  | U9 |
| 300 | `defaults.mono_font.family` | SF Mono | yes | U10 |
| 301 | `defaults.mono_font.size_pt` | 17.0 |  | U11 |
| 302 | `defaults.mono_font.weight` | 400 |  | U11 |
| 305 | `defaults.icon_sizes.toolbar_px` | 24.0 | yes | U12 |
| 306 | `defaults.icon_sizes.small_px` | 16.0 | yes | U12 |
| 307 | `defaults.icon_sizes.large_px` | 32.0 | yes | U12 |
| 308 | `defaults.icon_sizes.dialog_px` | 22.0 | yes | U12 |
| 309 | `defaults.icon_sizes.panel_px` | 20.0 | yes | U12 |
| 333 | `button.min_width_px` | 64.0 | yes | U13 |
| 335 | `button.icon_text_gap_px` | 6.0 | yes | U14 |
| 336 | `button.hover_background` | #3a3a3c |  | U15 |
| 337 | `button.hover_text_color` | #ffffff |  | U15 |
| 338 | `button.active_background` | #48484a | yes | U16 |
| 339 | `button.active_text_color` | #ffffff | yes | U16 |
| 340 | `button.disabled_background` | #2c2c2e80 | yes | U17 |
| 349 | `input.hover_border_color` | #0a84ff |  | U15 |
| 350 | `input.focus_border_color` | #0a84ff | yes | U18 |
| 351 | `input.disabled_background` | #2c2c2e | yes | U19 |
| 352 | `input.disabled_text_color` | #3a3a3c | yes | U19 |
| 358 | `checkbox.indicator_width_px` | 22.0 |  | U20 |
| 359 | `checkbox.label_gap_px` | 8.0 |  | U20 |
| 360 | `checkbox.hover_background` | #3a3a3c |  | U20 |
| 361 | `checkbox.disabled_background` | #2c2c2e80 |  | U20 |
| 362 | `checkbox.disabled_text_color` | #3a3a3c |  | U20 |
| 363 | `checkbox.unchecked_background` | #1c1c1e |  | U20 |
| 364 | `checkbox.unchecked_border_color` | #38383a |  | U20 |
| 367 | `tooltip.background_color` | #2c2c2e | yes | U21 |
| 368 | `tooltip.max_width_px` | 300.0 | yes | U21 |
| 372 | `tooltip.font.color` | #ffffff | yes | U21 |
| 374 | `scrollbar.groove_width_px` | 6.0 | yes | U22 |
| 375 | `scrollbar.thumb_width_px` | 3.0 | yes | U22 |
| 377 | `scrollbar.min_thumb_length_px` | 30.0 | yes | U22 |
| 378 | `scrollbar.thumb_hover_color` | #636366 | yes | U22 |
| 379 | `scrollbar.thumb_active_color` | #8e8e93 | yes | U22 |
| 384 | `slider.tick_mark_length_px` | 4.0 |  | U23 |
| 385 | `slider.thumb_hover_color` | #3a3a3c |  | U15 |
| 386 | `slider.disabled_fill_color` | #0a84ff80 | yes | U24 |
| 387 | `slider.disabled_track_color` | #38383a | yes | U24 |
| 388 | `slider.disabled_thumb_color` | #2c2c2e | yes | U24 |
| 392 | `progress_bar.min_width_px` | 100.0 |  | U25 |
| 395 | `tab.min_width_px` | 64.0 | yes | U26 |
| 396 | `tab.min_height_px` | 49.0 | yes | U26 |
| 397 | `tab.hover_background` | #2c2c2e |  | U15 |
| 398 | `tab.hover_text_color` | #ffffff |  | U15 |
| 399 | `tab.active_background` | #000000 | yes | U27 |
| 400 | `tab.active_text_color` | #0a84ff | yes | U27 |
| 404 | `menu.icon_text_gap_px` | 8.0 | yes | U28 |
| 405 | `menu.hover_background` | #0a84ff | yes | U28 |
| 406 | `menu.hover_text_color` | #ffffff | yes | U28 |
| 407 | `menu.disabled_text_color` | #3a3a3c | yes | U28 |
| 411 | `list.alternate_row_background` | #1c1c1e |  | U29 |
| 412 | `list.hover_background` | #2c2c2e |  | U30 |
| 413 | `list.hover_text_color` | #ffffff |  | U30 |
| 414 | `list.disabled_text_color` | #3a3a3c |  | U31 |
| 420 | `sidebar.background_color` | #1c1c1e | yes | U32 |
| 421 | `sidebar.hover_background` | #2c2c2e |  | U15 |
| 427 | `popover.background_color` | #1c1c1e | yes | U33 |
| 436 | `splitter.divider_width_px` | 1.0 |  | U34 |
| 437 | `splitter.hover_color` | #0a84ff |  | U34 |
| 441 | `switch.hover_checked_background` | #25813c |  | U15 |
| 442 | `switch.hover_unchecked_background` | #48484a |  | U15 |
| 443 | `switch.disabled_checked_background` | #30d15880 | yes | U35 |
| 444 | `switch.disabled_unchecked_background` | #38383a80 | yes | U35 |
| 450 | `switch.track_radius_px` | 15.5 | yes | U36 |
| 454 | `dialog.min_width_px` | 270.0 | yes | U37 |
| 455 | `dialog.max_width_px` | 560.0 | yes | U37 |
| 456 | `dialog.min_height_px` | 140.0 | yes | U37 |
| 457 | `dialog.max_height_px` | 600.0 | yes | U37 |
| 458 | `dialog.button_gap_px` | 8.0 | yes | U31 |
| 459 | `dialog.icon_size_px` | 32.0 | yes | U31 |
| 462 | `link.underline_enabled` | false |  | U38 |
| 463 | `link.background_color` | #00000000 |  | U39 |
| 464 | `link.hover_background` | #0a84ff18 |  | U39 |
| 465 | `link.hover_text_color` | #0a84ff |  | U39 |
| 466 | `link.active_text_color` | #409cff |  | U39 |
| 467 | `link.disabled_text_color` | #3a3a3c |  | U39 |
| 470 | `spinner.diameter_px` | 24.0 | yes | U40 |
| 471 | `spinner.min_diameter_px` | 16.0 | yes | U40 |
| 472 | `spinner.stroke_width_px` | 2.0 | yes | U40 |
| 476 | `combo_box.min_width_px` | 120.0 | yes | U41 |
| 477 | `combo_box.arrow_icon_size_px` | 12.0 | yes | U41 |
| 478 | `combo_box.hover_background` | #3a3a3c |  | U15 |
| 479 | `combo_box.disabled_background` | #2c2c2e80 | yes | U31 |
| 480 | `combo_box.disabled_text_color` | #3a3a3c | yes | U31 |
| 484 | `segmented_control.segment_height_px` | 32.0 | yes | U42 |
| 485 | `segmented_control.separator_width_px` | 1.0 | yes | U42 |
| 486 | `segmented_control.hover_background` | #3a3a3c |  | U15 |
| 487 | `segmented_control.active_background` | #0a84ff | yes | U42 |
| 488 | `segmented_control.active_text_color` | #ffffff | yes | U42 |
| 494 | `expander.header_height_px` | 44.0 | yes | U43 |
| 495 | `expander.arrow_icon_size_px` | 12.0 | yes | U43 |
| 496 | `expander.hover_background` | #2c2c2e |  | U15 |
| 497 | `expander.arrow_color` | #8e8e93 | yes | U43 |
| 500 | `window.inactive_title_bar_background` | #1c1c1e |  | U44 |
| 501 | `window.inactive_title_bar_text_color` | #ffffff |  | U44 |
| 504 | `layout.widget_gap_px` | 8.0 |  | U45 |
| 506 | `layout.window_margin_px` | 20.0 | yes | U46 |
| 507 | `layout.section_gap_px` | 20.0 |  | U47 |

</details>

### Same rows

Rows whose cell holds at the versions checked; the first citation is given.

<details><summary>51 same rows</summary>

| Line | Property | Values | Source |
|---|---|---|---|
| 4 | `icon_set` | flat iOS 18: SF Symbols; Liquid Glass iOS 26: SF Symbols | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/sf-symbols.json (read 2026-10-11) |
| 5 | `icon_theme` | flat iOS 18: SF Symbols; Liquid Glass iOS 26: SF Symbols | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/sf-symbols.json (read 2026-10-11) |
| 11 | `defaults.background_color` | flat iOS 18: #ffffff; Liquid Glass iOS 26: #ffffff | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 12 | `defaults.text_color` | flat iOS 18: #000000; Liquid Glass iOS 26: #000000 | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 13 | `defaults.surface_color` | flat iOS 18: #f2f2f7; Liquid Glass iOS 26: #f2f2f7 | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 21 | `defaults.success_color` | flat iOS 18: #34c759; Liquid Glass iOS 26: #34c759 | https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Green, Li… |
| 47 | `defaults.font.family` | flat iOS 18: SF Pro; Liquid Glass iOS 26: SF Pro | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json ('SF Pro is the system f… |
| 48 | `defaults.font.size_pt` | flat iOS 18: 17; Liquid Glass iOS 26: 17 | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Body 1… |
| 49 | `defaults.font.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Body R… |
| 64 | `text_scale.caption.size_pt` | flat iOS 18: 12; Liquid Glass iOS 26: 12 | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Captio… |
| 65 | `text_scale.caption.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Captio… |
| 75 | `text_scale.dialog_title.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Title… |
| 79 | `text_scale.display.size_pt` | flat iOS 18: 34; Liquid Glass iOS 26: 34 | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Large… |
| 80 | `text_scale.display.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Large… |
| 86 | `button.min_height_px` | flat iOS 18: 44 (hit target); Liquid Glass iOS 26: 44 (default control size) | https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json ('a hit target needs… |
| 99 | `input.background_color` | flat iOS 18: #ffffff (Canvas = systemBackgroundColor); Liquid Glass iOS 26: #ffffff (Canvas = systemBackgroundColor) | https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L463 ; https://github.com/WebKit/WebKit/blob/de320ba3… |
| 100 | `input.min_height_px` | flat iOS 18: 44 (hit target); Liquid Glass iOS 26: 44 (default control size) | https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json ('a hit target needs… |
| 108 | `input.font.color` | flat iOS 18: #000000 (labelColor); Liquid Glass iOS 26: #000000 (labelColor) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 128 | `scrollbar.overlay_mode` | flat iOS 18: true (transient indicator); Liquid Glass iOS 26: true (transient indicator) | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/scroll-views.json (read 2026-10-11; 'a translucent scroll indicator that typically… |
| 134 | `slider.track_height_px` | flat iOS 18: 4; Liquid Glass iOS 26: 4 | https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L560 |
| 177 | `sidebar.font.color` | flat iOS 18: #000000 (labelColor); Liquid Glass iOS 26: #000000 (labelColor) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 183 | `popover.font.color` | flat iOS 18: #000000 (labelColor); Liquid Glass iOS 26: #000000 (labelColor) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 197 | `switch.disabled_thumb_color` | flat iOS 18: white; Liquid Glass iOS 26: white (light mode) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L763 |
| 198 | `switch.thumb_background` | flat iOS 18: white; Liquid Glass iOS 26: white | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L763 |
| 200 | `switch.track_height_px` | flat iOS 18: 31; Liquid Glass iOS 26: 31 | https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L668 |
| 205 | `dialog.button_order` | flat iOS 18: default button on the trailing side; Liquid Glass iOS 26: default button on the trailing side | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/alerts.json (read 2026-10-11; 'Always place the default button on the trailing side… |
| 227 | `combo_box.min_height_px` | flat iOS 18: 44 (hit target); Liquid Glass iOS 26: 44 (default control size) | https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json ('a hit target needs… |
| 259 | `defaults.background_color` | flat iOS 18: #000000; Liquid Glass iOS 26: #000000 | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 260 | `defaults.text_color` | flat iOS 18: #ffffff; Liquid Glass iOS 26: #ffffff | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 261 | `defaults.surface_color` | flat iOS 18: #1c1c1e; Liquid Glass iOS 26: #1c1c1e | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 269 | `defaults.success_color` | flat iOS 18: #30d158; Liquid Glass iOS 26: #30d158 | https://web.archive.org/web/20250607225442/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/color.json (iOS system colors, Green, Da… |
| 295 | `defaults.font.family` | flat iOS 18: SF Pro; Liquid Glass iOS 26: SF Pro | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json ('SF Pro is the system f… |
| 296 | `defaults.font.size_pt` | flat iOS 18: 17; Liquid Glass iOS 26: 17 | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Body 1… |
| 297 | `defaults.font.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Body R… |
| 312 | `text_scale.caption.size_pt` | flat iOS 18: 12; Liquid Glass iOS 26: 12 | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Captio… |
| 313 | `text_scale.caption.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Captio… |
| 323 | `text_scale.dialog_title.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Title… |
| 327 | `text_scale.display.size_pt` | flat iOS 18: 34; Liquid Glass iOS 26: 34 | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Large… |
| 328 | `text_scale.display.weight` | flat iOS 18: 400 (Regular); Liquid Glass iOS 26: 400 (Regular) | https://web.archive.org/web/20250523115548/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json (Large (default); Large… |
| 334 | `button.min_height_px` | flat iOS 18: 44 (hit target); Liquid Glass iOS 26: 44 (default control size) | https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json ('a hit target needs… |
| 348 | `input.min_height_px` | flat iOS 18: 44 (hit target); Liquid Glass iOS 26: 44 (default control size) | https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json ('a hit target needs… |
| 356 | `input.font.color` | flat iOS 18: #ffffff (labelColor); Liquid Glass iOS 26: #ffffff (labelColor) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 376 | `scrollbar.overlay_mode` | flat iOS 18: true (transient indicator); Liquid Glass iOS 26: true (transient indicator) | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/scroll-views.json (read 2026-10-11; 'a translucent scroll indicator that typically… |
| 382 | `slider.track_height_px` | flat iOS 18: 4; Liquid Glass iOS 26: 4 | https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L560 |
| 425 | `sidebar.font.color` | flat iOS 18: #ffffff (labelColor); Liquid Glass iOS 26: #ffffff (labelColor) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 431 | `popover.font.color` | flat iOS 18: #ffffff (labelColor); Liquid Glass iOS 26: #ffffff (labelColor) | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt#… |
| 446 | `switch.thumb_background` | flat iOS 18: white; Liquid Glass iOS 26: white | https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L763 |
| 448 | `switch.track_height_px` | flat iOS 18: 31; Liquid Glass iOS 26: 31 | https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L668 |
| 453 | `dialog.button_order` | flat iOS 18: default button on the trailing side; Liquid Glass iOS 26: default button on the trailing side | https://developer.apple.com/tutorials/data/design/human-interface-guidelines/alerts.json (read 2026-10-11; 'Always place the default button on the trailing side… |
| 475 | `combo_box.min_height_px` | flat iOS 18: 44 (hit target); Liquid Glass iOS 26: 44 (default control size) | https://web.archive.org/web/20241114144601/https://developer.apple.com/tutorials/data/design/human-interface-guidelines/accessibility.json ('a hit target needs… |
| 505 | `layout.container_margin_px` | flat iOS 18: 8; Liquid Glass iOS 26: 8 | https://developer.apple.com/tutorials/data/documentation/uikit/uiview/directionallayoutmargins.json (read 2026-10-11; 'For most views, the default layout margin… |

</details>

### Researcher notes

Each researcher's own notes, verbatim: coverage, method caveats and errors found outside the *differs* rows (wrong citations on ✅ rows, unpinned links). Step 7 corrects the citation errors (S4) as part of the row fixes.

<details><summary>preset</summary>

TARGET VERSION: ios.toml names no version, so cell_version is "none" for every row. Its palette is the flat-generation (iOS 13-18) one. accent, danger and warning, light and dark, equal the HIG values captured on 2025-06-07 and WebKit's platform/ios-18 baselines exactly. The light border/separator #c6c6c8 is exactly separatorColor rgba(60,60,67,0.29) composited on white, which is the iOS 18 alpha; iOS 26 lowered it to 0.12. Fonts and Dynamic Type sizes did not change between 18 and 26. Against iOS 26 the preset is stale for: blue (#0088ff/#0091ff), red (#ff383c/#ff4245), orange (#ff8d28/#ff9230), separator, the switch geometry (WebKit) and the progress-bar thickness (WebKit). No source separates iOS 27 from 26: the HIG colour and typography change logs end at 2025-12-16, and WebKit's platform/ios baseline was committed as the iOS 26 baseline. No iOS 27 value is claimed.
COUNTS: 330 keys checked: 51 same, 54 differs, 225 unverifiable. Line 3 (name) skipped. No line range is left uncovered.
METHOD CAVEATS: (1) Values tagged (WebKit) come from WebKit's own iOS rendering of HTML form controls, the platform-facts '(WebKit)' source tag. UIKit may differ, so every such row sets probe_could_settle. (2) The 44 pt min_height comes from the HIG's control and hit-target size, not from a drawn outer box. (3) The text_scale roles were mapped as in the macOS column (.caption1/.headline/.title1/.largeTitle); the notes on lines 69 and 74 give the alternatives (Subhead, Title 2).
STEP 1 MAP, CORRECTIONS AND ADDITIONS: (a) There is a better iOS 18 source than mapping WebKit tags to iOS releases: WebKit main carries LayoutTests/platform/ios-18 baselines, which hold UIKit's iOS 18 dynamic-colour values, next to platform/ios, which holds iOS 26. (b) The iOS 26 dark values missing from Step 1 are Red 255,66,69 and Orange 255,146,48 (HIG plus WebKit). (c) HIG layout.json (updated 2026-09-09) no longer holds iOS margin or spacing numbers, so it did not settle any layout key.
ALREADY ON todo.md: disabled_opacity 0.3 together with disabled colours (todo.md:688-697); iOS sizes unsourced (todo.md:2317-2322). This review adds a HIG source for the 44 min_height, a UIKit source for container_margin 8, and the UIStackView spacing 0.0 default for widget_gap. NOT on todo.md (new): the palette is iOS 13-18, not iOS 26 (Step 1 said so too); muted is systemGray rather than secondaryLabel; the text_scale line heights are size x 1.2 rather than the HIG leading; section_heading 15/700 matches no iOS style; dark input background #1c1c1e is not systemBackground; dark border #38383a matches no source.
LEGEND for the unverifiable notes:
U1 UIKit has no shadow-colour API; the HIG gives no value.
U2 No iOS API or HIG value for text on a status or accent fill.
U3 No published RGB. WebKit leaves selection to UIKit (platformActiveSelectionBackgroundColor = transparent, https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L1005-L1008).
U4 The HIG lists Link (UIColor.link) without an RGB; WebKit has no baseline for it.
U5 No iOS API. Already on todo.md:688-697. For reference, WebKit iOS dims disabled inputs by opacity 0.4 (https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L962-L964).
U6 No iOS API or HIG value for focus-ring geometry.
U7 No iOS global corner radius. WebKit's iOS non-capsule controls use 5 (https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/ios/RenderThemeIOS.mm#L259; https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L465). Buttons 45 pt or taller get 0.175 x their smaller side; smaller buttons are capsules.
U8 No iOS global line width. WebKit iOS text fields use 1 px at the pre-refresh tag (https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L466) and 0.5 in the iOS 26 painter (https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm#L2915).
U9 No iOS API or HIG statement.
U10 UIFont.monospacedSystemFont's doc names no family; the HIG lists SF Mono only as an SF variant.
U11 monospacedSystemFont takes the size and weight as arguments; there is no default.
U12 SF Symbols scales are relative to text; there is no point-size constant.
U13 The HIG gives a 44x44 default control size, not a 64 minimum width. On todo.md:2317-2322.
U14 No HIG or UIKit value; imagePadding has no documented default.
U15 Touch controls have no hover state; iPadOS has pointer hover, but no hover colour is published.
U16 No published pressed-state colours; WebKit keeps systemBlue text on :active.
U17 WebKit uses opaque-secondary-fill-disabled (secondarySystemFill at 0.75, made opaque, RenderThemeIOS.mm:1148), which has no RGB baseline.
U18 No published colour for a focused text-field border.
U19 No published value; WebKit dims a disabled input by opacity 0.4 instead.
U20 iOS has no checkbox. HIG Toggles (toggles.json, read 2026-10-11) says macOS supports the checkbox style. WebKit draws HTML checkboxes on iOS at 16x16 (https://github.com/WebKit/WebKit/blob/31adf9d1605fad1fb548e86d36384161a2a060b4/Source/WebCore/css/html.css#L1008-L1013).
U21 iPadOS has tooltips (UIToolTipInteraction, iOS 15+), but no colours or width are published.
U22 UIScrollView indicators have no published sizes or colours.
U23 UISlider has no tick marks. WebKit draws datalist ticks of 1x3 (old RenderThemeIOS.mm:796-800), and the refreshed painter uses track/4 = 1 (RenderThemeCocoa.mm:3880).
U24 No published disabled slider colours.
U25 UIProgressView has no published minimum width.
U26 UITabBar and HIG Tab bars give no iOS height or item width; tab-bars.json's 68 pt is tvOS. On todo.md:2317-2322.
U27 No published selected-tab colours.
U28 UIMenu spacing and highlight colours are not published.
U29 iOS tables do not alternate row colours.
U30 Same as U15. Table selection is tableCellDefaultSelectionTintColor (209,209,214 / 58,58,60, WebKit baselines L150-154), which is not hover.
U31 No published value.
U32 The iPadOS sidebar background is not published. systemGroupedBackground is #f2f2f7 / #000000 (baselines L54-58).
U33 The popover fill is not published. Dark #1c1c1e equals systemBackground at the elevated level (baseline L10; uiuserinterfacelevel/elevated.json, read 2026-10-11). Light #ffffff is the same at both levels.
U34 No iOS splitter API.
U35 No published colours. WebKit's refreshed switch uses disabled opacity 0.5 (RenderThemeCocoa.mm:415).
U36 The track corner fraction comes from Apple's internal SDK; open-source WebKit sets 0 (RenderThemeCocoa.mm:108-110).
U37 UIAlertController's size is not published. On todo.md:2317-2322.
U38 No stated default; HIG Accessibility only suggests an underline in addition to colour.
U39 No published link-state colours; hover is not a touch state.
U40 UIActivityIndicatorView styles have no published sizes.
U41 Pop-up and pull-down buttons have no published value.
U42 UISegmentedControl metrics and selected-segment colours are not published.
U43 HIG Disclosure controls gives no iOS metrics or colours.
U44 iOS windows have no inactive title bar colours.
U45 UIStackView.spacing defaults to 0.0 (uistackview/spacing.json, read 2026-10-11), which platform-facts treats as (none) in the Windows column; no 8 pt gap is published. On todo.md:2317-2322.
U46 The systemMinimumLayoutMargins doc gives 20 only as an example. On todo.md:2317-2322.
U47 No published value. On todo.md:2317-2322.
Working files: /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/ios-preset/ (hig/, wb/, uikit/, wk-old/, wk-main/, wk-lt/).

</details>

## M2d — Material 3 and Material 3 Expressive

**Status:** Steps 1, 2, 4, 5 and 6 done 2026-10-11, beside M1 (D7), in the `m2-research` worktree; Step 3 awaits the maintainer; Step 7 not started. One Workflow ran all five parts (41 Opus agents at high effort, 5.41M subagent tokens, 36.5 min). Triage values: *fix* (Step 7), *Step 8* / *Step 13* (the part's build steps, already decided), *maintainer (Step 3 probe)*, *maintainer (version gap)*, *M4* (reader code), *defer* (an existing docs/todo.md entry), *drop*.

**D6's premise does not hold at a stable release, so Step 1's pick is the maintainer's.** The latest stable Compose Material 3
is **1.4.0** (2025-09-24, androidx `5113b82a`); in it `MaterialExpressiveTheme`, `expressiveLightColorScheme` and
`MotionScheme.expressive()` are `internal` and the Expressive components were removed (1.4.0-beta01). They are public only
from **1.5.0-beta01** (2026-10-07, `19991f2c`, not stable; `LoadingIndicator` and `MaterialShapes` still
`@ExperimentalMaterial3ExpressiveApi`). The facts' `1e67768` pin matches 1.5.0-beta01 (tokens and API identical). For
`material-3` the choice barely matters (1.4.0 and 1.5.0-beta01 token values differ only in Checkbox, List, Typography
plumbing and seven new list/menu/chip token files). The token database has no single version: each generated `*Tokens.kt`
file names its own (v0_210, v0_11_0, 14_1_0, …). Every v0_192 colour hex in the column is confirmed and the Compose colour
roles are identical, except Expressive's four light `on-*-container` roles at tone 30. Expressive changes nothing else
through the theme (no component reads `LocalUsingExpressiveTheme`): its look comes from the new opt-in components. All 50
*differs* rows are CONFIRMED: they are what Step 13 re-bases (slider 16 track and 4 x 44 handle, progress
`secondary-container` and round caps, checkbox drawn 20, outlined button `outline-variant`, menu and list paddings, dialog
without shadow or height bounds, spinner 40, select 280, `bodyLarge` 16/24 as the ambient text style).

### Step 1 — version map

| Version | Released | Support on 2026-10-11 | Generation |
|---|---|---|---|
| androidx.compose.material3:material3 1.4.0 Compose Material 3 1.4.0 (latest STABLE) | 2025-09-24 (release notes heading 'Version 1.4.0 / September 24, 2025'; built from commit 5113b82a4ed6416d6344efa50e41db8cb65928ff, branch androidx-compose-mate… | Current stable per https://developer.android.com/jetpack/androidx/releases/compose-material3 (read 2026-10-11; version table: Latest Update October 07, 2026 / Stable 1.4.0 / RC - / Beta 1.5.0-beta01 / Alpha -). No 1.4.x patch exists: https://dl.google.com/andr… | Material 3 (baseline MaterialTheme only; Expressive is internal) |
| androidx.compose.material3:material3 1.5.0-beta01 Compose Material 3 1.5.0-beta01 (pre-release, not stable) | 2026-10-07 (release notes; built from 19991f2c0858aadfcb442ce02b2c8e4743c96898 on androidx-compose-material3-release, 2026-09-28, 'Snap for 16458435 from 1358a4… | Beta per https://developer.android.com/jetpack/androidx/releases/compose-material3 (read 2026-10-11); Maven metadata <latest>/<release> = 1.5.0-beta01 (lastUpdated 20261007170356). No 1.5.0 stable date published. | Material 3 + Material 3 Expressive (MaterialExpressiveTheme public) |
| androidx commit 1e6776853023ebe4266aca2b308305723e3a2d57 platform-facts.md's current androidx pin (androidx-main, 2026-0… | not a release; libraryversions.toml:29 at this commit says COMPOSE_MATERIAL3 = "1.5.0-beta01" | n/a (snapshot between 1.5.0-beta01 and the next pre-release) | Material 3 + Material 3 Expressive |
| material-web v2.5.0 / main 47adb655bd7a88c4d62e8faac2873084eed555dc @material/web (latest release v2.5.0 = b4de401eb665e… | v2.5.0: 2026-07-15 (gh api releases/latest); token sets last changed 2026-04-06 (763a97326fc2d0f425366cc3fa87bd0eccfaabdc 'chore: update latest tokens to v34') | GitHub releases https://github.com/material-components/material-web/releases (read 2026-10-11); no lifecycle policy | cross-check: tokens/versions/latest (Version: 34.0.21) and tokens/versions/v0_19… |
| com.google.android.material:material 1.14.0 material-components-android 1.14.0 (tag commit 66c334b7946dabf33adfe1a2b7cad… | 2026-05-13 (GitHub release published_at 2026-05-13T15:02:52Z) | Latest: https://github.com/material-components/material-components-android/releases and https://dl.google.com/android/maven2/com/google/android/material/material/maven-metadata.xml (<latest>/<release> 1.14.0), read 2026-10-11. Release notes list 'Expressive Th… | Views toolkit (secondary to Compose for D6) |
| Android 17 (API 37) Android 17 | 26Q2; first tag android-17.0.0_r1 (CP2A.260605.016), security patch level 2026-06-05 (https://source.android.com/docs/setup/reference/build-numbers, read 2026-1… | Receiving updates: October 2026 bulletin (published 2026-10-05, updated 2026-10-08) lists 'Updated AOSP versions: 14, 15, 16, 16-qpr2, 17' (https://source.android.com/docs/security/bulletin/2026/2026-10-01) | OS context only (secondary) |
| Android 16 QPR2 (API 36.1) / Android 16 (API 36) Android 16 | Android 16: 25Q2, android-16.0.0_r1 SPL 2025-06-05; 16 QPR2: 25Q4 (build-numbers page, read 2026-10-11) | In the October 2026 bulletin's updated AOSP versions (16, 16-qpr2) | OS context; Pixel system UI moved to Material 3 Expressive with 16 QPR1 (Sept 20… |
| Android 15 (API 35) Android 15 | 24Q3; android-15.0.0_r1 SPL 2024-09-05 (build-numbers page) | In the October 2026 bulletin's updated AOSP versions | OS context (pre-Expressive system UI) |
| Android 14 Android 14 | (not checked) | Still in the October 2026 bulletin's updated AOSP versions for some CVEs | OS context |

Generations:

- **Material 3 baseline as material-web renders it (token set v0_192)** (material-web cbd34a8921915af94d5ef65c2a69eece41d5b4f3 (2026-09-24) .. main 47adb655 (2026-10-07): tokens/versions/ unchanged across those 17 commits (compare API: no tokens/ file touched)): What material.toml is built from today. Older component specs than Compose: e.g. slider 4 track / 20 round handle, square progress bar (per D6 in the plan). Source: https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions (dirs: README.md, latest, v0_192); last commit touching v0_192: 60c0cfa58ad135c189cb0fa95c5f2744499f0327 (2025-10-07)
- **Material 3 as Compose MaterialTheme renders it (D6 'material-3')** (Compose Material 3 1.4.0 (stable, 5113b82) and 1.5.0-beta01 (19991f2c) / pin 1e67768): Generated *Tokens.kt (113 files at 1.4.0, 120 at beta01). Token VALUES are identical between 1.4.0 and beta01 except: CheckboxTokens (header v0_117 -> 14_1_0; ContainerHeight/Width 18 -> ContainerSize 18, error outline widths 0/2 dropped, FocusIndicatorColor added), ListTokens (v0_210 -> 29.0.0; ListItem* renamed Item*, Expressive shapes added, LargeLeadingVideoHeight 69 -> 64), TypographyTokens (v0_103 -> 31.0.11; class taking a default FontFamily, same TypeScaleTokens refs), and 7 new files (ChipsTokens, ExpandedListTokens, ReorderListTokens, RevealListTokens, SegmentedMenuTokens, StandardMenuTokens, VibrantMenuTokens); the rest is a code-style change (val X = v -> inline val X: T get() =… Source: https://developer.android.com/jetpack/androidx/releases/compose-material3 (read 2026-10-11) + sparse checkouts of compose/material3/material3/src/commonMain at 5113b82, 19991f2c, 1e67768 (normalised name=value diff)
- **Material 3 Expressive (D6 'material-3-expressive')** (public only from 1.5.0-alpha18 (2026-04-22) onward, i.e. 1.5.0-beta01 / pin 1e67768; NOT in any stable release): 1.4.0-beta01 (2025-07-30): 'All public APIs tagged with ExperimentalMaterial3ExpressiveApi ... have been removed, please switch to 1.5.0-alpha'. In 1.4.0 source: MaterialExpressiveTheme internal (MaterialTheme.kt:185), expressiveLightColorScheme internal (ColorScheme.kt:1138), MotionScheme internal interface (MotionScheme.kt:42), expressive() internal (:129); 1.4.0 api/current.txt has no MaterialExpressiveTheme, MotionScheme, ButtonGroup, LoadingIndicator, Wavy entries. 1.5.0 promotions: alpha15 'Graduate motion scheme from experimental'; alpha18 'Promote WavyProgressIndicator apis', 'Promote materialExpressTheme, expressiveLightColorScheme', 'Add Material3ExpressiveApi annotation that doesn… Source: https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/api/1.5.0-beta01.txt ; https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/api/current.txt ; release notes page above
- **material-web 'latest' token set (cross-check)** (tokens/versions/latest/sass, 195 files, every header '// Version: 34.0.21', unchanged since 763a97326fc2d0f425366cc3fa87bd0eccfaabdc (2026-04-06); identical at cbd34a8 and main 47adb655): Newer token database than v0_192 (contains Expressive comp tokens such as _md-comp-button-small.scss, emphasized typescale) Source: https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass

Pinned sources:

- androidx Compose Material 3 1.4.0 (stable) release commit: https://github.com/androidx/androidx/tree/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens — 5113b82a4ed6416d6344efa50e41db8cb65928ff (androidx-compose-material3-release, 2025-09-17; no git tag on the GitHub mirror, which has none; commit range from the 1.4.0 release notes 122e65c9..5113b82a); covers material-3 if D6 requires a STABLE release: generated tokens (113 files) + component sources; no Expressive public API
- androidx Compose Material 3 1.5.0-beta01 release commit: https://github.com/androidx/androidx/tree/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens — 19991f2c0858aadfcb442ce02b2c8e4743c96898 (release branch, 2026-09-28; snap of androidx-main 1358a48e9c23e257ae3357f6f645cb60a3610a7f; release-notes range b2b5ae7b..19991f2c); covers material-3 and material-3-expressive at the first release where MaterialExpressiveTheme is public (beta, not stable)
- androidx androidx-main snapshot already cited by platform-facts.md:865: https://github.com/androidx/androidx/tree/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/material3 — 1e6776853023ebe4266aca2b308305723e3a2d57 (2026-09-28T15:40:05Z); covers 34 commits after 1358a48e (compare: ahead 34, behind 0). Versus 19991f2c: tokens/ identical (diff -rq empty), api/current.txt identical, whole commonMain differs only in ScrollField.kt. So the 8 facts…
- Compose Material 3 release notes: https://developer.android.com/jetpack/androidx/releases/compose-material3 — read 2026-10-11 (Latest Update October 07, 2026); covers stable/beta status, dates, commit ranges, API promotions
- material-web token sets: https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions — main 47adb655bd7a88c4d62e8faac2873084eed555dc (2026-10-07); tokens identical to facts pin cbd34a8921915af94d5ef65c2a69eece41d5b4f3; latest release tag v2.5.0 = b4de401eb665ec63474f39319a4ba8f2145974cc; covers latest (34.0.21) cross-check and v0_192 (current preset)
- material-components-android: https://github.com/material-components/material-components-android/tree/1.14.0 — tag 1.14.0 = 66c334b7946dabf33adfe1a2b7cad6bcaa4ea3ad (2026-05-13), same as facts pin; covers Views implementation, Expressive styles; secondary to Compose
- Android build numbers / security bulletin: https://source.android.com/docs/setup/reference/build-numbers ; https://source.android.com/docs/security/bulletin/2026/2026-10-01 — read 2026-10-11 (bulletin published 2026-10-05, updated 2026-10-08); covers Android versions in support (14, 15, 16, 16-qpr2, 17)

Preset target: /home/tibi/Rust/native-theme-m2/native-theme/src/presets/material.toml:1-7: "# Material Design 3 baseline theme / # Source: the material-web token set v0_192 at material-web cbd34a8921915af94d5ef65c2a69eece41d5b4f3 (tokens/versions/v0_192/), the one its components use; docs/platform-facts.md's Material column." Colours via _md-sys-color.scss:19-135 / _md-ref-palette.scss:15-109, state opacities _md-sys-state.scss:17-20. It targets material-web v0_192, not any Compose release; there is one preset (material.toml), no -live base. platform-facts.md:865 pins the column to material-web v0_192 @cbd34a8 (latest = 34.0.21), MDC-Android 1.14.0 @66c334b, androidx @1e67768 (no Compose version named; that commit is 1.5.0-beta01 + 34 commits). Row citation counts (lines starting with '\|'): 174 cite v0_192, 35 cite material-web latest, 8 cite androidx 1e67768, 2 cite MDC-Android.

<details><summary>Version-map notes</summary>

DECISION NEEDED (D6 premise does not hold): the latest STABLE Compose Material 3 is 1.4.0 (2025-09-24), and in it MaterialExpressiveTheme, expressiveLightColorScheme, MotionScheme and MotionScheme.expressive() are `internal` and every Expressive component API was removed (1.4.0-beta01 note). They are public only in 1.5.0-alpha18+ / 1.5.0-beta01 (2026-10-07), not stable; LoadingIndicator and MaterialShapes remain @ExperimentalMaterial3ExpressiveApi even there. Options for the maintainer: (a) pin both presets at 1.5.0-beta01 (19991f2c), which the facts' 1e67768 already matches (tokens and api identical; only ScrollField.kt differs), and re-pin at 1.5.0 stable when published; (b) material-3 at 1.4.0 (what Compose BOM 2026.09.00 users render), material-3-expressive waits for 1.5.0 stable. For material-3 the choice barely matters: 1.4.0 and beta01 tokens have identical values except Checkbox, List, Typography (font-family plumbing only) and 7 new list/menu/chip token files. Token-database version: there is no single one. Each generated file has its own '// VERSION:' header (line 16), mixed schemes: v0_210 (21 files incl. ColorLight/ColorDark/Palette/State/Switch/Menu), v0_11_0 (22: ButtonSmall/XSmall/Medium/Large/XLarge, ButtonGroupSmall, Filled/Tonal/Outlined/Text/Elevated buttons, nav bar/rail), 14_1_0 (12: Shape, ShapeKey, Checkbox, icon buttons, ConnectedButtonGroupSmall), v0_14_0 (12: FAB, Expressive/StandardMotion), v0_103 (10: TypeScale, Elevation, OutlinedTextField, FilledTonalButton, Snackbar), 14_0_0 (7: app bars, DockedToolbar), 7_0_1 (5 chips/DragHandle), 18_0_18 (5 SplitButton*), v0_162 (4: ColorSchemeKey, tabs, OutlinedSegmentedButton), 29.0.0 (4 lists), v0_7_0 (3: Circular/LinearProgressIndicator, LoadingIndicator), v0_117 (3: Divider, RadioButton, Scrim), 24.1.2 (3 menus), v0_4_0 (ProgressIndicator), v0_192 (OutlinedCard), v0_161 (DateInputModal), 37.2.1 (Chips), 31.0.11 (Typography), 12_0_0 (FloatingToolbar); SliderTokens.kt, PlainTooltipTokens.kt, MotionSchemeKeyTokens.kt have no VERSION header. Tokens dir: compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/ (confirmed). Expressive has no dark scheme: dark = darkColorScheme() (ColorScheme.kt:1166), relevant for D6 'light and dark'. D6's cited lines verified at 1e67768: ColorScheme.kt:1172-1179 (onPrimary/Secondary/Tertiary/ErrorContainer = *30), MaterialTheme.kt:243-262. The GitHub mirror androidx/androidx has no git tags; releases are pinned by commit. material-web: no token change since the facts pin. MDC-Android 1.14.0 still latest (no 1.15 on Maven). Android system UI and Material 3 Expressive: SECONDARY only (press: Android Authority, Tech Advisor 2025-09-08, Droid-Life 2025-06-10) say it reached Pixel 6+ with Android 16 QPR1 in early September 2025, with QPR1 AOSP source published 2025-11-11; no Google primary page was opened, so treat it as unverified context. No primary source gives Android version usage shares (the dashboard is inside Android Studio only). No docs/todo.md item covers the Compose stable/Expressive status (the Material items at todo.md:225-244 and :787-812 are about radio and slider values). Scratch files: /tmp/claude-1000/-home-tibi-Rust-native-theme/a0421ea2-02c5-43d6-8b7a-682d83a2e8e4/scratchpad/m2-src/m2d/ (ax-s140, ax-b01, ax-pin sparse checkouts, mweb, rel/page.txt); token value diff: .../scratchpad/m2-scripts/diff_s140_b01.txt.

</details>

### Step 2 — rows checked

282 rows: 232 same, 50 differs, 0 unverifiable (0 of them a probe or reference app could settle). Scopes: globals 41, widgets-1 154, widgets-2 87. Every researcher reported its scope covered with no line range left out.

Step 5 verdicts on the *differs* rows: 50 CONFIRMED.

Step 6 triage: 49 Step 13 (D6), 1 fix.

### Findings — *differs* rows

#### M2d-01 · bug · S2 · docs/platform-facts.md:1026
**Claim:** `size` (2.1.1 Base Font): the cell says body-medium-size 0.875rem = 14; v0_192 (cell) material-web cbd34a8: body-medium-size 0.875rem = 14; Compose M3 baseline 1.4.0 (5113b82): 16: MaterialTheme sets the default text style to typography.bodyLarge (BodyLargeSize 16.sp). BodyMediumSize 14.sp also exists (TypeScaleTokens.kt:31); Compose M3 Expressive 1.5.0-beta01 (19991f2c): 16, the same: the internal MaterialTheme that MaterialExpressiveTheme calls also does ProvideTextStyle(typography.bodyLarge), with Typography() as the default.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L56 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt#L107 ; https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypeScaleTokens.kt#L26 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt#L113 Note: This is a mapping decision. A plain Text() under a Compose MaterialTheme renders bodyLarge (16/24), not body-medium (14/20). The material-web side gives no app-level default, so the cell picked body-medium. The Expressive entry is identical to the baseline and is listed only for completeness.
**Verdict:** CONFIRMED — Read all three: the mw token is 0.875rem, and both Compose versions wrap content in bodyLarge (16sp); MaterialExpressiveTheme (1.5.0-beta01 :243-268) calls the public MaterialTheme with Typography(), so it gets the same default. Confirmed value: material-web v0_192 body-medium-size 0.875rem (=14) at _md-sys-typescale.scss:56; Compose 1.4.0 MaterialTheme.kt:107 and 1.5.0-beta01 MaterialTheme.kt:113 ProvideTextStyle(typography.bodyLarge), BodyL….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-02 · bug · S2 · docs/platform-facts.md:1030
**Claim:** `line_height` (2.1.1 Base Font): the cell says 1.43 = body-medium-line-height 20 / 14; v0_192 (cell) material-web cbd34a8: 20 / 14 = 1.43; Compose M3 baseline 1.4.0 (5113b82): default style bodyLarge: 24.sp / 16.sp = 1.5 (bodyMedium 20/14 = 1.43 also exists, :30-31).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L55 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypeScaleTokens.kt#L25-L26 ; .../MaterialTheme.kt#L107 Note: Follows row 1026: it differs only if the base font maps to Compose's default bodyLarge. Compose's BodyMediumTracking is 0.2.sp (TypeScaleTokens.kt:32), material-web's is 0.015625rem = 0.25 (v0_192 :57); no row in scope covers tracking.
**Verdict:** CONFIRMED — The cited lines read exactly these values, and bodyLarge is the default text style from MaterialTheme.kt:107. Confirmed value: mw body-medium-line-height 1.25rem = 20 / 14 = 1.43 (:55); Compose default bodyLarge 24.sp/16.sp = 1.5 (TypeScaleTokens.kt:25-26); bodyMedium 20/14 (:30-31).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-03 · bug · S2 · docs/platform-facts.md:1051
**Claim:** `border.color` (2.1.3 Base Colors): the cell says outline L #79747e, D #938f99 (outlined text field, button, select, segmented button); decorative outline-variant; v0_192 (cell) material-web cbd34a8: outlined-button outline-color = outline; outlined-text-field outline-color = outline; outline = neutral-variant50 #79747e / neutral-variant60 #938f99; Compose M3 baseline 1.4.0 (5113b82): Text field: Outline (OutlinedTextFieldTokens:82, used at TextFieldDefaults.kt:1393). Segmented button: Outline (OutlinedSegmentedButtonTokens:33, SegmentedButton.kt:552-555). OUTLINED BUTTON: OutlineVariant #cac4d0 / #49454f (OutlinedButtonTokens:36, used by B….
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-button.scss#L71 ; .../v0_192/_md-comp-outlined-text-field.scss#L145 ; .../v0_192/_md-sys-color.scss#L108 , #L49 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/OutlinedButtonTokens.kt#L36 ; .../Button.kt#L898-L909 ; .../tokens/OutlinedTextFieldTokens.kt#L82 ; .../tokens/OutlinedSegmentedButtonTokens.kt#L33 Note: The role (outline) still holds for fields, select and segmented button. Only the parenthetical 'button' is wrong for Compose: its outlined button uses outline-variant, from token set v0_11_0. material-web latest agrees: md.comp.button.outlined outline-color = outline-variant (latest/_md-comp-button-outlined.scss:58), and the old md.comp.outlined-button is marked [Deprecated] there.
**Verdict:** CONFIRMED — Every cited line holds; in Compose only the outlined button departs from the cell, using outline-variant rather than outline. Confirmed value: mw: outlined-button and outlined-text-field outline-color = outline (#79747e / #938f99). Compose 1.4.0: OutlinedButtonTokens.OutlineColor = OutlineVariant (:36; L #cac4d0 NeutralVariant80, D #49454f N….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-04 · bug · S2 · docs/platform-facts.md:1053
**Claim:** `shadow_color` (2.1.3 Base Colors): the cell says md.sys.color.shadow #000000; material-web key 0.3 / ambient 0.15; v0_192 (cell) material-web cbd34a8: shadow = neutral0 #000; elevation key-shadow opacity 0.3, ambient 0.15; Compose M3 baseline 1.4.0 (5113b82) + Android 16: ColorScheme has no shadow role (ColorScheme.kt:148-197). Surface draws a graphicsLayer shadow (Surface.kt:463-468) in DefaultShadowColor = Color.Black. Its alpha is the platform's: ambient_shadow_alpha 0.039, spot_shadow_alpha 0.19.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L119 ; https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/elevation/internal/_elevation.scss#L105 , #L174 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/ui/ui/src/commonMain/kotlin/androidx/compose/ui/graphics/GraphicsLayerScope.kt#L37 ; https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Surface.kt#L463-L468 ; https://android.googlesource.com/platform/frameworks/base/+/refs/tags/android-16.0.0_r1/core/res/res/values/dimens.xml#859 (read 2026-10-11) Note: The black colour is the same. The 0.3 / 0.15 opacities are material-web's CSS only. On Android, Compose uses the platform's light-source shadow (ambient 0.039 / spot 0.19, from AOSP dimens.xml:859-860).
**Verdict:** CONFIRMED — All values read at the cited lines; these alphas are framework resource defaults that a device overlay can override, so a measurement would settle what a given device shows. Confirmed value: mw: shadow = neutral0 #000 (_md-sys-color.scss:119), key opacity 0.3 (_elevation.scss:105), ambient 0.15 (:174). Compose 1.4.0: no shadow role in ColorScheme; Surface uses graphicsLayer(shadowElevatio….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-05 · bug · S2 · docs/platform-facts.md:1058
**Claim:** `text_selection_background` (2.1.3 Base Colors): the cell says (none): no text-selection token; material-web leaves ::selection to the browser; v0_192 (cell) material-web cbd34a8: no token (claim not re-checked in material-web component code); Compose M3 baseline 1.4.0 (5113b82): (impl) MaterialTheme provides LocalTextSelectionColors with background = colorScheme.primary at alpha 0.4 (TextSelectionBackgroundOpacity) and handle = primary; OutlinedTextField uses it (TextFieldDefaults.kt:1390). L #6750a4 @0.4, D #d0bcff @0.4; Compose M3 baseline 1.5.0-beta01 (19991f2c): unchanged: primary @ 0.4.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/tree/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt#L98-L104 , #L218-L229 ; .../TextFieldDefaults.kt#L1390 [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt#L276-L287 Note: This is an implementation constant, not an md token. The ColorScheme KDoc (:60) says 'secondary ... Highlighting selected text', but the code uses primary. Expressive uses the same code with the same light primary.
**Verdict:** CONFIRMED — Read as claimed; the four material-web textfield/field internal scss files at cbd34a8 have no selection rule, so the cell's material-web half also holds. Confirmed value: Compose 1.4.0: MaterialTheme provides LocalTextSelectionColors (MaterialTheme.kt:98,104) = primary @ TextSelectionBackgroundOpacity 0.4, handle = primary (:218-229); OutlinedTextField uses it (TextFie….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-06 · bug · S2 · docs/platform-facts.md:1104
**Claim:** `focus_ring_color` (2.1.5 Focus Ring): the cell says (impl) md-focus-ring color = secondary L #625b71, D #ccc2dc; v0_192 (cell) material-web cbd34a8: focus-ring 'color' = secondary (secondary40 #625b71 / secondary80 #ccc2dc); Compose M3 baseline 1.4.0 (5113b82): no focus ring. Focus is a state layer: content colour at FocusStateLayerOpacity 0.1 (RippleDefaults.RippleAlpha). The many *Tokens.FocusIndicatorColor = Secondary tokens are unused (no non-token source references them); Compose M3 baseline 1.5.0-beta01 (19991f2c): default is still Focus.Opacity (RippleDefaults.ThemeConfiguration = OpacityFocusThemeConfiguration, :360). An inset ring is opt-in through LocalRippleThemeConfiguration = InsetFocusRingThemeConfiguration: outer stroke colorScheme.secondary, inner stroke onSeco….
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-focus-ring.scss#L47 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Ripple.kt#L134-L140 ; .../tokens/StateTokens.kt#L23 [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Ripple.kt#L349-L360 , #L784-L801 Note: MaterialExpressiveTheme does not switch the ripple configuration (beta01 MaterialTheme.kt:243-270), so Expressive equals the baseline. Under Compose the focus indication has no ring colour; it is a 10% state layer. The opt-in ring's colour (secondary) matches the cell.
**Verdict:** CONFIRMED — Read all cited lines, and grep of a sparse checkout of 1.4.0 commonMain found no non-token use of FocusIndicatorColor; MaterialTheme does not provide the inset-ring configuration. Confirmed value: mw focus-ring color = secondary (_md-comp-focus-ring.scss:47; secondary40 #625b71 / secondary80 #ccc2dc). Compose 1.4.0: no ring, focus = state layer FocusStateLayerOpacity 0.1 (Ripple.kt:134-140, Sta….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-07 · bug · S2 · docs/platform-facts.md:1105
**Claim:** `focus_ring_width` (2.1.5 Focus Ring): the cell says (impl) material-web 3; latest focus-indicator.thickness 3; v0_192 (cell) material-web cbd34a8: width 3px; latest thickness 3px; Compose M3 baseline 1.4.0 (5113b82): no ring (0); focus = 0.1 state layer; Compose M3 baseline 1.5.0-beta01 (19991f2c): opt-in inset ring: outerStrokeWidth 2.dp, innerStrokeWidth 3.dp (default off).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-focus-ring.scss#L52 ; .../tokens/versions/latest/sass/_md-sys-state-focus-indicator.scss#L23 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Ripple.kt#L134-L140 [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Ripple.kt#L349-L357
**Verdict:** CONFIRMED — The values are as cited, and Ripple.kt:360 sets the default to the opacity configuration. Confirmed value: mw width 3px (_md-comp-focus-ring.scss:52), latest thickness 3px (_md-sys-state-focus-indicator.scss:23); Compose 1.4.0 no ring; 1.5.0-beta01 opt-in InsetFocusRingThemeConfiguration outerStrokeWidth 2….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-08 · bug · S2 · docs/platform-facts.md:1106
**Claim:** `focus_ring_offset` (2.1.5 Focus Ring): the cell says (impl) outward-offset 2; latest outer-offset 2; v0_192 (cell) material-web cbd34a8: outward-offset 2px (inward-offset 0px :49); latest outer-offset 2px, inner-offset -3px; Compose M3 baseline 1.4.0 (5113b82): no ring; Compose M3 baseline 1.5.0-beta01 (19991f2c): opt-in ring is INSET (drawn inside the shape): outerStrokeInset 0.dp, innerStrokeInset 1.dp; no outward offset.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-focus-ring.scss#L50 ; .../tokens/versions/latest/sass/_md-sys-state-focus-indicator.scss#L19-L21 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Ripple.kt#L134-L140 [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Ripple.kt#L349-L357
**Verdict:** CONFIRMED — Read as cited; the Compose ring is named and configured as an inset ring, so it has no outward offset. Confirmed value: mw outward-offset 2px (:50), inward-offset 0px (:49); latest inner-offset -3px, outer-offset 2px (_md-sys-state-focus-indicator.scss:19,21); Compose 1.4.0 no ring; 1.5.0-beta01 opt-in inset ring outer….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-09 · bug · S2 · docs/platform-facts.md:1116
**Claim:** `border.opacity` (2.1.6 Global Geometry): the cell says (none): enabled outlines opaque; 0.12 is only the disabled outline opacity (outlined button :38-39); v0_192 (cell) material-web cbd34a8: outlined-button disabled-outline on-surface @ 0.12; Compose M3 baseline 1.4.0 (5113b82): Enabled outlines are opaque (same). Disabled outlined-button outline = OutlineVariant @ 0.1 (OutlinedButtonTokens.DisabledContainerOpacity, Button.kt:905-906). Disabled outlined text field outline = OnSurface @ 0.12 (OutlinedTextFieldTokens:33-34). Segmented =….
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-button.scss#L38-L39 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt#L898-L909 ; .../tokens/OutlinedButtonTokens.kt#L22 , #L36 ; .../tokens/OutlinedTextFieldTokens.kt#L33-L34 Note: '(none)' holds. Only the outlined-button example differs: outline-variant @ 0.1 in Compose against on-surface @ 0.12 in v0_192. material-web latest button.outlined agrees with Compose (disabled-outline-color outline-variant, container opacity 0.1; latest/_md-comp-button-outlined.scss:22, :32).
**Verdict:** CONFIRMED — Every cited line holds; the Compose outlined button really does use 0.1 on outline-variant, not mw's 0.12 on on-surface. Confirmed value: mw outlined-button disabled-outline on-surface @ 0.12 (:38-39). Compose 1.4.0: disabled outlined button = OutlineColor (OutlineVariant) @ DisabledContainerOpacity 0.1 (OutlinedButtonTokens:22,36; Butt….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-10 · bug · S4 · docs/platform-facts.md:1133
**Claim:** `small` (2.1.8 Icon Sizes): the cell says (none): no 16 dp icon token; smallest in-component icon is button with-icon 18; v0_192 (cell) material-web cbd34a8: switch selected-icon-size 16px and unselected-icon-size 16px exist, so the cell's own source contradicts it; filled-tonal-button with-icon-icon-size 18px; Compose M3 baseline 1.4.0 (5113b82): SwitchTokens Selected/UnselectedIconSize 16.dp (:50, :73). ButtonDefaults.IconSize 18.dp (Button.kt:537). FilledTonalButtonTokens.IconSize 18 (:46); Compose M3 Expressive 1.5.0-beta01 (19991f2c): the size-based button API's ExtraSmall/Small icon = 20.dp (ButtonDefaults.ExtraSmallIconSize/SmallIconSize, Button.kt:1084-1087; ButtonXSmallTokens/ButtonSmallTokens IconSize 20); ButtonDefaults.IconSize stays 18 (:1081).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L76 , #L122 ; .../v0_192/_md-comp-filled-tonal-button.scss#L98 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/SwitchTokens.kt#L50 , #L73 ; .../Button.kt#L537 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt#L1081-L1087 Note: The 'no 16 dp icon token' claim is false at v0_192 as well: the switch thumb icon is 16. Whether a switch thumb icon counts as the 'small' context is a mapping question, so I propose no value; the prose should be corrected either way.
**Verdict:** CONFIRMED — The cell's claim that there is no 16 dp icon token is contradicted by the switch icon tokens in its own token set (they are in-thumb icons, but they are 16 dp component icon tokens). Confirmed value: mw v0_192 switch selected-icon-size 16px (:76), unselected-icon-size 16px (:122); filled-tonal-button with-icon-icon-size 18px (:98). Compose 1.4.0 SwitchTokens Selected/UnselectedIconSize 16.dp (:50,….
**Triage:** fix — The cell contradicts its own source: v0_192 has 16 dp switch icon tokens (_md-comp-switch.scss:76, :122).
**Status:** open

#### M2d-11 · bug · S2 · docs/platform-facts.md:1148
**Claim:** `title_bar_font.family` (2.2 Window): the cell says (none); nearest title-large -> brand Roboto; v0_192 (cell) v0_192: md.ref.typeface.brand = Roboto; Compose M3 baseline 1.4.0: TitleLargeFont = TypefaceTokens.Brand = FontFamily.SansSerif (generic family, no named typeface).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L17 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypeScaleTokens.kt#L84 ; .../tokens/TypefaceTokens.kt#L25-L26 Note: Token-level difference: Compose names no typeface (Brand and Plain are both FontFamily.SansSerif); which face that resolves to on a device was not checked. Applies to every Roboto family cell (1229, 1252, 1346 and the family inside composite font rows).
**Verdict:** CONFIRMED — Read as cited; Compose names only the generic sans-serif family, so what renders on a device is the system default (Roboto on stock Android), which a measurement would settle. Confirmed value: mw md.ref.typeface.brand = Roboto (_md-ref-typeface.scss:17); Compose 1.4.0 TitleLargeFont = TypefaceTokens.Brand (TypeScaleTokens.kt:84) = FontFamily.SansSerif (TypefaceTokens.kt:24).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-12 · bug · S2 · docs/platform-facts.md:1166
**Claim:** `font.color` (2.3 Button): the cell says on-secondary-container L #1d192b (latest #4a4458) D #e8def8; outlined: primary; v0_192 (cell) v0_192: tonal on-secondary-container; outlined label primary; Compose M3 baseline 1.4.0: tonal OnSecondaryContainer (= Secondary10 #1d192b light); outlined label OnSurfaceVariant (L #49454f, D #cac4d0); Compose M3 Expressive 1.5.0-beta01: tonal light OnSecondaryContainer = Secondary30 #4a4458 (expressiveLightColorScheme); dark unchanged #e8def8 (darkColorScheme).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L61 ; .../v0_192/_md-comp-outlined-button.scss#L50 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/FilledTonalButtonTokens.kt#L37 ; .../tokens/ColorLightTokens.kt#L36 ; .../tokens/OutlinedButtonTokens.kt#L35 ; .../Button.kt#L740 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ColorScheme.kt#L1166-L1179 Note: Tonal value same as the cell; the outlined sub-claim differs (Compose OutlinedButtonTokens is VERSION v0_11_0, the newer button spec).
**Verdict:** CONFIRMED — All values read; in Compose the outlined button label is on-surface-variant, not mw's primary. Confirmed value: mw tonal label on-secondary-container (:61), outlined label primary (:50). Compose 1.4.0 tonal OnSecondaryContainer (FilledTonalButtonTokens:37) = Secondary10 #1d192b light (ColorLightTokens:36) / Sec….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-13 · bug · S2 · docs/platform-facts.md:1167
**Claim:** `border.color` (2.3 Button): the cell says (none) tonal/filled; outlined outline L #79747e D #938f99; v0_192 (cell) v0_192: outlined outline-color = outline; Compose M3 baseline 1.4.0: tonal/filled border null; outlined OutlinedButtonTokens.OutlineColor = OutlineVariant (L #cac4d0, D #49454f).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-button.scss#L71 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/OutlinedButtonTokens.kt#L36 ; .../Button.kt#L898-L909 ; .../Button.kt#L284 Note: Tonal/filled (none) agrees; outlined colour differs.
**Verdict:** CONFIRMED — Read at both pins: material-web v0_192 uses outline (#79747e/#938f99) while Compose 1.4.0 OutlinedButton draws OutlineVariant, so the Compose value differs from the cell. Confirmed value: Compose 1.4.0: tonal/filled border = null (Button.kt:284, :118); OutlinedButton border = outlinedButtonBorder = OutlinedButtonTokens.OutlineColor = OutlineVariant (OutlinedButtonTokens.kt:36; Button.k….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-14 · bug · S2 · docs/platform-facts.md:1169
**Claim:** `min_width` (2.3 Button): the cell says (impl) material-web 64; v0_192 (cell) material-web cbd34a8: 64 (min-width calc(64px - leading - trailing) on the padded element); Compose M3 baseline 1.4.0: ButtonDefaults.MinWidth = 58.dp.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/button/internal/_shared.scss#L91-L94 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt#L527 Note: Same 58 in 1.5.0-beta01 (Button.kt:1053). No token in either.
**Verdict:** CONFIRMED — Both values read at the cited lines; both are outer-box minimums, so 58 vs 64 is a real difference. Confirmed value: Compose 1.4.0 ButtonDefaults.MinWidth = 58.dp (Button.kt:527), applied via defaultMinSize before .padding(contentPadding) so it is the outer width (Button.kt:144-147); 1.5.0-beta01 still 58 (Button.kt….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-15 · bug · S2 · docs/platform-facts.md:1172
**Claim:** `border.padding_vertical` (2.3 Button): the cell says (derived) 10 = (40 - 20)/2; mdc 6; v0_192 (cell) material-web cbd34a8: padding-block (40 - max(20,18))/2 = 10; Compose M3 baseline 1.4.0: ButtonVerticalPadding 8.dp (declared); a one-line label is centred in the 40 min height, so it sits 10 from the edge; Compose M3 Expressive 1.5.0-beta01: smallVerticalPadding 10.dp (8 under the opt-in precision-pointer flag).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/button/internal/_shared.scss#L31-L32 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt#L469 ; .../Button.kt#L143-L151 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt#L1652-L1653 Note: Differs only for wrapped labels (Compose 8+text+8). Expressive equals the cell.
**Verdict:** CONFIRMED — All cited values hold; note the 10 for 1.4.0 is arithmetic (centred 20 line in 40), and in 1.5.0-beta01 smallVerticalPadding is used only by the @OptIn Expressive shapes overloads (contentPaddingFor, Button.kt:244, :1578-1594) while the plain Button still defaults to ContentPadding with 8 (Button.kt:147, :915-928). Confirmed value: material-web (40 - max(20,18))/2 = 10 (_shared.scss:31-32, outlined-button container-height 40 :34, icon 18 :84); Compose 1.4.0 ContentPadding top/bottom = ButtonVerticalPadding 8.dp (Button.kt:469, :….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-16 · bug · S2 · docs/platform-facts.md:1193
**Claim:** `selection_background` (2.4 Text Input): the cell says (none) - no token; v0_192 (cell) v0_192: no token; Compose M3 baseline 1.4.0: (impl) MaterialTheme provides TextSelectionColors(backgroundColor = primary @ TextSelectionBackgroundOpacity 0.4, handleColor = primary).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt#L218-L229 ; .../MaterialTheme.kt#L98-L104 Note: No token, but Compose has an implementation constant the cell could cite as (impl). Selection text colour: Compose TextSelectionColors has none (1194 stays (none)).
**Verdict:** CONFIRMED — grep finds no 'selection' in the v0_192 outlined-text-field token file, and MaterialTheme.kt:223/:229 read primary.copy(alpha = TextSelectionBackgroundOpacity = 0.4f); it is a theme-wide default, not a text-field token, so the (impl) tag is right. Confirmed value: (impl) Compose 1.4.0 MaterialTheme provides LocalTextSelectionColors = TextSelectionColors(handleColor = primary, backgroundColor = primary @ 0.4) (MaterialTheme.kt:98-104, :218-229); material-web v0_….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-17 · bug · S2 · docs/platform-facts.md:1212
**Claim:** `indicator_width` (2.5 Checkbox/Radio): the cell says checkbox container-size 18; radio 20; v0_192 (cell) v0_192: checkbox 18, radio 20; Compose M3 baseline 1.4.0: checkbox drawn in requiredSize(CheckboxSize = 20.dp) with 2.dp padding; CheckboxTokens.ContainerHeight/Width 18 unused; radio IconSize 20; Compose M3 Expressive 1.5.0-beta01: still 20 by default; 18 (CheckboxTokens.ContainerSize) only when ComposeMaterial3Flags.isCheckboxStylingFixEnabled (default false; doc: false = 'older Material Design 2 styling').
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L32 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt#L444 ; .../Checkbox.kt#L736-L738 ; .../tokens/CheckboxTokens.kt#L25-L27 ; .../tokens/RadioButtonTokens.kt#L28 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt#L518-L523 ; .../Checkbox.kt#L1064-L1069 ; .../ComposeMaterial3Flags.kt#L40-L49 Note: Checkbox only: Compose renders a 20 dp box by default in both releases. Radio 20 same. Not tied to the Expressive theme; the flag is @ExperimentalMaterial3Api.
**Verdict:** CONFIRMED — The drawn box is the full 20 dp outer size (stroke drawn inside), not 18, and the flag doc says false means 'older Material Design 2 styling'; radio 20 matches the cell. Confirmed value: Compose 1.4.0 checkbox box = 20.dp outer (Canvas requiredSize(CheckboxSize=20), drawBox fills size.width; Checkbox.kt:444, :461-486, :737) with 2.dp outside padding (:303, :736), CheckboxTokens.Contai….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-18 · bug · S2 · docs/platform-facts.md:1228
**Claim:** `separator_color` (2.6 Menu): the cell says (latest) menu divider-color = surface-variant L #e7e0ec D #49454f; v0_192 (cell) latest 34.0.21: surface-variant; Compose M3 baseline 1.4.0: no menu divider token; a menu separator is HorizontalDivider: DividerTokens.Color = OutlineVariant (L #cac4d0, D #49454f), 1.dp; Compose M3 Expressive 1.5.0-beta01: MenuDefaults.HorizontalDividerPadding documents HorizontalDivider for menu groups (same OutlineVariant).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L76 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/DividerTokens.kt#L24-L25 ; .../Divider.kt#L113-L117 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MenuDefaults.kt#L176-L179 Note: Light differs (#cac4d0 vs #e7e0ec); dark equal.
**Verdict:** CONFIRMED — All cited values read as claimed; additionally the cited latest token itself is deprecated in favour of md.comp.divider.color = outline-variant, which agrees with Compose. Confirmed value: material-web latest menu divider-color = surface-variant (latest/_md-comp-menu.scss:76, marked @deprecated in favour of md.comp.divider.color = outline-variant, latest/_md-comp-divider.scss:23); Compo….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-19 · bug · S2 · docs/platform-facts.md:1229
**Claim:** `font.family` (2.6 Menu): the cell says Roboto (label-large); v0_192 (cell) v0_192: plain = Roboto; Compose M3 baseline 1.4.0: LabelLargeFont = TypefaceTokens.Plain = FontFamily.SansSerif.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L18 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypeScaleTokens.kt#L69 ; .../tokens/TypefaceTokens.kt#L26 Note: See 1148. Cell has no citation.
**Verdict:** CONFIRMED — Values hold; caveat: FontFamily.SansSerif is a generic family resolved by the platform, so the difference is 'named Roboto' vs 'platform sans', not a different named face (no Roboto literal in the Compose token files). Confirmed value: Compose 1.4.0 LabelLargeFont = TypefaceTokens.Plain = FontFamily.SansSerif (TypeScaleTokens.kt:69; TypefaceTokens.kt:26); material-web v0_192 plain = Roboto (_md-ref-typeface.scss:18).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-20 · bug · S2 · docs/platform-facts.md:1235
**Claim:** `border.padding_horizontal` (2.6 Menu): the cell says (impl) material-web 16; latest menus 16; v0_192 (cell) v0_192: list-item leading/trailing-space 16; Compose M3 baseline 1.4.0: DropdownMenuItemContentPadding horizontal = DropdownMenuItemHorizontalPadding 12.dp; Compose M3 Expressive 1.5.0-beta01: token ItemLeadingSpace 16; impl: selectable item content padding 12 inside a 4.dp horizontal item inset (text 16 from the menu edge, 12 from the item highlight).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L152 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Menu.kt#L255-L256 ; .../Menu.kt#L525 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MenuDefaults.kt#L869-L884 ; .../Menu.kt#L2381 ; .../tokens/SegmentedMenuTokens.kt#L135-L136 Note: Classic Compose DropdownMenuItem is 12, not 16.
**Verdict:** CONFIRMED — Read as claimed; the container adds no horizontal inset in 1.4.0, so text is 12 from the menu edge, and 4+12 = 16 from the edge in the Expressive selectable item (DropdownMenuGroupContentPadding horizontal 0). Confirmed value: Compose 1.4.0 DropdownMenuItemContentPadding horizontal = 12.dp (Menu.kt:255-256, :525) with the menu column padded only vertically (Menu.kt:410); 1.5.0-beta01 selectable items: content padding 12 (Me….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-21 · bug · S2 · docs/platform-facts.md:1241
**Claim:** `icon_text_gap` (2.6 Menu): the cell says (impl) material-web 16; latest menus between-space 12; v0_192 (cell) material-web cbd34a8: gap 16; Compose M3 baseline 1.4.0: text start padding DropdownMenuItemHorizontalPadding 12.dp when a leading icon exists; Compose M3 Expressive 1.5.0-beta01: impl spacing 8.dp (12 under precision-pointer flag); token ItemBetweenSpace 12.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/menu/internal/menuitem/_menu-item.scss#L39 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Menu.kt#L462-L468 ; .../Menu.kt#L525 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MenuDefaults.kt#L855 ; .../MenuDefaults.kt#L889-L890 ; .../tokens/SegmentedMenuTokens.kt#L108-L109 Note: Three different values: mw 16, Compose classic 12, Compose Expressive impl 8 (token 12).
**Verdict:** CONFIRMED — All cited values read at the pinned lines; the 1.5 selectable item uses MenuArrangement with the 8 dp spacing (Menu.kt:2025-2030, :2350-2356), while the plain DropdownMenuItem still pads 12 (Menu.kt:2188). Confirmed value: material-web md-menu-item gap 16px (_menu-item.scss:39); Compose 1.4.0 text start padding 12.dp after a leading icon (Menu.kt:462-468, :525); 1.5.0-beta01 itemHorizontalArrangement spacing 8.dp (12 un….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-22 · bug · S2 · docs/platform-facts.md:1252
**Claim:** `font.family` (2.7 Tooltip): the cell says Roboto (body-small); v0_192 (cell) v0_192: plain = Roboto; Compose M3 baseline 1.4.0: BodySmallFont = Plain = FontFamily.SansSerif.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L18 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypeScaleTokens.kt#L34 ; .../tokens/TypefaceTokens.kt#L26 Note: See 1148.
**Verdict:** CONFIRMED — Same as index 18: values hold, but SansSerif is a platform-resolved generic family rather than a different named face. Confirmed value: Compose 1.4.0 BodySmallFont = TypefaceTokens.Plain = FontFamily.SansSerif (TypeScaleTokens.kt:34; TypefaceTokens.kt:26); material-web v0_192 plain = Roboto (:18).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-23 · bug · S2 · docs/platform-facts.md:1277
**Claim:** `track_color` (2.8 Scrollbar): the cell says (none) - M3 has no scrollbar component; v0_192 (cell) v0_192 / latest: no scrollbar token file; Compose M3 baseline 1.4.0: none (no scrollbar in api/current.txt); Compose M3 baseline 1.5.0-beta01: (impl) Modifier.nonInteractiveScrollbar trackColor = Color.Transparent.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/tree/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/api/current.txt [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scrollbar.kt#L85-L89 Note: NEW: Compose Material3 1.5.0-beta01 has a public, non-experimental visual-only scrollbar (api/1.5.0-beta01.txt:2754-2773 NonInteractiveScrollbarDefaults, :3148 ScrollbarKt), not in 1.4.0; not tied to the Expressive theme. The cell's 'M3 has no scrollbar component' holds only for 1.4.0 and material-web. No token file (constants in Scrollbar.kt).
**Verdict:** CONFIRMED — Confirmed at both pins; 1.5.0-beta01 is a beta, so the new value applies only once 1.5 ships stable. Confirmed value: material-web cbd34a8 tokens/versions has no scrollbar file (294 token paths, none match 'scroll'); Compose 1.4.0 api/current.txt has no scrollbar; 1.5.0-beta01 Modifier.nonInteractiveScrollbar trackCo….
**Triage:** Step 13 (D6) — New in Compose 1.5.0-beta01 (non-interactive scrollbar); applies only if the maintainer pins 1.5.
**Status:** open

#### M2d-24 · bug · S2 · docs/platform-facts.md:1278
**Claim:** `thumb_color` (2.8 Scrollbar): the cell says (none); Compose M3 baseline 1.4.0: none; Compose M3 baseline 1.5.0-beta01: (impl) NonInteractiveScrollbarDefaults.thumbColor = colorScheme.outline @ ThumbOpacity 0.7.
**Evidence:** [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/api/current.txt [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scrollbar.kt#L122-L132 Note: outline = L #79747e, D #938f99 at alpha 0.7.
**Verdict:** CONFIRMED — Read as claimed at the pinned lines; like index 22, it is beta-only (1.5.0-beta01). Confirmed value: Compose 1.4.0: none; 1.5.0-beta01 NonInteractiveScrollbarDefaults.thumbColor = colorScheme.outline.copy(alpha = ThumbOpacity 0.7f) (Scrollbar.kt:122-132).
**Triage:** Step 13 (D6) — New in Compose 1.5.0-beta01 (non-interactive scrollbar); applies only if the maintainer pins 1.5.
**Status:** open

#### M2d-25 · bug · S2 · docs/platform-facts.md:1280
**Claim:** `groove_width` (2.8 Scrollbar): the cell says (none); Compose M3 baseline 1.5.0-beta01: (impl) Thickness 4.dp + CrossAxisTrackInset 0.dp each side = 4.
**Evidence:** [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scrollbar.kt#L135-L147 Note: (derived) from two impl constants; 1.4.0 none. MainAxisTrackInset 2.dp (:144) insets the track ends.
**Verdict:** CONFIRMED — Scrollbar.kt@19991f2c L135 Thickness 4.dp, L147 CrossAxisTrackInset 0.dp, and the track is drawn Size(thicknessPx, trackLength) (L539-544); commit is on androidx-compose-material3-release with libraryversions.toml COMPOSE_MATERIAL3 = 1.5.0-beta01, though the cross-axis inset is applied only at the edge side (L533-536), not each side, and the default track colour is transparent (L89). Confirmed value: (impl) Compose material3 1.5.0-beta01 Modifier.nonInteractiveScrollbar: NonInteractiveScrollbarDefaults.Thickness = 4.dp, CrossAxisTrackInset = 0.dp; groove 4 wide, transparent by default (trackColor….
**Triage:** Step 13 (D6) — New in Compose 1.5.0-beta01 (non-interactive scrollbar); applies only if the maintainer pins 1.5.
**Status:** open

#### M2d-26 · bug · S2 · docs/platform-facts.md:1281
**Claim:** `min_thumb_length` (2.8 Scrollbar): the cell says (none); Compose M3 baseline 1.5.0-beta01: (impl) ThumbMinLength 24.dp (max 0.9 of viewport).
**Evidence:** [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scrollbar.kt#L138-L141 Note: 1.4.0 none.
**Verdict:** CONFIRMED — Scrollbar.kt@19991f2c L138 ThumbMinLength = 24.dp and L141 ThumbMaxLengthFraction = 0.9f; the code applies the 0.9 to the track length (viewport minus 2 x MainAxisTrackInset 2.dp, L497-511), not to the raw viewport as the claim words it. Confirmed value: (impl) ThumbMinLength 24.dp; ThumbMaxLengthFraction 0.9.
**Triage:** Step 13 (D6) — New in Compose 1.5.0-beta01 (non-interactive scrollbar); applies only if the maintainer pins 1.5.
**Status:** open

#### M2d-27 · bug · S2 · docs/platform-facts.md:1282
**Claim:** `thumb_width` (2.8 Scrollbar): the cell says (none); Compose M3 baseline 1.5.0-beta01: (impl) Thickness 4.dp.
**Evidence:** [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scrollbar.kt#L135 Note: 1.4.0 none.
**Verdict:** CONFIRMED — Scrollbar.kt@19991f2c L135 Thickness = 4.dp; the thumb is drawn Size(thicknessPx, thumbLengthPx) with CornerRadius(thickness/2) (L524-527, L547-552). Confirmed value: (impl) Thickness 4.dp, thumb fully rounded (corner radius thickness/2).
**Triage:** Step 13 (D6) — New in Compose 1.5.0-beta01 (non-interactive scrollbar); applies only if the maintainer pins 1.5.
**Status:** open

#### M2d-28 · bug · S2 · docs/platform-facts.md:1283
**Claim:** `overlay_mode` (2.8 Scrollbar): the cell says (none); Compose M3 baseline 1.5.0-beta01: (impl) drawn over the content at the end edge; isFadeEnabled = true by default (fade 250 ms after 400 ms idle): overlay behaviour.
**Evidence:** [Compose M3 baseline] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scrollbar.kt#L50-L55 ; .../Scrollbar.kt#L94 ; .../Scrollbar.kt#L122-L128 Note: Mapping 'drawn over content and fades' to overlay_mode = true is an inference; 1.4.0 none.
**Verdict:** CONFIRMED — Scrollbar.kt@19991f2c L50-55 (end edge, shown while scrolling then fades), L94 isFadeEnabled = true, L125/L128 250 ms / 400 ms, and draw() calls drawContent() before painting the bar (L426-427). Confirmed value: (impl) overlay: drawn after drawContent() at the end/bottom edge, takes no layout space; isFadeEnabled = true, fade 250 ms after 400 ms idle.
**Triage:** Step 13 (D6) — New in Compose 1.5.0-beta01 (non-interactive scrollbar); applies only if the maintainer pins 1.5.
**Status:** open

#### M2d-29 · bug · S2 · docs/platform-facts.md:1290
**Claim:** `track_color` (2.9 Slider): the cell says surface-container-highest; latest secondary-container; v0_192 (cell) v0_192: surface-container-highest L #e6e0e9 D #36343b; Compose M3 baseline 1.4.0: InactiveTrackColor = SecondaryContainer (L #e8def8, D #4a4458).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L62 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/SliderTokens.kt#L55 ; .../Slider.kt#L1170 Note: Compose baseline already uses the newer slider spec (the cell's `latest` value).
**Verdict:** CONFIRMED — SliderTokens.kt@5113b82a L55 SecondaryContainer and Slider.kt L1170 inactiveTrackColor = fromToken(InactiveTrackColor); PaletteTokens L98/L92 give 232,222,248 / 74,68,88; v0_192 _md-comp-slider.scss L62-63 is surface-container-highest; commit libraryversions.toml = 1.4.0. Confirmed value: Compose M3 1.4.0: inactiveTrackColor = SliderTokens.InactiveTrackColor = SecondaryContainer (L #e8def8 Secondary90, D #4a4458 Secondary30); v0_192 surface-container-highest.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-30 · bug · S2 · docs/platform-facts.md:1292
**Claim:** `track_height` (2.9 Slider): the cell says 4; latest 16; v0_192 (cell) v0_192: 4; Compose M3 baseline 1.4.0: TrackHeight = SliderTokens.InactiveTrackHeight 16.dp (ActiveTrackHeight 16).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L38 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt#L2652 ; .../tokens/SliderTokens.kt#L32 ; .../tokens/SliderTokens.kt#L56 Note: Same in 1.5.0-beta01 (Slider.kt:3339-3343 per Step 1).
**Verdict:** CONFIRMED — Slider.kt@5113b82a L2652 TrackHeight = SliderTokens.InactiveTrackHeight, used by the default Track canvas height (L1324, L1599); SliderTokens L32/L56 both 16.0.dp; v0_192 scss L38/L64 4px. Confirmed value: Compose M3 1.4.0: TrackHeight = InactiveTrackHeight 16.dp (ActiveTrackHeight 16.dp); v0_192 4.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-31 · bug · S2 · docs/platform-facts.md:1293
**Claim:** `thumb_diameter` (2.9 Slider): the cell says 20 disc; latest 4x44 bar; v0_192 (cell) v0_192: 20 x 20; Compose M3 baseline 1.4.0: ThumbSize = HandleWidth 4.dp x HandleHeight 44.dp (a bar, CornerFull); 6.dp gap to the track each side.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L54-L57 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt#L2653-L2657 ; .../tokens/SliderTokens.kt#L48-L50 Note: A bar thumb with a track gap is not a diameter; the model's thumb_diameter cannot carry it (and the press-width animation to 2.dp, SliderTokens:62).
**Verdict:** CONFIRMED — Slider.kt@5113b82a L2653-2657 ThumbSize = DpSize(HandleWidth, HandleHeight), ThumbTrackGapSize = ActiveHandleLeadingSpace; SliderTokens L48-50 44.dp / CornerFull / 4.dp, L26 6.0.dp; v0_192 scss L54/L57 20px. Confirmed value: Compose M3 1.4.0: ThumbSize = 4.dp x 44.dp bar (HandleShape CornerFull), 6.dp thumb-track gap; v0_192 20 x 20.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-32 · bug · S2 · docs/platform-facts.md:1294
**Claim:** `tick_mark_length` (2.9 Slider): the cell says 2 (dots); v0_192 (cell) v0_192: with-tick-marks-container-size 2; Compose M3 baseline 1.4.0: TickSize = SliderTokens.StopIndicatorSize 4.dp.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L108 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt#L2162 ; .../tokens/SliderTokens.kt#L69
**Verdict:** CONFIRMED — Slider.kt@5113b82a L2162 TickSize = SliderTokens.StopIndicatorSize, default drawTick draws drawStopIndicator(size = TickSize) as a circle of diameter size (L1442-1443, L2155); SliderTokens L69 4.0.dp; v0_192 scss L108 2px. Confirmed value: Compose M3 1.4.0: TickSize = StopIndicatorSize 4.dp (circle diameter); v0_192 2.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-33 · bug · S2 · docs/platform-facts.md:1302
**Claim:** `track_color` (2.10 Progress): the cell says surface-container-highest; latest secondary-container; v0_192 (cell) v0_192: surface-container-highest; Compose M3 baseline 1.4.0: TrackColor = SecondaryContainer (L #e8def8, D #4a4458).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-linear-progress-indicator.scss#L39 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/ProgressIndicatorTokens.kt#L26 ; .../ProgressIndicator.kt#L822
**Verdict:** CONFIRMED — ProgressIndicatorTokens.kt@5113b82a L26 TrackColor = SecondaryContainer, ProgressIndicator.kt L821-822 linearTrackColor getter reads it; v0_192 _md-comp-linear-progress-indicator.scss L39 surface-container-highest. Confirmed value: Compose M3 1.4.0: linearTrackColor = ProgressIndicatorTokens.TrackColor = SecondaryContainer (L #e8def8, D #4a4458); v0_192 surface-container-highest.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-34 · bug · S2 · docs/platform-facts.md:1305
**Claim:** `border.corner_radius` (2.10 Progress): the cell says 0 corner-none; latest corner-full; v0_192 (cell) v0_192: corner-none; Compose M3 baseline 1.4.0: rounded: LinearStrokeCap = StrokeCap.Round; tokens ActiveShape/TrackShape CornerFull; plus 4.dp track gap and 4.dp stop dot.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-linear-progress-indicator.scss#L30 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt#L845 ; .../tokens/ProgressIndicatorTokens.kt#L23-L27 ; .../ProgressIndicator.kt#L855-L859 Note: Track gap and stop indicator are not modelled.
**Verdict:** CONFIRMED — ProgressIndicator.kt@5113b82a L845 StrokeCap.Round is the default strokeCap of LinearProgressIndicator (L144, L253), L855/L859 read LinearProgressIndicatorTokens StopSize / TrackActiveSpace = 4.0.dp (that file L29, L31); ProgressIndicatorTokens L23/L27 CornerFull; v0_192 scss L30/L41 corner-none. Confirmed value: Compose M3 1.4.0: fully rounded (LinearStrokeCap = StrokeCap.Round on a 4.dp line; ActiveShape/TrackShape CornerFull), 4.dp track gap, 4.dp stop dot; v0_192 corner-none.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-35 · bug · S2 · docs/platform-facts.md:1313
**Claim:** `active_background` (2.11 Tab Bar): the cell says (none); primary indicator 3 high, corners 3 3 0 0; v0_192 (cell) v0_192: indicator 3, shape 3 3 0 0; Compose M3 baseline 1.4.0: no fill; PrimaryIndicator 3.dp, ActiveIndicatorShape RoundedCornerShape(3.dp) on all four corners, width = content (matchContentSize = true).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L37-L40 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/PrimaryNavigationTabTokens.kt#L27-L28 ; .../TabRow.kt#L155-L159 ; .../TabRow.kt#L1054-L1066 Note: Only the indicator corner shape differs (all-round vs top-only).
**Verdict:** CONFIRMED — PrimaryNavigationTabTokens.kt@5113b82a L27-28 3.0.dp / RoundedCornerShape(3.0.dp); TabRow.kt L155-159 PrimaryTabRow default indicator uses tabIndicatorOffset(matchContentSize = true), width Dp.Unspecified, and PrimaryIndicator (L1054-1066) paints background(color, shape); v0_192 scss L38-40 confirms the cell's 3 3 0 0. Confirmed value: Compose M3 1.4.0: no fill; PrimaryIndicator 3.dp high, RoundedCornerShape(3.dp) on all four corners, width = content (matchContentSize = true); v0_192 3px, (3px 3px 0px 0px).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-36 · bug · S2 · docs/platform-facts.md:1315
**Claim:** `bar_background` (2.11 Tab Bar): the cell says surface; latest adds 1-high surface-variant divider; v0_192 (cell) latest 34.0.21: divider 1, surface-variant; Compose M3 baseline 1.4.0: container Surface; default divider = HorizontalDivider(): OutlineVariant (L #cac4d0, D #49454f), 1.dp.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-primary-navigation-tab.scss#L62 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt#L161 ; .../tokens/DividerTokens.kt#L24-L25 Note: Container same; divider colour differs from latest (light).
**Verdict:** CONFIRMED — TabRow.kt@5113b82a L161 divider default HorizontalDivider(), Divider.kt L51-52/L113/L117 default to DividerTokens, DividerTokens.kt L24-25 OutlineVariant / 1.0.dp, PaletteTokens NeutralVariant80/30 = 202,196,208 / 73,69,79; latest/sass pnt L34/L62 mark divider-height 1px and divider-color surface-variant @deprecated ('use standalone divider component'), which is what Compose does. Confirmed value: Compose M3 1.4.0: container Surface; default divider HorizontalDivider() = DividerTokens OutlineVariant (L #cac4d0, D #49454f), 1.dp; latest tokens: deprecated nested divider 1px surface-variant.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-37 · bug · S2 · docs/platform-facts.md:1317
**Claim:** `min_height` (2.11 Tab Bar): the cell says 48; icon+label 64; v0_192 (cell) v0_192: 48 / 64; Compose M3 baseline 1.4.0: SmallTabHeight = ContainerHeight 48.dp; icon+label LargeTabHeight 72.dp (token IconAndLabelTextContainerHeight 64 unused).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L53 ; .../_md-comp-primary-navigation-tab.scss#L67-L68 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tab.kt#L420-L421 ; .../tokens/PrimaryNavigationTabTokens.kt#L31 ; .../tokens/PrimaryNavigationTabTokens.kt#L37 Note: 48 same; only the icon+label sub-claim differs (64 vs 72).
**Verdict:** CONFIRMED — mw v0_192 primary-navigation-tab.scss L53 = 48px and L67-68 = 64px; Compose Tab.kt@5113b82 L420-421 SmallTabHeight = ContainerHeight (48, PrimaryNavigationTabTokens.kt L31) and LargeTabHeight = 72.dp used at L329 when both icon and text exist, while IconAndLabelTextContainerHeight (L37, 64) has no reference in the files read. Confirmed value: v0_192: 48 / icon+label 64; Compose 1.4.0: 48 / icon+label 72 (IconAndLabelTextContainerHeight 64 unused).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-38 · bug · S2 · docs/platform-facts.md:1325
**Claim:** `font.color` (2.11 Tab Bar): the cell says inactive on-surface-variant; v0_192 (cell) v0_192: on-surface-variant; Compose M3 baseline 1.4.0: Tab unselectedContentColor defaults to selectedContentColor (= LocalContentColor = Primary inside PrimaryTabRow); InactiveLabelTextColor OnSurfaceVariant token exists but no default uses it.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L97 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tab.kt#L98-L99 ; .../TabRow.kt#L154 ; .../tokens/PrimaryNavigationTabTokens.kt#L49 Note: Compose renders inactive tabs in primary unless the app passes unselectedContentColor; same default in 1.5.0-beta01 (Tab.kt:99).
**Verdict:** CONFIRMED — mw L97-98 = on-surface-variant; Compose Tab.kt L98-99 defaults unselectedContentColor to selectedContentColor = LocalContentColor, which PrimaryTabRow (TabRow.kt L154) sets to primaryContentColor = ActiveLabelTextColor = Primary (TabRow.kt L1013-1014), and the InactiveLabelTextColor token (PrimaryNavigationTabTokens.kt L49) is not referenced by Tab/TabRow. Confirmed value: v0_192 on-surface-variant; Compose 1.4.0 unselected = selectedContentColor = Primary (inside PrimaryTabRow).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-39 · bug · S2 · docs/platform-facts.md:1334
**Claim:** `font` (2.12 Sidebar): the cell says label-large Roboto 14/500; v0_192 (cell) v0_192: label-large 14/500; Compose M3 baseline 1.4.0: NavigationDrawerItem provides no text style, so the label inherits LocalTextStyle = typography.bodyLarge 16/400 (LabelTextFont = LabelLarge token is unused).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L116 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt#L1068-L1110 ; .../MaterialTheme.kt#L107 ; .../tokens/NavigationDrawerTokens.kt#L52 Note: Inference from absence of ProvideTextStyle in NavigationDrawer.kt (grep: none); a reference-app screenshot would confirm.
**Verdict:** CONFIRMED — mw navigation-drawer.scss L116 = label-large-size; Compose NavigationDrawerItem (NavigationDrawer.kt L1068-1110) provides only LocalContentColor and no text style, so the label inherits MaterialTheme's ProvideTextStyle(typography.bodyLarge) (MaterialTheme.kt L107; BodyLarge 16sp Regular, TypeScaleTokens.kt L26-28), and LabelTextFont (NavigationDrawerTokens.kt L52) is unused in NavigationDrawer.kt. Confirmed value: v0_192 label-large 14/500; Compose 1.4.0 bodyLarge 16/400 (inherited).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-40 · bug · S2 · docs/platform-facts.md:1340
**Claim:** `border.corner_radius` (2.12 Sidebar): the cell says container-shape corner-large-end 0 16 16 0; v0_192 (cell) v0_192: corner-large-end; Compose M3 baseline 1.4.0: standard (Permanent/Dismissible) drawerShape = RectangleShape (0); only Modal uses DrawerDefaults.shape = CornerLargeEnd.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L68 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt#L766 ; .../NavigationDrawer.kt#L681 ; .../NavigationDrawer.kt#L592 ; .../tokens/NavigationDrawerTokens.kt#L38 Note: The column maps sidebar to the standard drawer, which Compose draws square.
**Verdict:** CONFIRMED — mw L68 = corner-large-end; Compose NavigationDrawer.kt L766 (Permanent) and L681 (Dismissible) default drawerShape = RectangleShape, and only ModalDrawerSheet L592 uses DrawerDefaults.shape = ContainerShape = CornerLargeEnd (NavigationDrawerTokens.kt L38). Confirmed value: v0_192 corner-large-end; Compose 1.4.0 Permanent/Dismissible RectangleShape (0), Modal CornerLargeEnd.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-41 · bug · S2 · docs/platform-facts.md:1341
**Claim:** `border.shadow_enabled` (2.12 Sidebar): the cell says standard false level0; modal true level1; v0_192 (cell) v0_192: standard level0, modal level1; Compose M3 baseline 1.4.0: standard false (PermanentDrawerElevation Level0); modal also no shadow: ModalDrawerElevation = Level0, used as tonalElevation only (drawer Surface sets no shadowElevation).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L159-L163 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt#L999-L1002 ; .../NavigationDrawer.kt#L823-L826 Note: Standard value same; modal sub-claim differs (token ModalContainerElevation Level1 at NavigationDrawerTokens.kt:56 is unused).
**Verdict:** CONFIRMED — mw L159 modal level1, L163 standard level0; Compose DrawerDefaults.ModalDrawerElevation = ElevationTokens.Level0 (L999) is passed only as drawerTonalElevation, and the DrawerSheet Surface (L823-826) sets tonalElevation only, with no shadowElevation anywhere in NavigationDrawer.kt. Confirmed value: v0_192 standard level0 / modal level1; Compose 1.4.0 no shadow for either (Modal tonal Level0).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-42 · bug · S2 · docs/platform-facts.md:1346
**Claim:** `font.family` (2.13 Toolbar): the cell says Roboto (title-large brand); v0_192 (cell) v0_192: brand Roboto; Compose M3 baseline 1.4.0: FontFamily.SansSerif.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L17 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypefaceTokens.kt#L25 Note: See 1148.
**Verdict:** CONFIRMED — mw _md-ref-typeface.scss L17 brand = Roboto; Compose TypefaceTokens.kt L25 Brand = FontFamily.SansSerif (TitleLargeFont = Brand, TypeScaleTokens.kt L84). Note: on Android the generic sans-serif resolves to Roboto, so the difference shows only off Android. Confirmed value: v0_192 Roboto; Compose 1.4.0 FontFamily.SansSerif.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-43 · bug · S2 · docs/platform-facts.md:1391
**Claim:** `border.padding_vertical` (2.15 List): the cell says (impl) material-web 12; (latest) 10; v0_192 (cell) material-web cbd34a8: top/bottom-space 12; Compose M3 baseline 1.4.0: ListItemVerticalPadding 8.dp (one/two-line), 12 three-line; Compose M3 Expressive 1.5.0-beta01: ListItemDefaults.ContentPadding top/bottom = ListTokens.ItemTopSpace/ItemBottomSpace 10.dp (12 under precision-pointer flag).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-list-item.scss#L139-L140 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ListItem.kt#L743-L745 ; .../ListItem.kt#L766-L767 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ListItemDefaults.kt#L42-L65 ; .../tokens/ListTokens.kt#L325-L326 Note: With a 56 min height and a 24 line both 8/10/12 render a one-line item the same (16 from the edge); they differ for multi-line items.
**Verdict:** CONFIRMED — mw _md-comp-list-item.scss L139-140 = 12px; Compose 1.4.0 ListItem.kt L743/L745 = 8/12 chosen at L766-767; 1.5.0-beta01 ListItemDefaults.kt L42-65 uses ListTokens.ItemTopSpace 10 (ListTokens.kt L326-327) and ItemBottomSpace 10 (at L45-46, not L325-326 as cited) as the contentPadding default of the new non-deprecated ListItem overloads (ListItem.kt L260-271), while the deprecated old overload still uses 8. Confirmed value: mw 12; Compose 1.4.0 8 (three-line 12); Compose 1.5.0-beta01 10 (12 with precision pointer).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-44 · bug · S2 · docs/platform-facts.md:1486
**Claim:** `border.shadow_enabled` (2.22 Dialog): the cell says true, container-elevation level3; v0_192 (cell) material-web cbd34a8: level3 (true); Compose M3 baseline 1.4.0 (5113b82): false as rendered: DialogTokens.ContainerElevation = Level3 exists, but AlertDialogContent's Surface sets only tonalElevation (AlertDialogDefaults.TonalElevation = 0.dp) and no shadowElevation; the Android dialog window's own shadow is hidden (outline alpha 0,…; Compose M3 Expressive 1.5.0-beta01 (19991f2c): same as baseline: AlertDialogDefaults.TonalElevation = 0.dp, no shadowElevation in AlertDialog.kt.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L74 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/DialogTokens.kt#L30 ; AlertDialog.kt#L241 ; AlertDialog.kt#L310-L315 ; https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/ui/ui/src/androidMain/kotlin/androidx/compose/ui/window/AndroidDialog.android.kt#L506-L519 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AlertDialog.kt#L219 Note: Token agrees (level3); the Compose implementation casts no shadow. Decision for the maintainer which one the material-3 preset follows; a reference-app screenshot would show it.
**Verdict:** CONFIRMED — mw dialog.scss L74 = level3 and DialogTokens.kt L30 = Level3, but AlertDialogDefaults.TonalElevation = 0.dp (AlertDialog.kt L241; 1.5.0-beta01 L219), AlertDialogContent's Surface (L310-315; 1.5.0 L308-312) sets only tonalElevation, and AndroidDialog.android.kt L506-519 sets the window outline alpha to 0 so the window draws no shadow either; DialogTokens.ContainerElevation has no reference in AlertDialog.kt. Confirmed value: v0_192 level3 (true); Compose 1.4.0 and 1.5.0-beta01 AlertDialog: no shadow (tonal 0.dp, no shadowElevation).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-45 · bug · S2 · docs/platform-facts.md:1489
**Claim:** `min_height` (2.22 Dialog): the cell says 140; v0_192 (cell) material-web cbd34a8: min-height 140px; Compose M3 baseline 1.4.0 (5113b82): no minimum height: BasicAlertDialog constrains only width (sizeIn(minWidth, maxWidth)); no height constant in AlertDialog.kt.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L41 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AlertDialog.kt#L170 ; AlertDialog.kt#L468-L478 Note: No token in either; 140 is a material-web implementation choice.
**Verdict:** CONFIRMED — mw dialog/internal/_dialog.scss L41 min-height 140px; Compose AlertDialog.kt L170 applies sizeIn(minWidth = DialogMinWidth, maxWidth = DialogMaxWidth) only, and L468-478 define widths (280/560) and paddings but no height constant. Confirmed value: mw 140; Compose 1.4.0 no minimum height.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-46 · bug · S2 · docs/platform-facts.md:1490
**Claim:** `max_height` (2.22 Dialog): the cell says 560, min(560px, calc(100% - 48px)); v0_192 (cell) material-web cbd34a8: min(560px, calc(100% - 48px)); Compose M3 baseline 1.4.0 (5113b82): no maximum height constant (width-only sizeIn); bounded only by the window.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L39 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AlertDialog.kt#L170
**Verdict:** CONFIRMED — mw _dialog.scss L39 = min(560px, calc(100% - 48px)); Compose AlertDialog.kt L170 constrains only width, with no max-height constant in the file. Confirmed value: mw min(560px, calc(100% - 48px)); Compose 1.4.0 no max height (window-bounded).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-47 · bug · S2 · docs/platform-facts.md:1495
**Claim:** `title_font.family` (2.22 Dialog): the cell says Roboto (headline-small, brand); v0_192 (cell) material-web cbd34a8: brand = Roboto; Compose M3 baseline 1.4.0 (5113b82): HeadlineSmallFont = TypefaceTokens.Brand = FontFamily.SansSerif (the platform's generic sans-serif; Roboto on Android, not elsewhere).
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L17 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/TypeScaleTokens.kt#L64 ; tokens/TypefaceTokens.kt#L25 Note: Cell cites no line; the v0_192 brand typeface line is above. On Android both render Roboto.
**Verdict:** CONFIRMED — Compose TypeScaleTokens.kt L64 HeadlineSmallFont = TypefaceTokens.Brand, and TypefaceTokens.kt L25 Brand = FontFamily.SansSerif, against mw ref-typeface L17 Roboto. As with claim 41, this renders as Roboto on Android. Confirmed value: v0_192 Roboto; Compose 1.4.0 FontFamily.SansSerif.
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-48 · bug · S2 · docs/platform-facts.md:1533
**Claim:** `diameter` (2.23 Spinner): the cell says v0_192 size 48; latest size 40; v0_192 (cell) material-web cbd34a8: 48 (v0_192); 40 (latest); Compose M3 baseline 1.4.0 (5113b82): 40: CircularIndicatorDiameter = CircularProgressIndicatorTokens.Size; Compose M3 Expressive 1.5.0-beta01 (19991f2c): CircularWavyProgressIndicator (non-experimental) container 48 = CircularProgressIndicatorTokens.WaveSize; LoadingIndicator (still @ExperimentalMaterial3ExpressiveApi) container 48, active 38.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-circular-progress-indicator.scss#L39 ; https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-progress-indicator-circular.scss#L25 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/CircularProgressIndicatorTokens.kt#L27 ; ProgressIndicator.kt#L1042 ; ProgressIndicator.kt#L537 [Compose M3 Expressive] https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/WavyProgressIndicator.kt#L472 ; tokens/CircularProgressIndicatorTokens.kt#L42-L43 ; tokens/LoadingIndicatorTokens.kt Note: Plain CircularProgressIndicator is 40 in both Compose versions (beta01 ProgressIndicator.kt:1043-1044). The wave and the LoadingIndicator's shape morphing are not carryable by the model.
**Verdict:** CONFIRMED — mw v0_192 L39 = 48px and latest L25 = 40px; Compose CircularProgressIndicatorTokens.kt L27 Size = 40, used through CircularIndicatorDiameter (ProgressIndicator.kt L1042, L537); 1.5.0-beta01 WavyProgressIndicator.kt L472 CircularContainerSize = WaveSize 48 (tokens L42-43) on non-experimental CircularWavyProgressIndicator (L281), and LoadingIndicatorTokens gives container 48 / ActiveSize 38, with LoadingIndicator marked @ExperimentalMaterial3ExpressiveApi (LoadingIndicator.kt L96). Confirmed value: v0_192 48, latest 40; Compose 1.4.0 40; Compose 1.5.0-beta01 CircularProgressIndicator 40, CircularWavy 48, LoadingIndicator 48 (active 38, experimental).
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-49 · bug · S2 · docs/platform-facts.md:1551
**Claim:** `min_width` (2.24 ComboBox): the cell says 210 (md-outlined-select min-width); v0_192 (cell) material-web cbd34a8: 210; Compose M3 baseline 1.4.0 (5113b82): 280: the exposed dropdown's anchor is an OutlinedTextField, OutlinedTextFieldDefaults.MinWidth 280.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/select/internal/_shared.scss#L19 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt#L900 Note: Neither is a token. beta01 unchanged (TextFieldDefaults.kt:1006 at 19991f2c).
**Verdict:** CONFIRMED — Both values read as cited and libraryversions.toml at 5113b82 says COMPOSE_MATERIAL3 = 1.4.0; one caveat: ExposedDropdownMenuBox does not fix the anchor (its KDoc at ExposedDropdownMenu.kt:113 says 'a [TextField] (or [OutlinedTextField])', and the official ExposedDropdownMenuSample uses the filled TextField), but both have the same 280 default, which a caller can override with widthIn. Confirmed value: 280 (impl, Compose M3 1.4.0): OutlinedTextFieldDefaults.MinWidth = 280.dp (TextFieldDefaults.kt:900), applied by every OutlinedTextField overload via defaultMinSize (OutlinedTextField.kt:257-258, :422….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

#### M2d-50 · bug · S2 · docs/platform-facts.md:1554
**Claim:** `arrow_area_width` (2.24 ComboBox): the cell says (none): no arrow-area token; icon 24 + 12 trailing space; v0_192 (cell) material-web cbd34a8: no token; Compose M3 baseline 1.4.0 (5113b82): 48 (impl): the trailing slot is a minimumInteractiveComponentSize box (48) flush with the end edge.
**Evidence:** [v0_192 (cell)] https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L29-L270 [Compose M3 baseline] https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/OutlinedTextField.kt#L694-L699 ; OutlinedTextField.kt#L1394-L1397 ; internal/TextFieldImpl.kt#L518-L524 Note: Compose impl constant exists where the cell says none; the whole field is the click target in ExposedDropdownMenuBox, so 48 is the icon slot, not a separate hit area.
**Verdict:** CONFIRMED — All cited lines read as claimed, and the 48 depends on LocalMinimumInteractiveComponentSize keeping its 48dp default (TextFieldImpl.kt:518-524 handles an unspecified value as 0, which would shrink the slot to the 24 icon); the 12 between icon and edge matches material-web's 12 trailing space, so the two agree. Confirmed value: 48 (impl, Compose M3 1.4.0): the trailing slot is a Box with minimumInteractiveComponentSize() (OutlinedTextField.kt:694-699), placed at width - trailingPlaceable.width, i.e. flush with the end edge (….
**Triage:** Step 13 (D6) — Compose Material 3 baseline differs from the material-web v0_192 cell; D6 re-bases the column on Compose (Step 13), at the release the maintainer pins.
**Status:** open

### Step 4 — preset-validator

114 findings: 10 mismatch, 8 no-citation, 2 other, 94 depends-on-differs-row.

<details><summary>Validator summary</summary>

I audited native-theme/src/presets/material.toml in /home/tibi/Rust/native-theme-m2 against docs/platform-facts.md (Material column, lines 1019-1645), docs/property-registry.toml and docs/inheritance-rules.toml. I made no edits.

Status: FAIL.

**Inherited values.** Many values the preset does not set come from rules in inheritance-rules.toml:
- defaults.text_selection_background and input.selection_background come from defaults.selection_background (inheritance-rules.toml:54, :167).
- scrollbar.thumb_color comes from muted_color (:197).
- scrollbar.track_color falls back to background_color (:310-315).
- tab.active_background and tab.bar_background come from background_color (:212, :214).
- dialog.border.shadow_enabled comes from defaults.border.shadow_enabled (:94).
- Every unset font family (window title bar, menu, tooltip, toolbar, dialog title) comes from defaults.font.family "Roboto" (:114-137).

**Rows that differ today (Step 2, not yet verified).** Each of the 50 rows is mapped to the preset keys that depend on it, light and dark separately, each as a depends-on-differs-row finding. Rows already contradicted by their own facts cell are also flagged as mismatches:
- 1116: the preset's border.opacity 0.12, against an enabled outline that the cell says is opaque.
- 1133: icon_sizes.small 16, where the cell says no token.
- 1277-1283: scrollbar values the cell lists as (none). Three are stated by the preset (groove_width 12, min_thumb_length 30, overlay_mode false) and two are inherited (track_color, thumb_color). thumb_width 4 is also uncited, but it happens to equal Compose 1.5.

Six rows have no dependent preset key, because the preset does not set the field and no inheritance rule fills it: 1340 and 1341 (sidebar corner radius and shadow), 1391 (list.border.padding_vertical), 1554 (combo_box.arrow_area_width), and 1305 and 1313 only partly (the tab indicator shape and the progress bar's shape are not modelled).

**Other blocking findings.** These values have no source in the facts:
- Status colours: warning, success and info, with their text colours. Row 1082 says M3 has no warning role; rows 1084 and 1086 say none for success and info.
- link_color (row 1054 says none).
- mono_font Roboto Mono 14/400 (rows 1036-1038 say none).
- icon_sizes.large 32 (row 1134 says none).
- checkbox.label_gap 8 (row 1213 says app-defined).
- progress_bar.min_width 100 (row 1304 says app-defined).
- spinner.min_diameter 16 (row 1534 says none).
- scrollbar.thumb_hover_color and thumb_active_color (row 1279 says none).
- splitter.divider_width 4 and hover_color (row 1419: none in v0_192).
- The whole [*.expander] block (rows 1603-1612 say none). Its hover_background uses #1c1b1f / #e6e1e5, which are not this preset's on-surface (#1d1b20 / #e6e0e9).
- The whole [*.link] block (rows 1621-1626 say none).
- The [layout] gaps 6/6/10/18 (rows 1453-1456 say none). They are the shared block copied in tokyo-night, solarized, gruvbox, one-dark, nord, dracula and windows-11.

**Non-blocking.** About 60 platform-facts.md line citations in the preset comments are 1-7 lines stale, pointing at neighbouring rows. About 40 values match the facts but carry no citation comment.

Every key used exists in the registry (checked: radio_indicator_width_px :229, active_indicator_side :324, unchecked_thumb_diameter_px :435, min_diameter_px :471, arrow_area_width_px :485). The `_px` suffixes are consistent. The icon set "material" is a single set, not mixed, and not a Linux preset.

</details>

#### M2d-P01 · bug · S2 · native-theme/src/presets/material.toml:63
**Claim:** `light.defaults.border.opacity` (mismatch): preset 0.12, facts (none): enabled outlines are opaque; 0.12 is only the disabled outline opacity (docs/platform-facts.md:1116).
**Evidence:** Blocking: the preset uses the disabled-outline opacity as the global border opacity, which contradicts its own facts cell. Uncited.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — the rewrite takes enabled outlines as opaque (the cell).
**Status:** open

#### M2d-P02 · bug · S2 · native-theme/src/presets/material.toml:562
**Claim:** `dark.defaults.border.opacity` (mismatch): preset 0.12, facts (none): enabled outlines opaque (docs/platform-facts.md:1116).
**Evidence:** Dark twin of line 63.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — dark twin of M2d-P01.
**Status:** open

#### M2d-P03 · bug · S2 · native-theme/src/presets/material.toml:77
**Claim:** `light.defaults.icon_sizes.small_px` (mismatch): preset 16.0, facts (none): no 16 dp icon token; smallest in-component icon 18 (docs/platform-facts.md:1133).
**Evidence:** Blocking: uncited value against a (none) cell. Step 2 also reports that the cell is self-contradicted (material-web switch icon 16).
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — with M2d-10 (the cell gets the switch icon token 16).
**Status:** open

#### M2d-P04 · bug · S2 · native-theme/src/presets/material.toml:29
**Claim:** `light.defaults.warning_color / success_color / info_color (+ *_text_color, lines 29-34)` (mismatch): preset #7d5700/#ffffff, #386a20/#ffffff, #6750a4/#ffffff, facts (none): M3 has no warning, success or info role (rows 1082-1087) (docs/platform-facts.md:1082).
**Evidence:** Blocking: invented values with no citation. Remediation: leave them unset, or cite a designer palette.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — M3 has no warning, success or info role (platform-facts.md:1082-1087): the rewrite leaves them unset, or cites a source; never kept uncited.
**Status:** open

#### M2d-P05 · bug · S2 · native-theme/src/presets/material.toml:528
**Claim:** `dark.defaults.warning_color / success_color / info_color (+ *_text_color, lines 528-533)` (mismatch): preset #e6c18e/#3e2e00, #b1d18a/#1b3700, #d0bcff/#381e72, facts (none) (docs/platform-facts.md:1082).
**Evidence:** Dark twin; blocking.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — dark twin of M2d-P04.
**Status:** open

#### M2d-P06 · bug · S2 · native-theme/src/presets/material.toml:39
**Claim:** `light.defaults.link_color (dark :538)` (mismatch): preset #6750a4 / #d0bcff, facts (none): no link component or link colour role (docs/platform-facts.md:1054).
**Evidence:** Uncited; primary is used as a stand-in.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — no link role; unset or sourced.
**Status:** open

#### M2d-P07 · bug · S2 · native-theme/src/presets/material.toml:71
**Claim:** `light.defaults.mono_font (dark :570-572)` (mismatch): preset Roboto Mono, 14, 400, facts (none): M3 defines no monospace typeface or style (rows 1036-1038) (docs/platform-facts.md:1036).
**Evidence:** Blocking: uncited; Roboto Mono is not an M3 token.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — Roboto Mono is no M3 token; unset or sourced (docs/todo.md:421 records mono_font's role).
**Status:** open

#### M2d-P08 · bug · S2 · native-theme/src/presets/material.toml:78
**Claim:** `light.defaults.icon_sizes.large_px (dark :577)` (no-citation): preset 32.0, facts (none) (docs/platform-facts.md:1134).
**Evidence:** No source.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6).
**Status:** open

#### M2d-P09 · bug · S2 · native-theme/src/presets/material.toml:178
**Claim:** `light.checkbox.label_gap_px (dark :677)` (no-citation): preset 8.0, facts (app-defined): no token (docs/platform-facts.md:1213).
**Evidence:** No source.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6).
**Status:** open

#### M2d-P10 · bug · S2 · native-theme/src/presets/material.toml:247
**Claim:** `light.progress_bar.min_width_px (dark :746)` (no-citation): preset 100.0, facts (app-defined): no token (docs/platform-facts.md:1304).
**Evidence:** No source.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6); also already docs/todo.md:2276 (sizing fields stated where platform-facts says (none)); not new, fixed with that entry.
**Status:** open

#### M2d-P11 · bug · S2 · native-theme/src/presets/material.toml:444
**Claim:** `light.spinner.min_diameter_px (dark :943)` (no-citation): preset 16.0, facts (none) (docs/platform-facts.md:1534).
**Evidence:** No source.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6); also docs/todo.md:364.
**Status:** open

#### M2d-P12 · bug · S2 · native-theme/src/presets/material.toml:227
**Claim:** `light.scrollbar.thumb_hover_color / thumb_active_color (dark :726-727)` (no-citation): preset #79747e, #625b71 / #938f99, #aea9b4, facts (none): M3 has no scrollbar (docs/platform-facts.md:1279).
**Evidence:** Dark #aea9b4 is not any M3 role cited in the facts.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6).
**Status:** open

#### M2d-P13 · bug · S2 · native-theme/src/presets/material.toml:375
**Claim:** `light.splitter.divider_width_px / hover_color (dark :874-875)` (no-citation): preset 4.0, #6750a4 / 4.0, #d0bcff, facts v0_192 (none); latest drag-handle container 24, handle 4x48, colour outline (docs/platform-facts.md:1419).
**Evidence:** Uncited. hover_color primary has no facts row.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6).
**Status:** open

#### M2d-P14 · bug · S2 · native-theme/src/presets/material.toml:501
**Claim:** `light.expander.* (header_height_px, arrow_icon_size_px, hover_background, arrow_color; dark :1000-1003)` (mismatch): preset 48, 12, #1c1b1f14, #49454f / 48, 12, #e6e1e514, #cac4d0, facts (none): M3 has no expander component (rows 1603-1612) (docs/platform-facts.md:1605).
**Evidence:** Blocking: invented values. hover_background is based on #1c1b1f / #e6e1e5, which are not this preset's on-surface (#1d1b20 / #e6e0e9).
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — no expander component in M3.
**Status:** open

#### M2d-P15 · bug · S2 · native-theme/src/presets/material.toml:434
**Claim:** `light.link.* (underline_enabled, background_color, hover_background, hover_text_color, active_text_color, disabled_text_color; dark :933-938)` (mismatch): preset false, #00000000, #6750a418, #6750a4, #4f3d8a, #c4c7c5 / ..., #d0bcff18, #d0bcff, #e8deff, #49454f, facts (none): no link component (rows 1621-1626) (docs/platform-facts.md:1621).
**Evidence:** Blocking: #4f3d8a, #e8deff and #c4c7c5 trace to no cited role.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — no link component in M3.
**Status:** open

#### M2d-P16 · bug · S2 · native-theme/src/presets/material.toml:1011
**Claim:** `layout.widget_gap_px / container_margin_px / window_margin_px / section_gap_px` (mismatch): preset 6, 6, 10, 18, facts (none): M3 has no layout-manager defaults (rows 1453-1456) (docs/platform-facts.md:1453).
**Evidence:** Blocking: the shared community block (identical in tokyo-night, solarized, gruvbox, one-dark, nord, dracula and windows-11), not a Material source.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — the community layout block is no Material source.
**Status:** open

#### M2d-P17 · bug · S2 · native-theme/src/presets/material.toml:60
**Claim:** `light.defaults.border.corner_radius_px / corner_radius_lg_px (dark :559-560)` (no-citation): preset 12.0 / 16.0, facts (none): no global radius; medium step 12, large 16 (rows 1112-1113) (docs/platform-facts.md:1112).
**Evidence:** Traceable to the shape steps, but the preset chooses them without a comment.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — cite the shape tokens.
**Status:** open

#### M2d-P18 · bug · S2 · native-theme/src/presets/material.toml:508
**Claim:** `light.window.inactive_title_bar_background / text_color (dark :1007-1008)` (other): preset #fef7ff, #1d1b20, facts (none): M3 has no window-focus state (docs/platform-facts.md:1153).
**Evidence:** The comment cites :1146-1147 (stale; now 1153-1154). The value is a 'nearest' substitute, not a token.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6).
**Status:** open

#### M2d-P19 · bug · S2 · native-theme/src/presets/material.toml:23
**Claim:** `uncited but matching values` (no-citation): preset various, facts matches facts (docs/platform-facts.md:1052).
**Evidence:** Values with no citation comment that match facts rows: muted_color 23/522 (1052), danger_color/text 27-28/526-527 (1080-1081), defaults.border.line_width 62/561 (1114), icon_sizes.toolbar 76/575 (1132), text_scale section_heading/dialog_title/display 90-103/589-602 (1436-1438), button background/min_height/icon_text_gap 106-109/605-608 (1164, 1170, 1174), input min_height and focus_border_color 142/145 (1195, 1189), tab active_text_color 266 (1314), separator line_color 372/871 (1426), switch geometry 395-400/894-899 (1467-1470), dialog button_order/min/max width/button_gap 403-409 (1487-1494), segmented height/separator 472-473 (1569-1570).
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — add the citations.
**Status:** open

#### M2d-P20 · bug · S2 · native-theme/src/presets/material.toml:17
**Claim:** `stale platform-facts.md line citations in comments` (other): preset various, facts rows moved by 1-7 lines (docs/platform-facts.md:1046).
**Evidence:** Comments citing the wrong row. Light lines are listed; dark twins match: 17(:1044 to 1046), 36(:1053-1054 to 1055-1056), 40(:1102 to 1104), 42(:1058 to 1060), 51(:1028 to 1030), 53(:1103-1104 to 1105-1106), 126(:1163 to 1165), 135(:1165-1166 to 1167-1168), 140(:1185 to 1186), 143(:1187 to 1189), 148(:1191 to 1192), 153(:1186 to 1187-1188), 184(:1204 to 1206), 186/196(:1207 to 1209), 190(:1208 to 1210), 192(:1212 to 1214), 194(:1213 to 1215), 200(:1249 to 1251), 202(:1255 to 1259), 207(:1254 to 1256), 209(:1251 to 1253), 216(:1259 to 1260), 218(:1260-1261 to 1261-1262), 220(:1262 to 1263), 238(:1288 to 1290), 248(:1300 to 1302), 285(:1239 to 1241), 287(:1241 to 1243), 289(:1242 to 1244), 291(…
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** Step 14 (D6) — the rewrite re-cites every row; until then the stale line numbers stand.
**Status:** open

<details><summary>Preset keys that depend on a <i>differs</i> row</summary>

| Preset:line | Key | Preset value | Facts row |
|---|---|---|---|
| material.toml:67 | `light.defaults.font.size_px` | 14.0 | docs/platform-facts.md:1026 |
| material.toml:566 | `dark.defaults.font.size_px` | 14.0 | docs/platform-facts.md:1026 |
| material.toml:52 | `light.defaults.line_height` | 1.43 | docs/platform-facts.md:1030 |
| material.toml:551 | `dark.defaults.line_height` | 1.43 | docs/platform-facts.md:1030 |
| material.toml:59 | `light.defaults.border.color` | #79747e | docs/platform-facts.md:1051 |
| material.toml:558 | `dark.defaults.border.color` | #938f99 | docs/platform-facts.md:1051 |
| material.toml:25 | `light.defaults.shadow_color` | #0000004d | docs/platform-facts.md:1053 |
| material.toml:524 | `dark.defaults.shadow_color` | #0000004d | docs/platform-facts.md:1053 |
| material.toml:37 | `light.defaults.selection_background (resolves defaults.text_selection_background…` | #e8def8 (inherited) | docs/platform-facts.md:1058 |
| material.toml:536 | `dark.defaults.selection_background (resolves defaults.text_selection_background)` | #4a4458 (inherited) | docs/platform-facts.md:1058 |
| material.toml:37 | `light.input.selection_background (inherited)` | #e8def8 (via text_selection_background <- selection_backgrou… | docs/platform-facts.md:1193 |
| material.toml:536 | `dark.input.selection_background (inherited)` | #4a4458 (inherited) | docs/platform-facts.md:1193 |
| material.toml:41 | `light.defaults.focus_ring_color` | #625b71 | docs/platform-facts.md:1104 |
| material.toml:540 | `dark.defaults.focus_ring_color` | #ccc2dc | docs/platform-facts.md:1104 |
| material.toml:54 | `light.defaults.focus_ring_width_px` | 3.0 | docs/platform-facts.md:1105 |
| material.toml:553 | `dark.defaults.focus_ring_width_px` | 3.0 | docs/platform-facts.md:1105 |
| material.toml:55 | `light.defaults.focus_ring_offset_px` | 2.0 | docs/platform-facts.md:1106 |
| material.toml:554 | `dark.defaults.focus_ring_offset_px` | 2.0 | docs/platform-facts.md:1106 |
| material.toml:63 | `light.defaults.border.opacity` | 0.12 | docs/platform-facts.md:1116 |
| material.toml:562 | `dark.defaults.border.opacity` | 0.12 | docs/platform-facts.md:1116 |
| material.toml:77 | `light.defaults.icon_sizes.small_px` | 16.0 | docs/platform-facts.md:1133 |
| material.toml:576 | `dark.defaults.icon_sizes.small_px` | 16.0 | docs/platform-facts.md:1133 |
| material.toml:66 | `light.defaults.font.family (resolves window.title_bar_font.family)` | Roboto (inherited) | docs/platform-facts.md:1148 |
| material.toml:565 | `dark.defaults.font.family (resolves window.title_bar_font.family)` | Roboto (inherited) | docs/platform-facts.md:1148 |
| material.toml:125 | `light.button.font.color` | #1d192b | docs/platform-facts.md:1166 |
| material.toml:112 | `light.button.hover_text_color` | #1d192b | docs/platform-facts.md:1166 |
| material.toml:115 | `light.button.active_text_color` | #1d192b | docs/platform-facts.md:1166 |
| material.toml:624 | `dark.button.font.color` | #e8def8 | docs/platform-facts.md:1166 |
| material.toml:136 | `light.button.border.line_width_px (button.border.color inherits defaults.border.…` | 0.0; color resolves #79747e | docs/platform-facts.md:1167 |
| material.toml:635 | `dark.button.border.line_width_px` | 0.0; color resolves #938f99 | docs/platform-facts.md:1167 |
| material.toml:107 | `light.button.min_width_px` | 64.0 | docs/platform-facts.md:1169 |
| material.toml:606 | `dark.button.min_width_px` | 64.0 | docs/platform-facts.md:1169 |
| material.toml:132 | `light.button.border.padding_vertical_px` | 10.0 | docs/platform-facts.md:1172 |
| material.toml:631 | `dark.button.border.padding_vertical_px` | 10.0 | docs/platform-facts.md:1172 |
| material.toml:171 | `light.checkbox.indicator_width_px` | 18.0 | docs/platform-facts.md:1212 |
| material.toml:670 | `dark.checkbox.indicator_width_px` | 18.0 | docs/platform-facts.md:1212 |
| material.toml:296 | `light.menu.separator_color` | #e7e0ec | docs/platform-facts.md:1228 |
| material.toml:795 | `dark.menu.separator_color` | #49454f | docs/platform-facts.md:1228 |
| material.toml:66 | `light.defaults.font.family (resolves menu.font.family)` | Roboto (inherited) | docs/platform-facts.md:1229 |
| material.toml:565 | `dark.defaults.font.family (resolves menu.font.family)` | Roboto (inherited) | docs/platform-facts.md:1229 |
| material.toml:304 | `light.menu.border.padding_horizontal_px` | 16.0 | docs/platform-facts.md:1235 |
| material.toml:803 | `dark.menu.border.padding_horizontal_px` | 16.0 | docs/platform-facts.md:1235 |
| material.toml:286 | `light.menu.icon_text_gap_px` | 16.0 | docs/platform-facts.md:1241 |
| material.toml:785 | `dark.menu.icon_text_gap_px` | 16.0 | docs/platform-facts.md:1241 |
| material.toml:66 | `light.defaults.font.family (resolves tooltip.font.family)` | Roboto (inherited) | docs/platform-facts.md:1252 |
| material.toml:565 | `dark.defaults.font.family (resolves tooltip.font.family)` | Roboto (inherited) | docs/platform-facts.md:1252 |
| material.toml:18 | `light.defaults.background_color (resolves scrollbar.track_color)` | #fef7ff (inherited) | docs/platform-facts.md:1277 |
| material.toml:517 | `dark.defaults.background_color (resolves scrollbar.track_color)` | #141218 (inherited) | docs/platform-facts.md:1277 |
| material.toml:23 | `light.defaults.muted_color (resolves scrollbar.thumb_color)` | #49454f (inherited) | docs/platform-facts.md:1278 |
| material.toml:522 | `dark.defaults.muted_color (resolves scrollbar.thumb_color)` | #cac4d0 (inherited) | docs/platform-facts.md:1278 |
| material.toml:223 | `light.scrollbar.groove_width_px` | 12.0 | docs/platform-facts.md:1280 |
| material.toml:722 | `dark.scrollbar.groove_width_px` | 12.0 | docs/platform-facts.md:1280 |
| material.toml:226 | `light.scrollbar.min_thumb_length_px` | 30.0 | docs/platform-facts.md:1281 |
| material.toml:725 | `dark.scrollbar.min_thumb_length_px` | 30.0 | docs/platform-facts.md:1281 |
| material.toml:224 | `light.scrollbar.thumb_width_px` | 4.0 | docs/platform-facts.md:1282 |
| material.toml:723 | `dark.scrollbar.thumb_width_px` | 4.0 | docs/platform-facts.md:1282 |
| material.toml:225 | `light.scrollbar.overlay_mode` | false | docs/platform-facts.md:1283 |
| material.toml:724 | `dark.scrollbar.overlay_mode` | false | docs/platform-facts.md:1283 |
| material.toml:239 | `light.slider.track_color` | #e6e0e9 | docs/platform-facts.md:1290 |
| material.toml:738 | `dark.slider.track_color` | #36343b | docs/platform-facts.md:1290 |
| material.toml:231 | `light.slider.track_height_px` | 4.0 | docs/platform-facts.md:1292 |
| material.toml:730 | `dark.slider.track_height_px` | 4.0 | docs/platform-facts.md:1292 |
| material.toml:232 | `light.slider.thumb_diameter_px` | 20.0 | docs/platform-facts.md:1293 |
| material.toml:731 | `dark.slider.thumb_diameter_px` | 20.0 | docs/platform-facts.md:1293 |
| material.toml:234 | `light.slider.tick_mark_length_px` | 2.0 | docs/platform-facts.md:1294 |
| material.toml:733 | `dark.slider.tick_mark_length_px` | 2.0 | docs/platform-facts.md:1294 |
| material.toml:249 | `light.progress_bar.track_color` | #e6e0e9 | docs/platform-facts.md:1302 |
| material.toml:748 | `dark.progress_bar.track_color` | #36343b | docs/platform-facts.md:1302 |
| material.toml:253 | `light.progress_bar.border.corner_radius_px` | 0.0 | docs/platform-facts.md:1305 |
| material.toml:752 | `dark.progress_bar.border.corner_radius_px` | 0.0 | docs/platform-facts.md:1305 |
| material.toml:18 | `light.defaults.background_color (resolves tab.active_background and tab.bar_back…` | #fef7ff (inherited) | docs/platform-facts.md:1313 |
| material.toml:269 | `light.tab.active_indicator_width_px` | 3.0 | docs/platform-facts.md:1313 |
| material.toml:18 | `light.defaults.background_color (resolves tab.bar_background)` | #fef7ff (inherited) | docs/platform-facts.md:1315 |
| material.toml:517 | `dark.defaults.background_color (resolves tab.active_background and tab.bar_backg…` | #141218 (inherited) | docs/platform-facts.md:1315 |
| material.toml:261 | `light.tab.min_height_px` | 48.0 | docs/platform-facts.md:1317 |
| material.toml:760 | `dark.tab.min_height_px` | 48.0 | docs/platform-facts.md:1317 |
| material.toml:274 | `light.tab.font.color` | #49454f | docs/platform-facts.md:1325 |
| material.toml:773 | `dark.tab.font.color` | #cac4d0 | docs/platform-facts.md:1325 |
| material.toml:353 | `light.sidebar.font.weight` | 500 (size inherits 14) | docs/platform-facts.md:1334 |
| material.toml:852 | `dark.sidebar.font.weight` | 500 | docs/platform-facts.md:1334 |
| material.toml:66 | `light.defaults.font.family (resolves toolbar.font.family)` | Roboto (inherited) | docs/platform-facts.md:1346 |
| material.toml:565 | `dark.defaults.font.family (resolves toolbar.font.family)` | Roboto (inherited) | docs/platform-facts.md:1346 |
| material.toml:64 | `light.defaults.border.shadow_enabled (resolves dialog.border.shadow_enabled)` | true (inherited) | docs/platform-facts.md:1486 |
| material.toml:563 | `dark.defaults.border.shadow_enabled (resolves dialog.border.shadow_enabled)` | true (inherited) | docs/platform-facts.md:1486 |
| material.toml:406 | `light.dialog.min_height_px` | 140.0 | docs/platform-facts.md:1489 |
| material.toml:905 | `dark.dialog.min_height_px` | 140.0 | docs/platform-facts.md:1489 |
| material.toml:408 | `light.dialog.max_height_px` | 560.0 | docs/platform-facts.md:1490 |
| material.toml:907 | `dark.dialog.max_height_px` | 560.0 | docs/platform-facts.md:1490 |
| material.toml:66 | `light.defaults.font.family (resolves dialog.title_font.family)` | Roboto (inherited) | docs/platform-facts.md:1495 |
| material.toml:565 | `dark.defaults.font.family (resolves dialog.title_font.family)` | Roboto (inherited) | docs/platform-facts.md:1495 |
| material.toml:443 | `light.spinner.diameter_px` | 48.0 | docs/platform-facts.md:1533 |
| material.toml:942 | `dark.spinner.diameter_px` | 48.0 | docs/platform-facts.md:1533 |
| material.toml:452 | `light.combo_box.min_width_px` | 210.0 | docs/platform-facts.md:1551 |
| material.toml:951 | `dark.combo_box.min_width_px` | 210.0 | docs/platform-facts.md:1551 |

</details>

### Unverifiable rows

No primary source found: the row keeps or gets a ❓ in Step 7; no value is proposed. *Probe* = a probe or the reference app could measure it.

<details><summary>0 unverifiable rows</summary>

| Line | Property | Cell | Probe | Note |
|---|---|---|---|---|

</details>

### Same rows

Rows whose cell holds at the versions checked; the first citation is given.

<details><summary>232 same rows</summary>

| Line | Property | Values | Source |
|---|---|---|---|
| 1025 | `family` | v0_192 (cell) material-web cbd34a8: 'plain': Roboto; Compose M3 baseline 1.4.0 (5113b82): TypefaceTokens.Plain = FontFamily.SansSerif (generic); body styles use… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L18 |
| 1027 | `weight` | v0_192 (cell) material-web cbd34a8: weight-regular 400; Compose M3 baseline 1.4.0 (5113b82): BodyMediumWeight and BodyLargeWeight = TypefaceTokens.WeightRegular =… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L58 ; .../v0_192… |
| 1028 | `style` | v0_192 (cell) material-web cbd34a8: no italic or font-style in v0_192 or latest _md-sys-typescale.scss (gi…; Compose M3 baseline 1.4.0 (5113b82): TypographyTokens styles set no fontStyle (0 hits for fontStyle/Italic)… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss |
| 1029 | `color` | v0_192 (cell) material-web cbd34a8: on-surface = neutral10 #1d1b20 / neutral90 #e6e0e9; Compose M3 baseline 1.4.0 (5113b82): Surface's contentColor = contentColorFor(surface) = onSurface; ColorLi… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L43 (dark), #L102 (l… |
| 1036 | `family` | v0_192 (cell) material-web cbd34a8: brand, plain, weight-* only; Compose M3 baseline 1.4.0 (5113b82): TypefaceTokens has only Brand, Plain and Weight* | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-ref-typeface.scss#L17-L21 |
| 1037 | `size` | v0_192 (cell) material-web cbd34a8: no monospace typescale entry; Compose M3 baseline 1.4.0 (5113b82): every TypeScaleTokens *Font = TypefaceTokens.Brand or Plain; no monosp… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss |
| 1046 | `background_color` | v0_192 (cell) material-web cbd34a8: surface and background = neutral98 #fef7ff / neutral6 #141218; Compose M3 baseline 1.4.0 (5113b82): Background = Surface = Neutral98 rgb(254,247,255) #fef7ff (light), Neu… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L84 , #L120 , #L25 ,… |
| 1047 | `text_color` | v0_192 (cell) material-web cbd34a8: #1d1b20 / #e6e0e9; Compose M3 baseline 1.4.0 (5113b82): OnSurface = Neutral10 #1d1b20 / Neutral90 #e6e0e9 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L102 |
| 1048 | `accent_color` | v0_192 (cell) material-web cbd34a8: primary40 #6750a4 / primary80 #d0bcff; Compose M3 baseline 1.4.0 (5113b82): Primary = Primary40 rgb(103,80,164) #6750a4 / Primary80 rgb(208,188,25… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L110 , #L51 |
| 1049 | `accent_text_color` | v0_192 (cell) material-web cbd34a8: primary100 #fff / primary20 #381e72; Compose M3 baseline 1.4.0 (5113b82): OnPrimary = Primary100 #ffffff / Primary20 rgb(56,30,114) #381e72 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L93 , #L34 |
| 1050 | `surface_color` | v0_192 (cell) material-web cbd34a8: neutral98 / neutral6; surface-container ladder at :63-67, :122-126; Compose M3 baseline 1.4.0 (5113b82): Surface Neutral98 / Neutral6; the same SurfaceContainer* palette steps… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L19-L135 |
| 1052 | `muted_color` | v0_192 (cell) material-web cbd34a8: neutral-variant30 #49454f / neutral-variant80 #cac4d0; Compose M3 baseline 1.4.0 (5113b82): OnSurfaceVariant = NeutralVariant30 rgb(73,69,79) / NeutralVariant80 r… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L103 , #L44 |
| 1054 | `link_color` | v0_192 (cell) material-web cbd34a8: no link role in sys-color; Compose M3 baseline 1.4.0 (5113b82): no link role among the ColorScheme properties; the KDoc recommends sec… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L19-L135 |
| 1055 | `selection_background` | v0_192 (cell) material-web cbd34a8: latest list :258 and v0_192 menu :35-36 = secondary-container (seconda…; Compose M3 baseline 1.4.0 (5113b82): SecondaryContainer = Secondary90 #e8def8 / Secondary30 #4a4458. In 1.4…; Compose M3 baseline 1.5.0-beta01 (19991f…: selectable ListItem (public API) uses ListTokens.ItemSelectedContainer… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-list.scss#L258 ; .../v0_1… |
| 1056 | `selection_text_color` | v0_192 (cell) material-web cbd34a8: on-secondary-container = secondary10 #1d192b / secondary90 #e8def8; la…; Compose M3 baseline 1.4.0 (5113b82): OnSecondaryContainer = Secondary10 rgb(29,25,43) #1d192b / Secondary90…; Compose M3 Expressive 1.5.0-beta01 (19991f…: light: onSecondaryContainer = PaletteTokens.Secondary30 #4a4458. Dark… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L37 ; .../v0_192/_md… |
| 1059 | `text_selection_color` | Compose M3 baseline 1.4.0 (5113b82): TextSelectionColors carries only handleColor and backgroundColor; sele… | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1060 | `disabled_text_color` | v0_192 (cell) material-web cbd34a8: filled-button disabled-label-text on-surface @ 0.38; Compose M3 baseline 1.4.0 (5113b82): Filled tonal button (the table's button): OnSurface @ 0.38 (FilledTona… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-button.scss#L45-L46 |
| 1080 | `danger_color` | v0_192 (cell) material-web cbd34a8: error40 #b3261e / error80 #f2b8b5; Compose M3 baseline 1.4.0 (5113b82): Error = Error40 rgb(179,38,30) / Error80 rgb(242,184,181) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L85 , #L26 |
| 1081 | `danger_text_color` | v0_192 (cell) material-web cbd34a8: error100 #fff / error20 #601410; Compose M3 baseline 1.4.0 (5113b82): OnError = Error100 #ffffff / Error20 rgb(96,20,16) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-color.scss#L91 , #L32 |
| 1082 | `warning_color` | Compose M3 baseline 1.4.0 (5113b82): no warning role in ColorScheme (48 roles, :148-197) or ColorSchemeKeyT… | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1084 | `success_color` | Compose M3 baseline 1.4.0 (5113b82): no success role | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1086 | `info_color` | Compose M3 baseline 1.4.0 (5113b82): no info role | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1112 | `border.corner_radius` | v0_192 (cell) material-web cbd34a8: extra-small 4, medium 12, extra-large 28, full 9999; Compose M3 baseline 1.4.0 (5113b82): CornerExtraSmall 4, Medium 12, ExtraLarge 28, Full = CircleShape. Outl… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-shape.scss#L17-L31 |
| 1113 | `border.corner_radius_lg` | v0_192 (cell) material-web cbd34a8: corner-large 16, extra-large 28; Compose M3 baseline 1.4.0 (5113b82): CornerLarge 16, CornerExtraLarge 28 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-shape.scss#L17 , #L24 |
| 1114 | `border.line_width` | v0_192 (cell) material-web cbd34a8: outlined button 1px (:72), outlined text field 1px (:146), outlined ca…; Compose M3 baseline 1.4.0 (5113b82): ButtonSmallTokens.OutlinedOutlineWidth 1 (:30); OutlinedTextFieldDefau… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-button.scss#L72 |
| 1115 | `disabled_opacity` | v0_192 (cell) material-web cbd34a8: filled-button disabled container on-surface 0.12, label on-surface 0.3…; Compose M3 baseline 1.4.0 (5113b82): dims by colour; filled tonal button container OnSurface 0.12, label On… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-button.scss#L42-L46 |
| 1117 | `border.shadow_enabled` | v0_192 (cell) material-web cbd34a8: level0..5 = 0,1,3,6,8,12; Compose M3 baseline 1.4.0 (5113b82): ElevationTokens Level0..5 = 0,1,3,6,8,12 dp; Surface casts a shadow on… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-elevation.scss#L17-L22 |
| 1125 | `high_contrast` | v0_192 (cell) material-web cbd34a8: latest has _md-sys-color__high-contrast.scss, __medium-contrast.scss a…; Compose M3 baseline 1.4.0 (5113b82): no contrast flag and no contrast scheme in material3 commonMain (0 hit… | https://github.com/material-components/material-web/tree/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass |
| 1132 | `toolbar` | v0_192 (cell) material-web cbd34a8: 24px; Compose M3 baseline 1.4.0 (5113b82): 24: AppBarTokens.IconSize 24.dp, which AppBar.kt does not use. App-bar… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L59 , #… |
| 1135 | `dialog` | v0_192 (cell) material-web cbd34a8: 24px; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.IconSize 24.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L122 |
| 1136 | `panel` | v0_192 (cell) material-web cbd34a8: 24px; Compose M3 baseline 1.4.0 (5113b82): NavigationDrawerTokens.IconSize 24.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L94 |
| 1147 | `title_bar_background` | v0_192 (cell) v0_192: container-color = surface; Compose M3 baseline 1.4.0: AppBarTokens.ContainerColor = Surface (TopAppBarColors.containerColor) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L34 |
| 1149 | `title_bar_font.size` | v0_192 (cell) v0_192: 22 (1.375rem); Compose M3 baseline 1.4.0: AppBarSmallTokens.TitleFont = TitleLarge, TitleLargeSize 22.sp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L248 |
| 1150 | `title_bar_font.weight` | v0_192 (cell) v0_192: weight-regular 400; Compose M3 baseline 1.4.0: TitleLargeWeight = WeightRegular = FontWeight.Normal (400) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L57 |
| 1152 | `title_bar_font.color` | v0_192 (cell) v0_192: on-surface; Compose M3 baseline 1.4.0: AppBarTokens.TitleColor = OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L38 |
| 1164 | `background_color` | v0_192 (cell) v0_192: secondary-container; Compose M3 baseline 1.4.0: FilledTonalButtonTokens.ContainerColor = SecondaryContainer | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L37 |
| 1165 | `font` | v0_192 (cell) v0_192: label-large 14 / 500 / 20; Compose M3 baseline 1.4.0: typography.labelLarge: 14.sp / Medium / line 20.sp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L62-L… |
| 1168 | `border.line_width` | v0_192 (cell) v0_192: outlined outline-width 1; Compose M3 baseline 1.4.0: tonal border null; outlined ButtonSmallTokens.OutlinedOutlineWidth 1.d… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-button.scss#L72 |
| 1170 | `min_height` | v0_192 (cell) v0_192: 40; Compose M3 baseline 1.4.0: ButtonDefaults.MinHeight = ButtonSmallTokens.ContainerHeight 40.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L39 |
| 1171 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: leading/trailing 24, with-icon 16, text button 12; Compose M3 baseline 1.4.0: ContentPadding start/end = BaselineButtonTokens.LeadingSpace/TrailingS…; Compose M3 Expressive 1.5.0-beta01: Expressive overload Button/FilledTonalButton(shapes=...) uses contentP… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-filled-tonal-button.scss#L96-L105 ; .../tokens… |
| 1173 | `border.corner_radius` | v0_192 (cell) v0_192: corner-full; Compose M3 baseline 1.4.0: filledTonalShape = ButtonSmallTokens.ContainerShapeRound = CornerFull; Compose M3 Expressive 1.5.0-beta01: rest CornerFull; pressed morphs to ButtonSmallTokens.PressedContainerS… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L41 |
| 1174 | `icon_text_gap` | v0_192 (cell) material-web cbd34a8: 8; Compose M3 baseline 1.4.0: ButtonDefaults.IconSpacing = ButtonSmallTokens.IconLabelSpace 8.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/button/internal/_shared.scss#L23 |
| 1175 | `primary_background` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: FilledButtonTokens.ContainerColor = Primary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-button.scss#L37 |
| 1176 | `primary_text_color` | v0_192 (cell) v0_192: on-primary; Compose M3 baseline 1.4.0: FilledButtonTokens.LabelTextColor = OnPrimary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-button.scss#L57 |
| 1177 | `disabled_opacity` | v0_192 (cell) v0_192: tonal container on-surface 0.12, label on-surface 0.38; Compose M3 baseline 1.4.0: tonal: OnSurface 0.12 / OnSurface 0.38 (same) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L42-L… |
| 1178 | `border.shadow_enabled` | v0_192 (cell) v0_192: level0, hover level1; Compose M3 baseline 1.4.0: ContainerElevation Level0, HoverContainerElevation Level1 (shadowEleva… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L38 |
| 1179 | `hover_background` | v0_192 (cell) v0_192: on-secondary-container @ hover-state-layer-opacity 0.08; Compose M3 baseline 1.4.0: ripple in content colour (OnSecondaryContainer) at StateTokens.HoverSt…; Compose M3 Expressive 1.5.0-beta01: same mechanism; light layer colour Secondary30 #4a4458 (the cell's `la… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L57-L… |
| 1180 | `hover_text_color` | v0_192 (cell) v0_192: on-secondary-container; Compose M3 baseline 1.4.0: HoverLabelTextColor = OnSecondaryContainer (ButtonColors has no hover… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-tonal-button.scss#L55 |
| 1181 | `checked_background` | v0_192 (cell) latest 34.0.21: secondary; Compose M3 baseline 1.4.0: none: no toggle button in 1.4.0 (ToggleButton.kt absent); Compose M3 Expressive 1.5.0-beta01: FilledTonalToggleButton checkedContainerColor = TonalButtonTokens.Sele… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-button-tonal.scss#L79 |
| 1182 | `checked_text_color` | v0_192 (cell) latest 34.0.21: on-secondary; Compose M3 Expressive 1.5.0-beta01: TonalButtonTokens.SelectedLabelTextColor = OnSecondary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-button-tonal.scss#L95 |
| 1186 | `background_color` | v0_192 (cell) v0_192: no container colour (filled: surface-container-highest); Compose M3 baseline 1.4.0: OutlinedTextField focused/unfocusedContainerColor = Color.Transparent | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-text-field.scss#L38 |
| 1187 | `font` | v0_192 (cell) v0_192: body-large; Compose M3 baseline 1.4.0: InputFont BodyLarge; textStyle = LocalTextStyle = typography.bodyLarge… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L94-L… |
| 1188 | `font.color` | v0_192 (cell) v0_192: on-surface; Compose M3 baseline 1.4.0: InputColor = OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L93 |
| 1189 | `border.color` | v0_192 (cell) v0_192: outline / hover on-surface / focus primary; Compose M3 baseline 1.4.0: OutlineColor = Outline; FocusOutlineColor = Primary; HoverOutlineColor… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L145 |
| 1190 | `border.line_width` | v0_192 (cell) v0_192: 1 / focus 2; Compose M3 baseline 1.4.0: UnfocusedBorderThickness 1.dp / FocusedBorderThickness 2.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L146 |
| 1191 | `placeholder_color` | v0_192 (cell) v0_192: on-surface-variant; Compose M3 baseline 1.4.0: InputPlaceholderColor = OnSurfaceVariant | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L97 |
| 1192 | `caret_color` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: CaretColor = Primary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L31 |
| 1195 | `min_height` | v0_192 (cell) latest 34.0.21: container-height 56; Compose M3 baseline 1.4.0: OutlinedTextFieldDefaults.MinHeight 56.dp; OutlinedTextFieldTokens.Con… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-outlined-text-field.scss#… |
| 1196 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: leading/trailing-space 16 from field edge, outline overlaid; Compose M3 baseline 1.4.0: contentPadding start/end = TextFieldPadding 16.dp, documented as dista… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-outlined-text-field.scss#L139-L140 |
| 1197 | `border.padding_vertical` | v0_192 (cell) material-web cbd34a8: top/bottom-space 16; Compose M3 baseline 1.4.0: contentPadding top/bottom = TextFieldPadding 16.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-outlined-text-field.scss#L141-L142 |
| 1198 | `border.corner_radius` | v0_192 (cell) v0_192: corner-extra-small; Compose M3 baseline 1.4.0: ContainerShape = CornerExtraSmall | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L32 |
| 1199 | `border.shadow_enabled` | v0_192 (cell) v0_192: no elevation token; Compose M3 baseline 1.4.0: OutlinedTextFieldTokens has no elevation token | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss |
| 1200 | `disabled_opacity` | v0_192 (cell) v0_192: outline on-surface 0.12, text on-surface 0.38; Compose M3 baseline 1.4.0: DisabledInput OnSurface 0.38; DisabledOutline OnSurface 0.12 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-text-field.scss#L33-L… |
| 1206 | `background_color` | v0_192 (cell) v0_192: no unselected container colour; Compose M3 baseline 1.4.0: uncheckedBoxColor = Color.Transparent | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L118-L120 |
| 1207 | `font` | v0_192 (cell) MDC 1.14.0: ?attr/textAppearanceBodyMedium; Compose M3 baseline 1.4.0: none: Checkbox has no label slot or text style | https://github.com/material-components/material-components-android/blob/66c334b7946dabf33adfe1a2b7cad6bcaa4ea3ad/lib/java/com/google/android/material/checkbox/r… |
| 1209 | `border.color` | v0_192 (cell) v0_192: unselected-outline-color on-surface-variant; Compose M3 baseline 1.4.0: UnselectedOutlineColor OnSurfaceVariant; checkedBorderColor = Selected… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L118 |
| 1210 | `border.line_width` | v0_192 (cell) v0_192: 2 / 0; radio 2 (impl); Compose M3 baseline 1.4.0: UnselectedOutlineWidth 2, SelectedOutlineWidth 0; drawn with CheckboxD… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L120 |
| 1211 | `indicator_color` | v0_192 (cell) v0_192: on-primary / primary; Compose M3 baseline 1.4.0: SelectedIconColor OnPrimary; radio SelectedIconColor Primary (ring and… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L79 ; .../v0_192… |
| 1213 | `label_gap` | v0_192 (cell) v0_192: no token; Compose M3 baseline 1.4.0: no token; Checkbox has no label | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss |
| 1214 | `border.corner_radius` | v0_192 (cell) v0_192: 2; Compose M3 baseline 1.4.0: ContainerShape RoundedCornerShape(2.dp); drawn with RadiusSize 2.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L31 |
| 1215 | `border.shadow_enabled` | v0_192 (cell) v0_192: no elevation token; Compose M3 baseline 1.4.0: no elevation token; Canvas drawing only | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss |
| 1216 | `border.padding_horizontal` | v0_192 (cell) v0_192: state-layer-size 40; Compose M3 baseline 1.4.0: StateLayerSize 40 (ripple radius 20); no inner padding | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L90 |
| 1218 | `checked_background` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: SelectedContainerColor Primary; radio draws stroke ring + dot, no fill | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L43 |
| 1219 | `disabled_opacity` | v0_192 (cell) v0_192: on-surface 0.38; Compose M3 baseline 1.4.0: checkbox SelectedDisabledContainer OnSurface 0.38, UnselectedDisabledO… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-checkbox.scss#L44-L46 |
| 1220 | `radio_dot_diameter` | v0_192 (cell) material-web cbd34a8: 10; Compose M3 baseline 1.4.0: RadioButtonDotSize 12.dp drawn at radius 12/2 - stroke/2 = 5, i.e. 10 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/radio/internal/radio.ts#L119 |
| 1221 | `check_mark_stroke_width` | v0_192 (cell) material-web cbd34a8: 2; Compose M3 baseline 1.4.0: CheckboxDefaults.StrokeWidth 2.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/checkbox/internal/_checkbox.scss#L18-L19 |
| 1222 | `radio_indicator_width` | v0_192 (cell) v0_192: 20; Compose M3 baseline 1.4.0: RadioButtonTokens.IconSize 20.dp (requiredSize) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-radio-button.scss#L34 |
| 1227 | `background_color` | v0_192 (cell) v0_192: surface-container; Compose M3 baseline 1.4.0: MenuTokens.ContainerColor = SurfaceContainer; Compose M3 Expressive 1.5.0-beta01: menu groups: groupStandardContainerColor = StandardMenuTokens.Containe… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L31 |
| 1230 | `font.size` | v0_192 (cell) latest 34.0.21: 14; Compose M3 baseline 1.4.0: DropdownMenuItem ProvideTextStyle(typography.labelLarge) 14.sp; Compose M3 Expressive 1.5.0-beta01: impl still labelLarge 14 ('TODO replace with token'); SegmentedMenuTok… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L132 |
| 1231 | `font.weight` | v0_192 (cell) latest 34.0.21: 500; Compose M3 baseline 1.4.0: LabelLargeWeight = Medium | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L140 |
| 1232 | `font.style` | v0_192 (cell) v0_192: no style token; Compose M3 baseline 1.4.0: TypographyTokens.LabelLarge sets no fontStyle (normal) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L191-L195 |
| 1233 | `font.color` | v0_192 (cell) latest 34.0.21: on-surface; Compose M3 baseline 1.4.0: textColor = ListTokens.ListItemLabelTextColor = OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L120 |
| 1234 | `row_height` | v0_192 (cell) v0_192: menu-list-item-container-height 48; Compose M3 baseline 1.4.0: MenuListItemContainerHeight 48.dp (minHeight); Compose M3 Expressive 1.5.0-beta01: Expressive items minHeight SegmentedMenuTokens.Item 44.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L47 |
| 1236 | `border.padding_vertical` | v0_192 (cell) v0_192: 14 derived; Compose M3 baseline 1.4.0: vertical padding 0, label centred in 48 min height -> 14; menu contain…; Compose M3 Expressive 1.5.0-beta01: selectable items: SelectableItemVerticalPadding 12 (12+20+12 = 44); to… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L47 |
| 1238 | `border.line_width` | v0_192 (cell) v0_192: no outline token; Compose M3 baseline 1.4.0: DropdownMenu border: BorderStroke? = null | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss |
| 1239 | `border.corner_radius` | v0_192 (cell) v0_192: 4; Compose M3 baseline 1.4.0: MenuTokens.ContainerShape = CornerExtraSmall; Compose M3 Expressive 1.5.0-beta01: SegmentedMenuTokens.ContainerShape CornerLarge (16); group shapes larg… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L34 |
| 1240 | `border.shadow_enabled` | v0_192 (cell) v0_192: level2; Compose M3 baseline 1.4.0: ShadowElevation = MenuTokens.ContainerElevation Level2 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L32 |
| 1242 | `icon_size` | v0_192 (cell) latest 34.0.21: 24; Compose M3 baseline 1.4.0: leading icon box minWidth ListTokens.ListItemLeadingIconSize 24.dp; Compose M3 Expressive 1.5.0-beta01: MenuDefaults.LeadingIconSize = SegmentedMenuTokens.ItemLeadingIconSize… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L48 |
| 1243 | `hover_background` | v0_192 (cell) latest 34.0.21: on-surface 0.08; Compose M3 baseline 1.4.0: ripple(true) in the menu's content colour (OnSurface) at HoverStateLay… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L112-L116 |
| 1244 | `hover_text_color` | v0_192 (cell) latest 34.0.21: on-surface; Compose M3 baseline 1.4.0: ListItemHoverLabelTextColor OnSurface; MenuItemColors has no hover sta… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L108 |
| 1245 | `disabled_text_color` | v0_192 (cell) latest 34.0.21: on-surface 0.38; Compose M3 baseline 1.4.0: ListItemDisabledLabelTextColor OnSurface @ 0.38 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-menu.scss#L92 |
| 1251 | `background_color` | v0_192 (cell) v0_192: inverse-surface; Compose M3 baseline 1.4.0: PlainTooltipTokens.ContainerColor = InverseSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-plain-tooltip.scss#L31 |
| 1253 | `font.size` | v0_192 (cell) v0_192: 12; Compose M3 baseline 1.4.0: SupportingTextFont BodySmall 12.sp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-plain-tooltip.scss#L39 |
| 1254 | `font.weight` | v0_192 (cell) v0_192: 400; Compose M3 baseline 1.4.0: BodySmallWeight = Normal | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L75 |
| 1255 | `font.style` | Compose M3 baseline 1.4.0: TypographyTokens.BodySmall sets no fontStyle | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1256 | `font.color` | v0_192 (cell) v0_192: inverse-on-surface; Compose M3 baseline 1.4.0: SupportingTextColor = InverseOnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-plain-tooltip.scss#L33 |
| 1257 | `border.padding_horizontal` | v0_192 (cell) androidx 1e67768: 8; Compose M3 baseline 1.4.0: PlainTooltipHorizontalPadding 8.dp | https://github.com/androidx/androidx/blob/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1258 | `border.padding_vertical` | v0_192 (cell) androidx 1e67768: 4; Compose M3 baseline 1.4.0: PlainTooltipVerticalPadding 4.dp | https://github.com/androidx/androidx/blob/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1259 | `max_width` | v0_192 (cell) androidx 1e67768: 200; min height 24; Compose M3 baseline 1.4.0: plainTooltipMaxWidth 200.dp; TooltipMinHeight 24.dp (min width 40) | https://github.com/androidx/androidx/blob/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1260 | `border.corner_radius` | v0_192 (cell) v0_192: 4; Compose M3 baseline 1.4.0: ContainerShape CornerExtraSmall | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-plain-tooltip.scss#L32 |
| 1262 | `border.line_width` | Compose M3 baseline 1.4.0: PlainTooltipTokens has no outline token; PlainTooltip takes no border | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1263 | `border.shadow_enabled` | v0_192 (cell) androidx 1e67768: shadowElevation 0; Compose M3 baseline 1.4.0: shadowElevation: Dp = 0.dp | https://github.com/androidx/androidx/blob/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1279 | `thumb_hover_color` | Compose M3 baseline 1.5.0-beta01: none: the scrollbar is visual-only and 'cannot be interacted with' | https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1289 | `fill_color` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: SliderTokens.ActiveTrackColor = Primary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L37 |
| 1291 | `thumb_color` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: HandleColor = Primary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L52 |
| 1295 | `disabled_opacity` | v0_192 (cell) v0_192: 0.38 / 0.12 / 0.38; Compose M3 baseline 1.4.0: OnSurface 0.38 / 0.12 / 0.38 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L40-L47 |
| 1301 | `fill_color` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: ProgressIndicatorTokens.ActiveIndicatorColor = Primary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-linear-progress-indicator.scss… |
| 1303 | `track_height` | v0_192 (cell) v0_192: 4; Compose M3 baseline 1.4.0: LinearProgressIndicatorTokens.Height 4.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-linear-progress-indicator.scss… |
| 1304 | `min_width` | Compose M3 baseline 1.4.0: no token and no minimum; default size LinearIndicatorWidth 240.dp ('Wi… | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1306 | `border.shadow_enabled` | Compose M3 baseline 1.4.0: no elevation token; Canvas drawing | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1312 | `background_color` | v0_192 (cell) v0_192: surface; Compose M3 baseline 1.4.0: PrimaryNavigationTabTokens.ContainerColor = Surface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L5… |
| 1314 | `active_text_color` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: primaryContentColor = ActiveLabelTextColor Primary | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L8… |
| 1316 | `min_width` | v0_192 (cell) androidx 1e67768: 90; Compose M3 baseline 1.4.0: ScrollableTabRowMinTabWidth 90.dp | https://github.com/androidx/androidx/blob/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1318 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: padding 0 16px; Compose M3 baseline 1.4.0: HorizontalTextPadding 16.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tabs/internal/_tab.scss#L23 |
| 1319 | `border.padding_vertical` | Compose M3 baseline 1.4.0: fixed height(SmallTabHeight 48), text centred, titleSmall line 20 -> 1… | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1321 | `border.line_width` | Compose M3 baseline 1.4.0: no outline token on tabs | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1322 | `border.corner_radius` | v0_192 (cell) v0_192: corner-none; Compose M3 baseline 1.4.0: ContainerShape CornerNone | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L5… |
| 1323 | `border.shadow_enabled` | v0_192 (cell) v0_192: level0; Compose M3 baseline 1.4.0: ContainerElevation Level0 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L5… |
| 1324 | `font` | v0_192 (cell) v0_192: title-small; Compose M3 baseline 1.4.0: LabelTextFont TitleSmall 14.sp / Medium / 20.sp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L1… |
| 1327 | `active_indicator_color` | v0_192 (cell) v0_192: primary; Compose M3 baseline 1.4.0: ActiveIndicatorColor Primary; content width (matchContentSize = true) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L3… |
| 1328 | `active_indicator_width` | v0_192 (cell) v0_192: 3; Compose M3 baseline 1.4.0: ActiveIndicatorHeight 3.dp (corners: see 1313) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-primary-navigation-tab.scss#L3… |
| 1329 | `active_indicator_side` | v0_192 (cell) material-web cbd34a8: inset auto 0 0 0; Compose M3 baseline 1.4.0: 'positioned at the bottom of the TabRow, on top of the divider' | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tabs/internal/_tab.scss#L92-L102 |
| 1333 | `background_color` | v0_192 (cell) v0_192: surface / modal surface-container-low; Compose M3 baseline 1.4.0: StandardContainerColor Surface; ModalContainerColor SurfaceContainerLo… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L162 |
| 1335 | `font.color` | v0_192 (cell) v0_192: on-surface-variant; Compose M3 baseline 1.4.0: InactiveLabelTextColor OnSurfaceVariant | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L106 |
| 1336 | `selection_background` | v0_192 (cell) v0_192: secondary-container, corner-full, 56; Compose M3 baseline 1.4.0: ActiveIndicatorColor SecondaryContainer, ActiveIndicatorShape CornerFu… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L37-L40 |
| 1337 | `selection_text_color` | v0_192 (cell) v0_192: on-secondary-container; Compose M3 baseline 1.4.0: ActiveLabelTextColor OnSecondaryContainer (#1d192b light); Compose M3 Expressive 1.5.0-beta01: light #4a4458 (Secondary30), dark #e8def8 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L56 |
| 1338 | `border.color` | v0_192 (cell) latest 34.0.21: divider-color outline (latest only); Compose M3 baseline 1.4.0: no divider/border token or parameter on drawer sheets | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-navigation-drawer.scss#L8… |
| 1339 | `border.line_width` | v0_192 (cell) v0_192: divider thickness 1; Compose M3 baseline 1.4.0: DividerTokens.Thickness 1.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-divider.scss#L26 |
| 1342 | `hover_background` | v0_192 (cell) v0_192: inactive-hover-state-layer on-surface @0.08; Compose M3 baseline 1.4.0: item Surface(onClick) ripple in content colour; unselected container T… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-navigation-drawer.scss#L103 |
| 1347 | `font.size` | v0_192 (cell) v0_192: 22; Compose M3 baseline 1.4.0: TitleLarge 22.sp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L42 |
| 1348 | `font.weight` | v0_192 (cell) v0_192: 400; Compose M3 baseline 1.4.0: Normal | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L57 |
| 1349 | `font.style` | Compose M3 baseline 1.4.0: TypographyTokens.TitleLarge sets no fontStyle | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1350 | `font.color` | v0_192 (cell) v0_192: on-surface; Compose M3 baseline 1.4.0: AppBarTokens.TitleColor OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L38 |
| 1351 | `bar_height` | v0_192 (cell) v0_192: 64; Compose M3 baseline 1.4.0: AppBarSmallTokens.ContainerHeight 64.dp (TopAppBarExpandedHeight); Doc… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L36 |
| 1352 | `item_gap` | v0_192 (cell) latest 34.0.21: 0; Compose M3 baseline 1.4.0: AppBarTokens.IconButtonSpace 0.dp; DockedToolbarTokens min 4 / max 32 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-app-bar.scss#L25 |
| 1353 | `border.padding_horizontal` | v0_192 (cell) latest 34.0.21: 4 / docked 16; Compose M3 baseline 1.4.0: TopAppBarHorizontalPadding 4.dp (tokens LeadingSpace/TrailingSpace 4);… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-app-bar.scss#L29 |
| 1354 | `border.padding_vertical` | Compose M3 baseline 1.4.0: no vertical-space token in AppBarTokens/AppBarSmallTokens | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1355 | `background_color` | v0_192 (cell) v0_192: surface / surface-container; Compose M3 baseline 1.4.0: ContainerColor Surface; OnScrollContainerColor SurfaceContainer; docke… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L34 |
| 1356 | `icon_size` | v0_192 (cell) v0_192: 24; Compose M3 baseline 1.4.0: AppBarTokens.IconSize 24.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-top-app-bar-small.scss#L59 |
| 1362 | `font.family` | Compose M3 baseline 1.4.0 / 1.5.0-beta01: none: no status-bar component in either API file (only ModalBottomShee… | https://github.com/androidx/androidx/blob/19991f2c0858aadfcb442ce02b2c8e4743c96898/compose/material3/material3/api/1.5.0-beta01.txt#L2528 |
| 1379 | `background_color` | v0_192 (cell) v0_192: surface; Compose M3 baseline 1.4.0: ListTokens.ListItemContainerColor Surface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L39 |
| 1380 | `item_font` | v0_192 (cell) v0_192: body-large; Compose M3 baseline 1.4.0: ListItemLabelTextFont BodyLarge | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L92 |
| 1381 | `item_font.color` | v0_192 (cell) v0_192: on-surface; Compose M3 baseline 1.4.0: ListItemLabelTextColor OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L91 |
| 1383 | `selection_background` | v0_192 (cell) latest 34.0.21: secondary-container; Compose M3 baseline 1.4.0: none: ListItem has no selected state; Compose M3 Expressive 1.5.0-beta01: ListItem(selected, onClick) selectedContainerColor = ListTokens.ItemSe… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-list.scss#L258 |
| 1384 | `selection_text_color` | v0_192 (cell) v0_192: menu list-item-selected-label on-secondary-container; Compose M3 Expressive 1.5.0-beta01: ItemSelectedLabelTextColor OnSecondaryContainer (light #4a4458 under e… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L37 |
| 1385 | `header_font` | v0_192 (cell) v0_192: title-small | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-data-table.scss#L53-L61 |
| 1386 | `header_font.color` | v0_192 (cell) v0_192: on-surface-variant | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-data-table.scss#L51 |
| 1387 | `header_background` | v0_192 (cell) v0_192: surface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-data-table.scss#L49 |
| 1388 | `grid_color` | v0_192 (cell) v0_192: outline-variant 1 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-data-table.scss#L84-L85 |
| 1389 | `row_height` | v0_192 (cell) v0_192: 56; Compose M3 baseline 1.4.0: ListItemOneLineContainerHeight 56.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L156 |
| 1390 | `border.padding_horizontal` | v0_192 (cell) v0_192: 16; Compose M3 baseline 1.4.0: ListItemStartPadding / EndPadding 16.dp | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L152 |
| 1392 | `hover_background` | v0_192 (cell) v0_192: on-surface 0.08; Compose M3 baseline 1.4.0: none built in (ListItem is not clickable); Compose M3 Expressive 1.5.0-beta01: interactive ListItem ripple in content colour (OnSurface) at 0.08 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-list.scss#L85-L87 |
| 1405 | `background_color` | v0_192 (cell) material-web cbd34a8: surface-container (L #f3edf7, D #211f26); Compose M3 baseline 1.4.0 (5113b82): MenuTokens.ContainerColor = SurfaceContainer (Neutral94 #f3edf7 / Neut…; Compose M3 Expressive 1.5.0-beta01 (19991f…: DropdownMenu unchanged (MenuDefaults.containerColor = MenuTokens.Conta… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L31 |
| 1409 | `border.line_width` | v0_192 (cell) material-web cbd34a8: no outline token in comp.menu; Compose M3 baseline 1.4.0 (5113b82): 0: DropdownMenu border: BorderStroke? = null | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L29-L60 |
| 1410 | `border.corner_radius` | v0_192 (cell) material-web cbd34a8: 4 (menu), 12 (rich tooltip); Compose M3 baseline 1.4.0 (5113b82): MenuTokens.ContainerShape = CornerExtraSmall = 4; RichTooltipTokens.Co…; Compose M3 Expressive 1.5.0-beta01 (19991f…: DropdownMenu still 4. Expressive DropdownMenuGroup standaloneGroupShap… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L34 ; _md-comp-rich-… |
| 1411 | `border.shadow_enabled` | v0_192 (cell) material-web cbd34a8: level2; Compose M3 baseline 1.4.0 (5113b82): true: MenuDefaults.ShadowElevation = MenuTokens.ContainerElevation = L…; Compose M3 Expressive 1.5.0-beta01 (19991f…: same: DropdownMenuGroup shadowElevation = MenuDefaults.ShadowElevation… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-menu.scss#L32 |
| 1412 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: 0 (only top-space/bottom-space new tokens); Compose M3 baseline 1.4.0 (5113b82): 0: menu content padded vertically only; items carry their own 12 horiz…; Compose M3 Expressive 1.5.0-beta01 (19991f…: 0: DropdownMenuGroupContentPadding horizontal = 0 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-menu.scss#L52-L55 |
| 1413 | `border.padding_vertical` | v0_192 (cell) material-web cbd34a8: 8 (top-space/bottom-space); Compose M3 baseline 1.4.0 (5113b82): 8: DropdownMenuVerticalPadding; Compose M3 Expressive 1.5.0-beta01 (19991f…: DropdownMenu still 8 (Menu.kt:2386-2387); Expressive DropdownMenuGroup… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-menu.scss#L53-L54 |
| 1419 | `divider_width` | v0_192 (cell) material-web cbd34a8…: container-width 24, width 4, height 48; Compose M3 baseline 1.4.0 (5113b82): DragHandleTokens (VERSION 7_0_1): ContainerWidth 24, Width 4, Height 4… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-drag-handle.scss#L26-L34 |
| 1420 | `divider_color` | v0_192 (cell) material-web cbd34a8…: outline; Compose M3 baseline 1.4.0 (5113b82): DragHandleTokens.Color = Outline (NeutralVariant50 #79747e / NeutralVa… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-drag-handle.scss#L36 |
| 1426 | `line_color` | v0_192 (cell) material-web cbd34a8: outline-variant; Compose M3 baseline 1.4.0 (5113b82): DividerTokens.Color = OutlineVariant (#cac4d0 / #49454f) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-divider.scss#L25 |
| 1427 | `line_width` | v0_192 (cell) material-web cbd34a8: 1; Compose M3 baseline 1.4.0 (5113b82): DividerTokens.Thickness 1 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-divider.scss#L26 |
| 1435 | `caption` | v0_192 (cell) material-web cbd34a8: body-small 0.75rem/regular/1rem; label-small 0.6875rem/medium/1rem; Compose M3 baseline 1.4.0 (5113b82): BodySmall 12sp / Regular / 16; LabelSmall 11 / Medium / 16; Compose M3 Expressive 1.5.0-beta01 (19991f…: same: MaterialExpressiveTheme defaults typography to Typography() ('TO… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L72-L75 ; #L230-… |
| 1436 | `section_heading` | v0_192 (cell) material-web cbd34a8: 1rem / medium / 1.5rem; Compose M3 baseline 1.4.0 (5113b82): TitleMedium 16 / Medium / 24 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L264-L267 |
| 1437 | `dialog_title` | v0_192 (cell) material-web cbd34a8: 1.5rem / regular / 2rem; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.HeadlineFont = HeadlineSmall: 24 / Regular / 32 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L175-L178 ; _md-… |
| 1438 | `display` | v0_192 (cell) material-web cbd34a8: 2.25rem / regular / 2.75rem; display-large 3.5625rem / regular / 4rem; Compose M3 baseline 1.4.0 (5113b82): DisplaySmall 36 / Regular / 44; DisplayLarge 57 / Regular / 64 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-sys-typescale.scss#L123-L126 ; #L89… |
| 1456 | `section_gap` | v0_192 (cell) androidx 1e67768 (ma…: horizontalPartitionSpacerSize 0 (Compact, Medium), 24 (Expanded, wider…; Compose M3 baseline 1.4.0 (5113b82): no layout-gap token or constant in material3 tokens/ | https://github.com/androidx/androidx/blob/1e6776853023ebe4266aca2b308305723e3a2d57/compose/material3/adaptive/adaptive-layout/src/commonMain/kotlin/androidx/com… |
| 1467 | `track_width` | v0_192 (cell) material-web cbd34a8: 52; Compose M3 baseline 1.4.0 (5113b82): SwitchTokens.TrackWidth 52 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L92 |
| 1468 | `track_height` | v0_192 (cell) material-web cbd34a8: 32; Compose M3 baseline 1.4.0 (5113b82): SwitchTokens.TrackHeight 32 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L89 |
| 1469 | `thumb_diameter` | v0_192 (cell) material-web cbd34a8: 24 / 16 / 28 / 24; Compose M3 baseline 1.4.0 (5113b82): SelectedHandle 24, UnselectedHandle 16, PressedHandle 28, IconHandle 2… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L63-L64 ; #L106-L1… |
| 1470 | `track_radius` | v0_192 (cell) material-web cbd34a8: corner-full; Compose M3 baseline 1.4.0 (5113b82): SwitchTokens.TrackShape = CornerFull (CircleShape) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L91 |
| 1471 | `checked_background` | v0_192 (cell) material-web cbd34a8: primary; hover primary; Compose M3 baseline 1.4.0 (5113b82): SelectedTrackColor = Primary (Primary40 #6750a4 / Primary80 #d0bcff);… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L86 ; #L73 |
| 1472 | `unchecked_background` | v0_192 (cell) material-web cbd34a8: surface-container-highest; outline-color outline; outline width 2; Compose M3 baseline 1.4.0 (5113b82): UnselectedTrackColor = SurfaceContainerHighest (#e6e0e9 / #36343b); Un… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L135-L137 ; #L90 |
| 1473 | `thumb_background` | v0_192 (cell) material-web cbd34a8: on-primary / outline; Compose M3 baseline 1.4.0 (5113b82): SelectedHandleColor = OnPrimary (Primary100 #ffffff / Primary20 #381e7… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L62 ; #L105 |
| 1474 | `disabled_opacity` | v0_192 (cell) material-web cbd34a8: 0.12 / 0.38 / surface@1; Compose M3 baseline 1.4.0 (5113b82): DisabledTrackOpacity 0.12 (OnSurface), DisabledUnselectedHandle OnSurf… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L31-L41 |
| 1475 | `unchecked_thumb_diameter` | v0_192 (cell) material-web cbd34a8: 16; Compose M3 baseline 1.4.0 (5113b82): UncheckedThumbDiameter = UnselectedHandleWidth 16 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L106-L107 |
| 1476 | `unchecked_thumb_background` | v0_192 (cell) material-web cbd34a8: outline; Compose M3 baseline 1.4.0 (5113b82): UnselectedHandleColor = Outline (#79747e / #938f99) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-switch.scss#L105 |
| 1483 | `background_color` | v0_192 (cell) material-web cbd34a8: surface-container-high; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.ContainerColor = SurfaceContainerHigh (Neutral92 #ece6f0… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L73 |
| 1484 | `body_font` | v0_192 (cell) material-web cbd34a8: body-medium; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.SupportingTextFont = BodyMedium: 14 / Regular / 20 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L99-L104 ; #L119-L… |
| 1485 | `body_font.color` | v0_192 (cell) material-web cbd34a8: on-surface-variant; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.SupportingTextColor = OnSurfaceVariant (#49454f / #cac4d0… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L97-L98 |
| 1487 | `min_width` | v0_192 (cell) material-web cbd34a8: min-width 280px; Compose M3 baseline 1.4.0 (5113b82): DialogMinWidth 280 (sizeIn minWidth) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L42 |
| 1488 | `max_width` | v0_192 (cell) material-web cbd34a8: min(560px, calc(100% - 48px)); Compose M3 baseline 1.4.0 (5113b82): DialogMaxWidth 560 (no 100%-48 term; the platform dialog window bounds… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L40 |
| 1491 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: 24 (headline 24 24 0, content 24, actions 16 24 24); Compose M3 baseline 1.4.0 (5113b82): DialogPadding = PaddingValues(all = 24) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L111 ; #L192 ; #L218 |
| 1492 | `border.padding_vertical` | v0_192 (cell) material-web cbd34a8: 24 top (headline), 24 bottom (actions); Compose M3 baseline 1.4.0 (5113b82): 24 all sides (DialogPadding); inner gaps differ: title bottom 16, text… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L111 ; #L218 |
| 1493 | `button_gap` | v0_192 (cell) material-web cbd34a8: gap 8px; Compose M3 baseline 1.4.0 (5113b82): ButtonsMainAxisSpacing 8 (wrapped rows: cross-axis 12) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L216 |
| 1494 | `button_order` | v0_192 (cell) material-web cbd34a8: flex-end; Compose M3 baseline 1.4.0 (5113b82): dismissButton then confirmButton, arranged Arrangement.End: primary ri… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/dialog/internal/_dialog.scss#L217 |
| 1496 | `title_font.size` | v0_192 (cell) material-web cbd34a8: headline-small-size 1.5rem; Compose M3 baseline 1.4.0 (5113b82): HeadlineSmallSize 24 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L80 |
| 1497 | `title_font.weight` | v0_192 (cell) material-web cbd34a8: headline-small-weight (regular); Compose M3 baseline 1.4.0 (5113b82): HeadlineSmallWeight = WeightRegular (FontWeight.Normal, 400) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L95-L96 |
| 1498 | `title_font.style` | Compose M3 baseline 1.4.0 (5113b82): Normal: HeadlineSmall TextStyle sets no fontStyle | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1499 | `title_font.color` | v0_192 (cell) material-web cbd34a8: on-surface; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.HeadlineColor = OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L76 |
| 1500 | `border.corner_radius` | v0_192 (cell) material-web cbd34a8: corner-extra-large; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.ContainerShape = CornerExtraLarge = 28 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L75 |
| 1502 | `border.line_width` | Compose M3 baseline 1.4.0 (5113b82): 0: AlertDialogContent Surface has no border | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1503 | `icon_size` | v0_192 (cell) material-web cbd34a8: 24; secondary; Compose M3 baseline 1.4.0 (5113b82): DialogTokens.IconSize 24, IconColor = Secondary (IconSize unused by th… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-dialog.scss#L121-L122 |
| 1535 | `stroke_width` | v0_192 (cell) material-web cbd34a8: 4; Compose M3 baseline 1.4.0 (5113b82): CircularStrokeWidth = TrackThickness 4 (ActiveThickness 4); track gap…; Compose M3 Expressive 1.5.0-beta01 (19991f…: wavy: circularIndicatorStroke width = ActiveThickness 4, round cap | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-circular-progress-indicator.sc… |
| 1536 | `fill_color` | v0_192 (cell) material-web cbd34a8: primary; Compose M3 baseline 1.4.0 (5113b82): circularColor = ProgressIndicatorTokens.ActiveIndicatorColor = Primary… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-circular-progress-indicator.sc… |
| 1545 | `background_color` | v0_192 (cell) material-web cbd34a8: token surface-container-highest, listed in $unsupported-tokens: transp…; Compose M3 baseline 1.4.0 (5113b82): transparent: ExposedDropdownMenuDefaults.outlinedTextFieldColors conta… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L82-L83 ;… |
| 1546 | `font` | v0_192 (cell) material-web cbd34a8: body-large; Compose M3 baseline 1.4.0 (5113b82): FieldInputTextFont = BodyLarge 16 / Regular (field textStyle = LocalTe… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L180-L185… |
| 1547 | `font.color` | v0_192 (cell) material-web cbd34a8: on-surface; Compose M3 baseline 1.4.0 (5113b82): FieldInputTextColor = OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L179 |
| 1548 | `border.color` | v0_192 (cell) material-web cbd34a8: outline; hover on-surface; Compose M3 baseline 1.4.0 (5113b82): TextFieldOutlineColor = Outline (unfocusedBorderColor); TextFieldHover… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L233 ; #L… |
| 1549 | `border.line_width` | v0_192 (cell) material-web cbd34a8: 1; Compose M3 baseline 1.4.0 (5113b82): TextFieldOutlineWidth 1; OutlinedTextFieldDefaults.UnfocusedBorderThic… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L234 |
| 1550 | `min_height` | Compose M3 baseline 1.4.0 (5113b82): OutlinedTextFieldDefaults.MinHeight 56 | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1552 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: leading-space 16; with-trailing-content-trailing-space 12; Compose M3 baseline 1.4.0 (5113b82): leading 16 (contentPadding start = TextFieldPadding 16); trailing icon… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-outlined-field.scss#L222 ; #L240-L241 |
| 1553 | `arrow_icon_size` | v0_192 (cell) material-web cbd34a8: 24; Compose M3 baseline 1.4.0 (5113b82): TextFieldTrailingIconSize 24 (Icons.Filled.ArrowDropDown) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L261 |
| 1555 | `border.corner_radius` | v0_192 (cell) material-web cbd34a8: corner-extra-small; Compose M3 baseline 1.4.0 (5113b82): TextFieldContainerShape = CornerExtraSmall 4; OutlinedTextFieldDefault… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L84-L85 |
| 1556 | `border.shadow_enabled` | v0_192 (cell) material-web cbd34a8: menu-container-elevation level2; Compose M3 baseline 1.4.0 (5113b82): field no elevation; menu MenuContainerElevation Level2 / MenuDefaults.… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L42 |
| 1557 | `border.padding_vertical` | v0_192 (cell) material-web cbd34a8: top-space / bottom-space 16; Compose M3 baseline 1.4.0 (5113b82): 16 top / 16 bottom (OutlinedTextFieldDefaults.contentPadding), text li… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-outlined-field.scss#L235 ; #L127 |
| 1558 | `disabled_opacity` | v0_192 (cell) material-web cbd34a8: 0.12 / 0.38; Compose M3 baseline 1.4.0 (5113b82): TextFieldDisabledOutline OnSurface 0.12; FieldDisabledInputText OnSurf… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-select.scss#L86-L102 |
| 1564 | `background_color` | v0_192 (cell) material-web cbd34a8: no unselected container token; Compose M3 baseline 1.4.0 (5113b82): inactiveContainerColor = Color.Transparent | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1565 | `font` | v0_192 (cell) material-web cbd34a8: label-large; Compose M3 baseline 1.4.0 (5113b82): LabelTextFont = LabelLarge 14 / Medium / 20 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1566 | `font.color` | v0_192 (cell) material-web cbd34a8: on-surface; Compose M3 baseline 1.4.0 (5113b82): UnselectedLabelTextColor = OnSurface | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1567 | `active_background` | v0_192 (cell) material-web cbd34a8: secondary-container; Compose M3 baseline 1.4.0 (5113b82): SelectedContainerColor = SecondaryContainer (Secondary90 #e8def8 / Sec… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1568 | `active_text_color` | v0_192 (cell) material-web cbd34a8: on-secondary-container: L #1d192b, D #e8def8 (latest light secondary30…; Compose M3 baseline 1.4.0 (5113b82): SelectedLabelTextColor = OnSecondaryContainer: L Secondary10 #1d192b,…; Compose M3 Expressive 1.5.0-beta01 (19991f…: L #4a4458 (expressiveLightColorScheme onSecondaryContainer = Secondary… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1569 | `segment_height` | v0_192 (cell) material-web cbd34a8: 40; Compose M3 baseline 1.4.0 (5113b82): ContainerHeight 40 (row min height); each segment min height ButtonDef… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1570 | `separator_width` | v0_192 (cell) material-web cbd34a8: 1; Compose M3 baseline 1.4.0 (5113b82): BorderWidth = OutlineWidth 1; row spacedBy(-BorderWidth) overlaps neig… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1571 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: 12; Compose M3 baseline 1.4.0 (5113b82): 12: SegmentedButtonDefaults.ContentPadding = TextButtonContentPadding… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/labs/segmentedbutton/internal/_outlined-segmented-button.scss#… |
| 1572 | `border.corner_radius` | v0_192 (cell) material-web cbd34a8: corner-full; Compose M3 baseline 1.4.0 (5113b82): Shape = CornerFull; itemShape: start()/end() on the outer segments, Re… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1573 | `border.color` | v0_192 (cell) material-web cbd34a8: outline; Compose M3 baseline 1.4.0 (5113b82): OutlineColor = Outline | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1574 | `border.line_width` | v0_192 (cell) material-web cbd34a8: 1; Compose M3 baseline 1.4.0 (5113b82): OutlineWidth 1 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1575 | `border.shadow_enabled` | Compose M3 baseline 1.4.0 (5113b82): false: no elevation token in OutlinedSegmentedButtonTokens | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1576 | `border.padding_vertical` | Compose M3 baseline 1.4.0 (5113b82): stated padding 8 (ButtonVerticalPadding via TextButtonContentPadding)… | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1577 | `disabled_opacity` | v0_192 (cell) material-web cbd34a8: 0.12 / 0.38; Compose M3 baseline 1.4.0 (5113b82): DisabledOutline OnSurface 0.12; DisabledLabelText OnSurface 0.38 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-segmented-button.scss… |
| 1587 | `background_color` | v0_192 (cell) material-web cbd34a8: filled surface-container-highest; elevated surface-container-low; outl…; Compose M3 baseline 1.4.0 (5113b82): FilledCard SurfaceContainerHighest (#e6e0e9 / #36343b); ElevatedCard S… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-card.scss#L34-L35 ; _md… |
| 1588 | `border.color` | v0_192 (cell) material-web cbd34a8: outlined: outline-variant; Compose M3 baseline 1.4.0 (5113b82): OutlinedCardTokens.OutlineColor = OutlineVariant; filled/elevated have… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-card.scss#L58 |
| 1589 | `border.line_width` | v0_192 (cell) material-web cbd34a8: outlined 1; Compose M3 baseline 1.4.0 (5113b82): OutlinedCardTokens.OutlineWidth 1 | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-outlined-card.scss#L59 |
| 1590 | `border.corner_radius` | v0_192 (cell) material-web cbd34a8: corner-medium; Compose M3 baseline 1.4.0 (5113b82): ContainerShape = CornerMedium 12 (filled, elevated, outlined) | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-card.scss#L38 ; _md-com… |
| 1591 | `border.shadow_enabled` | v0_192 (cell) material-web cbd34a8: level0 / level1 / level0; Compose M3 baseline 1.4.0 (5113b82): Filled Level0, Elevated Level1, Outlined Level0, applied as shadowElev… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-filled-card.scss#L36 ; _md-com… |
| 1592 | `border.padding_horizontal` | Compose M3 baseline 1.4.0 (5113b82): none: Card puts content in a Column with no padding | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1593 | `border.padding_vertical` | Compose M3 baseline 1.4.0 (5113b82): none (Column with no padding) | https://github.com/androidx/androidx/blob/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1603 | `font` | Compose M3 baseline 1.4.0 (5113b82): no expander component or token file (113 token files, none for expand); Compose M3 Expressive 1.5.0-beta01 (19991f…: ExpandedListTokens (VERSION 29.0.0) holds only colours and shapes and… | https://github.com/androidx/androidx/tree/5113b82a4ed6416d6344efa50e41db8cb65928ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/… |
| 1621 | `font` | v0_192 (cell) material-web cbd34a8: text-button label-text-color primary; leading/trailing-space 12; Compose M3 baseline 1.4.0 (5113b82): TextButton content colour Primary (hard-coded: 'TODO replace with the… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-text-button.scss#L46 ; https:/… |
| 1639 | `border.padding_horizontal` | v0_192 (cell) material-web cbd34a8: leading/trailing-space 16; Compose M3 baseline 1.4.0 (5113b82): 16 start/end (OutlinedTextFieldDefaults.contentPadding, TextFieldPaddi… | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-outlined-text-field.scss#L139-L140 |
| 1640 | `border.padding_vertical` | v0_192 (cell) material-web cbd34a8: top/bottom-space 16; Compose M3 baseline 1.4.0 (5113b82): 16 top/bottom | https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/_md-comp-outlined-text-field.scss#L141-L142 |

</details>

### Researcher notes

Each researcher's own notes, verbatim: coverage, method caveats and errors found outside the *differs* rows (wrong citations on ✅ rows, unpinned links). Step 7 corrects the citation errors (S4) as part of the row fixes.

<details><summary>globals</summary>

All 22 v0_192 hex values in scope confirmed against _md-ref-palette.scss. Compose 1.4.0 ColorLight/Dark/PaletteTokens give the same hexes, and these token files have identical values in 1.4.0 and 1.5.0-beta01 (Step 1 diff_s140_b01.txt lists no Color, Palette, TypeScale, Typeface, Elevation, Shape or State change).

Main findings, none of them on docs/todo.md:
(1) 1026/1030: Compose MaterialTheme's ambient text style is bodyLarge 16/24, not body-medium 14/20 (MaterialTheme.kt:107). This is a mapping decision for the maintainer.
(2) 1051/1116: Compose's outlined button uses outline-variant (and disabled outline-variant @0.1), from token set v0_11_0; material-web latest agrees. Fields, select and segmented button keep outline.
(3) 1104-1106: Compose 1.4.0 draws no focus ring; focus is a 0.1 state layer. 1.5.0-beta01 adds an opt-in inset ring (outer secondary 2dp at inset 0, inner onSecondary 3dp at inset 1), default off, and MaterialExpressiveTheme does not enable it.
(4) 1058: Compose has a text-selection colour, primary @ 0.4 (MaterialTheme.kt:218-229).
(5) 1053: Compose shadows are black at Android's platform alphas (ambient 0.039 / spot 0.19), not material-web's 0.3/0.15.
(6) 1133: 'no 16 dp icon token' is false even at v0_192: the switch icon is 16px (_md-comp-switch.scss:76, 122). Compose SwitchTokens:50, 73 agree.
(7) 1056: Expressive light on-secondary-container = #4a4458 (Secondary30). Expressive has no dark scheme (ColorScheme.kt:1166).
(8) 1060/1115: the filled-button citations differ in Compose (FilledButtonTokens v0_11_0: label on-surface-variant @0.38, container @0.1). The tonal button matches the cell values.
(9) 1055: beta01's new selectable menu items use tertiary-container or tertiary, not secondary-container.

Not modelled: Expressive shape morphing (pressed button to CornerSmall 8) and the focus-ring animation specs.

Step 1's version map holds; no pin is wrong. The whole scope (1019-1139) is covered.

</details>

<details><summary>widgets-1</summary>

Scope 2.2-2.15 (lines 1140-1400) fully covered. Every v0_192 / latest / mw-impl / MDC / pin citation in scope was opened and matches its line. Step 1's version map holds. The 1.4.0 colour roles (palette plus light/dark role map) are identical to v0_192, so all L/D hexes hold for the Compose baseline. The Expressive theme changes only 4 light on*Container roles (*30), plus motion. MaterialExpressiveTheme defaults Shapes() and Typography(), and no component reads LocalUsingExpressiveTheme (grep). So Expressive differences come from the new 1.5 composables, not from the theme. A scratch checkout is in .../scratchpad/m2-src/m2d-w2/ (mw full at cbd34a8, mdc files).

Rows where Compose 1.4.0 differs from the cell:
- 1166/1167 (outlined button only): label on-surface-variant, outline outline-variant.
- 1169: min width 58 vs 64.
- 1172: declared vertical padding 8 vs 10. Expressive gives 10.
- 1193: Compose has selection bg primary@0.4 (impl).
- 1212: checkbox drawn 20, not 18. 18 needs ComposeMaterial3Flags.isCheckboxStylingFixEnabled, which is false by default in beta01 too.
- 1228: menu divider outline-variant.
- 1235: menu item padding 12 vs 16.
- 1241: menu icon gap 12 vs 16.
- 1290/1292/1293/1294: Compose baseline slider is the 16 dp track, 4x44 bar, 4 dp ticks, secondary-container spec.
- 1302/1305: progress track secondary-container, round caps.
- 1313: tab indicator corners 3 on all four sides.
- 1315: tab divider outline-variant.
- 1317: icon+label tab 72, not 64.
- 1325: inactive tab label defaults to primary.
- 1334: drawer label bodyLarge (inferred: no text style).
- 1340: standard drawer square.
- 1341: modal drawer has no shadow.
- 1391: list vertical 8.
- Font families: Compose names no typeface (FontFamily.SansSerif).

NEW, not in todo.md: Compose 1.5.0-beta01 has a public non-experimental Modifier.nonInteractiveScrollbar. It has thumb outline@0.7, thickness 4, min thumb 24, transparent track, and fades. So "M3 has no scrollbar" (1277) is true only for 1.4.0 and material-web.

Also new in beta01: an opt-in desktop "precision pointer" density (ComposeMaterial3Flags.isPrecisionPointerComponentSizingEnabled, default false, @ExperimentalMaterial3Api). It changes button MinHeight to 36, small button vertical padding to 8, menu/list paddings to 12, and more. It may matter for a desktop theme.

The beta01 classic ListItem(headlineContent) is @Deprecated. The Expressive ListItem (10 vertical, selected secondary-container) replaces it.

Expressive menus (SegmentedMenuTokens: corner-large 16, item 44, icon 20, group SurfaceContainerLow) come only through new composables.

Not carriable by the model (notes only): button press shape morph, slider thumb gap and press width, progress track gap, stop dot and wavy variants, menu group shapes.

Already on todo.md: radio dot (todo.md:225-244, row 1220) and slider thumb colour (todo.md:787-812, row 1291).

Probe-settleable: 1148-family rows, 1212, 1325, 1334.

</details>

<details><summary>widgets-2</summary>

Scope 1401-1647 fully covered; 73 rows checked. Every material-web citation in scope was re-read at cbd34a8 and matches (line numbers right; where a cell cites a key line, the value sits on the next line, e.g. switch :135/136, card :34/35, select :82/83). The Step 1 version map holds; no mis-pin found.

Compose 1.4.0 baseline DIFFERS from the cell in 7 rows:
- 1486 dialog shadow. The token is level3 in both, but Compose's AlertDialog casts no shadow: Surface tonalElevation 0, no shadowElevation, and the window shadow has alpha 0. A probe can confirm.
- 1489 dialog min_height 140: Compose has none.
- 1490 dialog max_height 560: Compose has none; only the width is constrained.
- 1495 title family: Roboto in the cell, FontFamily.SansSerif in Compose. Both are Roboto on Android.
- 1533 spinner diameter: 48 (v0_192) in the cell, 40 in Compose (CircularProgressIndicatorTokens.Size).
- 1551 combobox min_width: 210 in material-web, 280 in Compose (OutlinedTextFieldDefaults.MinWidth).
- 1554 arrow_area_width: (none) in the cell; Compose's trailing slot is a 48 minimum-interactive box.

Expressive (1.5.0-beta01) changes for this scope:
- No component branches on the Expressive theme (LocalUsingExpressiveTheme is used only in MaterialTheme.kt). So Expressive differs only through the 4 light on*Container roles and through new opt-in components.
- 1568 segmented active_text_color light: #4a4458 (Secondary30).
- Menus: the new DropdownMenuPopup/DropdownMenuGroup use surface-container-low, a 16 standalone corner, 2 vertical padding and level2. Their group shapes morph on hover, which the model cannot carry.
- Spinner: CircularWavyProgressIndicator is 48 with a 4 stroke; its wave cannot be carried. LoadingIndicator (48/38, shape morphing) is still experimental.
- Text scale: unchanged, because MaterialExpressiveTheme defaults to Typography().
- The Expressive alternative to the segmented button is the connected ButtonGroup (ConnectedButtonGroupSmallTokens: BetweenSpace 2, height 40). SegmentedButton is not deprecated in Compose beta01.

New, not on docs/todo.md:
- beta01 adds ComposeMaterial3Flags.isPrecisionPointerComponentSizingEnabled (default false, ComposeMaterial3Flags.kt:66). It is a denser desktop/pointer sizing: dialog padding 20, title 20/26, text padding 16; menu icon-text gap 12; it also touches Button, ToggleButton and ListItem. It is not in 1.4.0, and it may matter for a desktop preset.
- In 1.4.0, TextButtonTokens.LabelColor is OnSurfaceVariant, but the impl hard-codes Primary with a TODO (Button.kt:783-784). Today's text button colour depends on that TODO.
- beta01 dialog ButtonsCrossAxisSpacing is 8 (1.4.0: 12).
- Compose VerticalDragHandle does not use the DragHandleTokens.ContainerWidth 24 token; its interactive area is 48.

No cell needs a ❓. Scratch: m2-src/m2d-s2c/ (mw clone, raw/), scripts in m2d-s2c-scripts/colors.py.

</details>

## M2e — KDE and GNOME currency

**Status:** Steps 1, 2, 4, 5 and 6 done 2026-10-11, beside M1 (D7), in the `m2-research` worktree; Step 3 awaits the maintainer; Step 7 not started. One Workflow ran all five parts (41 Opus agents at high effort, 5.41M subagent tokens, 36.5 min). Triage values: *fix* (Step 7), *Step 8* / *Step 13* (the part's build steps, already decided), *maintainer (Step 3 probe)*, *maintainer (version gap)*, *M4* (reader code), *defer* (an existing docs/todo.md entry), *drop*.

Currency only. Latest stable on 2026-10-11: Plasma 6.7.5 (Breeze `v6.7.5` = `ca217f12`; the facts' `f0b1d75` is a pre-6.8
master commit whose three cited kstyle files are byte-identical at v6.7.5 and v6.7.91), Plasma 6.8.0 due 2026-10-14 (6.7.91 =
Beta 2 checked too), KDE Frameworks 6.31.0 (2026-10-09; every cited file unchanged from 6.30.0), libadwaita 1.10.0
(`a5030fc6` is 1.10.0 + 5 untagged commits with identical cited content, so it re-pins to 1.10.0), GTK 4.24.1, GNOME 51,
gsettings-desktop-schemas 51.0. Six cells changed: Plasma 6.8 drops `forceFontDPI` (three rows; the KDE reader reads it: M4),
GNOME 49+ sets `document-font-name` to Adwaita Sans 12, libadwaita 1.10 draws the popover outline as a shadow. Citation
hygiene: the appendix cites moving branches (KDE `master` at 11 lines, libadwaita/GTK `main` at 23, gsettings-desktop-schemas
`master`, which is now a dead branch frozen at 48.0 since its default became `main`); Step 7 pins them.

### Step 1 — version map

| Version | Released | Support on 2026-10-11 | Generation |
|---|---|---|---|
| Plasma 6.7.5 KDE Plasma 6.7 (latest stable bugfix); Breeze tag v6.7.5 = ca217f12ce16a28e601c78b7f2fdceca586725b2 | 2026-09-08 (https://kde.org/announcements/plasma/6/6.7.5/ 'Tuesday, 8 September 2026'; wiki row 'Schedules/Plasma_6?action=raw' line 182) | In support. Next 6.7.6 on 2026-11-03; 6.7.7 on 2027-01-19, 'This extra release marks the termination of support for the X11 window manager' (https://community.kde.org/Schedules/Plasma_6, raw wikitext lines 127, 132-133, page last edited 2026-10-06, read 2026-1… | Plasma 6.7 |
| Plasma 6.7.91 (6.8 Beta 2) KDE Plasma 6.8 beta; Breeze tag v6.7.91 = b93b56b479abed4ff6f28e338920b12928cd1ad9 (v6.7.90 =… | 2026-09-24 (https://kde.org/announcements/plasma/6/6.7.91/ 'KDE Plasma 6.8 Beta Release', 'Thursday, 24 September 2026') | Pre-release. 6.8.0 scheduled release Wed 2026-10-14 (tarball 2026-10-08); no v6.8.0 tag on invent.kde.org/plasma/breeze as of 2026-10-11 (git ls-remote), announcement page /announcements/plasma/6/6.8.0/ is an empty index. Source: https://community.kde.org/Sche… | Plasma 6.8 |
| Plasma 6.6.x LTS KDE Plasma 6.6 LTS (6.6.6 2026-07-07 latest shipped; 6.6.7 scheduled 2026-10-12, postponed from 2026-10… | 6.6.0 2026-02-17 (https://community.kde.org/Schedules/Plasma_6) | 'Plasma 6.6 is the current LTS release, supported until approximately February 2029'; final LTS 6.6.11 on 2029-02-17 (https://community.kde.org/Schedules/Plasma_6 raw lines 24, 123, 155, read 2026-10-11) | Plasma 6.6 LTS |
| KDE Frameworks 6.31.0 KF6 6.31 (kcolorscheme, kirigami, qqc2-desktop-style, kwidgetsaddons, kguiaddons, kiconthemes, bre… | 2026-10-09 (https://kde.org/announcements/frameworks/6/6.31.0/ 'KDE Ships Frameworks 6.31.0'; tags created 2026-10-09 per git for-each-ref) | Rolling monthly releases, no LTS: 'tagging happens first Friday every month, and the public release is one week later' (https://community.kde.org/Schedules/Frameworks, read 2026-10-11) | KF6 |
| KDE Frameworks 6.30.0 KF6 6.30 (previous; pinned by facts doc for qqc2-desktop-style, kwidgetsaddons, and effectively kc… | 2026-09-09 (https://community.kde.org/Schedules/Frameworks: tag Fri September 4, release Wed September 9, 2026) | Superseded by 6.31.0 (no backports in KF model) | KF6 |
| GNOME 51 GNOME 51 'A Coruña' | 2026-09-16 (https://release.gnome.org/51/ and https://release.gnome.org/calendar/ 'Past Releases') | Stable branch; 51.1 tarballs due 2026-10-10, 51.5 old-stable 2027-03-13 (https://release.gnome.org/calendar/, read 2026-10-11). Dev notes: 'GNOME 51 includes GTK 4.23 and Libadwaita 1.10' (https://release.gnome.org/51/developers/) | GNOME 51 |
| GNOME 50 GNOME 50 | 2026-03-18 (https://release.gnome.org/calendar/) | Old-stable; 50.10 marked 'eol', 'Last release' 2027-03-13 (https://release.gnome.org/calendar/) | GNOME 50 |
| GNOME 49 GNOME 49 | 2025-09-17 (https://release.gnome.org/calendar/) | EOL: not in 'Current branches' (Unstable 52, Stable 51, Old stable 50); 'Once a new major stable release is available, the previous old stable branch is marked as EOL' (https://release.gnome.org/calendar/) | GNOME 49 |
| GTK 4.24.1 GTK 4.24 stable (tag 4.24.1 = 2ea935e3ec680fb122f245b28e18c3495df8c344; 4.24.0 = 1f47f368b17701e693918fcd0784… | 2026-10-01 (https://download.gnome.org/sources/gtk/4.24/ gtk-4.24.1.tar.xz 2026-Oct-01, LATEST-IS-4.24.1) | Current stable series (gtk-4-24 branch); older stable 4.22.5 (tag bd25f1e2, 2026-09-10) | GTK 4.24 / GNOME 51 |
| libadwaita 1.10.0 libadwaita 1.10 (tag 1.10.0 = c9ab118612eee92324143783c4641c98eaded3f0) | 2026-09-14 (https://download.gnome.org/sources/libadwaita/1.10/ libadwaita-1.10.0.tar.xz 2026-Sep-14, LATEST-IS-1.10.0) | Latest stable; 1.9.4 (dfc1d654, 2026-09-14) and 1.8.8 released same day on older branches (git tags gitlab.gnome.org/GNOME/libadwaita) | libadwaita 1.10 / GNOME 51 |
| gsettings-desktop-schemas 51.0 gsettings-desktop-schemas 51.0 (tag commit 1db238b6a349ea7fae6f1c0713afe04d1bb7ea5c) | 2026-09-13 (https://download.gnome.org/sources/gsettings-desktop-schemas/51/ tarball 2026-Sep-13) | Follows GNOME 51 lifecycle | GNOME 51 |

Generations:

- **Plasma 6.7 (Breeze v6.7.x)** (6.7.0 (2026-06-16) .. 6.7.5 (2026-09-08); 6.7.6, 6.7.7 scheduled): Currency only: the facts document's Breeze commit f0b1d7534aa2 is NOT in any 6.7.x tag (it is master, 2026-08-14, describe v6.6.90-46, first contained in v6.7.90 = 6.8 Beta 1; branches Plasma/6.8 and master). But its change was cherry-picked to Plasma/6.7 as a74ad953 and shipped in v6.7.5: kstyle/breezemetrics.h (8811816), breezehelper.cpp (43c47ee), breezestyle.cpp (f0d4dfe) are byte-identical at v6.7.5, f0b1d753, v6.7.91 and master. colors/Breeze{Light,Dark}.colors differ v6.7.5 vs f0b1d753 only in translated Name[ja]/Name[ug] lines (one inserted line near 185/192 shifts later line numbers by +1 at f0b1d753 vs v6.7.5). Other Breeze pins: 2cd5b37d and 35967f0a are pre-6.6 master commits (No… Source: git clone --filter=blob:none https://invent.kde.org/plasma/breeze.git (git describe/merge-base/rev-parse, 2026-10-11); https://community.kde.org/Schedules/Plasma_6
- **Plasma 6.8** (6.7.90 beta1 (2026-09-10), 6.7.91 beta2 (2026-09-24), 6.8.0 due 2026-10-14): Not checked row-by-row (out of scope). For cited kstyle files: no change v6.7.5 -> v6.7.91 -> master (identical blobs). Breeze.themerc/CMakeLists/config json changes only. Source: https://community.kde.org/Schedules/Plasma_6; invent.kde.org/plasma/breeze tags
- **KDE Frameworks 6.30 -> 6.31** (6.30.0 (2026-09-09), 6.31.0 (2026-10-09)): Pinned refs: kcolorscheme 27066d47 = tag v6.30.0 (also v6.30.0-rc1); kguiaddons 7c766f6f = tag v6.30.0; kirigami 8319acc7 = master 2026-09-23 between v6.30.0 and v6.31.0-rc1 (describe v6.30.0-32, contained in v6.31.0); frameworkintegration 178c522c = v6.30.0+1 'Update version to 6.31.0' (in v6.31.0); kcolorscheme fce11e20 = 2020-05-20 'Introduce the Header color set' (historical, first in v5.245.0). Cited-file blobs identical at pin, v6.30.0, v6.31.0, master for kcolorscheme src/kcolorscheme.cpp, kguiaddons src/colors/kcolorutils.cpp, kirigami src/{controls,templates}/Heading.qml, frameworkintegration src/kstyle/kstyle.cpp, qqc2-desktop-style org.kde.desktop/private/SwitchIndicator.qml. CHAN… Source: git clones of invent.kde.org/frameworks/{kcolorscheme,kirigami,kguiaddons,frameworkintegration,kwidgetsaddons,qqc2-desktop-style} 2026-10-11
- **GNOME 51 stack (GTK 4.24, libadwaita 1.10, gds 51)** (GTK 4.24.0/4.24.1; libadwaita 1.10.0; gsettings-desktop-schemas 51.0): Pinned refs: libadwaita 5789add9 = tag 1.9.3 (2026-08-03, branch libadwaita-1-9); libadwaita a5030fc6 = main 2026-09-22 'header-bar: Improve docs', describe 1.10.0-5, NOT in any tag (only 1.11 dev: post-release bump 6c79245, text-selection scss move 6cbaa86, translations); GTK 57f569a2 = main merge 2026-09-18, describe 4.24.0-11, contained in 4.24.1; GTK bd25f1e2 = tag 4.22.5 (2026-09-10, gtk-4-22, previous stable). Cited-file blobs: libadwaita files at a5030fc6 identical to 1.10.0 for all cited (adw-expander-row.ui, adw-toggle-group.c, _buttons, _deprecated, _lists, _toggle-group.scss). 1.9.3 vs 1.10.0 differ: src/adw-tab-box.c, widgets/_dropdowns.scss, _misc.scss, _toggle-group.scss; src/s… Source: git clone --filter=tree:0 https://gitlab.gnome.org/GNOME/{libadwaita,gtk,gsettings-desktop-schemas}.git 2026-10-11; https://download.gnome.org/sources/; https://release.gnome.org/51/developers/

Pinned sources:

- Breeze (Plasma 6.7 stable): https://invent.kde.org/plasma/breeze/-/tree/v6.7.5 — v6.7.5 = ca217f12ce16a28e601c78b7f2fdceca586725b2; covers Plasma 6.7 kstyle metrics/helpers and colors/*.colors; kstyle files identical to the doc's f0b1d753
- Breeze (Plasma 6.8 beta): https://invent.kde.org/plasma/breeze/-/tree/v6.7.91 — v6.7.91 = b93b56b479abed4ff6f28e338920b12928cd1ad9 (switch to v6.8.0 once tagged, due 2026-10-14); covers Plasma 6.8
- kcolorscheme: https://invent.kde.org/frameworks/kcolorscheme/-/tree/v6.31.0 — v6.31.0 = b01fde3096de99584786aa412e0130661c97f7e8 (doc pin 27066d47 = v6.30.0; kcolorscheme.cpp unchanged); covers KF 6.31
- kirigami: https://invent.kde.org/frameworks/kirigami/-/tree/v6.31.0 — v6.31.0 = eecf183a956fdbfbf331c0ef61fd6e35220d4275 (doc pin 8319acc7 is an ancestor); covers KF 6.31
- qqc2-desktop-style: https://invent.kde.org/frameworks/qqc2-desktop-style/-/tree/v6.31.0 — v6.31.0 = 2efe38d55ebcfa97599643daada20c157fa1a8a5 (v6.30.0 = 4751e3969690c84afb87818412754ad65c0a4e13); covers KF 6.31
- kwidgetsaddons: https://invent.kde.org/frameworks/kwidgetsaddons/-/tree/v6.31.0 — v6.31.0 = 259b994c32e20a77593af4d2fbdc7c13bc948bd8 (v6.30.0 = 759d687197173e4da56f7fab2bebdd9e9f3de0bb); covers KF 6.31
- kguiaddons: https://invent.kde.org/frameworks/kguiaddons/-/tree/v6.31.0 — v6.31.0 = 92ebbc758fa58bb89e6d73fdd24e50a296259bd3 (doc pin 7c766f6f = v6.30.0); covers KF 6.31
- kiconthemes: https://invent.kde.org/frameworks/kiconthemes/-/tree/v6.31.0 — v6.31.0 = 420364ec1a81b77ea43e4055df8254f29c14bce4; covers KF 6.31 (doc cites master only)
- breeze-icons: https://invent.kde.org/frameworks/breeze-icons/-/tree/v6.31.0 — v6.31.0 = 60ca51729f00b85813f01580fc9e85723245054d; covers KF 6.31 (doc cites master only)
- frameworkintegration: https://invent.kde.org/frameworks/frameworkintegration/-/tree/v6.31.0 — v6.31.0 = d2be52439d0556c04080b02179af80b0544b03fb; covers KF 6.31
- qqc2-breeze-style: https://invent.kde.org/plasma/qqc2-breeze-style/-/tree/v6.7.5 — v6.7.5 = 2a54be10ffe1fcdb7bcc2cbf0b27a915d5f749f5; covers Plasma 6.7 (doc cites master)
- plasma-integration: https://invent.kde.org/plasma/plasma-integration/-/tree/v6.7.5 — v6.7.5 = d5b8362bc1d0f1a0059c02e6c75427548fa64bd4; covers Plasma 6.7 (doc cites master)
- plasma-workspace: https://invent.kde.org/plasma/plasma-workspace/-/tree/v6.7.5 — v6.7.5 = cc1ab7963f94a9b76836412855caed6b94066188; covers Plasma 6.7 (doc cites master)
- kwin: https://invent.kde.org/plasma/kwin/-/tree/v6.7.5 — v6.7.5 = ab7df7ccb7c6af20f4b279cd6220f7cd3d2267d7; covers Plasma 6.7 (doc cites master)
- plasma-desktop: https://invent.kde.org/plasma/plasma-desktop/-/tree/v6.7.5 — v6.7.5 = 43f55fff3480c6eb5f60133f045bfc74b9c67d84; covers Plasma 6.7
- libadwaita: https://gitlab.gnome.org/GNOME/libadwaita/-/tree/1.10.0 — 1.10.0 = c9ab118612eee92324143783c4641c98eaded3f0 (5789add9 = 1.9.3; a5030fc6 = main 1.10.0+5, untagged); covers GNOME 51
- GTK: https://gitlab.gnome.org/GNOME/gtk/-/tree/4.24.1 — 4.24.1 = 2ea935e3ec680fb122f245b28e18c3495df8c344 (57f569a2 is in 4.24.1; bd25f1e2 = 4.22.5); covers GNOME 51 stable GTK
- gsettings-desktop-schemas: https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/tree/51.0 — 51.0 = 1db238b6a349ea7fae6f1c0713afe04d1bb7ea5c; covers GNOME 51 (default branch is main; master frozen at e8599d1, 2025-04-12)
- KDE Plasma 6 schedule: https://community.kde.org/Schedules/Plasma_6 — page last edited 2026-10-06, read 2026-10-11; covers Plasma release dates and 6.6 LTS
- KDE Frameworks schedule: https://community.kde.org/Schedules/Frameworks — read 2026-10-11; covers KF monthly cadence
- GNOME release calendar: https://release.gnome.org/calendar/ — read 2026-10-11; covers GNOME branches, dates, EOL

Preset target: No preset names a Plasma/GNOME version in its header: native-theme/src/presets/kde-breeze.toml:1 '# KDE Breeze theme - sourced from KDE/breeze repository' (no ref); kde-breeze-live.toml:1 no version; adwaita.toml:1 '# GNOME Adwaita theme - sourced from libadwaita CSS variables' (no ref); adwaita-live.toml:1 no version; presets/README.md:16-19 only says values are 'cited at a pinned line'. Inline comments pin: breeze f0b1d75 (kde-breeze.toml:386, :463 — commit is pre-6.8 master, but cited kstyle files equal v6.7.5; colors files differ only by translation lines), kirigami 8319acc (kde-breeze.toml:72, :78, :449, :455; kde-breeze-live.toml:35, :41, :253, :259 — in KF 6.31.0), libadwaita 1.10.0 (adwaita.toml:136, :497; adwaita-live.toml:72, :304). platform-facts.md:725 labels GNOME font defaults 'GNOME 48+'. So effective targets: Plasma 6.7 / KF 6.30–6.31 / libadwaita 1.10 (GNOME 51), with GT…

<details><summary>Version-map notes</summary>

Moving-branch cells (all in Appendix 'Verification Sources (2026-03-24)'; none in §1.3/§1.4/Ch.2): KDE lines 1762, 1766, 1767, 1769, 1770, 1773, 1775, 1776, 1777, 1778, 1779 (github.com/KDE/*/blob/master, invent.kde.org/*/-/blob|raw/master). GNOME lines 1785, 1787, 1811 (gsettings-desktop-schemas master), 1788, 1790, 1792, 1793, 1795, 1796, 1797, 1798, 1799, 1800, 1801, 1802, 1805, 1806, 1807, 1808, 1809, 1810, 1817, 1818 (libadwaita /-/blob/main), 1804, 1811 (gtk /-/blob/main), 1827 (cantarell-fonts master, which is its default branch); 1815 links the gsettings-desktop-schemas repo with no ref at all. Related: 1828 rsms/inter blob/master.
Finding: gsettings-desktop-schemas' default branch is now main (HEAD -> main on GitLab and the GitHub mirror); 'master' is a stale branch frozen at e8599d1 (2025-04-12, describe 48.0-2), 213 commits behind main. Lines 1785, 1787, 1811 therefore cite a dead branch. Consequence spotted (row checkers to confirm): at tag 51.0 schemas/org.gnome.desktop.interface.gschema.xml.in:158-159 document-font-name defaults to 'Adwaita Sans 12' (commit 87022e2b, 2025-07-30, first in 49.beta), while platform-facts.md:728 says 'Adwaita Sans 11'; font-name stays 'Adwaita Sans 11' (:107-108), monospace 'Adwaita Mono 11' (:165-166). Not found on docs/todo.md.
Breeze f0b1d7534aa2 is no tag: master commit 2026-08-14 'Use font metrics to only inset where the text can spare it', first in v6.7.90 (Plasma 6.8 Beta 1); its backport a74ad953 shipped in v6.7.5, and the three cited kstyle files are byte-identical at v6.7.5, f0b1d753, v6.7.91, master. todo.md:176 already records v6.7.5 as 'the release Plasma 6.7 ships'.
libadwaita a5030fc6 is untagged main (1.10.0+5, 1.11 dev cycle); for every file the doc cites at it, content equals tag 1.10.0, so it can be re-pinned to 1.10.0 with no value change. check-symbolic.svg cited at 1.9.3 no longer exists at 1.10.0.
GTK 57f569a2 is main after 4.24.0, contained in 4.24.1, cited files equal 4.24.0/4.24.1. bd25f1e2 = 4.22.5 (old stable); gtknotebook.c and gtktextview.c changed in 4.24.
KF: kwidgetsaddons src/kcollapsiblegroupbox.cpp (cited at v6.30.0) changed in v6.31.0. Other cited KF files are unchanged through v6.31.0/master.
GNOME 51 developer notes say 'GTK 4.23' (the dev series that became 4.24); stable GTK tarball series is 4.24.
Plasma 6.8.0 is 3 days away (2026-10-14). Pins to v6.7.5 will be one release behind then. 6.6 is LTS to ~Feb 2029.
No usage-share source consulted. Clones kept under scratchpad/m2-src/m2e/.

</details>

### Step 2 — rows checked

77 rows: 71 same, 6 differs, 0 unverifiable (0 of them a probe or reference app could settle). Scopes: kde 5, gnome 72. Every researcher reported its scope covered with no line range left out.

Step 5 verdicts on the *differs* rows: 5 CONFIRMED, 1 PLAUSIBLE.

Step 6 triage: 6 fix.

### Findings — *differs* rows

#### M2e-01 · bug · S2 · docs/platform-facts.md:713
**Claim:** `Font DPI override` (1.3.7 Accessibility): the cell says ~/.config/kcmfontsrc [General] forceFontDPI; fonts KCM shows it on X11 only (main.qml visible: pluginName === "xcb"); plasma6.0 migration deletes forceFontDPIWayland; key still works if set by hand; pinned (cell) / latest stable Plasma 6.7.5 (plasma-workspace v6.7.5 = cc1ab7963f94a9b76836412855caed6b94066188): Same as the cell. forceFontDPI is a UInt, default 0, labelled 'Force font DPI on X11' (fontsaasettingsbase.kcfg:8-11). The KCM row is visible only when Qt.platform.pluginName === "xcb" (main.qml:424-426). krdb xftDpi() reads it on X11 only; on Wayland it uses…; Plasma 6.8 (beta) Plasma 6.7.91 (plasma-workspace v6.7.91 = 40cde89adc84ec57ebf8de6862fd52e8310b8c87): Removed. Commit c675077b8 (2026-06-10, 'kcms/fonts: Drop DPI handling. This was only relevant on X11', first in v6.7.90) deletes the KCM's DPI UI and the forceFontDPI kcfg entry. Commit fe367323b (2026-06-15, 'Drop krdb') removes the krdb reader. At v6.7.91, g….
**Evidence:** [pinned (cell) / latest stable] https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/ui/main.qml#L424-426 ; https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontsaasettingsbase.kcfg#L8-11 ; https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L328-341 [Plasma 6.8 (beta)] https://invent.kde.org/plasma/plasma-workspace/-/commit/c675077b8f7371eb65bfa73c20d86a50828b13fc ; https://invent.kde.org/plasma/plasma-workspace/-/commit/fe367323b30bfa239243b2164d75faacb1be15e7 ; https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.91/startkde/plasma-setup-xwayland/main.cpp#L47-58 ; https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.91/kcms/kconf_update/plasma6.0-remove-dpi-settings.cpp#L25 Note: The cell is right for 6.7.5 (latest stable). It is wrong for Plasma 6.8, which ships 2026-10-14: there Plasma neither writes nor reads forceFontDPI, yet native-theme/src/detect.rs:437-444 still reads it, so a stale value would be honoured. The kconf_update path is kcms/fonts/kconf_update/ (the citation URL above omits 'fonts/'; the correct path is kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp#L25). The citation should be pinned. This is not on todo.md: todo.md:2583 asks a different D…
**Verdict:** CONFIRMED — Every cited value holds at the pinned tags (I grepped all three repos at v6.7.5 and v6.7.91), but the migration file's path is wrong: it is kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp:25, not kcms/kconf_update/. Confirmed value: Plasma 6.7.5 (cc1ab796): forceFontDPI UInt default 0 'Force font DPI on X11' (fontsaasettingsbase.kcfg:8-11); KCM row visible only when Qt.platform.pluginName === "xcb" (main.qml:426); krdb xftDpi() r….
**Triage:** fix — Correct for Plasma 6.7.5; Plasma 6.8 (6.8.0 due 2026-10-14) drops forceFontDPI. Fix in Step 7 once v6.8.0 is tagged; the verifier also found the migration file's path wrong (kcms/fonts/kconf_update/...). The KDE reader reads forceFontDPI: M4.
**Status:** open

#### M2e-02 · bug · S2 · docs/platform-facts.md:714
**Claim:** `Scale factor` (1.3.7 Accessibility): the cell says forceFontDPI / 96 (from kcmfontsrc); pinned (cell) / latest stable Plasma 6.7.5: forceFontDPI is meaningful only on X11 (default 0 = unset). The Wayland path is kwinrc [Xwayland] Scale*96.; Plasma 6.8 (beta) Plasma 6.7.91: Plasma has no forceFontDPI any more; Xft.dpi = kwinrc [Xwayland] Scale × 96..
**Evidence:** [pinned (cell) / latest stable] https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L328-341 [Plasma 6.8 (beta)] https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.91/startkde/plasma-setup-xwayland/main.cpp#L47-58 Note: Same cause as line 713. From 6.8 the ratio has no Plasma-side source.
**Verdict:** PLAUSIBLE — krdb.cpp:328-341 at v6.7.5 and plasma-setup-xwayland/main.cpp:47-58 at v6.7.91 do read forceFontDPI on X11 only and set Xft.dpi from kwinrc [Xwayland] Scale*96 on Wayland, but that value is the Xft.dpi handed to X11 clients, so mapping it to the row's general 'Scale factor' (native Wayland output scale) needs a measurement.
**Triage:** fix — As :713; PLAUSIBLE because the Wayland path (kwinrc [Xwayland] Scale x 96) is Xwayland's DPI, not a text scale.
**Status:** open

#### M2e-03 · bug · S2 · docs/platform-facts.md:1123
**Claim:** `text_scaling_factor` (2.1.7 Accessibility): the cell says `forceFontDPI` / 96; pinned (cell) / latest stable Plasma 6.7.5: forceFontDPI is read on X11 only.; Plasma 6.8 (beta) Plasma 6.7.91: forceFontDPI was dropped from Plasma (c675077b8, fe367323b); no Plasma component reads it..
**Evidence:** [pinned (cell) / latest stable] https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L328-341 [Plasma 6.8 (beta)] https://invent.kde.org/plasma/plasma-workspace/-/commit/c675077b8f7371eb65bfa73c20d86a50828b13fc Note: Same as lines 713-714. Needs a 6.8 decision: either keep it as a legacy X11 key or read nothing on KDE.
**Verdict:** CONFIRMED — krdb.cpp at v6.7.5 reads it only in the non-Wayland branch, and git grep at v6.7.91 finds it only in the kconf_update migration; the cited commit c675077b8 alone removes only the KCM and kcfg, while krdb's removal is fe367323b. Confirmed value: 6.7.5: forceFontDPI read on X11 only (krdb.cpp:340, inside the non-Wayland branch). 6.8 beta (v6.7.91): no reader left; c675077b8 dropped the kcfg/KCM, fe367323b dropped krdb..
**Triage:** fix — As :713.
**Status:** open

#### M2e-04 · bug · S2 · docs/platform-facts.md:728
**Claim:** `org.gnome.desktop.interface document-font-name default` (1.4.1 Fonts): the cell says Adwaita Sans 11 (GNOME 48+); pinned (cell) gsettings-desktop-schemas 48.0 (= frozen 'master' e8599d1 for this key): 'Adwaita Sans 11'; latest stable gsettings-desktop-schemas 51.0 (1db238b6): 'Adwaita Sans 12'.
**Evidence:** [pinned (cell)] https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/48.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L158-159 [latest stable] https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L158-159 Note: Changed by commit 87022e2 'schemas: Increase document font size to 12' (2025-07-30), first in tag 49.beta; 49.0, 50.0, 51.0 all 'Adwaita Sans 12'. Cell is right only for GNOME 48. Not on todo.md. Appendix 1785 cites dead branch master.
**Verdict:** CONFIRMED — The schema reads 'Adwaita Sans 11' at 48.0 and 'Adwaita Sans 12' from 49.0 onward, so the cell's 'GNOME 48+' holds only for GNOME 48; docs/todo.md does not list this item. Confirmed value: 48.0: 'Adwaita Sans 11'; 49.0, 50.0 and 51.0 (1db238b6): 'Adwaita Sans 12' (org.gnome.desktop.interface.gschema.xml.in:158-159), changed by commit 87022e2 'schemas: Increase document font size to 12'….
**Triage:** fix — CONFIRMED: document-font-name is Adwaita Sans 12 from gsettings-desktop-schemas 49.0; the cell's "GNOME 48+" holds for 48 only.
**Status:** open

#### M2e-05 · bug · S2 · docs/platform-facts.md:1408
**Claim:** `border.color (GNOME)` (2.16 popover): the cell says <- defaults.border.color; pinned (cell) libadwaita 1.9.3 (5789add9) / main@1e097d51: > arrow, > contents { border: 1px solid RGB(0 0 0 / 14%) } ($border_color under prefers-contrast: more); latest stable libadwaita 1.10.0: > contents has no CSS border; outline is the first box shadow '0 0 0 1px RGB(0 0 0 / 5%)' ($border_color under prefers-contrast: more); > arrow 'border: 1px solid transparent'.
**Evidence:** [pinned (cell)] https://gitlab.gnome.org/GNOME/libadwaita/-/blob/5789add99c79cee0fae624b56706c7c0bea7fb2b/src/stylesheet/widgets/_popovers.scss#L13-24 [latest stable] https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_popovers.scss#L13-46 Note: Inheritance cell, reported because its source changed in 1.10.0. 1.10.0 popover now matches the .card pattern row 1588 records as '(none) — outline is a box shadow'. Already a 14% black border in 1.9, not $border_color. Not on todo.md.
**Verdict:** CONFIRMED — At both revisions the popover's outline is a fixed black alpha rather than $border_color (currentColor at 15%, platform-facts.md:1116) except under prefers-contrast: more, so the '<- defaults.border.color' cell differs; no other stylesheet rule overrides it. Confirmed value: libadwaita 1.9.3 (5789add9): popover > arrow, > contents { border: 1px solid RGB(0 0 0 / 14%) }, $border_color under prefers-contrast: more (_popovers.scss:13-24). 1.10.0: > contents has no border, ou….
**Triage:** fix — CONFIRMED: libadwaita 1.10.0 draws the popover outline as a 1px box-shadow of black 5% (border colour only under prefers-contrast: more), not defaults.border.color.
**Status:** open

#### M2e-06 · bug · S2 · docs/platform-facts.md:1409
**Claim:** `border.line_width (GNOME)` (2.16 popover): the cell says <- defaults.border.line_width; pinned (cell) libadwaita 1.9.3: 1px CSS border; latest stable libadwaita 1.10.0: no CSS border on contents (1px outline drawn as box-shadow spread).
**Evidence:** [pinned (cell)] https://gitlab.gnome.org/GNOME/libadwaita/-/blob/5789add99c79cee0fae624b56706c7c0bea7fb2b/src/stylesheet/widgets/_popovers.scss#L18 [latest stable] https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_popovers.scss#L13-27 Note: Same change as 1408; also means padding 8 (1412/1413) is now from the outer edge, not inside a 1px border.
**Verdict:** CONFIRMED — The source reads as claimed. The pinned 1.9.3 value (1px) matches defaults.border.line_width, so only 1.10.0 differs: its contents have no CSS border, and a shadow outline takes no layout space. Confirmed value: 1.9.3: 1px CSS border on > contents (_popovers.scss:18), equal to the default 1px. 1.10.0: no CSS border on > contents; the 1px outline is a box-shadow spread (_popovers.scss:13-27), i.e. **(none)** u….
**Triage:** fix — CONFIRMED: 1.10.0 has no CSS border on the popover contents; the 1px outline is a shadow spread and takes no layout space.
**Status:** open

### Step 4 — preset-validator

13 findings: 5 no-citation, 8 depends-on-differs-row.

<details><summary>Validator summary</summary>

M2e (Step 4) audit of kde-breeze.toml, kde-breeze-live.toml, adwaita.toml and adwaita-live.toml in /home/tibi/Rust/native-theme-m2/native-theme/src/presets/, checked against the six facts rows the Step 2 researchers reported as different from today's platform.

Rows with dependent preset keys: only the two GNOME popover rows, docs/platform-facts.md:1408 (border.color) and :1409 (border.line_width). Neither Adwaita preset sets popover.border.color or popover.border.line_width_px. Both values come through the inheritance rules (docs/inheritance-rules.toml:85-94: popover is listed at :88, color comes from defaults.border.color at :91, line_width from defaults.border.line_width at :93). The registry declares the slot at docs/property-registry.toml:386-389. The resolved values are:
- adwaita.toml: #d5d5d5 / 1.0 for light (lines 42/45, via [light.popover.border] line 260) and #4a4a4e / 1.0 for dark (lines 403/406, via line 621).
- adwaita-live.toml: line_width 1.0 (lines 21/253, via lines 154/386); the colour comes from the reader.

That gives eight depends-on-differs-row findings. If libadwaita 1.10.0 is confirmed (contents outline is the box-shadow '0 0 0 1px RGB(0 0 0 / 5%)', not a CSS border), the colour source changes. The 1px width may stay the same visually, but its mechanism and citation change.

There is also a pin conflict inside the facts file. The GNOME column of §2.16 already cites libadwaita 1.10.0 _popovers.scss for padding (platform-facts.md:1412-1413), while the border rows at :1408-1409 describe the 1.9.3 rule. The presets also cite 1.10.0 for check.svg (adwaita.toml:136/497, adwaita-live.toml:72/304).

Rows with no dependent preset key:
- KDE :713, :714 and :1123 (forceFontDPI / scale factor / text_scaling_factor). None of the four presets holds text_scaling_factor or font_dpi. The presets README says the reader fills them (native-theme/src/presets/README.md:79-87). The real consumers are reader code: native-theme/src/kde/mod.rs:144-164 and native-theme/src/detect.rs:416-457 read forceFontDPI. If the Plasma 6.8 removal is confirmed, that belongs in an M2/M3 code finding, not a preset change.
- GNOME :728 (document-font-name). adwaita.toml's defaults.font (lines 49-51 / 410-412) comes from font-name (platform-facts.md:727, :1025-1026), not from document-font-name. Nothing under native-theme/src reads document-font-name.

Other checks:
- Icon sets match the platform: Breeze for KDE, Adwaita for GNOME, one set per preset. icon_theme names the preferred theme, which fits the three-tier fallback model.
- The two Adwaita defaults.border.color values that feed the popover rows carry no citation (2 no-citation findings).
- Grouped no-citation findings cover the main uncited blocks. They are not a full per-key list, and those values were not re-checked against facts in this pass.

No violations were found in the values that are cited.

</details>

#### M2e-P01 · bug · S2 · native-theme/src/presets/adwaita.toml:42
**Claim:** `light.defaults.border.color` (no-citation): preset #d5d5d5, facts GNOME: (Adwaita CSS), no literal value; §2.1.6 :1116 gives --border-opacity 15% of currentColor (docs/platform-facts.md:1051).
**Evidence:** This value feeds the popover border through inheritance (see the depends-on findings above), but its line has no citation and no derivation (which currentColor, which background, which libadwaita commit). Add the source before deciding whether popover should keep inheriting it.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — cite adwaita.toml's border colour derivation (S4), with M2e-05.
**Status:** open

#### M2e-P02 · bug · S2 · native-theme/src/presets/adwaita.toml:403
**Claim:** `dark.defaults.border.color` (no-citation): preset #4a4a4e, facts GNOME: (Adwaita CSS), no literal value; :1116 --border-opacity 15% (docs/platform-facts.md:1051).
**Evidence:** Same as light: uncited, and it feeds dark.popover.border.color through inheritance.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — dark twin of M2e-P01 (S4).
**Status:** open

#### M2e-P03 · bug · S2 · native-theme/src/presets/kde-breeze.toml:10
**Claim:** `light.defaults.* colours (accent_color..focus_ring_color, lines 10-29), line_height (36), focus_ring_* (37-38), border block (42-47), font/mono_font (49-56), icon_sizes (59-63), and most widget geometry without comment lines` (no-citation): preset , facts  ().
**Evidence:** Grouped finding, not a full per-key list. These values have no per-line platform-facts citation, unlike the cited lines (e.g. :30, :34, :66, :101). Values were not re-checked against facts in this M2e pass. No KDE key depends on the reported rows 713/714/1123: text_scaling_factor and font_dpi are not in the preset and are reader-filled (presets/README.md:79-87).
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — add per-line platform-facts citations to kde-breeze.toml:10-29 (S4); values not re-checked in M2e (currency only).
**Status:** open

#### M2e-P04 · bug · S2 · native-theme/src/presets/adwaita.toml:10
**Claim:** `light.defaults.* colours (lines 10-29), line_height (36), focus_ring_* (37-38), border block (42-47), font/mono_font (49-56), icon_sizes (59-63), text_scale (65-77), and the matching dark block (371-438)` (no-citation): preset , facts  ().
**Evidence:** Grouped finding, not a full per-key list. These values have no per-line platform-facts citation. defaults.font Adwaita Sans 11 (lines 49-50, 410-411) comes from font-name (platform-facts.md:727, :1025-1026), not from the reported document-font-name row :728. No preset key depends on row 728, and nothing under native-theme/src reads document-font-name.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — add per-line citations to adwaita.toml:10-29 (S4); values not re-checked in M2e.
**Status:** open

#### M2e-P05 · bug · S2 · native-theme/src/presets/adwaita-live.toml:12
**Claim:** `light.defaults.disabled_opacity` (no-citation): preset 0.5, facts (Adwaita CSS) 0.5, --disabled-opacity 50% (docs/platform-facts.md:1115).
**Evidence:** The value matches the facts. adwaita.toml:34 cites platform-facts.md:1115 for the same key, but adwaita-live.toml:12 and :244 carry no citation. kde-breeze-live.toml:8 does cite its value.
**Verdict:** preset-validator (Step 4); not re-verified.
**Triage:** fix — add the citation adwaita.toml:34 carries (S4).
**Status:** open

<details><summary>Preset keys that depend on a <i>differs</i> row</summary>

| Preset:line | Key | Preset value | Facts row |
|---|---|---|---|
| adwaita.toml:260 | `light.popover.border.color` | (not set; inherits light.defaults.border.color = "#d5d5d5" f… | docs/platform-facts.md:1408 |
| adwaita.toml:260 | `light.popover.border.line_width_px` | (not set; inherits light.defaults.border.line_width_px = 1.0… | docs/platform-facts.md:1409 |
| adwaita.toml:621 | `dark.popover.border.color` | (not set; inherits dark.defaults.border.color = "#4a4a4e" fr… | docs/platform-facts.md:1408 |
| adwaita.toml:621 | `dark.popover.border.line_width_px` | (not set; inherits dark.defaults.border.line_width_px = 1.0… | docs/platform-facts.md:1409 |
| adwaita-live.toml:154 | `light.popover.border.color` | (not set; live preset has no colours, so it resolves from th… | docs/platform-facts.md:1408 |
| adwaita-live.toml:154 | `light.popover.border.line_width_px` | (not set; inherits light.defaults.border.line_width_px = 1.0… | docs/platform-facts.md:1409 |
| adwaita-live.toml:386 | `dark.popover.border.color` | (not set; resolves from the reader's defaults.border.color v… | docs/platform-facts.md:1408 |
| adwaita-live.toml:386 | `dark.popover.border.line_width_px` | (not set; inherits dark.defaults.border.line_width_px = 1.0… | docs/platform-facts.md:1409 |

</details>

### Unverifiable rows

No primary source found: the row keeps or gets a ❓ in Step 7; no value is proposed. *Probe* = a probe or the reference app could measure it.

<details><summary>0 unverifiable rows</summary>

| Line | Property | Cell | Probe | Note |
|---|---|---|---|---|

</details>

### Same rows

Rows whose cell holds at the versions checked; the first citation is given.

<details><summary>71 same rows</summary>

| Line | Property | Values | Source |
|---|---|---|---|
| 715 | `Animation factor` | latest stable Plasma 6.7.5 (kwin v…: group KDE, entry AnimationDurationFactor, Double, default 1, min 0; Plasma 6.8 (beta) Plasma 6.7.91 (kwin…: unchanged: default 1, min 0 | https://invent.kde.org/plasma/kwin/-/blob/v6.7.5/src/kwin.kcfg#L313-317 |
| 1124 | `reduce_motion` | latest stable Plasma 6.7.5: AnimationDurationFactor Double, default 1, min 0; Plasma 6.8 (beta) Plasma 6.7.91: unchanged | https://invent.kde.org/plasma/kwin/-/blob/v6.7.5/src/kwin.kcfg#L313-317 |
| 727 | `org.gnome.desktop.interface font-name default` | latest stable gsettings-desktop-sc…: 'Adwaita Sans 11' | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L107-108 |
| 729 | `org.gnome.desktop.interface monospace-font-name default` | latest stable gsettings-desktop-sc…: 'Adwaita Mono 11' | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L165-166 |
| 730 | `org.gnome.desktop.wm.preferences titlebar-font default` | latest stable gsettings-desktop-sc…: 'Adwaita Sans Bold 11' | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.wm.preferences.gschema.xml.in#L187-188 |
| 744 | `.caption` | latest stable libadwaita 1.10.0: font-weight 400; font-size 82% | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L69-73 |
| 745 | `.caption-heading` | latest stable libadwaita 1.10.0: 700; 82% | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L64-67 |
| 746 | `.body` | latest stable libadwaita 1.10.0: font-weight 400, no font-size (line-height 140%) | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L59-62 |
| 747 | `.heading` | latest stable libadwaita 1.10.0: font-weight 700 | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L55-57 |
| 748 | `.title-4` | latest stable libadwaita 1.10.0: 700; 118% | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L50-53 |
| 749 | `.title-3` | latest stable libadwaita 1.10.0: 700; 136% | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L45-48 |
| 750 | `.title-2` | latest stable libadwaita 1.10.0: 800; 136% | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L40-43 |
| 751 | `.title-1` | latest stable libadwaita 1.10.0: 800; 181% | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_labels.scss#L35-38 |
| 777 | `Control radius $button_radius` | latest stable libadwaita 1.10.0: $button_radius: 9px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_common.scss#L8 |
| 778 | `Card radius $card_radius` | latest stable libadwaita 1.10.0: $card_radius: 12px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_common.scss#L9 |
| 779 | `Window/dialog radius` | latest stable libadwaita 1.10.0: $dialog_radius: $button_radius + 6; --window-radius: $button_radius +… | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_common.scss#L14-17 ; https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesh… |
| 780 | `Frame/border width` | latest stable libadwaita 1.10.0: 1px borders, e.g. tooltip 'border: 1px solid $tooltip_border_color' | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_tooltip.scss#L7 |
| 781 | `Focus ring width` | latest stable libadwaita 1.10.0: @mixin focus-ring($width: 2px ...) -> outline-width: $width | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_drawing.scss#L12-22 |
| 782 | `Focus ring offset` | latest stable libadwaita 1.10.0: $offset: -$width -> outline-offset: $offset | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_drawing.scss#L12-22 |
| 783 | `Disabled opacity` | latest stable libadwaita 1.10.0: --disabled-opacity: 50%; 40% under prefers-contrast: more | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_colors.scss#L320-327 |
| 784 | `Drop shadows` | latest stable libadwaita 1.10.0: box-shadow on popover contents / cards | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_popovers.scss#L19-21 |
| 792 | `Button CSS min-height` | latest stable libadwaita 1.10.0: min-height: 24px; padding: 5px 10px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_buttons.scss#L19-21 |
| 793 | `Entry CSS min-height` | latest stable libadwaita 1.10.0: min-height: 34px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_entries.scss#L4 |
| 794 | `CheckButton indicator size` | latest stable libadwaita 1.10.0: check, radio { min-height: 14px; min-width: 14px; padding: 3px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_checks.scss#L20-26 |
| 795 | `Scale trough min-height` | latest stable libadwaita 1.10.0: scale { min-height: 10px; min-width: 10px; padding: 12px } (the trough… | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_scale.scss#L25-27 |
| 796 | `Scale thumb diameter` | latest stable libadwaita 1.10.0: > slider { min-width: 20px; min-height: 20px; margin: -8px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_scale.scss#L56-58 |
| 797 | `ProgressBar bar height` | latest stable libadwaita 1.10.0: horizontal trough, > progress { min-height: 8px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_progress-bar.scss#L6 |
| 798 | `Notebook tab min height` | latest stable libadwaita 1.10.0: > tabs > tab { min-height: 30px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_notebook.scss#L119-123 |
| 799 | `Scrollbar slider width` | latest stable libadwaita 1.10.0: $_slider_width: 8px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_scrolling.scss#L3 |
| 800 | `Tooltip padding` | latest stable libadwaita 1.10.0: padding: 6px 10px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_tooltip.scss#L13 |
| 801 | `GtkSwitch thumb size` | latest stable libadwaita 1.10.0: > slider { min-width: 20px; min-height: 20px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_switch.scss#L24-26 |
| 802 | `GtkSwitch track padding` | latest stable libadwaita 1.10.0: padding: 3px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_switch.scss#L4 |
| 803 | `GtkSwitch track radius` | latest stable libadwaita 1.10.0: border-radius: 14px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_switch.scss#L3 |
| 804 | `GtkSwitch total track size` | latest stable libadwaita 1.10.0: 3+20+20+3 = 46 wide, 3+20+3 = 26 high | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_switch.scss#L3-26 |
| 805 | `GtkSpinner default size` | pinned (cell) GTK 4.22.5 (bd25f1e2…: DEFAULT_SIZE 16 defined (:91); measure returns CSS -gtk-icon-size (:11…; latest stable GTK 4.24.1: DEFAULT_SIZE 16 defined (:97) but unused; size = -gtk-icon-size (:122)… | https://gitlab.gnome.org/GNOME/gtk/-/blob/4.22.5/gtk/gtkspinner.c#L91-L116 |
| 806 | `GtkDropDown arrow size` | latest stable libadwaita 1.10.0: arrow { min-height: 16px; min-width: 16px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_dropdowns.scss#L14-18 |
| 807 | `GtkDropDown box spacing` | latest stable libadwaita 1.10.0: > button > box { border-spacing: 6px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_dropdowns.scss#L2-3 |
| 808 | `AdwAlertDialog preferred width` | latest stable libadwaita 1.10.0: #define DIALOG_PREFERRED_WIDTH 300 /* sp */ | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/adw-alert-dialog.c#L177 |
| 809 | `AdwAlertDialog max width` | latest stable libadwaita 1.10.0: DIALOG_MAX_WIDTH 372; DIALOG_MAX_WIDE_WIDTH 600 | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/adw-alert-dialog.c#L175-176 |
| 810 | `AdwAlertDialog button spacing` | latest stable libadwaita 1.10.0: .response-area { border-spacing: 12px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_message-dialog.scss#L114-117 |
| 811 | `AdwAlertDialog message padding` | latest stable libadwaita 1.10.0: .message-area { padding-top: 32px; padding-bottom: 9px } children padd… | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_message-dialog.scss#L95-111 |
| 812 | `AdwAlertDialog button (response-area) padding` | latest stable libadwaita 1.10.0: padding: 24px; padding-top: 12px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_message-dialog.scss#L115-116 |
| 813 | `AdwAlertDialog border radius` | latest stable libadwaita 1.10.0: $alert_radius: 18px; dialog.alert sheet { border-radius: $alert_radius… | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_message-dialog.scss#L1-L61 |
| 814 | `GtkExpander arrow size` | latest stable libadwaita 1.10.0: expander { min-width: 16px; min-height: 16px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_expanders.scss#L1-3 |
| 815 | `AdwExpanderRow header min-height` | latest stable libadwaita 1.10.0: row > box.header { min-height: 50px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_lists.scss#L129-133 |
| 816 | `Card border radius` | latest stable libadwaita 1.10.0: .card { border-radius: $card_radius } = 12px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_misc.scss#L201-205 |
| 817 | `Button padding` | latest stable libadwaita 1.10.0: padding: 5px 10px; &.text-button { padding-left/right: 17px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_buttons.scss#L21-L75 |
| 818 | `Entry horizontal padding` | latest stable libadwaita 1.10.0: padding-left: 9px; padding-right: 9px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_entries.scss#L5-6 |
| 819 | `Menu item min-height` | latest stable libadwaita 1.10.0: modelbutton { min-height: 32px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_menus.scss#L134-135 |
| 820 | `Menu item padding` | latest stable libadwaita 1.10.0: padding: 0 $menu_padding; $menu_padding: 12px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_menus.scss#L137 ; https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/st… |
| 821 | `Notebook tab padding` | latest stable libadwaita 1.10.0: padding: 3px 12px | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_notebook.scss#L123 |
| 822 | `Headerbar min-height` | latest stable libadwaita 1.10.0: headerbar { min-height: 47px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_header-bar.scss#L1-2 |
| 832 | `icon-theme default` | latest stable gsettings-desktop-sc…: 'Adwaita' | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L86-87 |
| 839 | `GTK_ICON_SIZE_INHERIT` | latest stable GTK 4.24.1: enum GtkIconSize { INHERIT, NORMAL, LARGE } | https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtkenums.h#L233-238 |
| 840 | `GTK_ICON_SIZE_NORMAL .normal-icons` | latest stable libadwaita 1.10.0: .normal-icons { -gtk-icon-size: 16px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_common.scss#L34-36 |
| 841 | `GTK_ICON_SIZE_LARGE .large-icons` | latest stable libadwaita 1.10.0: .large-icons { -gtk-icon-size: 32px; -gtk-icon-weight: 134 } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/_common.scss#L38-41 |
| 850 | `Text scaling factor key` | latest stable gsettings-desktop-sc…: key text-scaling-factor (d), default 1.0 | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L121-123 |
| 851 | `High contrast key` | latest stable gsettings-desktop-sc…: key high-contrast (b), default false | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.a11y.interface.gschema.xml.in#L4-5 |
| 852 | `Reduce motion source` | latest stable gsettings-desktop-sc…: enable-animations still exists (b, default true); NEW: org.gnome.deskt… | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L11-12 ; https://gitlab.gnome.org/GNOME/… |
| 1030 | `line_height (GNOME, Cantarell pre-48)` | latest stable cantarell-fonts 0.31…: ascender 739, descender -217, unitsPerEm 1000 | https://gitlab.gnome.org/GNOME/cantarell-fonts/-/blob/0.311/src/Cantarell-Regular.ufo/fontinfo.plist#L5-12 |
| 1124 | `reduce_motion (GNOME)` | latest stable gsettings-desktop-sc…: enable-animations (b) default true; plus a11y.interface reduced-motion… | https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L11-12 ; https://gitlab.gnome.org/GNOME/… |
| 1221 | `check_mark_stroke_width (GNOME)` | pinned (cell) libadwaita 1.9.3 (57…: check-symbolic.svg (stroke 2.5 per cell); latest stable libadwaita 1.10.0: check:checked draws assets/check.svg | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/5789add99c79cee0fae624b56706c7c0bea7fb2b/src/stylesheet/assets/check-symbolic.svg |
| 1326 | `item_gap (GNOME)` | pinned (cell) GTK 4.22.5 (bd25f1e2…: anchor += child_allocation.width; SPACING 5; latest stable GTK 4.24.1; libadwai…: anchor += child_allocation.width (surrounding 31 lines identical); #de… | https://gitlab.gnome.org/GNOME/gtk/-/blob/bd25f1e2dc2c2fbf3b8864e61afe642910ba359c/gtk/gtknotebook.c#L5294 |
| 1411 | `border.shadow_enabled (GNOME)` | latest stable libadwaita 1.10.0: three box shadows on > contents | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_popovers.scss#L19-21 |
| 1413 | `border.padding_vertical (GNOME)` | latest stable libadwaita 1.10.0: > contents { padding: 8px } | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_popovers.scss#L13-14 |
| 1533 | `diameter (GNOME)` | latest stable GTK 4.24.1: size = CSS -gtk-icon-size, initial 16px | https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtkspinner.c#L122 ; https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtkcssstylepropertyimpl.c#L1285-L1… |
| 1535 | `stroke_width (GNOME)` | pinned (cell) libadwaita 1.9.3 / G…: -gtk-icon-source: process-working-symbolic + CSS spin animation; latest stable GTK 4.24.1 / libadwa…: GTK draws a GtkSvg paintable '/org/gtk/libgtk/icons/process-working.gp… | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.9.0/src/stylesheet/widgets/_spinner.scss#L1-18 |
| 1553 | `arrow_icon_size (GNOME)` | pinned (cell) libadwaita 1.9.3: arrow { min-height: 16px; min-width: 16px }; latest stable libadwaita 1.10.0: same rule | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/5789add99c79cee0fae624b56706c7c0bea7fb2b/src/stylesheet/widgets/_dropdowns.scss#L10-L14 |
| 1588 | `border.color (GNOME)` | pinned (cell) libadwaita 1.9.3: box-shadow: 0 0 0 1px RGB(0 0 6 / 3%), ...; latest stable libadwaita 1.10.0: identical rule | https://gitlab.gnome.org/GNOME/libadwaita/-/blob/5789add99c79cee0fae624b56706c7c0bea7fb2b/src/stylesheet/widgets/_misc.scss#L194-L201 |
| 1639 | `border.padding_horizontal (GNOME)` | pinned (cell) GTK 4.22.5 (bd25f1e2…: left-margin, right-margin: g_param_spec_int 0..G_MAXINT, default 0; latest stable GTK 4.24.1: same defaults (props[] style) | https://gitlab.gnome.org/GNOME/gtk/-/blob/bd25f1e2dc2c2fbf3b8864e61afe642910ba359c/gtk/gtktextview.c#L1021-L1042 |
| 1640 | `border.padding_vertical (GNOME)` | pinned (cell) GTK 4.22.5: top-margin, bottom-margin default 0; latest stable GTK 4.24.1: same defaults | https://gitlab.gnome.org/GNOME/gtk/-/blob/bd25f1e2dc2c2fbf3b8864e61afe642910ba359c/gtk/gtktextview.c#L1055-L1076 |

</details>

### Researcher notes

Each researcher's own notes, verbatim: coverage, method caveats and errors found outside the *differs* rows (wrong citations on ✅ rows, unpinned links). Step 7 corrects the citation errors (S4) as part of the row fixes.

<details><summary>kde</summary>

Method: partial clones under scratchpad/m2-src/m2e-kde/. For each cited file I compared blob ids at the pinned revision, the latest tag and master, then checked the actual values for the rows that cite no revision. Latest stable means Plasma 6.7.5 and KF 6.31.0; I also checked Plasma 6.8 Beta 2 (v6.7.91).

UNCHANGED cited files (pinned..latest, also identical at master):
- breeze kstyle/breezemetrics.h: unchanged f0b1d753..v6.7.5..v6.7.91 (blob 88118164)
- breeze kstyle/breezehelper.cpp: unchanged f0b1d753..v6.7.5..v6.7.91 (blob 43c47eee)
- breeze kstyle/breezehelper.h: unchanged f0b1d753..v6.7.5..v6.7.91 (blob 9ce055a1)
- breeze kstyle/breezestyle.cpp: unchanged f0b1d753..v6.7.5..v6.7.91 (blob f0d4dfe4)
- The same four files cited at v6.7.5 (lines 1209, 1211, 1218, 1220, 1222): unchanged v6.7.5..v6.7.91.
- breeze colors/BreezeLight.colors and BreezeDark.colors: f0b1d753..v6.7.5..v6.7.91 differ only in Name[..] translation lines; :9-16 ([ColorEffects:Disabled]) is unchanged.
- kcolorscheme src/kcolorscheme.cpp: unchanged 27066d47(v6.30.0)..v6.31.0 (be1c95e1)
- kguiaddons src/colors/kcolorutils.cpp: unchanged 7c766f6f(v6.30.0)..v6.31.0 (24ba00bc)
- kirigami src/controls/Heading.qml: unchanged 8319acc7..v6.31.0 (7ec9bbb5)
- kirigami src/templates/Heading.qml: unchanged 8319acc7..v6.31.0 (61189b00)
- qqc2-desktop-style org.kde.desktop/private/SwitchIndicator.qml: unchanged v6.30.0..v6.31.0 (5db68dda)
- kwidgetsaddons src/kcollapsiblegroupbox.cpp: unchanged v6.30.0..v6.31.0 (d5a3571e)
- frameworkintegration src/kstyle/kstyle.cpp: unchanged 178c522c..v6.31.0 (6b062a17)

STEP 1 CORRECTION: kwidgetsaddons kcollapsiblegroupbox.cpp did not change in v6.31.0. Its last change, f0ea87b2, is before v6.30.0. Step 1's 8445264a and b5f16b7b are the annotated tag objects of v6.30.0 and v6.31.0, not file blobs, and eba96da2 is the master commit. Rows 1613, 1615 and 1616 need no re-read.

RE-PINS with no value change:
- The Breeze citations at f0b1d753 (a master commit that is in no 6.7 tag, first in v6.7.90) can move to v6.7.5. The line numbers stay the same, including BreezeLight.colors:9-16.
- kirigami 8319acc7 and frameworkintegration 178c522c can move to v6.31.0.
- kcolorscheme, kguiaddons, qqc2-desktop-style and kwidgetsaddons can move from v6.30.0 to v6.31.0.

UNPINNED citations in §1.3 that should be pinned. Values checked at the latest tags all match; details are in skipped.
- kfontsettingsdata.cpp: plasma-integration v6.7.5 qt6/src/platformtheme/:44-53
- kcolorscheme.cpp
- Breeze*.colors
- breezemetrics.h
- kicontheme.cpp: v6.31.0 :160-166, :686
- breeze-icons index.theme: generated from icons/commonthemeinfo.theme.in v6.31.0:8-18; the repo has no index.theme
- fonts KCM main.qml and plasma6.0-remove-dpi-settings.cpp: plasma-workspace kcms/fonts/
- kwin.kcfg: kwin src/kwin.kcfg

NEW FINDING (not on todo.md): Plasma 6.8 (6.8.0 due 2026-10-14) removes forceFontDPI.
- plasma-workspace commit c675077b8 (2026-06-10, first in v6.7.90) drops the fonts KCM's DPI UI and kcfg entry.
- Commit fe367323b (2026-06-15) drops krdb, the only reader.
- Xft.dpi now comes from kwinrc [Xwayland] Scale × 96 (startkde/plasma-setup-xwayland/main.cpp v6.7.91:47-58).
- This affects lines 713, 714 and 1123 (reported as rows). native-theme/src/detect.rs:437-444 still reads forceFontDPI first.
- Line 713's cell is still correct for 6.7.5. Once v6.8.0 is tagged, re-check these rows.

PLASMA 6.6 LTS divergence (supported until about 2029-02; only spot-checked):
- breezemetrics.h v6.6.6:156 has a single ItemView_ItemMarginWidth = 3. It lacks ItemMarginLeft/Right 2 and Top/Bottom 1, which affects 664-665 and 1390-1391.
- It also lacks Blend_Value. breezehelper.cpp v6.6.6:39 has highlightBackgroundAlpha = 0.33, which affects 1218; that row mentions only v6.4.0.
- breezestyle.cpp and breezehelper.cpp changed substantially from v6.6.6 to v6.7.5 (346+/163-). Rows citing them were not re-checked for 6.6.

ASIDE (not a currency item; the file has been unchanged since 2024): line 673 says the QQC2 Switch height is fontMetrics.height. SwitchIndicator.qml v6.31.0:14-15 sets implicitHeight to Kirigami.Units.gridUnit, and the track is inset by floor(h/6) at the top and bottom (:44-45).

COVERAGE: all of lines 548-718 and 857-1647 are covered. No line range is left unchecked.

</details>

<details><summary>gnome</summary>

Method: partial clones (git clone --bare --filter=blob:none, 2026-10-11) under scratchpad/m2-src/m2e-gnome/. I compared blobs between each pin and the latest stable tag. §1.4 cells and unlinked Ch.2 GNOME cells name only a file, and appendix lines 1788-1818 cite libadwaita/gtk main. For those I used main@1e097d51 (last main commit before the appendix date, 2026-03-24) and 1.9.0/1.9.3 as the base, and reported §1.4 rows with their 1.10.0 value and a line to pin.

Unchanged cited files:
libadwaita src/stylesheet/widgets/_views.scss: unchanged 5789add9..1.10.0
libadwaita src/stylesheet/widgets/_scale.scss: unchanged 5789add9..1.10.0
libadwaita src/stylesheet/widgets/_notebook.scss: unchanged 5789add9..1.10.0
libadwaita src/stylesheet/widgets/_lists.scss: unchanged 5789add9..1.10.0 (and a5030fc6)
libadwaita src/stylesheet/_colors.scss: unchanged 5789add9..1.10.0 (and a5030fc6)
libadwaita src/adw-expander-row.ui: unchanged 5789add9..1.10.0 (and a5030fc6)
libadwaita src/stylesheet/widgets/_toggle-group.scss: unchanged a5030fc6..1.10.0
libadwaita src/stylesheet/widgets/_deprecated.scss: unchanged a5030fc6..1.10.0
libadwaita src/stylesheet/widgets/_buttons.scss: unchanged a5030fc6..1.10.0
libadwaita src/adw-toggle-group.c: unchanged a5030fc6..1.10.0
(a5030fc6 is 1.10.0+5 on main, so every a5030fc6 cite can be re-pinned to 1.10.0 with no value change.)
libadwaita files pinned at 1.10.0 (_toolbars, _checks, _notebook, _switch, _popovers, _menus, _header-bar, _colors, check.svg, bullet.svg): 1.10.0 is the latest stable tag.
GTK gtk/ui/gtkdropdown.ui: unchanged 57f569a2..4.24.1
GTK gtk/gtksizerequest.c: unchanged 57f569a2..4.24.1
GTK gtk/gtkbutton.c: unchanged 57f569a2..4.24.1
GTK gtk/deprecated/gtkstatusbar.c: unchanged 57f569a2..4.24.1
gsettings-desktop-schemas schemas/org.gnome.desktop.wm.preferences.gschema.xml.in: unchanged master(e8599d1)..51.0
cantarell-fonts src/Cantarell-Regular.ufo/fontinfo.plist: unchanged 0.311..master(e8997b6)

Changed cited files: libadwaita (5789add9..1.10.0) _misc.scss, _dropdowns.scss, adw-tab-box.c and check-symbolic.svg (removed); GTK (bd25f1e2..4.24.1) gtknotebook.c and gtktextview.c; gds interface and a11y.interface schemas. Unlinked sources that also changed: _popovers.scss, _spinner.scss, _sidebars.scss, _toolbars.scss, adw-alert-dialog.c, gtkspinner.c, gtksettings.c.

Findings (none on docs/todo.md):
(1) 728: document-font-name has been 'Adwaita Sans 12' since gds 49.beta (87022e2). It is also 12 in GNOME 49, 50 and 51, so the cell is wrong for every supported GNOME. This confirms Step 1's flag.
(2) 1408/1409: in libadwaita 1.10.0 the popover has no CSS border. Its outline is a box-shadow, 0 0 0 1px RGB(0 0 0 / 5%). In 1.9 it was border: 1px solid RGB(0 0 0 / 14%). The inherited defaults.border cells no longer describe GNOME 51.
(3) 852/1124: GNOME 50+ has org.gnome.desktop.a11y.interface reduced-motion and GtkSettings gtk-interface-reduced-motion (GTK 4.22+). The cells list only enable-animations.
(4) 805: the cell gives gtkspinner.c DEFAULT_SIZE as the source of 16. That macro is defined but unused in 4.22.5 and 4.24.1. The size comes from -gtk-icon-size, initial 16px. The value is right, the reason is not.

Re-pins with no value change: 1326 gtknotebook.c:5294 -> 4.24.1:5236; 1553 _dropdowns.scss:10-14 -> 1.10.0:14-18; 1588 _misc.scss:194-201/:203-207 -> 1.10.0:201-208/:210-214; 1639 gtktextview.c:1021-1042 -> 4.24.1:1053-1070; 1640 :1055-1076 -> 4.24.1:1083-1100. All 5789add9 cites can move to 1.10.0.

Step-1 map: no errors found. libadwaita's latest tags are 1.10.0, 1.9.4 and 1.8.8. GTK's are 4.24.1 and 4.22.5. gds's is 51.0, and its master is frozen at e8599d1 (2025-04-12).

Not covered: the Adwaita Sans/Inter half of 1030 (rsms/inter, not a GNOME repo). §1.4 Pre-48 column (frozen history).

</details>

## M3 — Core, part 1

## M4 — Core, part 2

## M5 — iced connector

## M6 — gpui connector

## M7 — egui connector and egui widgets

## M8 — Cross-cutting

## M9 — Showcases
