# Full per-widget theming for the gpui connector

Status: Pending
Date: 2026-04-08

---

## 1 -- The problem

The gpui connector (`native-theme-gpui`) maps native-theme's resolved data to
gpui-component's theming system. The mapping is lopsided:

- **Colors: well-covered.** 108 `ThemeColor` fields are populated. Button
  background, hover, active; scrollbar thumb; slider fill; tab active -- all
  mapped directly from per-widget resolved data.

- **Global geometry: covered.** `radius`, `radius_lg`, `shadow`, `font_family`,
  `font_size`, `mono_font_family`, `mono_font_size` -- all mapped via
  `ThemeConfig`.

- **Per-widget geometry: not mapped.** Button padding, min-height, border
  color, corner radius; input padding; scrollbar width; slider track height;
  checkbox indicator size -- none of these reach the rendered UI. They exist in
  `ResolvedThemeVariant` but gpui-component's `Theme` / `ThemeConfig` has no
  fields to receive them.

This means a KDE Breeze button in the gpui showcase has the correct background
color (#292c30) and hover color (#93cee9), but uses gpui-component's hardcoded
padding (16px instead of 6px), hardcoded height logic, and no visible 1px
border stroke with the correct per-widget border color (#535659). The result
looks noticeably different from a real KDE System Settings button.

### What native-theme provides vs what the connector uses

| Widget | native-theme fields | Connector uses | Gap |
|--------|---------------------|----------------|-----|
| button | background_color, font.color, primary_background, primary_text_color, hover_background, hover_text_color, active_background, active_text_color, disabled_background, disabled_text_color, **border.color**, **border.corner_radius**, **border.line_width**, **border.padding_horizontal**, **border.padding_vertical**, **min_width**, **min_height**, icon_text_gap, disabled_opacity | 6 colors | 11 geometry fields |
| input | background_color, font.color, border.color, caret_color, hover_border_color, focus_border_color, disabled_background, disabled_text_color, **border.padding_horizontal**, **border.padding_vertical**, **min_height**, border.corner_radius | 2 colors | 4 geometry fields |
| checkbox | indicator_width, label_gap, border colors, disabled states | 0 | all fields |
| scrollbar | track_color, thumb_color, thumb_hover_color, **groove_width**, **min_thumb_length**, **thumb_width**, overlay_mode | 3 colors | 4 geometry fields |
| slider | fill_color, thumb_color, **track_height**, **thumb_diameter**, **tick_mark_length** | 2 colors | 3 geometry fields |
| tab | background_color, active_background, active_text_color, bar_background, font.color, **min_width**, **min_height**, **border.padding_horizontal**, **border.padding_vertical** | 5 colors | 4 geometry fields |
| menu | row_height, icon_text_gap, icon_size, border padding, colors | 0 | all fields |
| tooltip | background_color, font.color, **max_width**, **border padding** | 0 | all fields |
| dialog | button_order, button_gap, min/max dimensions, border padding, icon_size, colors | dialog_content_padding + button_spacing helpers | most geometry |
| progress_bar | fill_color, **track_height**, **min_width** | 1 color | 2 geometry fields |
| switch | unchecked_background, thumb_background, **track_width**, **track_height**, **thumb_diameter**, **track_radius** | 2 colors | 4 geometry fields |
| toolbar | **bar_height**, **item_gap**, icon_size, colors | 0 | all geometry |
| list | alternate_row, hover_background, selection_background, **row_height**, **border padding** | 3 colors | 3 geometry fields |
| spinner | **diameter**, **min_diameter**, **stroke_width** | 0 | 3 geometry fields |
| combo_box | **min_height**, **min_width**, **arrow_icon_size**, **arrow_area_width**, border padding | 0 | 5 geometry fields |
| splitter | **divider_width**, colors | 0 | 1 geometry field |
| separator | **line_width**, colors | 0 | 1 geometry field |
| segmented_control | **segment_height**, **separator_width**, border padding | 0 | 3 geometry fields |
| expander | **header_height**, **arrow_icon_size** | 0 | 2 geometry fields |
| layout | **widget_gap**, **container_margin**, **window_margin**, **section_gap** | 0 | 4 geometry fields |

**Bold** = geometry fields that exist in native-theme but cannot be mapped.

Total: ~65 per-widget geometry fields with no path to the rendered UI.


## 2 -- Why the gap exists

### 2.1 gpui vs gpui-component

**gpui** is a low-level UI framework. It provides `div()`, the `Styled` trait
(`.px()`, `.bg()`, `.rounded()`), flexbox layout, event handling, text
rendering, and window management. It has no pre-built widgets.

**gpui-component** is a widget library built on gpui. It provides `Button`,
`Input`, `Checkbox`, `Tabs`, `Slider`, `Switch`, `Modal`, `Popover`, `Table`,
and ~30 other widgets. Each widget is a composition of gpui `div()` elements
with state management, theme color reads, and accessibility features.

### 2.2 How gpui-component themes work

gpui-component's `Theme` struct has two layers:

1. **`ThemeColor`** -- a flat bag of 108 named HSLA colors. No geometry. No
   per-widget structure. Button, input, tab, scrollbar each get 2-5 color
   slots. Our connector populates all 108.

2. **`ThemeConfig`** -- serializable config with font family, font size,
   radius, radius_lg, shadow, mode, name, colors (as hex strings), and
   syntax highlighting. No per-widget geometry fields.

gpui-component's individual widget implementations (Button, Input, Checkbox,
etc.) read colors from `ThemeColor` via `cx.theme()`, but hardcode their own
geometry. For example, Button's height and padding come from its `Size` enum
(XSmall/Small/Medium/Large), not from the theme.

Our connector maps `ResolvedThemeVariant` -> `ThemeColor` + `ThemeConfig` ->
`Theme`. Since neither target type has per-widget geometry fields, there is
nowhere to put the data.

### 2.3 What gpui-component's Button looks like inside

gpui-component's Button is 984 lines of Rust (1,733 total for the button
module including ButtonGroup, Toggle, DropdownButton). It provides:

- 9 color variants (Primary, Secondary, Danger, Success, Warning, Info,
  Ghost, Link, Text) with 5 states each (normal, hover, active, selected,
  disabled)
- Keyboard navigation (Tab, Enter, Space)
- Focus ring, loading spinner, tooltip
- Icon + label layout with auto gap sizing
- Compact mode, outline mode
- ButtonGroup compatibility (shared borders, clipped inner corners)

The hardcoded geometry lives in ~20 lines of the render() method:

```rust
// Button padding -- hardcoded per Size enum:
match self.size {
    Size::XSmall => this.h_5().px_1(),       // h=20px, pad_h=4px
    Size::Small  => this.h_6().px_3(),       // h=24px, pad_h=12px
    _            => this.h_8().px_4(),       // h=32px, pad_h=16px
}

// Border radius -- reads global theme.radius, not per-widget:
let rounding = match self.rounded {
    ButtonRounded::Medium => cx.theme().radius,   // global defaults.border.corner_radius
    ...
};

// Border color -- reads global theme.border, not per-widget:
fn border_color(&self, ...) -> Hsla {
    cx.theme().border   // global defaults.border.color, not button.border.color
}
```

KDE Breeze wants `h=32px, pad_h=6px, pad_v=6px, border=#535659, radius=5px`.
Button gives `h=32px, pad_h=16px, border=defaults.border.color, radius=theme.radius`.

### 2.4 Style override capability

Button implements gpui's `Styled` trait. Its render() method calls
`refine_style(&self.style)` as the **last** step, meaning user-provided style
overrides are applied on top of the widget's internal styles. Padding,
min-size, border-width, and border-radius CAN be overridden. But:

- **Per-widget border color cannot be overridden.** The hover/active/disabled
  state handlers set border_color from `cx.theme().border` (the global color).
  These fire AFTER refine_style in gpui's conditional style layers, so a
  border_color override on the base style gets overwritten by the hover state.

- **Icon-label gap cannot be overridden.** It is set on an inner `h_flex()`
  child element, not on the root div.

- **Icon sizing cannot be overridden.** It is set on the `Icon` child element.


## 3 -- Options

### Option A: Upstream ThemeConfig extension

Submit a proposal to gpui-component to add optional per-widget metric fields
to `ThemeConfig`. Each widget reads from the config with fallback to its
current hardcoded default.

**What changes in gpui-component:**

```rust
pub struct ThemeConfig {
    // ... existing fields ...

    // Per-widget metrics (all Option -- None = use hardcoded default)
    pub button_height: Option<f32>,
    pub button_padding_h: Option<f32>,
    pub button_padding_v: Option<f32>,
    pub button_border_width: Option<f32>,
    pub button_corner_radius: Option<f32>,
    pub input_height: Option<f32>,
    pub input_padding_h: Option<f32>,
    pub checkbox_size: Option<f32>,
    pub scrollbar_width: Option<f32>,
    pub slider_track_height: Option<f32>,
    pub slider_thumb_size: Option<f32>,
    pub tab_height: Option<f32>,
    pub tooltip_padding: Option<f32>,
    // ... ~30 more fields ...
}
```

Each widget reads from the theme:
```rust
fn height(&self, cx: &App) -> Pixels {
    cx.theme().button_height.unwrap_or(px(32.))
}
```

**What changes in our connector:**

`to_theme_config()` maps resolved per-widget metrics to ThemeConfig fields:
```rust
ThemeConfig {
    button_height: Some(resolved.button.min_height),
    button_padding_h: Some(resolved.button.border.padding_horizontal),
    scrollbar_width: Some(resolved.scrollbar.groove_width),
    ...
}
```

**Pros:**
- Transparent to applications -- widgets automatically use native sizing
- No application code changes needed
- Clean separation: connector sets config, widgets read it
- Backward-compatible: all new fields are Option with fallback
- Simplest possible user-facing API: just call `from_preset()` and everything
  works

**Cons:**
- Requires upstream acceptance (gpui-component is a third-party project)
- ~40 fields to add to ThemeConfig, plus changes to every widget's render()
- Timeline entirely depends on upstream maintainers
- gpui-component may not want this complexity -- their audience may not care
  about platform-native sizing
- If rejected, time spent on the proposal is wasted

**Status:** A draft proposal exists at
`connectors/native-theme-gpui/proposals/README.md`. Not yet submitted.


### Option B: Raw div replacement

Replace gpui-component widgets with raw `div()` compositions that read all
styling from `ResolvedThemeVariant`.

```rust
div()
    .min_w(px(button_min_size(&resolved).0))
    .min_h(px(button_min_size(&resolved).1))
    .px(px(button_padding(&resolved).0))
    .py(px(button_padding(&resolved).1))
    .rounded(px(button_corner_radius(&resolved)))
    .border_1()
    .border_color(rgba_to_gpui(button_border_color(&resolved)))
    .bg(theme.colors().secondary)
    .child("Click")
```

**Pros:**
- Works immediately, no upstream dependency
- Full control over every visual detail
- Demonstrates native-theme's complete data model

**Cons:**
- Loses all widget behavior: keyboard navigation, focus ring, tab indexing,
  hover/active/disabled state machines, loading spinners, tooltips
- Must reimplement ~1,700 lines per widget to match gpui-component behavior
- Every application using the connector must also reimplement widgets or
  accept raw divs without behavior
- Massive maintenance burden
- Terrible user-facing API: users build raw layouts instead of calling
  `Button::new("id").label("Click")`


### Option C: Style overrides on gpui-component widgets

Use gpui-component's widgets but apply style overrides via the `Styled` trait.
Keep the widget's behavior while overriding visual dimensions.

```rust
Button::new("id")
    .label("Click")
    .px(px(6.0))
    .py(px(6.0))
    .min_h(px(32.0))
    .min_w(px(80.0))
    .rounded(px(5.0))
    .border_1()
```

**Pros:**
- Preserves widget behavior (focus, hover, keyboard, accessibility)
- Works today, no upstream dependency
- Minimal code changes

**Cons:**
- Per-widget border COLOR cannot be overridden -- hover/active state handlers
  overwrite it from `cx.theme().border` (the global color)
- Icon-label gap cannot be overridden (set on inner child element)
- Icon sizing cannot be overridden (set on inner child element)
- Fragile: gpui-component internal changes can silently break overrides.
  A future version might reorder its style application, change its div
  nesting, or add new internal styles that conflict
- No compile-time guarantee that overrides are effective -- breakage is
  silent visual regression
- Ugly user-facing API: every button creation requires ~6 extra style calls
  with resolved metrics threaded through. Users must remember to apply
  overrides everywhere, and forgetting one instance silently produces wrong
  visuals:
  ```rust
  // Every single button in the entire application:
  Button::new("save").label("Save").primary()
      .px(px(pad_h)).py(px(pad_v)).min_h(px(h)).min_w(px(w))
      .rounded(px(r)).border_1()
  ```
- Style override code must be repeated at every call site, or wrapped in a
  helper that returns a Button -- but that helper is essentially a thin
  wrapper around gpui-component's Button, adding indirection without solving
  the underlying problem


### Option D: Fork all of gpui-component

Fork the entire gpui-component repository. Modify widget implementations to
read metrics from ThemeConfig or a custom extension. Maintain the fork as a
patched dependency.

**Pros:**
- Full control, no upstream blocking
- Can implement exactly what's needed
- Can be contributed back upstream later
- All widgets get themed consistently

**Cons:**
- Maintaining a fork of a 50,000+ line UI library
- Every upstream release requires merge conflict resolution across the
  entire codebase, not just the widgets we modified
- Divergence makes future upstream contributions harder -- the fork drifts
- Confusing for users: "which gpui-component do I depend on?"
- Our project's scope is a theme library, not a widget library. A full fork
  changes the project's nature


### Option E: Status quo + documentation

The connector already returns `(Theme, ResolvedThemeVariant)`. Users access
per-widget metrics directly from `ResolvedThemeVariant`. Add comprehensive
documentation showing how.

```rust
let (theme, resolved) = from_preset("kde-breeze", true)?;
let btn_pad_h = resolved.button.border.padding_horizontal; // 6.0
let btn_border = resolved.button.border.color;              // #535659
```

**Pros:**
- Zero code changes beyond documentation
- Users already have full access to all data
- No upstream dependency
- No maintenance burden

**Cons:**
- Shifts the entire burden to every application developer
- No visual difference in the showcase -- our own demo looks wrong
- The most common use case (make widgets look native) remains completely
  unsolved
- Documentation alone does not make an API. Users must figure out HOW to
  apply each metric to each widget, discovering the same limitations
  (border color override doesn't work, icon gap can't be changed) by trial
  and error


### Option F: Selective widget fork into the connector

Copy individual widget source files from gpui-component into
`native-theme-gpui` and modify them to read all metrics from
`ResolvedThemeVariant`. Only fork the widgets where the visual difference
matters (Button, Input, Checkbox, Slider, Switch, Scrollbar). Continue
using gpui-component's other widgets (Modal, Popover, Table) unmodified.

**How it works:**

```rust
// In native-theme-gpui/src/widgets/button.rs
// (copied from gpui-component/src/button/button.rs, ~984 lines)
pub struct NativeButton { ... }
```

The forked widget reads metrics from a stored reference to resolved theme
data instead of hardcoded values:

```rust
// BEFORE (gpui-component original, hardcoded):
match self.size {
    _ => this.h_8().px_4(),  // 32px height, 16px padding
}
let rounding = cx.theme().radius;  // global radius
let border = cx.theme().border;    // global border color

// AFTER (forked, theme-driven, ~60 lines changed):
let m = &self.metrics;
this.min_h(px(m.min_height))
    .px(px(m.border.padding_horizontal))
    .py(px(m.border.padding_vertical))
let rounding = px(m.border.corner_radius);     // per-widget radius
let border = rgba_to_hsla(m.border.color);     // per-widget border color
```

The variant color system (9 variants x 5 states = 45 match arms reading
`cx.theme().primary`, `cx.theme().danger`, etc.) stays unchanged -- those
colors already come from our connector via ThemeColor.

**How the widget gets theme data -- two sub-options:**

**F1. Constructor parameter (explicit):**
```rust
NativeButton::new("save", &resolved.button)
    .label("Save")
    .primary()
```
Pro: clear data flow, no hidden state. Con: every call site passes metrics.

**F2. gpui global state (implicit, same pattern as cx.theme()):**
```rust
// At app init (once):
cx.set_global(resolved.clone());

// Widget reads it in render():
let metrics = cx.global::<ResolvedThemeVariant>().button;

// User code -- identical to gpui-component's API:
NativeButton::new("save")
    .label("Save")
    .primary()
```
Pro: clean API, no per-call-site boilerplate. Con: requires global setup.

**What gets forked vs what stays:**

| Widget | Lines | Visual sensitivity | Fork? |
|--------|-------|--------------------|-------|
| Button + ButtonGroup | 1,238 | High -- padding, border, radius all wrong | Yes |
| Input | ~600 | High -- padding, height, border | Yes |
| Checkbox | ~400 | Medium -- indicator size, gap | Yes |
| Slider | ~500 | Medium -- track height, thumb size | Yes |
| Switch | ~400 | Medium -- track/thumb dimensions | Yes |
| Scrollbar | ~300 | Medium -- width, thumb size | Maybe |
| Tab | ~500 | Low-medium -- padding, height | Maybe |
| All others | ~15,000 | Low -- mostly color-driven | No, use gpui-component |

Estimated: ~3,100 lines copied for the core 5 widgets, ~300 lines of
actual modifications (the rest is kept verbatim).

**Keeping the widget's builder API identical** means the user-facing code
looks the same as gpui-component:

```rust
// gpui-component original:
Button::new("save").label("Save").primary()

// Our NativeButton (F2 variant):
NativeButton::new("save").label("Save").primary()
```

The only difference is the import. All builder methods (`.label()`,
`.primary()`, `.icon()`, `.compact()`, `.loading()`, `.tooltip()`,
`.on_click()`) work identically because the code is the same -- only
the render() geometry reads are changed.

**Pros:**
- Full theme fidelity for the widgets that matter most
- Preserves ALL widget behavior: focus ring, keyboard navigation,
  hover/active/disabled states, loading spinners, tooltips, accessibility
- Per-widget border color works (unlike Option C)
- Icon-label gap reads from theme (unlike Option C)
- No upstream dependency or blocking
- Scope is bounded: 5-7 widgets, not the entire library
- User-facing API is simple (especially F2 variant -- identical to
  gpui-component except the type name)
- Changes are concentrated in ~60 lines per widget, not scattered across
  the entire codebase
- Forked widgets can be contributed back upstream as a reference
  implementation for Option A

**Cons:**
- Initial copy of ~3,100 lines into the connector crate
- Must track upstream changes for the forked widgets. When gpui-component
  releases a new version, each forked widget needs a diff review and merge
  of non-conflicting changes. Since our modifications are concentrated in
  ~60 lines of the render() method, most upstream changes (new builder
  methods, new variants, accessibility improvements) merge cleanly
- Two button types exist in the ecosystem: `gpui_component::Button` and
  `native_theme_gpui::NativeButton`. Users must know which to use. (Mitigated
  by re-exporting NativeButton as Button from the connector crate)
- ButtonGroup and DropdownButton may need forking too if they compose Button
  internally (they do -- ButtonGroup creates Button children)
- If gpui-component fundamentally restructures a widget (rare but possible),
  the fork requires a significant re-merge


## 4 -- Recommendation

### The key insight

We are building a theme library. Our users will create thousands of buttons,
inputs, checkboxes, and sliders across their applications. The API for widget
creation is the most repeated code pattern in any GUI application:

```rust
// This line appears hundreds of times in a real application:
Button::new("id").label("Save").primary()
```

Any option that adds per-call-site boilerplate (Option C's style overrides,
Option F1's constructor parameter) multiplies friction across every widget
in every application. A user who forgets to apply the overrides at one call
site gets a silently wrong-looking button. The cost of that friction is not
paid once during implementation -- it is paid every time anyone writes a
button, forever.

Conversely, any option that makes the happy path identical to the current API
(Option A's transparent upstream integration, Option F2's global state) pays
its implementation cost once and then gets out of the user's way.

### Why Option F2 is the right answer

**Option A is the ideal end state** but we cannot ship it. It depends on
upstream acceptance, and even if accepted, the timeline is unbounded. We
cannot ask our users to wait for a third-party project to implement our
proposal.

**Option C seems cheap but is not.** Style overrides cannot fix per-widget
border color (the primary visual issue that motivated this investigation).
They are fragile -- a gpui-component update can silently break them. And they
add ~6 lines of boilerplate to every button creation, multiplied across every
widget in every application. The low implementation cost is offset by
permanently higher usage cost.

**Option D is disproportionate.** Forking 50,000+ lines to change 300 is a
maintenance trap. The scope of our project is theming, not widget libraries.

**Option F2 (selective widget fork with global state) is the sweet spot:**

1. **User-facing API is one import change.** Replace
   `use gpui_component::button::Button` with
   `use native_theme_gpui::Button`. The rest of the code is identical.
   This is the simplest possible API for users.

2. **Full theme fidelity.** Per-widget border color, padding, corner radius,
   icon-label gap, min dimensions -- all read from the resolved theme. No
   limitations, no caveats, no "works except for X".

3. **Full widget behavior.** The forked code IS gpui-component's code with
   ~60 lines changed per widget. Focus rings, keyboard navigation,
   hover/active/disabled states, loading spinners, tooltips -- all work
   exactly as they do in gpui-component because they ARE gpui-component.

4. **Bounded scope.** We fork 5-7 widgets (~3,100 lines), not the entire
   library. The other ~30 widgets continue to come from gpui-component
   unmodified.

5. **Maintenance is manageable.** Our changes are concentrated in the
   render() method of each widget. Upstream changes to builder methods,
   new variants, accessibility improvements, or new features merge cleanly
   because they don't touch the same ~60 lines we modified.

6. **Path to Option A.** If upstream accepts the proposal, we delete our
   forked widgets and switch back to gpui-component. The user-facing API
   does not change (we re-export gpui-component's Button as our Button
   either way). Option F2 is a bridge, not a dead end.

### Execution plan

**Step 1: Infrastructure.**
Store `ResolvedThemeVariant` as a gpui global so widgets can access it
via `cx.global::<NativeThemeState>()` in their render() methods. Set it
up in `to_theme()` or via a one-time init function.

**Step 2: Fork the 5 core widgets.**
Copy Button, Input, Checkbox, Slider, Switch from gpui-component into
`native-theme-gpui/src/widgets/`. Modify each widget's render() method
to read geometry from the global resolved theme (~60 lines per widget).
Keep the builder API identical.

**Step 3: Re-export.**
`pub use widgets::button::NativeButton as Button;` from the connector
crate's lib.rs. Users import `native_theme_gpui::Button` instead of
`gpui_component::button::Button`.

**Step 4: Update the showcase.**
Replace gpui-component widget imports with native-theme-gpui widget
imports. The showcase code itself barely changes -- just the imports.

**Step 5: Submit upstream proposal (Option A).**
Include before/after screenshots. If accepted, delete the forked widgets
and switch re-exports back to gpui-component.

### Not recommended

- **Option B** (raw div replacement) -- loses all widget behavior, terrible
  user API, massive rewrite.
- **Option C** (style overrides) -- cannot fix border color, fragile,
  per-call-site boilerplate that scales poorly.
- **Option D** (full fork) -- disproportionate scope, maintenance trap.
- **Option E** (status quo) -- leaves the core problem unsolved, shifts
  burden to every user.
- **Option F1** (fork with constructor parameter) -- same fork benefits as
  F2 but with per-call-site boilerplate that scales poorly.


## 5 -- Scope estimate

| Item | Files | Lines |
|------|-------|-------|
| Global state infrastructure | `lib.rs`, new `state.rs` | ~50 |
| Button fork + modification | `widgets/button.rs` | ~1,000 (copy) + ~60 (modify) |
| ButtonGroup fork | `widgets/button_group.rs` | ~250 (copy) + ~20 (modify) |
| Input fork + modification | `widgets/input.rs` | ~600 (copy) + ~40 (modify) |
| Checkbox fork + modification | `widgets/checkbox.rs` | ~400 (copy) + ~30 (modify) |
| Slider fork + modification | `widgets/slider.rs` | ~500 (copy) + ~30 (modify) |
| Switch fork + modification | `widgets/switch.rs` | ~400 (copy) + ~30 (modify) |
| Re-exports in lib.rs | `lib.rs` | ~20 |
| Showcase import changes | `showcase-gpui.rs` | ~30 (import swaps) |
| Tests | `widgets/tests.rs` | ~100 |
| **Total new code** | | **~330 lines written** |
| **Total copied code** | | **~3,150 lines** |

The upstream proposal (Option A) is a separate document already written at
`connectors/native-theme-gpui/proposals/README.md` and can be submitted
in parallel.
