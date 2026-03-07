# Phase 7: Extended Presets - Research

**Researched:** 2026-03-07
**Domain:** TOML preset authoring for platform themes and community color schemes
**Confidence:** HIGH

## Summary

Phase 7 extends the existing preset system (3 presets: default, kde-breeze, adwaita) with 15 additional TOML preset files: 4 platform presets (windows-11, macos-sonoma, material, ios) and 11 community presets (Catppuccin Latte/Frappe/Macchiato/Mocha, Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark). All infrastructure is already in place from Phase 2 -- the `include_str!()` embedding, TOML deserialization into `NativeTheme`, the `preset()`/`list_presets()` API, and the integration test harness that validates every preset. The work is purely additive: author TOML files, add match arms and `include_str!()` constants, update the `PRESET_NAMES` array, and extend the test count assertion.

The primary risk is color accuracy -- each preset must contain correct hex values sourced from official documentation. This research compiles verified color references for all 15 themes from authoritative sources (official repos, design specs, official docs). A secondary consideration is that some themes (Catppuccin Latte, Dracula Alucard, Gruvbox Light, Solarized Light, Tokyo Night Day) are light-only schemes. The TOML structure requires both light and dark variants, but community themes that have no inherent second variant must be paired: Catppuccin Latte (light) pairs with Mocha (dark), Solarized pairs light with dark naturally, etc. This is documented below.

The pattern from Phase 2 is well-proven and requires zero new dependencies. Each new TOML file follows the exact same 150-line structure as the existing presets. The `preset()` match statement grows from 3 arms to 18 arms, which remains efficient and compile-time exhaustive.

**Primary recommendation:** Author 15 new TOML files under `src/presets/`, each with both light and dark variants, accurate colors sourced from official references. Update `presets.rs` with 15 new `include_str!()` constants, 15 new match arms, and the `PRESET_NAMES` array. Existing tests auto-validate all new presets.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PRESET-03 | Additional platform presets: windows-11, macos-sonoma, material, ios | Color references for all 4 platforms documented in Platform Preset Color Reference section; TOML structure identical to existing presets |
| PRESET-04 | Community presets: Catppuccin (4 flavors), Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark | Verified color palettes from official repos for all 11 themes documented in Community Preset Color Reference section |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| toml | 1.x (already in Cargo.toml) | TOML deserialization of preset files | Already used; no changes needed |
| serde | 1.x (already in Cargo.toml) | Derive Serialize/Deserialize for all types | Already used; no changes needed |
| std::include_str!() | stable | Compile-time embedding of TOML preset files | Proven in Phase 2 for 3 presets |

### Supporting
No additional dependencies needed. All required functionality is covered by std and existing Cargo.toml dependencies.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| 18-arm match | HashMap<&str, &str> | Match is compile-time exhaustive, no allocation; 18 arms is still fast and readable |
| Separate TOML per variant | Single TOML with both variants | Current structure (name + [light.*] + [dark.*]) already supports both variants in one file |

**Installation:**
```bash
# No new dependencies needed
```

## Architecture Patterns

### Recommended Project Structure
```
src/
    presets/
        default.toml              # (existing)
        kde-breeze.toml           # (existing)
        adwaita.toml              # (existing)
        windows-11.toml           # NEW - PRESET-03
        macos-sonoma.toml         # NEW - PRESET-03
        material.toml             # NEW - PRESET-03
        ios.toml                  # NEW - PRESET-03
        catppuccin-latte.toml     # NEW - PRESET-04
        catppuccin-frappe.toml    # NEW - PRESET-04
        catppuccin-macchiato.toml # NEW - PRESET-04
        catppuccin-mocha.toml     # NEW - PRESET-04
        nord.toml                 # NEW - PRESET-04
        dracula.toml              # NEW - PRESET-04
        gruvbox.toml              # NEW - PRESET-04
        solarized.toml            # NEW - PRESET-04
        tokyo-night.toml          # NEW - PRESET-04
        one-dark.toml             # NEW - PRESET-04
    presets.rs                    # Updated with 15 new constants + match arms
```

### Pattern 1: Extending the Preset Registry
**What:** Add new `include_str!()` constants, match arms, and PRESET_NAMES entries for each new preset.
**When to use:** For every new preset file.
**Example:**
```rust
// Add 15 new constants after existing ones
const WINDOWS_11_TOML: &str = include_str!("presets/windows-11.toml");
const MACOS_SONOMA_TOML: &str = include_str!("presets/macos-sonoma.toml");
const MATERIAL_TOML: &str = include_str!("presets/material.toml");
const IOS_TOML: &str = include_str!("presets/ios.toml");
const CATPPUCCIN_LATTE_TOML: &str = include_str!("presets/catppuccin-latte.toml");
const CATPPUCCIN_FRAPPE_TOML: &str = include_str!("presets/catppuccin-frappe.toml");
const CATPPUCCIN_MACCHIATO_TOML: &str = include_str!("presets/catppuccin-macchiato.toml");
const CATPPUCCIN_MOCHA_TOML: &str = include_str!("presets/catppuccin-mocha.toml");
const NORD_TOML: &str = include_str!("presets/nord.toml");
const DRACULA_TOML: &str = include_str!("presets/dracula.toml");
const GRUVBOX_TOML: &str = include_str!("presets/gruvbox.toml");
const SOLARIZED_TOML: &str = include_str!("presets/solarized.toml");
const TOKYO_NIGHT_TOML: &str = include_str!("presets/tokyo-night.toml");
const ONE_DARK_TOML: &str = include_str!("presets/one-dark.toml");

// Update PRESET_NAMES to include all 18
const PRESET_NAMES: &[&str] = &[
    "default", "kde-breeze", "adwaita",
    "windows-11", "macos-sonoma", "material", "ios",
    "catppuccin-latte", "catppuccin-frappe", "catppuccin-macchiato", "catppuccin-mocha",
    "nord", "dracula", "gruvbox", "solarized", "tokyo-night", "one-dark",
];

// Extend the match in preset()
pub fn preset(name: &str) -> Result<NativeTheme> {
    let toml_str = match name {
        "default" => DEFAULT_TOML,
        "kde-breeze" => KDE_BREEZE_TOML,
        "adwaita" => ADWAITA_TOML,
        "windows-11" => WINDOWS_11_TOML,
        "macos-sonoma" => MACOS_SONOMA_TOML,
        "material" => MATERIAL_TOML,
        "ios" => IOS_TOML,
        "catppuccin-latte" => CATPPUCCIN_LATTE_TOML,
        "catppuccin-frappe" => CATPPUCCIN_FRAPPE_TOML,
        "catppuccin-macchiato" => CATPPUCCIN_MACCHIATO_TOML,
        "catppuccin-mocha" => CATPPUCCIN_MOCHA_TOML,
        "nord" => NORD_TOML,
        "dracula" => DRACULA_TOML,
        "gruvbox" => GRUVBOX_TOML,
        "solarized" => SOLARIZED_TOML,
        "tokyo-night" => TOKYO_NIGHT_TOML,
        "one-dark" => ONE_DARK_TOML,
        _ => return Err(Error::Unavailable(format!("unknown preset: {name}"))),
    };
    from_toml(toml_str)
}
```

### Pattern 2: Light/Dark Variant Pairing for Community Themes
**What:** Community color schemes are typically dark-only or light-only. Each preset TOML must have both variants. Use the theme's inherent mode as primary and derive/adapt the opposite.
**When to use:** For all community presets.
**Mapping:**

| Preset | Light Source | Dark Source |
|--------|-------------|------------|
| catppuccin-latte | Latte palette (primary) | Latte-adapted dark (use Mocha bg tones with Latte accents) |
| catppuccin-frappe | Frappe-adapted light (use Latte bg tones with Frappe accents) | Frappe palette (primary) |
| catppuccin-macchiato | Macchiato-adapted light (use Latte bg tones with Macchiato accents) | Macchiato palette (primary) |
| catppuccin-mocha | Mocha-adapted light (use Latte bg tones with Mocha accents) | Mocha palette (primary) |
| nord | Snow Storm backgrounds + Frost/Aurora accents | Polar Night backgrounds + Frost/Aurora accents |
| dracula | Alucard (official light variant) | Classic Dracula |
| gruvbox | Gruvbox Light (official) | Gruvbox Dark (official) |
| solarized | Solarized Light (official) | Solarized Dark (official) |
| tokyo-night | Tokyo Night Day (official light) | Tokyo Night Night/Storm (official dark) |
| one-dark | One Light colors (adapted from Atom One Light syntax) | Atom One Dark (official) |

### Pattern 3: Theme Display Names
**What:** Each TOML file's `name` field should use proper display names.
**Mapping:**

| Preset key | `name` field value |
|-----------|-------------------|
| windows-11 | Windows 11 |
| macos-sonoma | macOS Sonoma |
| material | Material |
| ios | iOS |
| catppuccin-latte | Catppuccin Latte |
| catppuccin-frappe | Catppuccin Frappe |
| catppuccin-macchiato | Catppuccin Macchiato |
| catppuccin-mocha | Catppuccin Mocha |
| nord | Nord |
| dracula | Dracula |
| gruvbox | Gruvbox |
| solarized | Solarized |
| tokyo-night | Tokyo Night |
| one-dark | One Dark |

### Anti-Patterns to Avoid
- **Inventing colors for the opposite variant:** Don't hand-pick arbitrary colors. Use official light/dark variants from each theme's documentation, or for Catppuccin use the Latte palette as the light base for all flavors.
- **Using alpha/opacity colors in hex strings:** Pre-compute all alpha colors to solid hex. The Rgba parser handles `#RRGGBBAA` format, but presets should use pre-computed solid colors for visual accuracy. Exception: shadow colors with alpha (e.g., `#00000040`) are fine since they are intentionally semi-transparent.
- **Forgetting to update all three locations:** Every new preset needs: (1) TOML file, (2) `include_str!()` constant, (3) match arm in `preset()`, (4) entry in `PRESET_NAMES`.
- **Copying Phase 2 font values blindly:** Each platform/community preset should have appropriate font families. Platform presets use their native font (Segoe UI, SF Pro, Roboto). Community presets use "sans-serif"/"monospace" since they are not tied to a platform.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Color palette lookup | Custom palette module | TOML preset files with hardcoded hex values | Presets are static data, not computed; TOML is human-readable and editable |
| Theme variant generation | Algorithm to derive dark from light | Author both variants explicitly from official sources | Derived colors are never as good as hand-tuned official values |
| Color accuracy validation | Visual inspection only | Round-trip TOML test + RGB sum dark-is-darker check | Automated tests catch regressions; existing test pattern proven |

**Key insight:** This phase is purely data authoring. No new code patterns, no new abstractions, no new dependencies. The existing Phase 2 infrastructure handles everything.

## Common Pitfalls

### Pitfall 1: Wrong Hex Values from Unofficial Sources
**What goes wrong:** Colors sourced from blog posts or third-party tools don't match the official theme.
**Why it happens:** Community themes have many forks and variations. The "wrong blue" is a common issue.
**How to avoid:** Use ONLY official sources: Catppuccin's catppuccin.com/palette, Nord's nordtheme.com, Dracula's draculatheme.com/spec, Gruvbox's morhetz/gruvbox repo, Solarized's ethanschoonover.com/solarized, Tokyo Night's folke/tokyonight.nvim, Atom's one-dark-syntax repo.
**Warning signs:** Visual comparison looks "off" compared to the theme in a real application.

### Pitfall 2: Forgetting the PRESET_NAMES Array Update
**What goes wrong:** A preset is loadable via `preset("name")` but does not appear in `list_presets()`.
**Why it happens:** Four locations must be updated (TOML file, const, match arm, PRESET_NAMES). The array is easy to forget.
**How to avoid:** The existing test `list_presets_returns_three_entries` must be updated to `list_presets_returns_eighteen_entries`. The test `preset_names_match_list` already verifies every name in the list is loadable.
**Warning signs:** `list_presets().len()` test fails.

### Pitfall 3: Mismatched Color Roles
**What goes wrong:** A community theme's "red" maps to `status.danger` but the theme uses different reds for errors vs. warnings.
**Why it happens:** Community color schemes define raw palette colors, not semantic roles. Mapping palette to semantic roles requires judgment.
**How to avoid:** Follow consistent mapping rules documented below in the Color Mapping Strategy section.
**Warning signs:** Theme looks wrong when used in a real UI context.

### Pitfall 4: Community Themes Missing Light or Dark Variant
**What goes wrong:** A dark-only theme (like One Dark) has no official light counterpart, leaving the light variant empty or poorly adapted.
**Why it happens:** Most community themes only define one mode.
**How to avoid:** Use the variant pairing strategy defined in Pattern 2 above. For themes with an official counterpart (Dracula/Alucard, Gruvbox light/dark, Solarized light/dark), use the official values. For others, create a reasonable adaptation using the theme's accent colors on appropriate light/dark base tones.
**Warning signs:** `theme.light.is_some()` test fails; or dark variant background is lighter than light variant background.

### Pitfall 5: Compile Time Increase
**What goes wrong:** Adding 15 new `include_str!()` calls increases binary size and compile time.
**Why it happens:** Each TOML file is ~2.7KB embedded as a string literal.
**How to avoid:** This is a non-issue. 15 * 2.7KB = ~40KB total additional binary size, which is negligible. Compile time impact is minimal since `include_str!()` just reads files.
**Warning signs:** None expected.

### Pitfall 6: Platform Preset Colors Becoming Stale
**What goes wrong:** Windows 11 or macOS update their system colors, making the preset outdated.
**Why it happens:** Platform themes evolve with OS releases.
**How to avoid:** Document the source OS version in the TOML comment header. The live reader (`from_windows()`, `from_system()`) provides current values; presets are reference baselines. Accept that presets are snapshots.
**Warning signs:** Users report visual mismatch between `preset("windows-11")` and `from_system()`.

## Color Mapping Strategy

Community color schemes define a palette of colors, not semantic UI roles. Use this consistent mapping to convert palette colors to the 36 semantic color roles in ThemeColors:

| Semantic Role | Mapping Strategy |
|--------------|-----------------|
| core.accent | Theme's primary accent/blue |
| core.background | Theme's main background |
| core.foreground | Theme's main text/foreground |
| core.surface | Theme's surface/card/view background (slightly offset from main bg) |
| core.border | Theme's border or muted line color |
| core.muted | Theme's secondary/dimmed text |
| core.shadow | Black with appropriate alpha (e.g. #00000018 light, #00000040 dark) |
| primary.background | Same as accent |
| primary.foreground | White or theme's on-accent color |
| secondary.background | Theme's secondary/muted background |
| secondary.foreground | Appropriate contrast text |
| status.danger | Theme's red |
| status.danger_foreground | White or contrast |
| status.warning | Theme's yellow or orange |
| status.warning_foreground | Dark text or contrast |
| status.success | Theme's green |
| status.success_foreground | White or contrast |
| status.info | Theme's blue or cyan |
| status.info_foreground | White or contrast |
| interactive.selection | Theme's accent/blue |
| interactive.selection_foreground | White or contrast |
| interactive.link | Theme's blue (same or slightly different from accent) |
| interactive.focus_ring | Theme's accent/blue |
| panel.sidebar | Slightly offset from main bg |
| panel.sidebar_foreground | Same as main foreground |
| panel.tooltip | Inverted (dark in light mode, light in dark mode) |
| panel.tooltip_foreground | Inverted foreground |
| panel.popover | Surface or slightly elevated bg |
| panel.popover_foreground | Main foreground |
| component.button | Slightly elevated from bg |
| component.button_foreground | Main foreground |
| component.input | Surface or white (light) / dark surface (dark) |
| component.input_foreground | Main foreground |
| component.disabled | Muted/border color |
| component.separator | Border or slightly lighter/darker |
| component.alternate_row | Slightly offset from main bg |

## Platform Preset Color Reference

### Windows 11 (PRESET-03)
Source: WinUI3 XAML theme resources, Fluent Design System

**Light variant:**
| Role | Hex | Source |
|------|-----|--------|
| accent | #0078d4 | Default Windows accent blue |
| background | #f3f3f3 | SolidBackgroundFillColorBase (light) |
| foreground | #1a1a1a | TextFillColorPrimary pre-computed (#E4000000 = ~90% black on white) |
| surface | #ffffff | CardBackgroundFillColor (light) |
| border | #e5e5e5 | ControlStrokeColorDefault (light) |
| danger | #c42b1c | SystemFillColorCritical |
| success | #0f7b0f | SystemFillColorSuccess |
| warning | #9d5d00 | SystemFillColorCaution |
| info | #0078d4 | SystemFillColorAttention |

**Dark variant:**
| Role | Hex | Source |
|------|-----|--------|
| accent | #0078d4 | Same accent blue |
| background | #202020 | SolidBackgroundFillColorBase (dark) |
| foreground | #ffffff | TextFillColorPrimary (dark) |
| surface | #2d2d2d | CardBackgroundFillColor (dark) |
| border | #454545 | ControlStrokeColorDefault (dark) |

**Font:** Segoe UI, 14px body
**Geometry:** radius 4px (WinUI3 default), frame_width 1px
**Confidence:** MEDIUM -- values based on WinUI3 theme resources documentation plus visual inspection; no single authoritative hex table from Microsoft

### macOS Sonoma (PRESET-03)
Source: Apple Human Interface Guidelines, NSColor system colors

**Light variant:**
| Role | Hex | Source |
|------|-----|--------|
| accent | #007aff | systemBlue (light) |
| background | #f0f0f0 | windowBackgroundColor approximation |
| foreground | #1d1d1f | labelColor (light) |
| surface | #ffffff | controlBackgroundColor |
| border | #d5d5d5 | separatorColor approximation |
| danger | #ff3b30 | systemRed (light) |
| success | #34c759 | systemGreen (light) |
| warning | #ff9500 | systemOrange (light) |
| info | #007aff | systemBlue (light) |

**Dark variant:**
| Role | Hex | Source |
|------|-----|--------|
| accent | #0a84ff | systemBlue (dark) |
| background | #1e1e1e | windowBackgroundColor approximation |
| foreground | #ffffff | labelColor (dark) |
| surface | #2d2d2d | controlBackgroundColor (dark) |
| border | #3d3d3d | separatorColor approximation |
| danger | #ff453a | systemRed (dark) |
| success | #30d158 | systemGreen (dark) |
| warning | #ff9f0a | systemOrange (dark) |
| info | #0a84ff | systemBlue (dark) |

**Font:** SF Pro (system font), 13px body (macOS standard)
**Geometry:** radius 6px, frame_width 1px
**Confidence:** MEDIUM -- NSColor system colors are dynamic and may vary; hex values from iOS UIColor documentation applied to macOS context

### Material Design (PRESET-03)
Source: Material Design 3 baseline color scheme (m3.material.io)

**Light variant (baseline):**
| Role | Hex | Source |
|------|-----|--------|
| accent | #6750a4 | Primary |
| background | #fffbfe | Background |
| foreground | #1c1b1f | onBackground |
| surface | #fffbfe | Surface |
| border | #79747e | Outline |
| danger | #b3261e | Error |
| success | #386a20 | (Material green baseline) |
| warning | #7d5700 | (Material warning baseline) |
| info | #6750a4 | Primary |

**Dark variant (baseline):**
| Role | Hex | Source |
|------|-----|--------|
| accent | #d0bcff | Primary (dark) |
| background | #1c1b1f | Background (dark) |
| foreground | #e6e1e5 | onBackground (dark) |
| surface | #1c1b1e | Surface (dark) |
| border | #938f99 | Outline (dark) |
| danger | #f2b8b5 | Error (dark) |

**Font:** Roboto, 14px body
**Geometry:** radius 12px (M3 full shape), frame_width 1px
**Confidence:** HIGH -- from official m3.material.io baseline scheme

### iOS (PRESET-03)
Source: Apple UIColor system colors, Human Interface Guidelines

**Light variant:**
| Role | Hex | Source |
|------|-----|--------|
| accent | #007aff | systemBlue (light) |
| background | #ffffff | systemBackground (light) |
| foreground | #000000 | label (light) |
| surface | #f2f2f7 | secondarySystemBackground (light) |
| border | #c6c6c8 | separator approximation |
| muted | #8e8e93 | secondaryLabel pre-computed |
| danger | #ff3b30 | systemRed (light) |
| success | #34c759 | systemGreen (light) |
| warning | #ff9500 | systemOrange (light) |
| info | #007aff | systemBlue (light) |

**Dark variant:**
| Role | Hex | Source |
|------|-----|--------|
| accent | #0a84ff | systemBlue (dark) |
| background | #000000 | systemBackground (dark) |
| foreground | #ffffff | label (dark) |
| surface | #1c1c1e | secondarySystemBackground (dark) |
| border | #38383a | separator (dark) |
| muted | #8e8e93 | secondaryLabel pre-computed |
| danger | #ff453a | systemRed (dark) |
| success | #30d158 | systemGreen (dark) |
| warning | #ff9f0a | systemOrange (dark) |
| info | #0a84ff | systemBlue (dark) |

**Font:** SF Pro (system font), 17px body (iOS standard)
**Geometry:** radius 10px (iOS card corners), frame_width 0.5px (retina hairline)
**Confidence:** HIGH -- from UIColor system color hex dumps verified against Apple documentation

## Community Preset Color Reference

### Catppuccin (PRESET-04)
Source: catppuccin.com/palette (official)
**Confidence:** HIGH -- values fetched directly from official palette page

**Latte (light theme):**
| Role | Color Name | Hex |
|------|-----------|-----|
| background (base) | Base | #eff1f5 |
| surface (mantle) | Mantle | #e6e9ef |
| foreground (text) | Text | #4c4f69 |
| muted (subtext0) | Subtext 0 | #6c6f85 |
| border (surface1) | Surface 1 | #bcc0cc |
| accent (blue) | Blue | #1e66f5 |
| red | Red | #d20f39 |
| green | Green | #40a02b |
| yellow | Yellow | #df8e1d |
| orange (peach) | Peach | #fe640b |
| pink | Pink | #ea76cb |
| purple (mauve) | Mauve | #8839ef |
| cyan (teal) | Teal | #179299 |
| sky | Sky | #04a5e5 |
| sapphire | Sapphire | #209fb5 |
| lavender | Lavender | #7287fd |

**Frappe (dark theme):**
| Role | Hex |
|------|-----|
| base | #303446 |
| mantle | #292c3c |
| crust | #232634 |
| text | #c6d0f5 |
| subtext0 | #a5adce |
| surface0 | #414559 |
| surface1 | #51576d |
| surface2 | #626880 |
| overlay0 | #737994 |
| blue | #8caaee |
| red | #e78284 |
| green | #a6d189 |
| yellow | #e5c890 |
| peach | #ef9f76 |
| mauve | #ca9ee6 |
| teal | #81c8be |
| lavender | #babbf1 |

**Macchiato (dark theme):**
| Role | Hex |
|------|-----|
| base | #24273a |
| mantle | #1e2030 |
| crust | #181926 |
| text | #cad3f5 |
| subtext0 | #a5adcb |
| surface0 | #363a4f |
| surface1 | #494d64 |
| blue | #8aadf4 |
| red | #ed8796 |
| green | #a6da95 |
| yellow | #eed49f |
| peach | #f5a97f |
| mauve | #c6a0f6 |
| teal | #8bd5ca |
| lavender | #b7bdf8 |

**Mocha (dark theme):**
| Role | Hex |
|------|-----|
| base | #1e1e2e |
| mantle | #181825 |
| crust | #11111b |
| text | #cdd6f4 |
| subtext0 | #a6adc8 |
| surface0 | #313244 |
| surface1 | #45475a |
| surface2 | #585b70 |
| overlay0 | #6c7086 |
| blue | #89b4fa |
| red | #f38ba8 |
| green | #a6e3a1 |
| yellow | #f9e2af |
| peach | #fab387 |
| mauve | #cba6f7 |
| teal | #94e2d5 |
| lavender | #b4befe |

**Catppuccin preset pairing strategy:**
- `catppuccin-latte`: light = Latte palette, dark = derive from Frappe (closest dark companion to Latte)
- `catppuccin-frappe`: light = derive from Latte, dark = Frappe palette
- `catppuccin-macchiato`: light = derive from Latte, dark = Macchiato palette
- `catppuccin-mocha`: light = derive from Latte, dark = Mocha palette

For derived variants: use the target flavor's accent colors on the Latte (for light) or Mocha (for dark) base/surface/text tones.

### Nord (PRESET-04)
Source: nordtheme.com/docs/colors-and-palettes (official)
**Confidence:** HIGH

| Group | Name | Hex |
|-------|------|-----|
| Polar Night | nord0 | #2e3440 |
| Polar Night | nord1 | #3b4252 |
| Polar Night | nord2 | #434c5e |
| Polar Night | nord3 | #4c566a |
| Snow Storm | nord4 | #d8dee9 |
| Snow Storm | nord5 | #e5e9f0 |
| Snow Storm | nord6 | #eceff4 |
| Frost | nord7 | #8fbcbb |
| Frost | nord8 | #88c0d0 |
| Frost | nord9 | #81a1c1 |
| Frost | nord10 | #5e81ac |
| Aurora | nord11 | #bf616a |
| Aurora | nord12 | #d08770 |
| Aurora | nord13 | #ebcb8b |
| Aurora | nord14 | #a3be8c |
| Aurora | nord15 | #b48ead |

**Mapping:** Dark: bg=nord0, surface=nord1, border=nord3, fg=nord4, accent=nord8. Light: bg=nord6, surface=nord5, border=nord4, fg=nord0, accent=nord10.

### Dracula (PRESET-04)
Source: draculatheme.com/spec (official)
**Confidence:** HIGH

**Classic (dark):**
| Role | Hex |
|------|-----|
| background | #282a36 |
| foreground | #f8f8f2 |
| current_line / surface | #44475a |
| comment / muted | #6272a4 |
| cyan | #8be9fd |
| green | #50fa7b |
| orange | #ffb86c |
| pink | #ff79c6 |
| purple | #bd93f9 |
| red | #ff5555 |
| yellow | #f1fa8c |

**Alucard (light):**
| Role | Hex |
|------|-----|
| background | #fffbeb |
| foreground | #1f1f1f |
| current_line / surface | #cfcfde |
| comment / muted | #6c664b |
| cyan | #036a96 |
| green | #14710a |
| orange | #a34d14 |
| pink | #a3144d |
| purple | #644ac9 |
| red | #cb3a2a |
| yellow | #846e15 |

### Gruvbox (PRESET-04)
Source: github.com/morhetz/gruvbox (official)
**Confidence:** HIGH

**Dark (medium contrast):**
| Role | Hex |
|------|-----|
| bg (dark0) | #282828 |
| bg1 (dark1) | #3c3836 |
| bg2 (dark2) | #504945 |
| fg (light1) | #ebdbb2 |
| fg muted (light4) | #a89984 |
| gray | #928374 |
| red (bright) | #fb4934 |
| green (bright) | #b8bb26 |
| yellow (bright) | #fabd2f |
| blue (bright) | #83a598 |
| purple (bright) | #d3869b |
| aqua (bright) | #8ec07c |
| orange (bright) | #fe8019 |

**Light (medium contrast):**
| Role | Hex |
|------|-----|
| bg (light0) | #fbf1c7 |
| bg1 (light1) | #ebdbb2 |
| bg2 (light2) | #d5c4a1 |
| fg (dark1) | #3c3836 |
| fg muted (dark4) | #7c6f64 |
| gray | #928374 |
| red (faded) | #9d0006 |
| green (faded) | #79740e |
| yellow (faded) | #b57614 |
| blue (faded) | #076678 |
| purple (faded) | #8f3f71 |
| aqua (faded) | #427b58 |
| orange (faded) | #af3a03 |

### Solarized (PRESET-04)
Source: ethanschoonover.com/solarized (official)
**Confidence:** HIGH

| Name | Hex | Usage |
|------|-----|-------|
| base03 | #002b36 | Dark bg |
| base02 | #073642 | Dark surface |
| base01 | #586e75 | Content tone (dark emphasis) |
| base00 | #657b83 | Content tone (light emphasis) |
| base0 | #839496 | Content tone (dark body) |
| base1 | #93a1a1 | Content tone (light emphasis) |
| base2 | #eee8d5 | Light surface |
| base3 | #fdf6e3 | Light bg |
| yellow | #b58900 | Accent |
| orange | #cb4b16 | Accent |
| red | #dc322f | Accent |
| magenta | #d33682 | Accent |
| violet | #6c71c4 | Accent |
| blue | #268bd2 | Accent |
| cyan | #2aa198 | Accent |
| green | #859900 | Accent |

**Mapping:** Dark: bg=base03, surface=base02, fg=base0, muted=base01, accent=blue. Light: bg=base3, surface=base2, fg=base00, muted=base1, accent=blue.

### Tokyo Night (PRESET-04)
Source: github.com/folke/tokyonight.nvim (official)
**Confidence:** MEDIUM -- values from DeepWiki analysis of official repo

**Night (dark):**
| Role | Hex |
|------|-----|
| background | #1a1b26 |
| surface | #16161e |
| foreground | #a9b1d6 |
| fg_bright | #c0caf5 |
| comment/muted | #565f89 |
| blue | #7aa2f7 |
| cyan | #7dcfff |
| green | #9ece6a |
| orange | #ff9e64 |
| purple | #bb9af7 |
| red | #f7768e |
| yellow | #e0af68 |
| teal | #73daca |

**Day (light):**
| Role | Hex |
|------|-----|
| background | #e1e2e7 |
| surface | #d0d5e3 |
| foreground | #3760bf |
| comment/muted | #848cb5 |
| blue | #2e7de9 |
| cyan | #007197 |
| green | #587539 |
| red | #c64343 |
| orange | #b15c00 |
| purple | #7847bd |
| yellow | #8c6c3e |

### One Dark (PRESET-04)
Source: github.com/atom/one-dark-syntax (official Atom repo)
**Confidence:** HIGH

**Dark:**
| Role | Variable | Hex |
|------|----------|-----|
| background | syntax-bg | #282c34 |
| foreground | mono-1 | #abb2bf |
| muted | mono-3 | #5c6370 |
| cyan | hue-1 | #56b6c2 |
| blue | hue-2 | #61aeee |
| purple | hue-3 | #c678dd |
| green | hue-4 | #98c379 |
| red | hue-5 | #e06c75 |
| dark_red | hue-5-2 | #be5046 |
| orange | hue-6 | #d19a66 |
| yellow | hue-6-2 | #e6c07b |

**Light (from Atom One Light syntax):**
| Role | Hex |
|------|-----|
| background | #fafafa |
| foreground | #383a42 |
| muted | #a0a1a7 |
| cyan | #0184bc |
| blue | #4078f2 |
| purple | #a626a4 |
| green | #50a14f |
| red | #e45649 |
| orange | #c18401 |
| yellow | #986801 |

## Code Examples

### Example TOML File: Catppuccin Mocha
```toml
# Catppuccin Mocha - darkest flavor of the Catppuccin palette
# Source: https://catppuccin.com/palette/
name = "Catppuccin Mocha"

# === Light variant (derived from Latte base with Mocha accents) ===

[light.colors.core]
accent = "#89b4fa"
background = "#eff1f5"
foreground = "#4c4f69"
surface = "#e6e9ef"
border = "#bcc0cc"
muted = "#6c6f85"
shadow = "#00000018"

# ... (remaining sections follow same pattern)

[light.fonts]
family = "sans-serif"
size = 10.0
mono_family = "monospace"
mono_size = 10.0

# === Dark variant (Mocha palette) ===

[dark.colors.core]
accent = "#89b4fa"
background = "#1e1e2e"
foreground = "#cdd6f4"
surface = "#181825"
border = "#45475a"
muted = "#a6adc8"
shadow = "#00000040"

# ... (remaining sections)

[dark.fonts]
family = "sans-serif"
size = 10.0
mono_family = "monospace"
mono_size = 10.0
```

### Test Updates Required
```rust
// In tests/preset_loading.rs, update:
#[test]
fn list_presets_returns_eighteen_entries() {
    assert_eq!(
        list_presets().len(),
        18,
        "expected 18 presets, got {}",
        list_presets().len()
    );
}

// In src/presets.rs unit tests, update:
#[test]
fn list_presets_returns_all_eighteen() {
    let names = list_presets();
    assert_eq!(names.len(), 18);
    // Existing 3
    assert!(names.contains(&"default"));
    assert!(names.contains(&"kde-breeze"));
    assert!(names.contains(&"adwaita"));
    // Platform presets
    assert!(names.contains(&"windows-11"));
    assert!(names.contains(&"macos-sonoma"));
    assert!(names.contains(&"material"));
    assert!(names.contains(&"ios"));
    // Community presets
    assert!(names.contains(&"catppuccin-latte"));
    assert!(names.contains(&"catppuccin-frappe"));
    assert!(names.contains(&"catppuccin-macchiato"));
    assert!(names.contains(&"catppuccin-mocha"));
    assert!(names.contains(&"nord"));
    assert!(names.contains(&"dracula"));
    assert!(names.contains(&"gruvbox"));
    assert!(names.contains(&"solarized"));
    assert!(names.contains(&"tokyo-night"));
    assert!(names.contains(&"one-dark"));
}
```

### Existing Tests That Auto-Cover New Presets
These existing tests iterate over `list_presets()` and will automatically validate all new presets once PRESET_NAMES is updated:
- `all_presets_parse_without_error` -- verifies TOML is valid
- `all_presets_have_both_variants` -- verifies light + dark exist
- `all_presets_have_core_colors` -- verifies accent, background, foreground populated
- `all_presets_have_status_colors` -- verifies danger, warning, success populated
- `all_presets_have_interactive_colors` -- verifies selection, link populated
- `all_presets_have_valid_fonts` -- verifies font family + size > 0
- `all_presets_have_geometry` -- verifies radius exists
- `all_presets_have_spacing` -- verifies spacing.m exists
- `all_presets_round_trip_toml` -- verifies TOML round-trip fidelity
- `dark_backgrounds_are_darker` -- verifies dark bg RGB sum < light bg RGB sum

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Catppuccin 4 HSL-only flavors | Catppuccin hex values on catppuccin.com/palette | 2023+ | Direct hex values available, no HSL conversion needed |
| Dracula dark-only | Dracula Alucard light variant | 2023 | Official light variant (Alucard) now available on draculatheme.com |
| Material Design 2 colors | Material Design 3 baseline scheme | 2022 | New role names (primary, secondary, tertiary, surface, outline) |
| Windows 10 accent colors | Windows 11 Fluent Design / WinUI3 | 2021 | New color ramp with Fill/Stroke/Background semantic names |
| macOS individual NSColor lookups | macOS Sonoma system colors | 2023 | System colors remain dynamic; hex approximations documented |

**Deprecated/outdated:**
- Material Design 2 color system: Replaced by Material Design 3 with tonal palettes
- Atom editor itself: Archived, but One Dark color scheme lives on in VS Code and neovim ports

## Open Questions

1. **Catppuccin light variant derivation strategy**
   - What we know: Catppuccin officially has Latte (light) and Frappe/Macchiato/Mocha (dark). Each flavor has its own accent colors.
   - What's unclear: For `catppuccin-mocha` preset, should the light variant use Latte's accent colors or Mocha's accent colors on Latte base tones?
   - Recommendation: Use each flavor's own accent colors on Latte base tones for the light variant. This preserves the unique character of each flavor while providing usable light mode.

2. **Windows 11 exact background hex values**
   - What we know: WinUI3 uses theme resources like SolidBackgroundFillColorBase but exact hex values are in XAML resource files, not publicly documented as a simple table.
   - What's unclear: The precise hex values for all semantic colors.
   - Recommendation: Use well-known values (#f3f3f3 light, #202020 dark) that are widely documented in community references. These are close enough for a preset baseline. The live `from_windows()` reader provides exact runtime values.

3. **Tokyo Night Day variant completeness**
   - What we know: Tokyo Night Day has limited color definitions compared to Night.
   - What's unclear: Some semantic roles may need to be inferred.
   - Recommendation: Use the available Day palette values, supplementing missing roles with reasonable derivations from the base palette.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test (cargo test) |
| Config file | Cargo.toml (already configured) |
| Quick run command | `cargo test --lib presets` |
| Full suite command | `cargo test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PRESET-03 | Platform presets parse into NativeTheme with light+dark | integration | `cargo test --test preset_loading` | Yes (auto-covers via list_presets iteration) |
| PRESET-03 | Platform presets have non-empty core colors | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-03 | Platform presets round-trip TOML | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-04 | Community presets parse into NativeTheme with light+dark | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-04 | Community presets have non-empty core colors | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-04 | Community presets round-trip TOML | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-03/04 | list_presets() returns all 18 names | unit+integration | `cargo test presets_returns` | Yes (needs count update from 3 to 18) |
| PRESET-03/04 | Dark backgrounds darker than light | integration | `cargo test dark_backgrounds_are_darker` | Yes (auto-covers) |

### Sampling Rate
- **Per task commit:** `cargo test`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] 15 new TOML files under `src/presets/` -- each ~150 lines, both light+dark variants
- [ ] `src/presets.rs` -- updated with 15 new constants, match arms, PRESET_NAMES entries
- [ ] `tests/preset_loading.rs` -- updated `list_presets_returns_three_entries` -> `list_presets_returns_eighteen_entries`
- [ ] `src/presets.rs` unit tests -- updated `list_presets_returns_all_three` -> `list_presets_returns_all_eighteen`, `presets_have_correct_names` expanded

## Sources

### Primary (HIGH confidence)
- [Catppuccin palette](https://catppuccin.com/palette/) - All 4 flavor hex values
- [Nord theme](https://www.nordtheme.com/docs/colors-and-palettes/) - All 16 color hex values
- [Dracula spec](https://draculatheme.com/spec) - Classic + Alucard hex values
- [Gruvbox](https://github.com/morhetz/gruvbox) via DeepWiki - Full palette with all variants
- [Solarized](https://ethanschoonover.com/solarized/) - All 16 color hex values
- [Atom One Dark](https://github.com/atom/one-dark-syntax) - Official syntax theme colors
- [iOS UIColor hex values](https://noahgilmore.com/blog/dark-mode-uicolor-compatibility) - Light+dark system color hex codes
- [Material Design 3 roles](https://m3.material.io/styles/color/roles) - Baseline scheme values

### Secondary (MEDIUM confidence)
- [Tokyo Night](https://github.com/folke/tokyonight.nvim) via DeepWiki - Night+Day variant colors
- [Windows 11 theme resources](https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-theme-resources) - TextFillColorPrimary values; semantic resource names
- [macOS system colors](https://developer.apple.com/documentation/uikit/standard-colors) - Dynamic color documentation

### Tertiary (LOW confidence)
- Windows 11 exact background/surface hex values (#f3f3f3, #202020) -- widely cited but not in a single authoritative Microsoft document
- macOS window background approximations (#f0f0f0, #1e1e1e) -- NSColor values are dynamic
- Tokyo Night Day variant completeness -- some colors inferred from limited documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Zero new dependencies; exact same pattern as Phase 2
- Architecture: HIGH - Purely additive; 15 new TOML files + match arm expansion
- Community theme colors: HIGH - Official sources for all 7 community themes verified
- Platform theme colors: MEDIUM - Platform colors are dynamic/approximate; presets are reasonable baselines
- Pitfalls: HIGH - Well-understood failure modes inherited from Phase 2

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (stable domain, community theme palettes rarely change)
