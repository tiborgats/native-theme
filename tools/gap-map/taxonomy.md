# Capability taxonomy for the v0.7.1 gap map

Each gap row is tagged with every capability it needs from the model (one or more IDs).
A capability is something the MODEL must be able to express; a row may need several
(e.g. "checked+hover border colour of the switch" = C07 + C12).

| ID | Capability the model lacks |
|---|---|
| C01 | Shadow geometry: layers, each with offset, blur, spread, colour, inset; per surface |
| C02 | Elevation: a named elevation ramp/levels per element, elevation per state, surface tint |
| C03 | Materials: translucency, backdrop blur, vibrancy, Mica/Acrylic, Liquid Glass, their tint/noise/luminosity and opaque fallbacks |
| C04 | Colour composition: a colour stated relative to what is behind it (alpha fill, mix of current text colour, overlay) rather than one resolved opaque colour |
| C05 | Colour roles missing from defaults (accent ramp, standalone accent, status background/foreground, tertiary, container/surface ladder, inverse, header/complementary colour sets, shade roles) |
| C06 | Colour variant axes beyond light/dark: increased/high contrast values, inactive/non-key window, elevated level, contrast levels |
| C07 | States and state combinations: pressed, focused, dragged, drop target, indeterminate, read-only, invalid/error, default, open, checked/selected × hover/pressed/disabled, per-state colours of a part |
| C08 | State-layer model: hover/pressed/focus/drag overlay opacities, ripple |
| C09 | Widget variants/styles: flat, outlined, filled, tonal, destructive, pill, circular, bezel styles, field styles, segmented styles, glass styles |
| C10 | Size variants and density: control sizes (mini…extraLarge), compact density, touch vs mouse sizing |
| C11 | Shape: per-corner radii, continuous corner curve, capsule/fully rounded, concentric radii, shape change between states |
| C12 | Border/outline model: per-side widths and colours, gradient strokes, inner+outer outlines, outline drawn outside the box, state-dependent border thickness |
| C13 | Parts missing from a widget the model has: icons, glyphs, labels, helper text, counters, headers, footers, close buttons, indicators/pills, ticks, grabbers, arrows, their geometry and colours |
| C14 | Motion: durations, easing curves, springs, delays per transition, a speed factor, reduced-motion replacements |
| C15 | Focus indication: style (ring stroked/filled, inner/outer, underline, rounded rect), width, offset, keyboard-only rule, per-widget focus style, animation |
| C16 | Text caret: colour, width, blink period / on-off / timeout, style |
| C17 | Selection and highlights: inactive/unemphasised selection, its text colour, find highlight, drag-and-drop highlight |
| C18 | Typography: letter spacing, line height per role, optical size / variable axes, font width, missing text styles, per-size text tables (Dynamic Type), font features, title fonts |
| C19 | Icons: state effects (disabled/hover/active), symbolic recolouring, size groups per context, rendering modes, symbol effects |
| C20 | Window chrome: title bar geometry and fills, caption/traffic-light buttons and their states, button layout, window corner radius, window border/outline, window shadow, maximised/tiled/snapped/fullscreen states, unfocused-window appearance |
| C21 | Visible timings: double-click, drag threshold, submenu/hover-open delays, tooltip show/hide/reshow, scrollbar auto-hide, toast durations |
| C22 | Pointer: cursor theme and size, pointer hover effects (iPad) |
| C23 | Accessibility variants as values: what high contrast, reduce transparency, bold text, button shapes, differentiate without colour, larger text change |
| C24 | A whole control the model lacks (the row is in §4.2) |
| C25 | Reader coverage: a value readable at run time that no reader reads today |
| C26 | Layout and placement: alignment, ordering, positions, insets, spacing rules not tied to one widget part |

Also mark each row's source kind as given in the row: R (read at run time), P (published), M (measure), V (to verify), or combinations.

Added after tagging (applied by script from each row's own text):

| ID | Capability the model lacks |
|---|---|
| C27 | Gradient paint: linear/radial gradient fills and strokes, bevels and highlights drawn as gradients |
| C28 | Colour space and precision: wide gamut (Display P3), HDR, more than 8 bits per channel |
