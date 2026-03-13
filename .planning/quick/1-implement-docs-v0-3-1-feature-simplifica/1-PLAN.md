---
phase: quick
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - native-theme/Cargo.toml
  - README.md
autonomous: true
requirements: [FEAT-SIMPLIFY]
must_haves:
  truths:
    - "Enabling `native` on macOS does not pull in ashpd, zbus, configparser, or the windows crate"
    - "Enabling `native` on Linux does not pull in objc2 or the windows crate"
    - "Enabling `native` on Windows does not pull in ashpd, zbus, configparser, or objc2"
    - "`cargo check --features native` succeeds on the current platform"
    - "README documents meta-features (native, linux) prominently and explains which DEs are already covered"
  artifacts:
    - path: "native-theme/Cargo.toml"
      provides: "Target-gated deps and meta-features"
      contains: "native ="
    - path: "README.md"
      provides: "Updated feature documentation"
      contains: "native"
  key_links:
    - from: "native-theme/Cargo.toml features"
      to: "native-theme/Cargo.toml target deps"
      via: "dep:foo references resolve to target-gated optional deps"
      pattern: "native.*linux.*macos.*windows"
---

<objective>
Implement the v0.3.1 feature flag simplification described in docs/v0.3.1-feature-simplification.md.

Purpose: Make it easy for users to enable full native support with a single `features = ["native"]` line, while ensuring OS-specific dependencies only compile on their target platform.

Output: Updated Cargo.toml with target-gated deps and meta-features, updated README with clear feature documentation.
</objective>

<execution_context>
@./.claude/get-shit-done/workflows/execute-plan.md
@./.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@docs/v0.3.1-feature-simplification.md
@native-theme/Cargo.toml
@README.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Target-gate OS-specific deps and add meta-features in Cargo.toml</name>
  <files>native-theme/Cargo.toml</files>
  <action>
In `native-theme/Cargo.toml`, make these changes:

**Move 3 dependencies to target-gated sections:**

1. Remove `ashpd` from `[dependencies]`. Add it under `[target.'cfg(target_os = "linux")'.dependencies]` (which already exists for `freedesktop-icons`):
   ```toml
   ashpd = { version = "0.13.5", optional = true, default-features = false, features = ["settings"] }
   ```

2. Remove `configparser` from `[dependencies]`. Add it under the same `[target.'cfg(target_os = "linux")'.dependencies]` section:
   ```toml
   configparser = { version = "3.1.0", optional = true }
   ```

3. Remove `windows` from `[dependencies]`. Create a new `[target.'cfg(target_os = "windows")'.dependencies]` section:
   ```toml
   [target.'cfg(target_os = "windows")'.dependencies]
   windows = { version = ">=0.59, <=0.62", optional = true, default-features = false, features = [
       "UI_ViewManagement",
       "Win32_UI_WindowsAndMessaging",
       "Win32_UI_HiDpi",
       "Win32_Graphics_Gdi",
       "Win32_UI_Shell",
       "Foundation_Metadata",
   ] }
   ```

**Add meta-features to the `[features]` section:**

Add these 4 lines after the existing features (before `material-icons`):
```toml
linux = ["kde", "portal-tokio"]
linux-async-io = ["kde", "portal-async-io"]
native = ["linux", "macos", "windows"]
native-async-io = ["linux-async-io", "macos", "windows"]
```

Keep all existing features unchanged. The `dep:ashpd`, `dep:configparser`, `dep:windows` references in existing features still work because Cargo resolves them against target-gated optional deps.
  </action>
  <verify>
    <automated>cd /home/tibi/Rust/native-theme && cargo check -p native-theme --features native 2>&1 | tail -5</automated>
  </verify>
  <done>
    - `ashpd`, `configparser` are under `[target.'cfg(target_os = "linux")'.dependencies]`
    - `windows` is under `[target.'cfg(target_os = "windows")'.dependencies]`
    - `[features]` contains `linux`, `linux-async-io`, `native`, `native-async-io`
    - `cargo check --features native` succeeds
  </done>
</task>

<task type="auto">
  <name>Task 2: Update README feature documentation with meta-features and DE coverage explanation</name>
  <files>README.md</files>
  <action>
In `README.md`, update the "Feature Flags" section. Replace the existing feature table with a two-tier table that leads with meta-features, followed by individual features, and add a note about DE coverage.

Replace the current "## Feature Flags" section content (from the heading through the "No features are enabled by default" line) with:

```markdown
## Feature Flags

**Recommended:** Most apps just need one feature:

```toml
[dependencies]
native-theme = { version = "0.3", features = ["native"] }
```

### Meta-features

| Feature | What it enables |
|---------|-----------------|
| `native` | Full native support on all platforms (tokio async runtime) |
| `native-async-io` | Same, but uses async-io instead of tokio |
| `linux` | Full Linux support: KDE + GNOME portal (tokio) |
| `linux-async-io` | Full Linux support: KDE + GNOME portal (async-io) |

All OS-specific dependencies are target-gated, so enabling `native` on macOS
only compiles macOS deps, not Linux or Windows deps.

### Individual features

| Feature | Description | Platform |
|---------|-------------|----------|
| `kde` | Sync KDE theme reader (`~/.config/kdeglobals`) | Linux |
| `portal-tokio` | GNOME portal reader with tokio backend | Linux |
| `portal-async-io` | GNOME portal reader with async-io backend | Linux |
| `macos` | macOS theme reader (NSAppearance) | macOS |
| `windows` | Windows theme reader (UISettings + system metrics) | Windows |
| `system-icons` | Platform icon theme lookup with bundled fallback | All |
| `material-icons` | Bundle Material Symbols SVGs | All |
| `lucide-icons` | Bundle Lucide SVGs | All |
| `svg-rasterize` | SVG-to-RGBA rasterization via resvg | All |

No features are enabled by default. The preset API works without any features.

### Which Linux DEs are supported?

`from_system()` auto-detects the desktop environment via `XDG_CURRENT_DESKTOP`.
GNOME, XFCE, Cinnamon, MATE, Budgie, and LXQt all use GTK themes and are
handled by the Adwaita preset (sync) or the portal reader (async). Only KDE
needs a separate reader because it uses INI-style config files. You do not need
a separate feature flag per desktop environment.
```

Keep the `portal` feature out of the individual features table since it is a base feature not meant to be enabled directly by users (it lacks an async runtime).
  </action>
  <verify>
    <automated>cd /home/tibi/Rust/native-theme && grep -c "native" README.md</automated>
  </verify>
  <done>
    - README "Feature Flags" section leads with recommended one-liner using `native`
    - Meta-features table lists `native`, `native-async-io`, `linux`, `linux-async-io`
    - Individual features table lists all granular features
    - "Which Linux DEs are supported?" subsection explains GNOME/XFCE/etc. coverage
    - Target-gating benefit is mentioned (wrong-OS deps compile away)
  </done>
</task>

</tasks>

<verification>
1. `cargo check -p native-theme --features native` succeeds
2. `cargo check -p native-theme --features linux` succeeds
3. `cargo check -p native-theme --features native-async-io` succeeds
4. `cargo check -p native-theme --features kde` succeeds (individual feature still works)
5. `cargo check -p native-theme` succeeds (no features)
6. README contains meta-features documentation
</verification>

<success_criteria>
- All OS-specific deps (ashpd, configparser, windows) are target-gated in Cargo.toml
- Meta-features (linux, linux-async-io, native, native-async-io) exist and resolve correctly
- All existing features still work unchanged
- README clearly documents meta-features as the recommended approach
- README explains which Linux DEs are covered without extra features
</success_criteria>

<output>
After completion, create `.planning/quick/1-implement-docs-v0-3-1-feature-simplifica/1-SUMMARY.md`
</output>
