# Animated Icons

## Problem Statement

native-theme's purpose is to make apps look native on every platform. Loading spinners are a fundamental UI element with distinct platform-native animations:

- **macOS**: Radial spokes/fins with sequential opacity fade, ~12 fps
- **Windows 11**: Arc/stroke that expands and contracts while rotating (Lottie-driven)
- **GNOME (libadwaita 1.6+)**: Programmatically drawn arcs (`AdwSpinnerPaintable`)
- **GNOME (older GTK)**: Symbolic SVG + CSS rotation (`GtkSpinner`)
- **KDE/Breeze**: 15-frame vertical sprite sheet SVG (`process-working.svg`)

Currently `StatusLoading` returns `None` on macOS, Windows, and Segoe because the icon model (`IconData`) only supports static images. A static icon cannot represent a loading state — animation IS the semantic.

## Current Architecture Constraints

```rust
pub enum IconData {
    Svg(Vec<u8>),
    Rgba { width: u32, height: u32, data: Vec<u8> },
}
```

- Both variants are static, single-frame data
- gpui connector: rasterizes SVGs via resvg (no SMIL/CSS animation support, by design)
- iced connector: passes SVGs to iced's renderer (also resvg-based, no animation)
- No animation primitives exist anywhere in the codebase

## Platform Research Summary

| Platform | Native mechanism | How it renders | Extractable? |
|----------|-----------------|----------------|--------------|
| macOS | `NSProgressIndicator` (spinning style) | Private AppKit rendering: fins with opacity gradient, timer-driven | No public API for frame capture |
| macOS 14+ | SF Symbols + `symbolEffect` on `NSImageView` | AppKit-native, but tied to view hierarchy | Cannot render to raw pixels |
| Windows 11 | `ProgressRing` via `AnimatedVisualPlayer` | Lottie animation compiled to composition visual | Could recreate the Lottie JSON |
| GNOME (new) | `AdwSpinnerPaintable` | Custom `GdkPaintable`, draws arcs procedurally | Source-readable but not themed |
| GNOME (old) | `GtkSpinner` | `process-working-symbolic` SVG + CSS `@keyframes spin` | SVG extractable, rotation is trivial |
| KDE | `KBusyIndicatorWidget` | Animates Breeze `process-working.svg` sprite sheet | Yes — 15-frame sprite sheet at `/usr/share/icons/breeze/animations/` |
| Freedesktop general | `process-working` in `animations/` context | Vertical sprite sheet SVG (theme-dependent) | Standard format, parseable |

### Renderer capabilities

| Renderer | SMIL animation | CSS `@keyframes` | Lottie | Frame sequence |
|----------|---------------|-----------------|--------|----------------|
| resvg (used by gpui) | No (by design, never will) | No | No | N/A — single frame only |
| iced's SVG renderer | No | No | No | N/A — single frame only |
| rlottie (Rust crate) | N/A | N/A | Yes — frame-by-frame RGBA | Yes |
| dotlottie-rs | N/A | N/A | Yes — CPU/GPU backends | Yes |

---

## Options

### Option A: Animation Hint on Static Icon

Return a static icon plus metadata telling the connector how to animate it.

```rust
pub enum AnimationHint {
    None,
    Spin { duration_ms: u32 },
    Pulse { duration_ms: u32 },
}

// Either as a separate return:
pub fn load_icon(role: IconRole, set: &str) -> Option<(IconData, AnimationHint)>

// Or as a method on IconRole:
impl IconRole {
    pub fn animation_hint(&self) -> AnimationHint { ... }
}
```

The connector (gpui/iced) applies a rotation transform on its render loop when it sees `Spin`.

**Pros:**
- Minimal API change — `IconData` stays the same
- No new dependencies
- Works with existing SVG/RGBA data
- Easy to implement in connectors (rotation transform is trivial)
- Bundled Material `progress_activity` + `Spin` already looks reasonable

**Cons:**
- A spinning Material icon does NOT look like the macOS spinner, Windows progress ring, or Breeze cogwheel animation
- All platforms get the same generic spinning SVG — defeats the "native look" goal
- Cannot express complex animations (Windows arc expansion, macOS fin opacity sequence)
- Mixes animation concerns into the icon loading API

---

### Option B: Frame Atlas

Pre-render N frames of the spinner into a single data structure. The connector cycles through frames on a timer.

```rust
pub struct FrameAtlas {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
    pub frame_duration_ms: u32,
    pub data: Vec<u8>, // frame_count * width * height * 4 bytes (RGBA)
}
```

Each platform produces its own atlas:
- macOS: 12 frames of the spinning fins (programmatically generated to match `NSProgressIndicator`)
- Windows: 30 frames of the arc/stroke animation (matching `ProgressRing` style)
- KDE: Parse the Breeze `process-working.svg` sprite sheet, rasterize each of the 15 frames via resvg
- GNOME: Generate arc frames matching `AdwSpinner` style
- Bundled: Ship pre-rendered atlas matching Material/Lucide spinner style

**Pros:**
- Universal — every renderer can display RGBA frames, no special capabilities needed
- Truly native appearance if we match each platform's animation faithfully
- Simple connector logic: advance frame index on timer tick, display frame
- Self-contained — no external animation runtime
- Freedesktop sprite sheets map naturally to this model

**Cons:**
- Memory cost: 12 frames x 48x48 x 4 = ~111 KB per spinner (acceptable), 30 frames x 48x48 x 4 = ~277 KB (heavier)
- Raster-only: no vector scaling. Fixed to one resolution (or need multiple atlas sizes)
- Capturing native macOS/Windows frames at runtime is impractical — need to draw them ourselves
- Pre-rendered frames are an approximation, not the actual platform animation
- `PartialEq`/`Eq` derives on `IconData` would be expensive for large frame buffers

---

### Option C: Lottie Animation Data

Return a Lottie JSON animation. The connector uses `rlottie` to render frames at display time.

```rust
pub enum IconData {
    Svg(Vec<u8>),
    Rgba { width: u32, height: u32, data: Vec<u8> },
    Lottie(Vec<u8>),  // JSON animation data
}
```

Ship platform-specific Lottie files:
- macOS-style: Lottie file mimicking the spinning fins
- Windows-style: The actual ProgressRing Lottie (or a recreation of it)
- Material-style: Material Design spinner Lottie (available from Google's design resources)
- Generic: A neutral spinner Lottie

**Pros:**
- Vector-based: scales to any resolution
- Compact: a Lottie JSON for a spinner is typically 2-5 KB
- Windows 11 actually uses Lottie for its ProgressRing — closest to native
- Rich animation capabilities (easing, path animation, color transitions)
- `rlottie` crate can render frame-by-frame to RGBA buffers
- Lottie has massive ecosystem support and tooling

**Cons:**
- Adds `rlottie` dependency (C++ FFI via `rlottie-sys`) or `dotlottie-rs` (ThorVG C++ FFI)
- Neither is pure Rust — adds build complexity (C++ compiler required)
- Connectors must integrate a Lottie renderer into their frame loop
- Creating platform-faithful Lottie files requires design work (or finding existing ones)
- macOS and GNOME don't use Lottie natively — the Lottie file is an approximation
- Overkill for a simple spinner — Lottie supports far more than we need

---

### Option D: Procedural Spinner Parameters

Return mathematical parameters describing the spinner's appearance. The connector draws it.

```rust
pub struct SpinnerParams {
    pub style: SpinnerStyle,
    pub duration_ms: u32,
    pub color: Option<[u8; 4]>,  // RGBA, None = use theme foreground
}

pub enum SpinnerStyle {
    /// macOS-style: radial spokes with opacity gradient
    Spokes { count: u8, width: f32, length: f32 },
    /// Windows-style: arc that expands/contracts while rotating
    Arc { stroke_width: f32, arc_length_range: (f32, f32) },
    /// Material-style: circular indeterminate stroke
    CircularStroke { stroke_width: f32 },
    /// GNOME-style: overlapping arcs
    Arcs { count: u8, stroke_width: f32 },
}
```

Each platform returns its native style parameters. The connector renders using its toolkit's drawing primitives.

**Pros:**
- Tiny data footprint (a few numbers)
- Resolution-independent (drawn at any size)
- No external dependencies
- Truly platform-adaptive: the parameters come from the platform, rendering uses toolkit primitives
- Connectors have full control over rendering quality and integration with their paint loop

**Cons:**
- Duplicates rendering logic in every connector — each must implement all spinner styles
- Hard to get pixel-perfect results across different rendering engines
- Limited expressiveness — adding new platform spinner styles requires updating every connector
- Tight coupling between native-theme and connectors (connectors must understand every style variant)
- Testing is hard — how do you verify a connector's spoke rendering matches macOS?

---

### Option E: SVG Sprite Sheet

Return an SVG sprite sheet with metadata. The connector slices and renders individual frames.

```rust
pub struct SpriteSheet {
    pub svg: Vec<u8>,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_count: u32,
    pub frame_duration_ms: u32,
    pub orientation: Orientation, // Vertical (breeze) or Horizontal
}
```

This is how freedesktop `process-working` already works. Extend the pattern to other platforms by creating sprite sheet SVGs for each native style.

**Pros:**
- Matches the existing freedesktop convention exactly
- SVG = vector, scales to any resolution
- Each connector rasterizes with resvg by adjusting the viewBox per frame
- No new runtime dependencies (resvg already used)
- Can bundle platform-specific sprite sheets

**Cons:**
- Creating macOS/Windows-style sprite sheet SVGs is manual design work
- SVG sprite sheets are larger than Lottie (15 frames of SVG paths vs one animation description)
- Connector must implement viewBox slicing logic (non-trivial for vertical/horizontal sheets)
- Not a standard format outside freedesktop — invented convention for macOS/Windows styles
- Only works for SVG-renderable spinners — macOS's RGBA rasterized spinner doesn't fit

---

### Option F: Separate `AnimatedIcon` API

Don't put animation in `IconData` at all. Create a parallel API specifically for animated icons.

```rust
/// Animated icon data with frame-by-frame playback information.
pub struct AnimatedIcon {
    pub frames: Vec<IconData>,
    pub frame_duration_ms: u32,
    pub repeat: Repeat,
}

pub enum Repeat {
    Infinite,
    Count(u32),
}

/// Load the platform-native loading indicator.
pub fn loading_indicator(icon_set: &str) -> Option<AnimatedIcon> { ... }

/// Load the platform-native loading indicator at a specific size.
pub fn loading_indicator_sized(icon_set: &str, size: u32) -> Option<AnimatedIcon> { ... }
```

The `AnimatedIcon` holds a sequence of `IconData` frames (each can be SVG or RGBA). Connectors get a `to_animated_image_source()` function that returns a frame iterator or similar.

**Pros:**
- Clean separation: static icons and animated icons are different things
- `IconData` stays unchanged — no breaking changes
- Each frame is a normal `IconData` — existing rendering code works per-frame
- Flexible: frames can be SVG (freedesktop sprite sheets) or RGBA (macOS/Windows)
- No new rendering dependencies — just cycles through existing `IconData` frames
- `StatusLoading` can be removed from `IconRole` (it was never really an icon)

**Cons:**
- New public API surface (new types, new functions)
- Memory proportional to frame count x frame size
- SVG frames still need rasterization per-frame (though can be cached)
- Connector must implement frame cycling logic
- Generating platform-native frames still requires platform-specific code

---

### Option G: Runtime Platform Capture

Actually capture frames from the OS's native spinner widget at runtime.

- macOS: Create a headless `NSProgressIndicator`, snapshot it N times via `cacheDisplay(in:to:)`
- Windows: Instantiate a `ProgressRing`, capture its composition output
- Linux: Parse the theme's `process-working` sprite sheet or invoke GtkSpinner off-screen

```rust
// Internal implementation detail — public API same as Option F
fn capture_macos_spinner(size: u32, frames: u32) -> AnimatedIcon {
    // Create NSProgressIndicator, start animation, snapshot each frame
}
```

**Pros:**
- THE most native result — literally capturing what the OS renders
- Automatically adapts to OS updates (new spinner design = automatically captured)
- No manual design work for platform-specific spinners

**Cons:**
- Extremely fragile — depends on private OS rendering internals
- macOS: requires a window server connection (not available in headless/CLI contexts)
- Windows: `ProgressRing` is a WinUI control, instantiating it requires XAML infrastructure
- Slow: creating OS widgets and snapshotting is heavy for what should be a simple icon load
- Non-deterministic: frame timing depends on system load, vsync
- Untestable in CI
- Over-engineered for what is ultimately a spinner animation

---

### Option H: Bundled Pre-rendered Animations

Ship platform-style spinner animations as bundled assets (like we do for bundled SVG icons), in a format the connectors can consume.

```
native-theme/animations/
  macos-spinner.apng      (or .webp, or .lottie, or frames/*.png)
  windows-spinner.apng
  material-spinner.apng
  breeze-spinner.apng
  adwaita-spinner.apng
```

At build time, `include_bytes!()` bundles the platform-appropriate animation. At runtime, decode and play it.

**Pros:**
- Fully controlled output — hand-crafted to match each platform exactly
- No runtime platform dependency — works on any OS (cross-compile friendly)
- Single source of truth: designers create the animation once, it's frozen
- APNG is well-supported in Rust (`png` crate decodes APNG frames)
- Alternative: animated WebP via `webp-animation` crate (smaller file sizes)

**Cons:**
- Manual design work: someone must create faithful platform-specific spinner animations
- Raster-only (APNG/WebP): fixed resolution, doesn't scale well. Need multiple sizes or accept blur.
- Won't automatically adapt when a platform updates its spinner design
- Adds binary size: even small animations are several KB per platform
- APNG/WebP decoding adds a dependency (though `png` is lightweight)

---

### Option I: Connector-Level Native Widget

Don't model animation in native-theme at all. Each connector provides a platform-native spinner widget using its toolkit's own animation system.

```rust
// In native-theme-gpui:
pub fn native_spinner(cx: &mut Context) -> impl IntoElement { ... }

// In native-theme-iced:
pub fn native_spinner<'a>() -> iced::widget::Container<'a, Message> { ... }
```

The connector uses its toolkit's built-in capabilities:
- gpui: Custom `Element` with `paint()` that draws spokes/arcs and requests animation frames
- iced: Custom widget with `draw()` + subscription for tick updates

**Pros:**
- Best visual integration — the spinner is a first-class toolkit widget
- Smooth animation — uses the toolkit's native frame clock, not frame cycling
- Resolution-independent — drawn via vector primitives at render time
- No data model changes in native-theme core
- Each connector optimizes for its own rendering pipeline
- Can use platform-specific drawing (gpui's `Path`, iced's `Canvas`)

**Cons:**
- Every connector must implement spinner rendering from scratch
- Spinner appearance must be manually designed per connector to match each platform
- native-theme's value proposition weakens — the core crate can't help here
- Apps that don't use gpui or iced get nothing
- Duplication: if we add a third connector, it needs its own spinner implementation
- Hard to keep all connector spinners visually consistent with their respective platforms

---

### Option J: Toolkit Animation Delegate

native-theme provides the animation description and frame data. The connector provides an animation "player" trait. native-theme drives the animation logic, the connector just renders each frame.

```rust
// In native-theme core:
pub trait AnimationRenderer {
    fn render_frame(&mut self, frame: &IconData);
    fn request_next_frame(&mut self, delay_ms: u32);
}

pub struct SpinnerAnimation { /* internal state */ }

impl SpinnerAnimation {
    pub fn new(icon_set: &str) -> Option<Self> { ... }
    pub fn tick(&mut self, renderer: &mut dyn AnimationRenderer) { ... }
}
```

**Pros:**
- Animation logic lives in native-theme (single implementation)
- Connector only implements rendering (which it already does for static icons)
- Frame timing controlled by native-theme (consistent across connectors)

**Cons:**
- Requires a stateful animation object (unlike the current stateless `load_icon` API)
- `AnimationRenderer` trait creates tight coupling with connector rendering loops
- Different toolkits have different frame scheduling models — `request_next_frame` may not map cleanly
- Over-abstracted for what is one or two animated icons

---

## Comparison Matrix

| Criterion | A: Hint | B: Atlas | C: Lottie | D: Procedural | E: Sprite | F: AnimatedIcon | G: Capture | H: Bundled | I: Widget | J: Delegate |
|-----------|---------|----------|-----------|---------------|-----------|----------------|------------|------------|-----------|-------------|
| Native fidelity | Low | Medium | Medium | High | Medium | Medium-High | Highest | High | High | Medium-High |
| Impl complexity | Low | Medium | High | High | Medium | Medium | Very High | Medium | High | High |
| New dependencies | None | None | C++ FFI | None | None | None | Platform FFI | png/webp | None | None |
| Binary size impact | None | ~100-300 KB | ~3-5 KB + rlottie | None | ~50-150 KB | ~100-300 KB | None | ~20-80 KB | None | None |
| Resolution scaling | Via SVG | Fixed | Vector | Vector | Via SVG | Mixed | Fixed | Fixed | Vector | Mixed |
| Connector effort | Low | Low | High | Very High | Medium | Low | N/A | Low | Very High | Medium |
| API cleanliness | Poor (mixed) | Medium | Medium | Medium | Medium | Good | Good | Medium | Good | Poor |
| Cross-compile | Yes | Yes | Yes | Yes | Yes | Yes | No | Yes | Yes | Yes |

---

## Hybrid Combinations

The options above aren't mutually exclusive. The best solution may combine multiple approaches, selected per context.

### Combo 1: F + B (Recommended)

**`AnimatedIcon` API (Option F) backed by frame atlas data (Option B).**

The public API is `AnimatedIcon` with a `Vec<IconData>` frames field. Under the hood:

- **Freedesktop/KDE**: Parse the theme's `process-working.svg` sprite sheet, rasterize each frame via resvg. Produces SVG-per-frame or RGBA-per-frame data.
- **macOS**: Programmatically generate 12 spoke frames as SVG (each frame is an SVG with spokes at different opacities). Matches `NSProgressIndicator` style without needing platform APIs.
- **Windows**: Programmatically generate arc animation frames as SVG. Matches `ProgressRing` style.
- **Material/Lucide bundled**: Include a pre-made set of SVG frames (or a single SVG + rotation, since Material's spinner IS just a rotating arc).

```rust
// Public API
pub fn loading_indicator(icon_set: &str) -> Option<AnimatedIcon>
pub fn loading_indicator_sized(icon_set: &str, size: u32) -> Option<AnimatedIcon>

// AnimatedIcon holds Vec<IconData> — connectors already know how to render IconData
```

**Why this works:**
- Clean API (Option F) with practical implementation (Option B)
- Connectors need zero new rendering code — just cycle `IconData` frames
- SVG frames = resolution independent on renderers that support it
- RGBA frames = universal compatibility
- Freedesktop themes get parsed at runtime (truly native)
- Other platforms get hand-crafted SVG frame sets compiled into the binary

**Weaknesses:**
- Memory: N frames x frame size. Acceptable for 12-15 frames at icon sizes.
- SVG frame generation for macOS/Windows requires one-time design work

### Combo 2: F + B + I (Maximum fidelity)

**`AnimatedIcon` API for programmatic use, plus optional connector-level widgets for best integration.**

native-theme core provides `loading_indicator()` returning `AnimatedIcon` (frame data). Connectors provide BOTH:
1. `to_animated_image_source(anim: &AnimatedIcon)` — for apps that want raw frame cycling
2. `native_spinner()` — a proper toolkit widget that uses the toolkit's drawing primitives and frame clock

The widget internally uses the `AnimatedIcon` data but renders with smooth interpolation, proper frame timing, and toolkit-native animation facilities.

**Why this works:**
- Apps that want simplicity use the widget
- Apps that need custom rendering use the frame data
- Maximum native fidelity from the widget path

**Weaknesses:**
- Most complex to implement — widget code is per-connector
- Two parallel APIs to maintain

### Combo 3: F + A (Pragmatic minimum)

**`AnimatedIcon` API for true animated content, plus animation hints for "close enough" cases.**

`StatusLoading` gets a proper `AnimatedIcon` via `loading_indicator()`. But for icons that just need a simple effect (e.g., a notification bell that should "ring" when active), use `AnimationHint::Pulse` or `AnimationHint::Wiggle` on the static icon.

```rust
// For the spinner — full animation
let spinner = loading_indicator("material"); // -> AnimatedIcon with frames

// For a bell shake — just a hint on a static icon
let hint = IconRole::Notification.animation_hint(); // -> AnimationHint::None normally
```

**Why this works:**
- Separates the two concerns: complex animations (spinner) vs simple effects (shake, pulse)
- Animation hints are nearly free to implement
- Full `AnimatedIcon` only needed for the spinner case

**Weaknesses:**
- Two animation systems in one crate
- Animation hints are still "generic, not native"

---

## Proposal

**Combo 1 (F + B)** — `AnimatedIcon` API backed by frame data.

### Rationale

1. **Clean separation**: `StatusLoading` is removed from `IconRole`. Loading indicators get their own API (`loading_indicator()`). Static icons stay static.

2. **Zero connector rework**: `AnimatedIcon` holds `Vec<IconData>`. Connectors already render `IconData`. The only new connector code is a timer that advances the frame index — trivial in both gpui and iced.

3. **Truly native on freedesktop**: Parse the actual theme's `process-working` sprite sheet at runtime. KDE/Breeze users see Breeze's cogwheel animation. Adwaita users see whatever their theme provides.

4. **Faithful approximation elsewhere**: For macOS/Windows, ship SVG frame sets that faithfully reproduce the native spinner style. These are static assets, hand-crafted once, compiled in via `include_bytes!()`. Not pixel-identical to the OS, but visually consistent.

5. **No heavy dependencies**: No Lottie runtime, no C++ FFI, no platform-specific capture code. Just SVG frames rendered through the existing resvg pipeline.

6. **Future-proof**: If we later want Lottie support (Option C) or connector widgets (Option I), the `AnimatedIcon` API doesn't need to change — those become alternative ways to produce or consume the same animation data.

### What needs to be built

| Component | Work |
|-----------|------|
| `AnimatedIcon` struct + `Repeat` enum | New types in `model/icons.rs` |
| `loading_indicator(icon_set: &str)` | New public function in `lib.rs` |
| Freedesktop sprite sheet parser | Parse `animations/*/process-working.svg`, slice into frames |
| macOS spinner SVG frames | Design 12 SVGs matching the spoke/fin style |
| Windows spinner SVG frames | Design ~20 SVGs matching the arc/stroke style |
| Material spinner SVG frames | Design ~12 SVGs of the circular stroke animation |
| Bundled frame sets | `include_bytes!()` for non-freedesktop platforms |
| gpui connector `to_animated_image_sources()` | Convert `AnimatedIcon` to `Vec<ImageSource>` + timing |
| iced connector `to_animated_handles()` | Convert `AnimatedIcon` to `Vec<svg::Handle>` + timing |
| Remove `StatusLoading` from `IconRole` | Breaking change (major version) or deprecation |

### Open questions

1. **Should `StatusLoading` be removed from `IconRole` or deprecated?** Removing is cleaner but a breaking change. Deprecating keeps compat but leaves a confusing variant that always returns `None`.

2. **Should `AnimatedIcon` support variable frame durations?** The current proposal uses a single `frame_duration_ms`. Platform spinners use constant frame rates, so this is fine for now. But `Vec<(IconData, u32)>` (frame + duration) would be more flexible.

3. **Should the bundled spinner SVG frames be feature-gated?** Like `material-icons` gates bundled SVGs. A `spinner-animations` feature could gate the ~50-100 KB of bundled frame data.

4. **What size should bundled frames target?** 24px (standard icon size) or 48px (gpui's rasterize size)? SVG frames are resolution-independent, so this only matters for the viewBox.
