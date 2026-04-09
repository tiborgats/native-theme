# GTK Symbolic Icon Recoloring

## 1. Problem Description

Adwaita icons are always dark-colored, even on dark themes where they should be light-colored for visibility. The problem affects all GTK-convention icon themes (Adwaita, Yaru, elementary, etc.), not just Adwaita specifically.

### Root Cause

GTK symbolic icons use a **hardcoded placeholder palette** that GTK's rendering pipeline replaces at paint time. When `native-theme` loads these SVGs from disk (`freedesktop.rs:69-70`), it returns the raw bytes with no recoloring. The icons retain their dark placeholder colors regardless of theme variant.

### Color Conventions by Theme Family

| Theme family | Color mechanism | Dark variant handling |
|---|---|---|
| **Breeze / KDE** | CSS `.ColorScheme-Text { color:#232629 }` + `fill:currentColor` | Separate `breeze-dark/` directory with `color:#fcfcfc` |
| **Adwaita / GTK** | Hardcoded `fill="#2e3436"` | **No dark directory** -- GTK recolors at runtime |

### Adwaita Palette (measured from `/usr/share/icons/Adwaita/symbolic/`)

| Color | Occurrences (fill attrs) | Role | Usage |
|---|---|---|---|
| `#2e3436` | 483 | Primary foreground | Main icon shapes. Also 8 icons via CSS `style="fill:#2e3436"`, 1 as `stroke="#2e3436"` |
| `#2e3434` | 108 (in 118 files) | Foreground variant | 68 files as primary fill (no opacity), 50 files with `fill-opacity="0.34902"` for dimmed elements |
| `#222222` | 27 | Foreground variant | Used both as primary fill and with `fill-opacity` for dimmed elements |
| `#474747` | 50 | Foreground (emotes/legacy) | Sole fill in 50 emote/legacy icons. Never mixed with `#2e3436` |
| `#33d17a` | 12 | Success (semantic) | Green badges, checkmarks |
| `#ff7800` | 9 | Warning (semantic) | Orange indicators |
| `#e01b24` | 5 | Error (semantic) | Red indicators |
| `#ed333b` | 4 | Error variant (semantic) | Red indicators |

**Key observations:**
- Zero Adwaita symbolic icons use `currentColor` (0 out of ~600+)
- The GTK4 documented recoloring palette defines `#2e3436` (Tango Aluminium 6) as the foreground placeholder
- `#2e3434` is interchangeable with `#2e3436`: off-by-2 in last hex digit, used identically as primary foreground in 68 icons and with fill-opacity in 50 icons
- `#222222` follows the same pattern (foreground + opacity for dimmed elements)
- `#474747` icons are exclusively emotes/legacy (50 files), never mixed with other foreground colors -- monochrome symbolic icons that would be invisible on dark backgrounds
- 8 icons use CSS style-attribute fills (`style="fill:#2e3436;..."`) instead of XML attributes
- 1 icon uses `stroke="#2e3436"` (night-light-disabled-symbolic.svg)
- Semantic colors (`#33d17a`, `#ff7800`, `#e01b24`) should NOT be replaced -- they are intentionally colored
- All hex values are lowercase (no uppercase variants found)

### Multi-color example (`mail-mark-important-symbolic.svg`)

```xml
<path fill="#2e3436"/>      <!-- envelope: should be recolored to foreground -->
<path fill="#ff7800"/>      <!-- badge: should stay orange (semantic warning) -->
```

### Secondary element example (`camera-switch-symbolic.svg`)

```xml
<path fill="#2e3436"/>                              <!-- main shape: recolor to fg -->
<path fill="#2e3434" fill-opacity="0.34902"/>       <!-- dimmed shape: recolor to fg, keep opacity -->
```

### Why Breeze works without recoloring

Breeze uses `fill:currentColor` in the SVG + CSS defining the color. Separate `breeze/` and `breeze-dark/` directories contain different CSS colors. The freedesktop-icons lookup resolves to the correct directory based on the active icon theme name. The SVGs are already correctly colored for their theme variant.

### Why this matters

A "native theme" crate that can't display readable icons on GNOME dark mode is fundamentally broken. Adwaita is the default icon theme on the most popular Linux desktop.


## 2. Solution Options

### Option A: Normalize GTK symbolics to `currentColor` in `freedesktop.rs`

After loading a `-symbolic` SVG that doesn't already contain `currentColor`, replace the GTK foreground placeholder colors with `currentColor`. The existing connector colorize logic (which already handles `currentColor`) then replaces it with the correct foreground color.

### Option B: Extend connector colorize functions to handle GTK palette

Add `#2e3436`, `#2e3434`, `#222222` to the list of colors replaced by `colorize_monochrome_svg()` in each connector (alongside `black`, `#000000`, `#000`).

### Option C: Add `fg_color` parameter to load functions

Change `load_freedesktop_icon(role, size)` to `load_freedesktop_icon(role, size, fg_color: Option<[u8; 3]>)`. If `fg_color` is `Some`, replace GTK palette colors in symbolic SVGs before returning.

### Option D: New `IconData::SymbolicSvg` variant

Add `IconData::SymbolicSvg(Vec<u8>)` to signal that the SVG needs recoloring. Consumers match on the variant and apply colorization.

### Option E: Inject CSS override into SVG

Prepend a `<defs><style>` block that sets `path { fill: currentColor !important; }` to override inline fills.

### Option F: Look for dark-variant theme directory

When the theme is dark, try loading from `{theme}-dark` before `{theme}`.

### Option G: Document the limitation, no code change

Document that GTK-convention icon themes need manual recoloring by the consumer.


## 3. Analysis of Each Option

### Option A: Normalize to `currentColor` in `freedesktop.rs`

**Pros:**
- Fix at the source -- every consumer benefits automatically
- No API changes to `load_freedesktop_icon` or `IconData`
- Connectors' existing `colorize_monochrome_svg` already handles `currentColor`
- `fill-opacity` attributes are preserved (they're separate from the fill value)
- Works for ALL GTK-convention themes, not just Adwaita
- Self-describing output: connectors seeing `currentColor` know to colorize
- The `find_icon` function already tries `-symbolic` first, so we know which variant matched
- Semantic colors (`#33d17a`, `#ff7800`, `#e01b24`) are untouched (only foreground is replaced)

**Cons:**
- Modifies SVG bytes in the core crate (bytes returned differ from bytes on disk)
- Direct users who render SVGs without any colorization get `currentColor` instead of `#2e3436`. Most SVG renderers default `currentColor` to black, so visually similar -- but technically different
- Requires minor refactor of `find_icon` to return whether `-symbolic` variant was matched

### Option B: Extend connector colorize functions

**Pros:**
- Minimal code change per connector (~3 lines each)
- No core crate changes

**Cons:**
- Every connector needs the same fix independently (currently 2: iced, gpui; future connectors too)
- Direct users of the core crate (no connector) still get dark icons -- the fix is invisible to them
- Replacing `#2e3436` in ALL SVGs is dangerous: non-symbolic freedesktop icons could legitimately use that color. At the connector level, there is no way to distinguish symbolic from non-symbolic
- The connector's `colorize_monochrome_svg` is documented for monochrome icons only; system icons are expected to pass `color=None` and be rendered as-is
- Violates the principle that the core crate should return correct, usable data

### Option C: Add `fg_color` parameter

**Pros:**
- Explicit control over the replacement color
- No surprises about modified SVG bytes

**Cons:**
- Breaking API change on `load_freedesktop_icon`, `load_freedesktop_icon_by_name`, `load_system_icon_by_name`
- Caller must already know the foreground color (chicken-and-egg: you load icons as part of theming, but need the theme to colorize icons)
- Adds complexity for ALL callers, even those using Breeze (which doesn't need recoloring)
- Still needs the `find_icon` refactor to know whether to apply recoloring (symbolic vs. non-symbolic)
- Connectors would need to thread the color through multiple layers of function calls

### Option D: New `IconData::SymbolicSvg` variant

**Pros:**
- Clean type-level distinction between "needs recoloring" and "ready to render"
- Consumers can handle each case differently with exhaustive pattern matching

**Cons:**
- Breaking API change: every `match` on `IconData` across user code and all connectors breaks
- More complex for users who just want icon bytes
- Splits what should be a single rendering path into two
- Still requires the consumer to implement recoloring logic
- The `Svg` vs `SymbolicSvg` distinction only matters for freedesktop icons; bundled and macOS/Windows icons never need it

### Option E: Inject CSS override

**Pros:**
- Standards-compliant approach using CSS specificity

**Cons:**
- **Does not work.** Inline `fill="#2e3436"` attributes have higher CSS specificity than `<style>` rules. To override them, we'd need `!important` on every property, which is fragile
- Would need to REMOVE inline fills first and replace with class-based fills, making this equivalent to Option A but with more complex SVG manipulation
- Multi-color icons would have ALL fills overridden (breaks semantic colors like warning/error)
- Some SVG renderers have incomplete CSS support

### Option F: Look for dark-variant theme directory

**Pros:**
- Would work for themes that ship dark variants (e.g., hypothetical Adwaita-dark)

**Cons:**
- **Does not solve the problem.** Adwaita does not ship an `Adwaita-dark` directory -- that's the entire reason GTK uses runtime recoloring instead
- Most GTK-based themes (Yaru, elementary, Pop) also do not ship dark icon variants
- This is the KDE/Breeze convention, not the GTK convention
- Would only help themes that already work via the current code (separate dirs = separate CSS colors)

### Option G: Document the limitation

**Pros:**
- Zero code change, zero risk

**Cons:**
- Every app using `native-theme` with Adwaita on dark mode ships broken icons
- Defeats the core value proposition: "Any Rust GUI app can look native on any platform"
- Pushes toolkit-level complexity onto every individual app developer
- The problem is solvable -- choosing not to solve it is a poor tradeoff


## 4. Deep Reasoning: Best Solution

### Elimination

**Options E and F are non-starters.** Option E doesn't work due to CSS specificity, and Option F doesn't work because Adwaita has no dark variant. Eliminated.

**Option G is unacceptable.** A native theme crate that can't display readable icons on the most popular Linux desktop environment is fundamentally broken. Eliminated.

**Option D (new variant) is disproportionate.** It's a breaking API change that affects all consumers just to carry a boolean signal ("needs recoloring") that can be resolved at load time. The complexity fans out to every consumer. Eliminated.

**Option C (fg_color parameter) is architecturally awkward.** The caller needs to know the foreground color before loading icons, but icon loading is typically part of theme initialization. It's also a breaking change. Eliminated.

### Remaining: Option A vs Option B

**Option B (connector-level fix)** seems simpler but has a fatal flaw: connectors cannot distinguish symbolic from non-symbolic icons. The `colorize_monochrome_svg` function would blindly replace `#2e3436` in ALL freedesktop SVGs, including non-symbolic multi-color icons that might legitimately use that color. The connector receives `IconData::Svg(bytes)` with no metadata about the icon's origin.

**Option A (normalize in `freedesktop.rs`)** operates at the point where we KNOW whether the icon is symbolic (because `find_icon` explicitly tries the `-symbolic` suffix first). This is the only place in the pipeline with that information. Normalizing at this point:

1. **Eliminates the ambiguity problem** -- only `-symbolic` SVGs are modified
2. **Normalizes GTK convention to Breeze convention** -- after normalization, all symbolic SVGs use `currentColor`, which is the universal standard
3. **Leverages existing infrastructure** -- connectors already handle `currentColor`
4. **Requires no API changes** -- the function signatures, return types, and `IconData` enum are unchanged
5. **Is safe for direct users** -- `currentColor` defaults to black in SVG renderers, which is visually identical to `#2e3436` on light backgrounds

### Why `currentColor` is the right replacement target (not a specific color)

Replacing `#2e3436` directly with a specific foreground color (e.g., `#ffffff` for dark themes) would require knowing the theme variant at icon load time. But `load_freedesktop_icon` is a pure icon-loading function -- it shouldn't need theme context.

`currentColor` is the CSS/SVG standard for "inherit the foreground color from context." By normalizing to `currentColor`, we make the SVG self-describing: it says "color me with whatever foreground you're using." The connector's existing colorize path handles the rest.

### Which foreground colors to replace

| Color | Replace? | Rationale |
|---|---|---|
| `#2e3436` | **Yes** | Canonical GTK symbolic foreground (483 fill attrs, 8 CSS style fills, 1 stroke) |
| `#2e3434` | **Yes** | Interchangeable foreground variant (68 files as primary, 50 with fill-opacity for dimmed) |
| `#222222` | **Yes** | Same foreground pattern (27 occurrences, primary + fill-opacity dimmed) |
| `#474747` | **Yes** | Monochrome symbolic icons (50 emote/legacy files). Never mixed with other foreground colors. Would be invisible on dark backgrounds without recoloring |
| `#33d17a`, `#ff7800`, `#e01b24`, `#ed333b` | **No** | Semantic colors. Must remain fixed regardless of theme variant |

### Verification that `fill-opacity` is preserved

```xml
<!-- Before normalization -->
<path fill="#2e3434" fill-opacity="0.34902"/>

<!-- After normalization -->
<path fill="currentColor" fill-opacity="0.34902"/>

<!-- After connector colorize (dark theme, fg=#ffffff) -->
<path fill="#ffffff" fill-opacity="0.34902"/>
```

The `fill-opacity` attribute is separate from `fill` and is untouched by string replacement. The result is 35% opacity white -- correct for a secondary element on a dark background.

### Conclusion

**Option A is the best solution.** It fixes the problem at the source, requires no API changes, normalizes GTK symbolic icons to the universal `currentColor` standard, and leverages existing connector infrastructure.


## 5. Implementation Proposal

### Step 1: Refactor `find_icon` to return symbolic flag

**File:** `native-theme/src/freedesktop.rs`

Change `find_icon` return type from `Option<PathBuf>` to `Option<(PathBuf, bool)>` where the `bool` indicates whether the icon is symbolic.

```rust
fn find_icon(name: &str, theme: &str, size: u16) -> Option<(PathBuf, bool)> {
    // First try: symbolic variant
    let symbolic = format!("{name}-symbolic");
    if let Some(path) = freedesktop_icons::lookup(&symbolic)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
    {
        return Some((path, true));
    }
    // Second try: plain name
    // If the name itself already ends with "-symbolic" (caller passed it
    // explicitly via load_freedesktop_icon_by_name), mark as symbolic.
    freedesktop_icons::lookup(name)
        .with_theme(theme)
        .with_size(size)
        .force_svg()
        .find()
        .map(|path| (path, name.ends_with("-symbolic")))
}
```

### Step 2: Add `normalize_gtk_symbolic` function

**File:** `native-theme/src/freedesktop.rs`

```rust
/// The GTK symbolic icon foreground placeholder colors.
///
/// GTK's icon rendering pipeline replaces these at paint time.
/// We normalize them to `currentColor` so that downstream colorize
/// logic (which already handles `currentColor`) can apply the correct
/// foreground color for the active theme variant.
///
/// Measured from `/usr/share/icons/Adwaita/symbolic/`:
/// - `#2e3436`: 483 fill attrs + 8 CSS style fills + 1 stroke (Tango Aluminium 6)
/// - `#2e3434`: 118 files (68 primary, 50 with fill-opacity)
/// - `#222222`: 27 occurrences (primary + dimmed)
/// - `#474747`: 50 emote/legacy icons (monochrome, never mixed with above)
const GTK_FG_COLORS: &[&str] = &["#2e3436", "#2e3434", "#222222", "#474747"];

/// Normalize a GTK-convention symbolic SVG to use `currentColor`.
///
/// GTK symbolic icons use hardcoded dark fill colors (e.g., `#2e3436`)
/// that GTK replaces at render time. Since native-theme returns raw SVG
/// bytes, we normalize these placeholders to `currentColor` so that
/// existing connector colorize logic handles the recoloring.
///
/// Handles three placement patterns found in Adwaita:
/// - XML attributes: `fill="#2e3436"`, `stroke="#2e3436"`
/// - CSS style attributes: `style="fill:#2e3436;..."`
///
/// Only foreground placeholder colors are replaced. Semantic colors
/// (success green `#33d17a`, warning orange `#ff7800`, error red
/// `#e01b24`/`#ed333b`) are preserved.
///
/// Returns the original bytes unchanged if the SVG already uses
/// `currentColor` (Breeze-style) or is not valid UTF-8.
fn normalize_gtk_symbolic(svg_bytes: Vec<u8>) -> Vec<u8> {
    let Ok(svg_str) = std::str::from_utf8(&svg_bytes) else {
        return svg_bytes;
    };

    // Already uses currentColor (Breeze convention) -- no normalization needed
    if svg_str.contains("currentColor") {
        return svg_bytes;
    }

    // Check if any GTK foreground colors are present
    if !GTK_FG_COLORS.iter().any(|c| svg_str.contains(c)) {
        return svg_bytes;
    }

    let mut result = svg_str.to_string();
    for color in GTK_FG_COLORS {
        // XML attributes: fill="..." and stroke="..."
        result = result.replace(
            &format!("fill=\"{color}\""),
            "fill=\"currentColor\"",
        );
        result = result.replace(
            &format!("stroke=\"{color}\""),
            "stroke=\"currentColor\"",
        );
        // CSS style attributes: fill:#2e3436 (8 icons use this form)
        result = result.replace(
            &format!("fill:{color}"),
            "fill:currentColor",
        );
        result = result.replace(
            &format!("stroke:{color}"),
            "stroke:currentColor",
        );
    }
    result.into_bytes()
}
```

### Step 3: Apply normalization in load functions

**File:** `native-theme/src/freedesktop.rs`

Update `load_freedesktop_icon` and `load_freedesktop_icon_by_name`:

```rust
pub fn load_freedesktop_icon(role: IconRole, size: u16) -> Option<IconData> {
    let theme = detect_theme();
    let name = icon_name(role, IconSet::Freedesktop)?;
    let (path, is_symbolic) = find_icon(name, &theme, size)?;
    let bytes = std::fs::read(&path).ok()?;
    let bytes = if is_symbolic { normalize_gtk_symbolic(bytes) } else { bytes };
    Some(IconData::Svg(bytes))
}

pub fn load_freedesktop_icon_by_name(name: &str, theme: &str, size: u16) -> Option<IconData> {
    let (path, is_symbolic) = find_icon(name, theme, size)?;
    let bytes = std::fs::read(&path).ok()?;
    let bytes = if is_symbolic { normalize_gtk_symbolic(bytes) } else { bytes };
    Some(IconData::Svg(bytes))
}
```

### Step 4: Update connector documentation

**Files:** `connectors/native-theme-iced/src/icons.rs`, `connectors/native-theme-gpui/src/icons.rs`

Update the `to_svg_handle` / `to_image_source` doc comments:

```
Before: "Pass `None` for multi-color system icons to preserve their native palette."
After:  "Pass the theme's foreground/icon color for monochrome and symbolic icons (Material,
         Lucide, and freedesktop symbolic). Pass `None` only for multi-color non-symbolic
         system icons to preserve their native palette."
```

### Step 5: Add tests

**File:** `native-theme/src/freedesktop.rs`

```rust
#[test]
fn normalize_gtk_symbolic_replaces_2e3436() {
    let svg = br#"<svg><path fill="#2e3436" d="M0 0"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains(r#"fill="currentColor""#));
    assert!(!s.contains("#2e3436"));
}

#[test]
fn normalize_gtk_symbolic_replaces_2e3434_preserves_opacity() {
    let svg = br#"<svg><path fill="#2e3434" fill-opacity="0.35" d="M0 0"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains(r#"fill="currentColor""#));
    assert!(s.contains(r#"fill-opacity="0.35""#));
}

#[test]
fn normalize_gtk_symbolic_replaces_222222() {
    let svg = br#"<svg><path fill="#222222" d="M0 0"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains(r#"fill="currentColor""#));
    assert!(!s.contains("#222222"));
}

#[test]
fn normalize_gtk_symbolic_replaces_474747() {
    let svg = br#"<svg><path fill="#474747" d="M0 0"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains(r#"fill="currentColor""#));
    assert!(!s.contains("#474747"));
}

#[test]
fn normalize_gtk_symbolic_replaces_stroke() {
    let svg = br#"<svg><path stroke="#2e3436" fill="none" d="M1 1l14 14"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains(r#"stroke="currentColor""#));
    assert!(!s.contains("#2e3436"));
}

#[test]
fn normalize_gtk_symbolic_replaces_css_style_fill() {
    let svg = br#"<svg><path style="fill:#2e3436;fill-opacity:1" d="M0 0"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains("fill:currentColor"));
    assert!(!s.contains("#2e3436"));
}

#[test]
fn normalize_gtk_symbolic_preserves_semantic_colors() {
    let svg = br#"<svg><path fill="#2e3436"/><path fill="#ff7800"/><path fill="#33d17a"/><path fill="#e01b24"/></svg>"#.to_vec();
    let result = normalize_gtk_symbolic(svg);
    let s = std::str::from_utf8(&result).unwrap();
    assert!(s.contains("currentColor"));
    assert!(s.contains("#ff7800"), "warning color must be preserved");
    assert!(s.contains("#33d17a"), "success color must be preserved");
    assert!(s.contains("#e01b24"), "error color must be preserved");
}

#[test]
fn normalize_gtk_symbolic_skips_currentcolor_svgs() {
    let svg = br#"<svg><defs><style>.ColorScheme-Text{color:#232629}</style></defs><path fill="currentColor"/></svg>"#.to_vec();
    let original = svg.clone();
    let result = normalize_gtk_symbolic(svg);
    assert_eq!(result, original, "Breeze-style SVGs should pass through unchanged");
}

#[test]
fn normalize_gtk_symbolic_skips_non_gtk_svgs() {
    let svg = br#"<svg><path fill="red"/></svg>"#.to_vec();
    let original = svg.clone();
    let result = normalize_gtk_symbolic(svg);
    assert_eq!(result, original, "non-GTK SVGs should pass through unchanged");
}
```

### What this does NOT change

- The `IconData` enum
- Any public function signatures
- Any connector function signatures
- Semantic icon colors (success, warning, error)
- Non-symbolic freedesktop icons (gated by `is_symbolic` from `find_icon`)
- Breeze-style icons (already use `currentColor`, detected and skipped)
- Bundled icons (Material, Lucide) -- unaffected
- macOS / Windows icon loading -- unaffected
- `load_freedesktop_spinner` -- Adwaita has no `process-working` icon; themes that do (Breeze) already use `currentColor`

### Verification checklist

1. Adwaita `edit-copy-symbolic.svg`: `fill="#2e3436"` becomes `fill="currentColor"`
2. Breeze `edit-copy-symbolic.svg`: unchanged (already has `currentColor`)
3. Adwaita `camera-switch-symbolic.svg`: primary `#2e3436` becomes `currentColor`, secondary `#2e3434 fill-opacity="0.34902"` becomes `currentColor fill-opacity="0.34902"`
4. Adwaita `mail-mark-important-symbolic.svg`: `#2e3436` becomes `currentColor`, `#ff7800` stays `#ff7800`
5. Adwaita `face-smile-symbolic.svg`: `#474747` becomes `currentColor`
6. Adwaita `night-light-disabled-symbolic.svg`: `stroke="#2e3436"` becomes `stroke="currentColor"`
7. Adwaita `view-sort-ascending-rtl-symbolic.svg`: `style="fill:#2e3436;..."` becomes `style="fill:currentColor;..."`
8. Non-symbolic icon (e.g., Breeze `edit-copy.svg`): unchanged (is_symbolic=false)
9. `load_freedesktop_icon_by_name("edit-copy-symbolic", ...)`: name ends with `-symbolic`, normalization applied
10. `cargo test --features watch,kde,portal-tokio,system-icons` passes
11. `./pre-release-check.sh` passes
