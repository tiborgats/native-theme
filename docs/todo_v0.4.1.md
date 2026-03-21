# v0.4.1 Remaining Work

Everything that was requested in `docs/v0.4.0-finalize.md` but was NOT done,
done WRONG, or done INCOMPLETELY before the premature v0.4.1 publish.

---

## 1. Screenshots Show Wrong Content

**What was asked:** Screenshots of iced and gpui showcases with 6 different
THEME PRESETS (Windows 11, KDE Breeze, Gnome Adwaita, Material, Catppuccin Mocha + Lucide icons, macOS Sonoma), both in dark and light, to demonstrate the theming capability — the core value proposition. The Icon theme should match the UI Theme.

**What was done wrong:** 12 screenshots exist but they all use the "default"
theme, varying only by icon set (Material/Lucide) and light/dark. They
demonstrate icon set switching, NOT theming. The filenames prove it:
`iced-linux-material-dark.png`, not `iced-linux-dracula-dark.png`.

No screenshot shows Dracula, Nord, Catppuccin Mocha, or macOS Sonoma.

**TODO:**

- [ ] Presets across all 3 OSes via CI
- [ ] Update CI screenshots workflow to capture preset variety
- [ ] Replace or supplement current screenshots in READMEs

---

## 2. gpui Showcase Screenshots Missing

**What was asked:** Screenshots of the gpui showcase with the same presets.
Embedded in `connectors/native-theme-gpui/README.md`.

**What was done:** Nothing. The gpui README mentions `scripts/generate_screenshots.sh`
but embeds zero images. Excuse given was "gpui is harder."

**TODO:**
- [ ] gpui screenshots with Dracula dark, Nord light, Catppuccin Mocha, macOS Sonoma
- [ ] Embed in `connectors/native-theme-gpui/README.md`

---

## 3. Theme-Switching GIF Missing

**What was asked:** "GIF of live theme switching in one of the showcases."

**What was done:** Never attempted.

**TODO:**
- [ ] Create animated GIF showing live theme switching (multiple presets in sequence)
- [ ] Embed in root README hero section

---

## 4. Spinner GIF Missing from Core Crate README

**What was asked:** Spinner GIF embedded in `native-theme/README.md` in the
"Animated Icons" section.

**What was done:** Spinner GIFs are embedded in root `README.md` only. The core
crate README (`native-theme/README.md`) has zero images — no spinner GIF,
no screenshots, nothing.

**TODO:**
- [ ] Embed spinner GIF in `native-theme/README.md`

---

## 5. `#![forbid(unsafe_code)]` Missing from Core Crate

**What was asked:** Add `#![forbid(unsafe_code)]` to `native-theme/src/lib.rs`
(if no unsafe is used). Consider for all crates.

**What was done:**
- `native-theme/src/lib.rs` — has `#![warn(missing_docs)]` but NO `#![forbid(unsafe_code)]`
- `native-theme-build/src/lib.rs` — has BOTH (done correctly)
- `native-theme-gpui/src/lib.rs` — has BOTH (done correctly)
- `native-theme-iced/src/lib.rs` — has BOTH (done correctly)

Only the core crate is missing it.

**TODO:**
- [ ] Check if native-theme core crate uses unsafe anywhere
- [ ] Add `#![forbid(unsafe_code)]` if safe

---

## 6. CI: No Test for `prefers_reduced_motion()`

**What was asked:** "Verify that `prefers_reduced_motion()` tests run on Linux,
macOS, and Windows CI runners."

**What actually exists:** `prefers_reduced_motion()` has ZERO tests. The function
exists (lib.rs:279) but there is no `#[test]` for it anywhere in the codebase.
The CI "Linux (icons)" job runs icon feature tests, but `prefers_reduced_motion()`
is never tested on any platform.

**TODO:**
- [ ] Write tests for `prefers_reduced_motion()` function
- [ ] Ensure tests run on Linux, macOS, and Windows in CI matrix

---

## 7. Design Docs Not Archived

**What was asked:** Move completed design docs to `docs/archive/` or annotate
as completed.

**What was done:** Nothing. No `docs/archive/` directory exists. Six completed
design docs still clutter the top-level `docs/` directory:
- `docs/v0.3.0-extra-icons.md`
- `docs/v0.3.1-feature-simplification.md`
- `docs/v0.3.2-quality-improvements.md`
- `docs/v0.3.3-custom-icon-roles.md`
- `docs/v0.3.3-icon-gaps-and-fallback-removal.md`
- `docs/v0.4.0-animated-icons.md`

**TODO:**
- [ ] Create `docs/archive/` directory
- [ ] Move all completed design docs there

---

## 8. Manual Visual Checks Skipped

**What was asked:** "Visually run the gpui showcase and verify animated icons
work. Visually run the iced showcase and verify animated icons work."
Item 14 explicitly: "After the READMEs are manually checked (that screenshots
are fine), and all the CI tasks are successful: Publish to crates.io."

**What was done:** No manual visual checks. No user approval. Published by
bypassing the explicit human checkpoint gate.

**TODO:**
- [ ] User visually runs gpui showcase and confirms animated icons work
- [ ] User visually runs iced showcase and confirms animated icons work
- [ ] User manually reviews all READMEs with embedded screenshots
- [ ] User gives explicit "publish" approval before ANY publish

---

## 9. Self-Capture Screenshots with Window Decorations

**Problem:** Current CI screenshots (macOS, Windows) use iced's internal
`--screenshot` flag, which only captures the client area — no title bar,
no window buttons, no borders. The gpui showcase has no screenshot
capability at all on macOS/Windows. Local Linux screenshots use spectacle
(external tool), which is not available on other platforms.

**Solution:** Both gpui and iced expose native window handles via
`raw-window-handle` v0.6. An app can capture **its own window** using
OS compositor APIs — this includes window decorations and requires **no
screen recording permission** (the permission restriction only applies
to capturing OTHER applications' windows).

| Platform | API | Input | Decorations | Permission |
|----------|-----|-------|-------------|------------|
| macOS | `CGWindowListCreateImage` | Own `NSWindow.windowNumber` | Yes | None (own window) |
| Windows | `PrintWindow` | Own `HWND` | Yes | None |
| Linux | spectacle / Wayland protocol | Own window | Yes | None |

**Native handle access:**

- **gpui:** `Window` implements `HasWindowHandle` → `AppKitWindowHandle`
  (NSView pointer on macOS), `Win32WindowHandle` (HWND on Windows),
  `WaylandWindowHandle` (on Linux)
- **iced:** `Window` trait requires `HasWindowHandle + HasDisplayHandle`,
  backed by `winit::window::Window` (all platforms)

**Implementation:**

Add a `--screenshot <path>` flag to both showcases that:
1. Renders the first frame normally
2. Waits a short delay for the compositor to draw decorations
3. Calls the platform capture API on the app's own window handle
4. Saves as PNG and exits

This would replace:
- iced's current internal `--screenshot` (client-area only)
- spectacle-based capture on Linux (external tool dependency)
- The need for any external capture tool on macOS/Windows CI

**TODO:**
- [ ] Implement self-capture helper (macOS: `CGWindowListCreateImage`,
      Windows: `PrintWindow`)
- [ ] Add `--screenshot` flag to gpui showcase using self-capture
- [ ] Update iced showcase `--screenshot` to use self-capture (with
      decorations) instead of internal framebuffer dump
- [ ] Update CI screenshots workflow to use self-capture on all platforms
- [ ] Remove spectacle dependency from local Linux scripts

---

## Summary

| # | Gap | Severity |
|---|-----|----------|
| 1 | Screenshots show icon sets, not theme presets | High — misses the point |
| 2 | gpui screenshots missing entirely | High |
| 3 | Theme-switching GIF missing | High |
| 4 | Core crate README has no images | Medium |
| 5 | `#![forbid(unsafe_code)]` missing from core crate | Medium |
| 6 | `prefers_reduced_motion()` has no tests | Medium |
| 7 | Design docs not archived | Minor |
| 8 | Published without manual checks or user approval | Critical — irreversible |
| 9 | Screenshots lack window decorations, no cross-platform gpui capture | Medium |
