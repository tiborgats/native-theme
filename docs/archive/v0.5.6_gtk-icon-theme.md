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

**Option B (connector-level fix)** seems simpler but has a fatal flaw: connectors cannot distinguish symbolic from non-symbolic icons. The `colorize_monochrome_svg` function would blindly replace `#2e3436` in ALL freedesktop SVGs, including non-symbolic multi-color icons that might legitimately use that color. The connector receives `IconData::Svg(bytes)` with no metadata about the icon's origin. Eliminated.

### Remaining: Option A vs Option C

**Option A (normalize to `currentColor`)** was implemented first, but it has a critical flaw: showcase examples (and any app that renders system icons "as-is" without colorization) rely on SVGs being self-contained with the correct colors. Breeze icons are self-contained because `breeze-dark/` SVGs embed a CSS `<style>` block with `color: #fcfcfc` that the SVG renderer resolves `currentColor` through. Adwaita icons normalized to bare `currentColor` have no such CSS cascade, so SVG renderers default `currentColor` to black — invisible on dark backgrounds. The fix would require every consumer to always apply colorization to system icons, which contradicts the existing design where system icons are rendered as-is.

**Option C (fg_color parameter)** mirrors what GTK itself does: replace foreground placeholders with the widget's CSS `color` property (the text/foreground color) at load time. The caller provides the foreground color from the resolved theme (`defaults.text_color`), and the SVG is returned with the correct color baked in — self-contained, like Breeze icons. No connector colorization needed.

The API change (`Option<[u8; 3]>` parameter) is small and the sole maintainer can update all callers. The chicken-and-egg concern from the original analysis ("you need the theme to colorize icons") is not a real problem: by the time an app loads icons, the theme is already resolved and the text color is known.

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

<!-- After normalization (dark theme, fg=#ffffff) -->
<path fill="#ffffff" fill-opacity="0.34902"/>
```

The `fill-opacity` attribute is separate from `fill` and is untouched by string replacement. The result is 35% opacity white -- correct for a secondary element on a dark background.

### Conclusion

**Option C is the best solution.** It mirrors GTK's own recoloring mechanism (replace foreground placeholders with the widget's text color at load time), produces self-contained SVGs that render correctly without connector colorization, and works with the existing showcase design where system icons are rendered as-is.


## 5. Implementation (actual)

### Approach: Option C — `fg_color` parameter with direct color replacement

Instead of normalizing to `currentColor` (Option A), the load functions accept `fg_color: Option<[u8; 3]>` and replace GTK foreground placeholders with the caller's theme text color directly. This mirrors GTK's own behavior: replace placeholder fills with the widget's CSS `color` property at load time.

When `fg_color` is `None`, falls back to `currentColor` for backward compatibility.

### Changes

**`native-theme/src/freedesktop.rs`:**
- `find_icon` returns `Option<(PathBuf, bool)>` (symbolic flag)
- `normalize_gtk_symbolic(svg_bytes, replacement)` takes a replacement string (hex color or `currentColor`)
- `load_freedesktop_icon(role, size, fg_color)` accepts `Option<[u8; 3]>`
- `load_freedesktop_icon_by_name(name, theme, size, fg_color)` accepts `Option<[u8; 3]>`
- `fg_to_replacement()` helper converts `Option<[u8; 3]>` to hex string or `"currentColor"`

**`native-theme/src/icons.rs`:**
- `load_icon(role, set, fg_color)` threads `fg_color` to freedesktop path
- `load_icon_from_theme(role, set, theme, fg_color)` threads `fg_color`
- `load_system_icon_by_name(name, set, fg_color)` threads `fg_color`
- `load_custom_icon(provider, set, fg_color)` threads `fg_color`

**Showcase examples:**
- Extract `fg_color` from `resolved.defaults.text_color` (iced) or `original_font.color` (gpui)
- Pass to all icon loading calls
- System icons now have correct foreground baked in; no connector colorization needed

### What this does NOT change

- The `IconData` enum
- Semantic icon colors (success, warning, error)
- Non-symbolic freedesktop icons (gated by `is_symbolic` from `find_icon`)
- Breeze-style icons (already use `currentColor`, detected and skipped)
- Bundled icons (Material, Lucide) -- `fg_color` ignored for non-freedesktop sets
- macOS / Windows icon loading -- `fg_color` ignored
- `load_freedesktop_spinner` -- not modified, uses direct `freedesktop_icons::lookup`

### Verification checklist

1. Adwaita `edit-copy-symbolic.svg` on dark: `fill="#2e3436"` becomes `fill="#ffffff"` (or whatever the theme text color is)
2. Breeze `edit-copy-symbolic.svg`: unchanged (already has `currentColor` with CSS-defined color)
3. Adwaita `camera-switch-symbolic.svg`: primary `#2e3436` replaced, secondary `#2e3434 fill-opacity="0.34902"` replaced with opacity preserved
4. Adwaita `mail-mark-important-symbolic.svg`: foreground replaced, `#ff7800` stays `#ff7800`
5. Adwaita `face-smile-symbolic.svg`: `#474747` replaced
6. Non-symbolic icon: unchanged (is_symbolic=false)
7. `fg_color=None`: falls back to `currentColor` (backward compatible)
8. `cargo test --features watch,kde,portal-tokio,system-icons` passes
9. `./pre-release-check.sh` passes
