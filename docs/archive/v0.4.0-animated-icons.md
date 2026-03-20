# Animated Icons

## Problem Statement

native-theme's purpose is to make apps look native on every platform. Loading spinners are a fundamental UI element with distinct platform-native animations:

- **macOS**: Radial spokes/fins with sequential opacity fade, ~12 fps
- **Windows 11**: Arc/stroke that expands and contracts while rotating (Lottie-driven)
- **GNOME (libadwaita 1.6+)**: Programmatically drawn arcs (`AdwSpinnerPaintable`)
- **GNOME (older GTK)**: Symbolic SVG + CSS rotation (`GtkSpinner`)
- **KDE/Breeze**: Single gear icon with continuous rotation (modern KF6); legacy apps use 15-frame vertical sprite sheet SVG (`process-working.svg`)

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
| GNOME (libadwaita 1.6+) | `AdwSpinnerPaintable` | Custom `GdkPaintable`, draws arcs procedurally via GSK path primitives (`GskPathBuilder`/`GskStroke`/`GskPathMeasure`). Rotation is linear (`ADW_LINEAR`); arc breathing (extend/contract) uses sinusoidal easing (`ADW_EASE_IN_OUT_SINE`). 1200 ms per cycle. | Source-readable but not themed |
| GNOME (GTK3/4) | `GtkSpinner` | CSS `rotate(1turn)` on `process-working-symbolic` SVG; GTK3 has procedural 12-spoke Cairo fallback (`gtk_css_image_builtin_draw_spinner`, `num_steps = 12`, linearly fading alpha per spoke). Note: `process-working-symbolic` removed from Adwaita icon theme v48 | SVG extractable, rotation is trivial |
| KDE (modern, KF6) | QML `BusyIndicator` / `KBusyIndicatorWidget` | Loads `process-working-symbolic` (single gear SVG) and applies continuous rotation via `RotationAnimator` (QML) or `QVariantAnimation` (C++), 2000 ms/rev. `KIconLoader::loadAnimated()` deprecated since KF 6.5 | Single-frame SVG extractable |
| KDE (legacy) | `KPixmapSequenceWidget` | Frame-by-frame playback of Breeze `process-working.svg` sprite sheet (200 ms/frame default). Used by Gwenview, Akonadi, DrKonqi | Yes — 15-frame sprite sheet at `/usr/share/icons/breeze/animations/` |
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
- Windows: 60 frames of the arc/stroke animation (matching `ProgressRing` 2-second cycle at 30 fps)
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
- Memory cost: 12 frames x 48x48 x 4 = ~111 KB per spinner (acceptable), 60 frames x 48x48 x 4 = ~553 KB (heavier)
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
- Adds `rlottie` dependency (C++ FFI via `rlottie-sys`) or `dotlottie-rs` (C++ FFI — ThorVG or rlottie backend)
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
- Connector must implement viewBox slicing logic (straightforward arithmetic, but new code per connector)
- Not a standard format outside freedesktop — invented convention for macOS/Windows styles
- Requires all spinner styles to be expressible as SVG paths — platform effects (macOS spoke anti-aliasing subtleties, Windows arc easing) may not reproduce faithfully in static SVG

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

#[non_exhaustive]
pub enum Repeat {
    Infinite,
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
- If each frame is an independent `IconData::Svg(Vec<u8>)`, SVG boilerplate (XML headers, namespace declarations) is duplicated per frame — less memory-efficient than a single sprite sheet SVG

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
- Hard to verify each connector's spinner accurately matches its target platform's native animation

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
| Native fidelity | Low | Medium | Medium | Medium-High | Medium | Medium-High | Highest | High | High | Medium-High |
| Impl complexity | Low | Medium | High | High | Medium | Medium | Very High | Medium | High | High |
| New dependencies | None | None | C++ FFI | None | None | None | Platform FFI | png/webp | None | None |
| Binary size impact | None | ~100-550 KB | ~3-5 KB JSON + ~1-2 MB rlottie lib | None | ~50-150 KB | ~100-550 KB | None | ~20-80 KB | None | None |
| Resolution scaling | Via SVG | Fixed | Vector | Vector | Via SVG | Mixed | Fixed | Fixed | Vector | Mixed |
| Connector effort | Low | Low | High | Very High | Medium | Low | N/A | Low | Very High | Medium |
| API cleanliness | Poor (mixed) | Medium | Medium | Medium | Medium | Good | Good | Medium | Good | Poor |
| Cross-compile | Yes | Yes | Yes | Yes | Yes | Yes | No | Yes | Yes | Yes |

---

## Hybrid Combinations

The options above aren't mutually exclusive. The best solution may combine multiple approaches, selected per context.

### Combo 1: F + B

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
- Connectors need minimal new rendering code — just a timer cycling `IconData` frames
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

### Combo 3: Unified F + A (Platform-adaptive) (Recommended)

**Single `AnimatedIcon` enum with both frame sequences and simple transforms as variants.**

Instead of two parallel systems (AnimatedIcon + AnimationHint), unify them into one type. Each platform uses whichever variant best matches its native animation model.

```rust
#[non_exhaustive]
pub enum AnimatedIcon {
    /// Frame sequence — for complex animations (macOS spokes, Windows arc, KDE sprite sheet)
    Frames {
        frames: Vec<IconData>,
        frame_duration_ms: u32,
        repeat: Repeat,
    },
    /// Simple transform on a static icon — for continuous rotation animations
    Transform {
        icon: IconData,
        animation: TransformAnimation,
    },
}

#[non_exhaustive]
pub enum TransformAnimation {
    /// Continuous rotation (e.g., GNOME GtkSpinner, Lucide loader)
    Spin { duration_ms: u32 },
}
```

Each platform uses the variant that matches its native animation:

| Platform | Variant | Implementation |
|----------|---------|----------------|
| KDE/Breeze | `Frames` | Parse native 15-frame sprite sheet at runtime |
| macOS | `Frames` | 12 SVG frames with spoke opacity steps |
| Windows | `Frames` | ~60 SVG arc frames (30 fps over 2-second cycle) with easing baked into frame content |
| GNOME (libadwaita 1.6+) | `Frames` | Generated arc frames matching `AdwSpinnerPaintable` style |
| GNOME (GTK3/4) | `Transform::Spin` | Static `process-working-symbolic` SVG + rotation |
| Bundled Material | `Frames` | Generated circular stroke arc frames |
| Bundled Lucide | `Transform::Spin` | `loader` icon is designed for rotation |

```rust
// Public API — same as Combo 1
pub fn loading_indicator(icon_set: &str) -> Option<AnimatedIcon>
```

**Why this works:**
- **One type, not two systems**: `AnimatedIcon` is a single `#[non_exhaustive]` enum that connectors match on. No parallel `AnimationHint` API.
- **Platform-native strategy per platform**: GNOME's CSS rotation becomes `Transform::Spin` (smooth 60fps continuous rotation) instead of 12 redundant copies of the same SVG at different angles (jerky 30° steps). macOS/Windows/KDE use `Frames` because their native animations are inherently multi-frame.
- **Memory-efficient for simple cases**: `Transform::Spin` stores one SVG, not 12 copies.
- **Smooth rotation where appropriate**: Connectors apply continuous rotation transforms for `Spin` icons, matching the native GTK behavior exactly. Frame-cycling the same SVG at 12 discrete rotation steps would be visibly jerkier.
- **Trivial connector implementation**: Two match arms — cycle frames on timer, or apply rotation transform. Both are ~10 lines per connector.

**Weaknesses:**
- Connectors must handle two variants (though both are simple)
- Transform animations are inherently "generic" — they work but don't capture platform-specific easing
- GNOME (GTK3/4) case is becoming less relevant as `process-working-symbolic` was removed from Adwaita icon theme v48

---

## Proposal

**Combo 3 (Unified F + A)** — single `AnimatedIcon` enum with platform-adaptive strategy.

### Why not Combo 1 (F + B)?

Combo 1 uses frame sequences for all platforms uniformly. This is suboptimal for one specific case and slightly inefficient for another:

1. **GNOME (GTK3/4)**: The native `GtkSpinner` animation IS CSS rotation of a static SVG (`process-working-symbolic`). Creating 12 copies of the same SVG at 30° intervals is wasteful (12x the memory) and produces visibly jerky 30°-step rotation instead of smooth continuous rotation. A `Transform::Spin` variant matches the native behavior exactly.

2. **Lucide bundled**: The `loader` icon is explicitly designed for rotation — spinning it continuously is the intended use. Frame-cycling 12 rotated copies works but is unnecessary overhead when the connector can just rotate the single icon smoothly.

For macOS (12 discrete spoke opacity steps), Windows (arc expansion/contraction), and KDE (15-frame sprite sheet), frame sequences are the natural fit. The unified approach uses each strategy where it fits best, rather than forcing frame sequences everywhere.

Note on Windows easing: the native `ProgressRing` has a 2-second cycle (120 Lottie frames at 60 fps, source: `ProgressRingIndeterminate.h` in microsoft-ui-xaml). The animation has two symmetric 1-second phases — arc grows (TrimEnd 0→0.5) then arc shrinks (TrimStart 0→0.5) — with near-linear cubic bezier easing ((0.167, 0.167) to (0.833, 0.833)) and a step crossfade between two overlapping ellipse shapes at the midpoint. Total rotation is 900° (2.5 turns) per cycle. With frame-based animation, the easing is baked into each frame's SVG geometry (the arc angle at that time step reflects the eased value). ~60 uniformly-timed frames (30 fps) captures the 2-second cycle faithfully. 40 frames at 50 ms (20 fps) is the minimum for visually smooth arc motion. Variable per-frame timing could allow fewer frames, but is not needed for visual correctness — this is deferred to a future enhancement if needed.

### Rationale

1. **Clean separation**: `StatusLoading` is removed from `IconRole`. Loading indicators get their own API (`loading_indicator()`). Static icons stay static.

2. **Minimal connector work**: Connectors match on two enum variants — frame cycling or rotation transform. Both are trivial. Existing `IconData` rendering code works for each frame.

3. **Truly native on freedesktop**: KDE/Breeze sprite sheets are parsed at runtime — users see their actual theme's animation. GNOME's `process-working-symbolic` SVG gets proper continuous rotation matching native `GtkSpinner` behavior.

4. **Faithful approximation on macOS/Windows**: Ship SVG frame sets that reproduce the native spinner style. Hand-crafted once, compiled in via `include_bytes!()`. For Windows, ~60 frames at uniform timing (30 fps over the native 2-second cycle) with easing baked into arc geometry. Not pixel-identical to the OS, but visually consistent.

5. **No heavy dependencies**: No Lottie runtime, no C++ FFI, no platform-specific capture code. SVG frames through the existing resvg pipeline, rotation transforms through the toolkit's built-in transform support.

6. **Future-proof**: The enum is `#[non_exhaustive]`. If we later want Lottie support (add a `Lottie` variant), connector widgets (Option I), or variable frame timing, the API extends naturally without breaking changes.

### Platform strategy summary

| Platform | Variant | Why this fits | Native fidelity |
|----------|---------|---------------|-----------------|
| KDE/Breeze | `Frames` (15 frames) | Legacy sprite sheet format IS a frame sequence; modern KDE uses rotation but sprite sheet is still shipped | High — theme's own sprite sheet frames |
| macOS | `Frames` (12 frames) | Native IS ~12 discrete opacity steps | High — matches real `NSProgressIndicator` |
| Windows | `Frames` (~60 frames) | Arc expansion/contraction over 2-second cycle with easing baked into frame geometry | Good — 30 fps matches native 2-second cycle smoothly |
| GNOME (libadwaita 1.6+) | `Frames` (~20 frames) | Procedural arcs, no extractable format | Good — generated frames approximate `AdwSpinner` |
| GNOME (GTK3/4) | `Transform::Spin` | Native IS CSS rotation of static SVG | Exact match — continuous rotation |
| Bundled Material | `Frames` (~12 frames) | Circular stroke animation has varying arc length | Good — matches Material Design style |
| Bundled Lucide | `Transform::Spin` | Icon is designed for rotation | Exact match — smooth continuous spin |

### What needs to be built

| Component | Work |
|-----------|------|
| `AnimatedIcon` enum + `TransformAnimation` + `Repeat` | New types in `model/icons.rs` |
| `loading_indicator(icon_set: &str)` | New public function in `lib.rs` |
| Freedesktop sprite sheet parser | Parse `animations/*/process-working.svg`, slice into frames |
| Freedesktop symbolic rotation detection | Detect `process-working-symbolic` (non-sprite-sheet), return `Transform::Spin` |
| macOS spinner SVG frames | Design 12 SVGs matching the spoke/fin style |
| Windows spinner SVG frames | Design ~60 SVGs matching the arc/stroke 2-second cycle (two phases: arc grow then shrink, 900° total rotation) with easing in geometry |
| Material spinner SVG frames | Design ~12 SVGs of the circular stroke animation |
| Bundled frame sets | `include_bytes!()` for non-freedesktop platforms |
| gpui connector: frame cycling | Handle `Frames` → `Vec<ImageSource>` + timer |
| gpui connector: rotation support | Handle `Transform::Spin` → animated rotation transform |
| iced connector: frame cycling | Handle `Frames` → `Vec<svg::Handle>` + timer |
| iced connector: rotation support | Handle `Transform::Spin` → animated rotation transform |
| Remove `StatusLoading` from `IconRole` | Remove the variant; bump minor version (pre-1.0) |
| `prefers_reduced_motion() -> bool` | New public function in `lib.rs`, per-platform OS query |

### Resolved questions

1. **`StatusLoading` removal: remove, don't deprecate.** The crate is pre-1.0 (v0.3.3), so breaking changes are expected on minor version bumps. Deprecation would leave a confusing variant that returns `None` on 2 of 3 platform icon sets (SF Symbols, Segoe) with no path to recovery. The `loading_indicator()` API is the proper replacement for the animated use case. For the rare case where an app wants a static loading icon (e.g., next to a "Loading..." label), `bundled_icon_by_name("progress_activity")` or `bundled_icon_by_name("loader")` provides that without `IconRole` involvement.

2. **Feature gating: yes, aligned with existing icon set features — no new features needed.** Bundled spinner frames follow the same gating as their parent icon set:
   - Material spinner frames → `material-icons` feature
   - Lucide spinner → `lucide-icons` feature
   - macOS/Windows/GNOME approximation frames → `system-icons` feature (these are bundled platform-native approximations, used when the native API can't provide animation data)
   - Freedesktop sprite sheet parsing → `system-icons` feature (runtime parsing, no bundled data)

   This mirrors the existing pattern exactly — `load_icon("material", ...)` requires `material-icons`, so `loading_indicator("material")` requires the same feature. Zero new feature flags.

3. **ViewBox coordinate space: 24×24 for all bundled spinner frames.** The viewBox is not a display resolution — it defines the internal coordinate system in which SVG paths are authored. An SVG with `viewBox="0 0 24 24"` renders at any pixel size the connector requests (48px, 96px, etc.) because SVG is vector. The actual rendered size is always determined at display time by the connector based on screen resolution, DPI, and icon size — this is inherent to SVG and requires no special handling. The 24×24 coordinate space is chosen because it's the most widely used icon viewBox convention (matches Lucide, most icon libraries), provides sufficient precision for spinner geometry (arcs, strokes, lines), and keeps path coordinates simple. All spinner frames are new assets authored by the project — even Material-style spinners use 24×24 rather than Material's 960×960, because they're a distinct asset category from the static icons and consistency across all spinner frames simplifies authoring and testing.

4. **Easing: not now, extensible later.** Both current `TransformAnimation::Spin` use cases (GtkSpinner CSS `animation: spin 1s linear infinite`, Lucide's symmetric loader icon) use linear rotation. Neither gpui nor iced expose built-in easing primitives, so connectors would need to implement easing from scratch with no current benefit. `TransformAnimation` is marked `#[non_exhaustive]`, so a future `SpinEased { duration_ms, easing: Easing }` variant can be added without breaking changes when a concrete use case arises.

5. **Reduced motion: always return `AnimatedIcon`, connectors decide presentation policy.** `loading_indicator()` always returns the animation data regardless of OS accessibility settings. This cleanly separates concerns:
   - **native-theme core**: provides animation data (what to show) via `loading_indicator()`
   - **native-theme core**: exposes OS preference via a new `prefers_reduced_motion() -> bool` function, analogous to the existing `system_is_dark() -> bool`
   - **Connector/app**: decides whether to animate based on the preference (show `frames[0]` as static image, or stop rotation)

   This matches how web browsers handle `prefers-reduced-motion` — the CSS animation data exists unconditionally; the browser decides whether to play it. Option (a) (returning static fallback) would conflate data loading with accessibility policy, and option (c) (parameter) would push the OS query to the caller and cache a preference that can change at runtime.

6. **No `loading_indicator_sized` variant.** All bundled spinner frames use SVG (`IconData::Svg`), which is resolution-independent — connectors rasterize at whatever pixel size they need. Freedesktop sprite sheet frames are also extracted as individual SVG slices (via viewBox manipulation), not as pre-rasterized RGBA. A sized variant would only be needed if we returned pre-rasterized `IconData::Rgba` frames, which we don't for any current platform. If a future platform requires RGBA frames, a `loading_indicator_sized(icon_set: &str, size: u32)` can be added without breaking changes.
