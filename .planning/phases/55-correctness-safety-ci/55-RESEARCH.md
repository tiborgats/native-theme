# Phase 55: Correctness, Safety, and CI - Research

**Researched:** 2026-04-07
**Domain:** Bug fixes, safety guards, CI pipeline improvements
**Confidence:** HIGH

## Summary

Phase 55 addresses 10 discrete correctness bugs, safety gaps, and CI deficiencies identified in prior audits. The work spans three categories: (1) detection/platform logic fixes in `native-theme/src/lib.rs` and `native-theme/src/presets.rs`, (2) spinner/animation safety guards in `native-theme/src/spinners.rs` and `native-theme/src/freedesktop.rs`, and (3) CI workflow and script improvements in `.github/workflows/` and `pre-release-check.sh`.

All changes are isolated, non-breaking, and can be executed in parallel. No new dependencies are needed. No library research is required -- this is pure codebase-internal work guided by the existing code patterns.

**Primary recommendation:** Group the 10 requirements into 3 plans by domain (correctness fixes, safety guards, CI improvements) since they share no cross-plan dependencies and can be executed in any order.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| std::process::Command | std | gsettings subprocess with timeout | Already used in gnome/mod.rs line 122 |
| std::time::Duration | std | Timeout parameter for Command | Standard Rust |
| std::fs | std | Reading gtk-3.0/settings.ini | Already used elsewhere in codebase |

### Supporting
No new libraries needed. All changes use existing standard library APIs.

### Alternatives Considered
None -- all requirements use standard Rust patterns already established in the codebase.

## Architecture Patterns

### Pattern 1: gsettings with timeout (CORRECT-01, CORRECT-05)

**What:** `detect_is_dark_inner()` currently calls `gsettings` via `std::process::Command::new("gsettings").args([...]).output()` with no timeout. Two changes needed:
1. Add timeout to prevent indefinite blocking (R-1 / CORRECT-05)
2. Add `GTK_THEME` env var check and `~/.config/gtk-3.0/settings.ini` fallback for non-GNOME/non-KDE desktops (C-1 / CORRECT-01)

**Current code** (`native-theme/src/lib.rs:280-311`):
```rust
fn detect_is_dark_inner() -> bool {
    #[cfg(target_os = "linux")]
    {
        // gsettings works across all modern DEs (GNOME, KDE, XFCE, ...)
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()   // <-- NO TIMEOUT
        // ...
        // KDE kdeglobals fallback
        // ...
        false  // <-- no GTK_THEME or settings.ini fallback
    }
}
```

**What's missing:**
- No `GTK_THEME` env var check (e.g., `GTK_THEME=Adwaita:dark` indicates dark)
- No `~/.config/gtk-3.0/settings.ini` reading (contains `gtk-application-prefer-dark-theme=1`)
- No timeout on `gsettings` subprocess

**Implementation approach:**
1. Before the gsettings call, check `GTK_THEME` env var for `:dark` suffix or `-dark`/`Dark` substring
2. After KDE fallback, read `$XDG_CONFIG_HOME/gtk-3.0/settings.ini` (fallback to `~/.config/gtk-3.0/settings.ini`) and parse `gtk-application-prefer-dark-theme` key
3. For gsettings timeout: spawn the child process, then wait with timeout using `child.wait_timeout()` (Note: `Command::output()` has no built-in timeout. Must use `Command::spawn()` + `child.wait()` with a manual timeout approach, or set a signal alarm.)

**Timeout implementation detail:** `std::process::Child` does not have `wait_timeout()` in stable Rust. Options:
- Use `child.try_wait()` in a polling loop (5 attempts, 200ms each = 1s max) -- simple, no deps
- Use `Command::new().output()` with `timeout` binary: `Command::new("timeout").args(["2", "gsettings", ...])` -- relies on coreutils `timeout` being present
- Recommended: polling `try_wait()` approach since it works on all Linux systems without external dependencies

**Also applies to** `read_gsetting()` in `native-theme/src/gnome/mod.rs:121-135` which also calls `gsettings` without timeout.

### Pattern 2: iOS platform detection (CORRECT-02)

**What:** `detect_platform()` in `native-theme/src/presets.rs:126-146` does not handle `target_os = "ios"`.

**Current code:**
```rust
fn detect_platform() -> &'static str {
    #[cfg(target_os = "macos")]  { return "macos"; }
    #[cfg(target_os = "windows")] { return "windows"; }
    #[cfg(target_os = "linux")]  { ... }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    { "linux" }  // <-- ios falls here, returns "linux" incorrectly
}
```

**Fix:** Add `#[cfg(target_os = "ios")]` block returning `"ios"` before the catch-all. The `PLATFORM_SPECIFIC` table already has `("ios", &["macos", "ios"])` so the preset filtering will work correctly once detect_platform returns "ios".

### Pattern 3: `#[must_use]` message correction (CORRECT-03)

**What:** `into_resolved()` at `native-theme/src/resolve.rs:419` has:
```rust
#[must_use = "this returns the resolved theme; it does not modify self"]
pub fn into_resolved(mut self) -> crate::Result<ResolvedThemeVariant> {
```

The message says "it does not modify self" but `into_resolved` consumes `self` (takes ownership via `mut self`). The message should indicate it consumes self and produces a Result, not that it leaves self unmodified.

**Fix:** Change to `#[must_use = "this consumes the variant and returns the resolved theme"]` or similar.

### Pattern 4: Spinner safety guards (CORRECT-04)

**What:** Four safety issues in spinner/animation code:

**S-1: width/height > 0 check in `rasterize_svg()`**
`native-theme/src/rasterize.rs:40` -- `rasterize_svg()` already handles size=0 via `Pixmap::new(size, size)` returning `None`. But the error message says "invalid rasterization size" without explaining that zero is the issue. This is already safe; the requirement may be about adding an explicit early-return guard for width/height=0 with a clearer error message, or about adding the same guard to `svg_to_bmp_source()` in the gpui connector.

Actually, looking more carefully, the S-1 requirement says "width/height > 0 check" which may refer to the spinner frame rendering in the gpui connector or the SVG generation in spinners.rs, not rasterize.rs. The `svg_to_spin_frames()` function in `spinners.rs` does not check if the SVG is empty or if the viewBox dimensions are valid (width/height could be 0 or negative from a malformed SVG).

**S-3: Empty frames guard**
`AnimatedIcon::Frames` can be constructed with an empty `frames` vec. The `first_frame()` method already returns `None` for empty frames, but consumers (connectors) may not guard against this. The guard should be added where frames are consumed, not at construction time (since `AnimatedIcon` is `#[non_exhaustive]` and public).

Key consumer locations:
- `connectors/native-theme-gpui/src/icons.rs` (frame-based animation rendering)
- `connectors/native-theme-iced/examples/showcase.rs` (animation display)
- `native-theme/src/spinners.rs` `svg_to_spin_frames()` -- should never produce empty vec, but guard is prudent

**S-4: Zero duration guard**
`AnimatedIcon::Frames { frame_duration_ms: 0, .. }` and `TransformAnimation::Spin { duration_ms: 0 }` would cause division-by-zero or infinite-speed animations. Guards should clamp to a minimum (e.g., 1ms or 16ms).

Key location: `with_spin_animation()` in `connectors/native-theme-gpui/src/icons.rs:1002-1012` passes `duration_ms` directly to `Duration::from_millis(duration_ms as u64)`. Zero duration would create an instant-repeat animation.

**S-5: Single-quote viewBox handling**
`svg_to_spin_frames()` in `spinners.rs:46` only handles `viewBox="..."` (double quotes). SVGs with `viewBox='...'` (single quotes) will miss the viewBox extraction and default to `(12.0, 12.0)` center.

The bundled SVGs (material/progress_activity.svg and lucide/loader.svg) both use double quotes, so this is a robustness issue for user-provided or system-provided SVGs. The fix from `freedesktop.rs:101-107` shows the pattern -- it already handles both quote styles.

### Pattern 5: Publish workflow gaps (CI-01 through CI-05)

**CI-01: gpui connector missing from publish CI gate**
`.github/workflows/publish.yml:29-41` -- The CI gate runs clippy and tests for `native-theme`, `native-theme-build`, and `native-theme-iced`, but NOT `native-theme-gpui`. The gpui connector is published (line 78-82) but never tested in the publish workflow.

**CI-02: Error handling in publish steps**
All four publish steps use `continue-on-error: true` (lines 57, 63, 73, 79). This means any publish failure is silently ignored. Should distinguish expected "already published" errors from real failures.

**CI-03: async-io variants not tested in CI**
`.github/workflows/ci.yml:53-60` tests `portal-tokio` but never tests `portal-async-io` or `linux-async-io`. The gpui connector depends on `linux-async-io` (Cargo.toml line 27). CI should test at least one async-io variant.

**CI-04: Example name disambiguation**
Both `native-theme-gpui` and `native-theme-iced` have `[[example]] name = "showcase"`. When running `cargo run --example showcase` in the workspace root, this is ambiguous. Rename to `showcase-gpui` and `showcase-iced`.

**CI-05: pre-release.sh max iteration timeout**
`scripts/pre-release.sh:120-134` has `while true` loop waiting for CI with `sleep 10` but no maximum iteration count. If CI hangs, the script runs forever.

Also, `pre-release-check.sh` iterates over workspace crates (line 236) and has no overall timeout -- but this is less critical since each `cargo check/test/clippy` step has its own timeout via cargo. The main concern is the `scripts/pre-release.sh` CI polling loop.

### Anti-Patterns to Avoid
- **Not using `try_wait()` polling for subprocess timeout**: `Command::output()` blocks indefinitely. Always use spawn + try_wait for external processes that might hang.
- **Not silencing all publish errors**: `continue-on-error: true` hides real failures. Use exit-code-based detection or `--skip-if-version-exists` flags.
- **Modifying example names without updating scripts**: `scripts/generate_screenshots.sh` and `scripts/generate_gpui_screenshots.sh` may reference `showcase` by name.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| INI file parsing | Custom parser | `configparser` crate (already a dep) | gtk-3.0/settings.ini is standard INI format |
| gsettings timeout | Thread-based timeout | `try_wait()` polling loop | Simple, no deps, 10 lines |

**Key insight:** All safety guards are simple bounds checks (3-5 lines each). No complex solutions needed.

## Common Pitfalls

### Pitfall 1: GTK_THEME env var parsing
**What goes wrong:** `GTK_THEME` format is `ThemeName:variant` where variant can be `dark`. But some users set `GTK_THEME=Adwaita-dark` (theme name contains "dark") rather than `GTK_THEME=Adwaita:dark`.
**Why it happens:** GTK supports both conventions.
**How to avoid:** Check for `:dark` suffix first (canonical), then check if the theme name itself contains case-insensitive "dark" as a secondary heuristic.
**Warning signs:** Test with `GTK_THEME=Adwaita:dark`, `GTK_THEME=Adwaita-dark`, `GTK_THEME=SomethingDark`, and `GTK_THEME=Adwaita`.

### Pitfall 2: settings.ini path resolution
**What goes wrong:** Hardcoding `~/.config/gtk-3.0/settings.ini` ignores `XDG_CONFIG_HOME`.
**Why it happens:** `XDG_CONFIG_HOME` defaults to `$HOME/.config` but can be overridden.
**How to avoid:** Use `std::env::var("XDG_CONFIG_HOME")` with fallback to `$HOME/.config`.
**Warning signs:** Tests that set `XDG_CONFIG_HOME` to a temp dir.

### Pitfall 3: Subprocess try_wait polling
**What goes wrong:** `try_wait()` returns `Ok(None)` while the process is still running. If you don't sleep between polls, you burn CPU.
**Why it happens:** Busy-wait loop.
**How to avoid:** Sleep 200ms between polls, max 5 iterations (1s total timeout).
**Warning signs:** High CPU usage in tests.

### Pitfall 4: Example rename breaks scripts
**What goes wrong:** Renaming `showcase` to `showcase-gpui`/`showcase-iced` breaks `scripts/generate_screenshots.sh` and `scripts/generate_gpui_screenshots.sh`.
**Why it happens:** Scripts reference examples by name.
**How to avoid:** Grep all `.sh` files for `showcase` and update references.
**Warning signs:** CI screenshots workflow fails after the rename.

### Pitfall 5: publish CI gate system deps
**What goes wrong:** Adding gpui connector tests to the publish CI gate fails because gpui needs system packages (`libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev`).
**Why it happens:** The main CI already installs these (ci.yml line 91), but publish.yml does not.
**How to avoid:** Copy the system dependency installation step from ci.yml to publish.yml.

## Code Examples

### GTK_THEME dark mode detection
```rust
// Check GTK_THEME env var for dark mode indicators
if let Ok(gtk_theme) = std::env::var("GTK_THEME") {
    // Canonical format: "ThemeName:dark"
    if gtk_theme.to_lowercase().ends_with(":dark") {
        return true;
    }
    // Some themes encode dark in the name: "Adwaita-dark"
    let lower = gtk_theme.to_lowercase();
    if lower.ends_with("-dark") || lower.ends_with("_dark") {
        return true;
    }
}
```

### gtk-3.0/settings.ini dark mode detection
```rust
// Read gtk-3.0/settings.ini for dark-theme preference
fn read_gtk3_settings_dark() -> Option<bool> {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            format!("{home}/.config")
        });
    let path = format!("{config_home}/gtk-3.0/settings.ini");
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(val) = trimmed.strip_prefix("gtk-application-prefer-dark-theme") {
            let val = val.trim_start_matches(|c: char| c == '=' || c.is_whitespace());
            return match val.trim() {
                "1" | "true" => Some(true),
                "0" | "false" => Some(false),
                _ => None,
            };
        }
    }
    None
}
```

### Subprocess with timeout via try_wait
```rust
fn gsettings_with_timeout(schema: &str, key: &str) -> Option<String> {
    let mut child = std::process::Command::new("gsettings")
        .args(["get", schema, key])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    // Poll up to 5 times with 200ms sleep = 1s max
    for _ in 0..5 {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let output = child.wait_with_output().ok()?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                let trimmed = stdout.trim().trim_matches('\'').to_string();
                return if trimmed.is_empty() { None } else { Some(trimmed) };
            }
            Ok(Some(_)) => return None, // Non-zero exit
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(200)),
            Err(_) => return None,
        }
    }
    // Timeout: kill the process
    let _ = child.kill();
    let _ = child.wait();
    None
}
```

**Note on the above:** After `try_wait()` returns `Ok(Some(status))`, the process has already exited and stdout/stderr pipes are closed. We need to collect stdout before the status check. Revised approach: use `spawn()` then `wait_with_output()` -- but that also blocks. Better approach:

```rust
fn gsettings_with_timeout(schema: &str, key: &str) -> Option<String> {
    let mut child = std::process::Command::new("gsettings")
        .args(["get", schema, key])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    // Poll for completion with timeout
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return None;
                }
                break;
            }
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(_) => return None,
        }
    }

    // Process exited successfully, read stdout
    use std::io::Read;
    let mut stdout = String::new();
    child.stdout.take()?.read_to_string(&mut stdout).ok()?;
    let trimmed = stdout.trim().trim_matches('\'').to_string();
    if trimmed.is_empty() { None } else { Some(trimmed) }
}
```

### Single-quote viewBox handling in spinners.rs
```rust
// Current code (double-quote only):
let (cx, cy) = if let Some(vb_start) = svg_tag.find("viewBox=\"") {
    // ...
};

// Fixed code (handles both quote styles, same pattern as freedesktop.rs:101-107):
let (cx, cy) = {
    let (vb_val_start, quote) = if let Some(i) = svg_tag.find("viewBox=\"") {
        (i + 9, '"')
    } else if let Some(i) = svg_tag.find("viewBox='") {
        (i + 9, '\'')
    } else {
        // No viewBox found -- use 24x24 default
        (0, '\0') // sentinel
    };
    if quote == '\0' {
        (12.0, 12.0)
    } else if let Some(vb_end) = svg_str[vb_val_start..].find(quote) {
        let vb = &svg_str[vb_val_start..vb_val_start + vb_end];
        let parts: Vec<f64> = vb.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
        if parts.len() == 4 {
            (parts[0] + parts[2] / 2.0, parts[1] + parts[3] / 2.0)
        } else {
            (12.0, 12.0)
        }
    } else {
        (12.0, 12.0)
    }
};
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `Command::output()` (blocking) | `spawn()` + `try_wait()` polling | Rust stable | Prevents indefinite blocking |
| Double-quote-only viewBox | Both quote styles | SVG spec | Handles more SVG variants |

## Open Questions

1. **gsettings timeout: shared helper vs inline**
   - What we know: Both `detect_is_dark_inner()` (lib.rs) and `read_gsetting()` (gnome/mod.rs) call gsettings without timeout.
   - What's unclear: Should we create one shared helper function, or add timeout to each callsite independently?
   - Recommendation: Create a single `gsettings_with_timeout()` helper in lib.rs (or a utility module) and refactor both callsites to use it. The gnome/mod.rs `read_gsetting()` function is the natural candidate to become the shared helper.

2. **Example rename scope**
   - What we know: Both connectors have `[[example]] name = "showcase"`. CI-04 asks to rename to `showcase-gpui` and `showcase-iced`.
   - What's unclear: Whether this also requires renaming the source files from `showcase.rs` to `showcase-gpui.rs` / `showcase-iced.rs` (Cargo convention: example name must match filename unless overridden with `path =`).
   - Recommendation: Rename both the `name` in Cargo.toml AND the source files, since the name must match the filename. Update all scripts that reference `showcase` by name.

3. **publish.yml error handling strategy**
   - What we know: All four `cargo publish` steps use `continue-on-error: true`.
   - What's unclear: Whether crates.io returns a recognizable exit code for "already published" vs real errors.
   - Recommendation: `cargo publish` exits 0 on success, non-zero on failure. The error message for "already published" contains "already uploaded". Use a shell wrapper that checks the error message and only fails on non-"already uploaded" errors.

## Sources

### Primary (HIGH confidence)
- Direct codebase analysis of all files listed in findings
- `native-theme/src/lib.rs:280-311` -- detect_is_dark_inner current implementation
- `native-theme/src/presets.rs:126-146` -- detect_platform current implementation
- `native-theme/src/resolve.rs:419` -- into_resolved #[must_use] message
- `native-theme/src/spinners.rs:26-80` -- svg_to_spin_frames viewBox parsing
- `native-theme/src/gnome/mod.rs:121-135` -- read_gsetting without timeout
- `native-theme/src/freedesktop.rs:101-107` -- dual-quote viewBox pattern
- `.github/workflows/publish.yml` -- full publish workflow
- `.github/workflows/ci.yml` -- full CI workflow
- `scripts/pre-release.sh:120-134` -- CI polling loop without timeout
- `connectors/native-theme-gpui/Cargo.toml:51` -- example named "showcase"
- `connectors/native-theme-iced/Cargo.toml:19` -- example named "showcase"

### Secondary (MEDIUM confidence)
- GTK_THEME env var format: GTK documentation specifies `THEME:VARIANT` format
- gtk-3.0/settings.ini: Standard freedesktop/GTK configuration file

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All standard library, no new deps
- Architecture: HIGH - All patterns already exist in codebase (e.g., freedesktop.rs dual-quote pattern)
- Pitfalls: HIGH - All identified from direct code analysis

**Research date:** 2026-04-07
**Valid until:** 2026-05-07 (stable -- no external dependencies changing)
