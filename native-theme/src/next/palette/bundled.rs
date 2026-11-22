//! The bundled palettes (spec §7, "Tier 1: `Palette`"), behind the `palettes`
//! feature.

use std::sync::Arc;

use super::{Color, Palette, Scheme};

/// The colour of `rgba`, 8-bit sRGB `0xRRGGBBAA`, through [`Color::from_srgb8`].
fn srgb8(rgba: u32) -> Color {
    let [r, g, b, a] = rgba.to_be_bytes();
    Color::from_srgb8(r, g, b, a)
}

impl Palette {
    /// The Catppuccin Latte palette, for the light scheme, named `catppuccin-latte`.
    ///
    /// Upstream sources:
    /// - catppuccin/catppuccin@d09787dd98ca6fba08af5ef2ae94a7e09f17daca docs/style-guide.md
    ///   §General Usage, §Terminals > Window Colors, §Code Editors > General; values
    ///   catppuccin/palette@v1.8.0 (07d02aa110ef9eb7e7427afca5c73ba9cf7f8ebd) palette.json.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: Base; U: Background Pane | Base
    /// - `view_bg`: Base; G1: = `window_bg`
    /// - `surface_bg`: Surface 0; C2: "Surface Elements | Surface 0, Surface 1, Surface 2", the
    ///   first step from Base
    /// - `header_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `sidebar_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `tooltip_bg`: Surface 0; G2: = `surface_bg`
    /// - `foreground`: Text; U: Body Copy | Text
    /// - `foreground_muted`: Subtext 0; C3: "Sub-Headlines, Labels | Subtext 0, Subtext 1", the
    ///   fainter one
    /// - `border`: Overlay 0; C4: "Inactive Border | Overlay 0"
    /// - `divider`: Overlay 0; G3: = `border`
    /// - `accent`: Blue; C5: "Tags, Pills | Blue" and "Links, URLs | Blue"; the guide names no
    ///   accent
    /// - `accent_fg`: Base; U: On Accent | Base
    /// - `selection_bg`: Overlay 2 at 30% opacity (alpha 0.30); C6: "Selection Background | Overlay
    ///   2 | 20% - 30% Opacity", the upper end
    /// - `selection_fg`: Text; G4: = `foreground`
    /// - `link`: Blue; U: Links, URLs | Blue
    /// - `caret`: Rosewater; U: Cursor | Rosewater
    /// - `error`: Red; U: Errors | Red
    /// - `warning`: Yellow; U: Warnings | Yellow
    /// - `success`: Green; U: Success | Green
    /// - `info`: Teal; U: Information | Teal
    #[must_use]
    pub fn catppuccin_latte() -> Self {
        Self {
            name: Arc::from("catppuccin-latte"),
            scheme: Scheme::Light,
            window_bg: srgb8(0xeff1_f5ff),        // Base
            view_bg: srgb8(0xeff1_f5ff),          // Base
            surface_bg: srgb8(0xccd0_daff),       // Surface 0
            header_bg: srgb8(0xe6e9_efff),        // Mantle
            sidebar_bg: srgb8(0xe6e9_efff),       // Mantle
            tooltip_bg: srgb8(0xccd0_daff),       // Surface 0
            foreground: srgb8(0x4c4f_69ff),       // Text
            foreground_muted: srgb8(0x6c6f_85ff), // Subtext 0
            border: srgb8(0x9ca0_b0ff),           // Overlay 0
            divider: srgb8(0x9ca0_b0ff),          // Overlay 0
            accent: srgb8(0x1e66_f5ff),           // Blue
            accent_fg: srgb8(0xeff1_f5ff),        // Base
            selection_bg: srgb8(0x7c7f_934d),     // Overlay 2 at 30% opacity (alpha 0.30)
            selection_fg: srgb8(0x4c4f_69ff),     // Text
            link: srgb8(0x1e66_f5ff),             // Blue
            caret: srgb8(0xdc8a_78ff),            // Rosewater
            error: srgb8(0xd20f_39ff),            // Red
            warning: srgb8(0xdf8e_1dff),          // Yellow
            success: srgb8(0x40a0_2bff),          // Green
            info: srgb8(0x1792_99ff),             // Teal
        }
    }

    /// The Catppuccin Frappé palette, for the dark scheme, named `catppuccin-frappe`.
    ///
    /// Upstream sources:
    /// - catppuccin/catppuccin@d09787dd98ca6fba08af5ef2ae94a7e09f17daca docs/style-guide.md
    ///   §General Usage, §Terminals > Window Colors, §Code Editors > General; values
    ///   catppuccin/palette@v1.8.0 (07d02aa110ef9eb7e7427afca5c73ba9cf7f8ebd) palette.json.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: Base; U: Background Pane | Base
    /// - `view_bg`: Base; G1: = `window_bg`
    /// - `surface_bg`: Surface 0; C2: "Surface Elements | Surface 0, Surface 1, Surface 2", the
    ///   first step from Base
    /// - `header_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `sidebar_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `tooltip_bg`: Surface 0; G2: = `surface_bg`
    /// - `foreground`: Text; U: Body Copy | Text
    /// - `foreground_muted`: Subtext 0; C3: "Sub-Headlines, Labels | Subtext 0, Subtext 1", the
    ///   fainter one
    /// - `border`: Overlay 0; C4: "Inactive Border | Overlay 0"
    /// - `divider`: Overlay 0; G3: = `border`
    /// - `accent`: Blue; C5: "Tags, Pills | Blue" and "Links, URLs | Blue"; the guide names no
    ///   accent
    /// - `accent_fg`: Base; U: On Accent | Base
    /// - `selection_bg`: Overlay 2 at 30% opacity (alpha 0.30); C6: "Selection Background | Overlay
    ///   2 | 20% - 30% Opacity", the upper end
    /// - `selection_fg`: Text; G4: = `foreground`
    /// - `link`: Blue; U: Links, URLs | Blue
    /// - `caret`: Rosewater; U: Cursor | Rosewater
    /// - `error`: Red; U: Errors | Red
    /// - `warning`: Yellow; U: Warnings | Yellow
    /// - `success`: Green; U: Success | Green
    /// - `info`: Teal; U: Information | Teal
    #[must_use]
    pub fn catppuccin_frappe() -> Self {
        Self {
            name: Arc::from("catppuccin-frappe"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x3034_46ff),        // Base
            view_bg: srgb8(0x3034_46ff),          // Base
            surface_bg: srgb8(0x4145_59ff),       // Surface 0
            header_bg: srgb8(0x292c_3cff),        // Mantle
            sidebar_bg: srgb8(0x292c_3cff),       // Mantle
            tooltip_bg: srgb8(0x4145_59ff),       // Surface 0
            foreground: srgb8(0xc6d0_f5ff),       // Text
            foreground_muted: srgb8(0xa5ad_ceff), // Subtext 0
            border: srgb8(0x7379_94ff),           // Overlay 0
            divider: srgb8(0x7379_94ff),          // Overlay 0
            accent: srgb8(0x8caa_eeff),           // Blue
            accent_fg: srgb8(0x3034_46ff),        // Base
            selection_bg: srgb8(0x949c_bb4d),     // Overlay 2 at 30% opacity (alpha 0.30)
            selection_fg: srgb8(0xc6d0_f5ff),     // Text
            link: srgb8(0x8caa_eeff),             // Blue
            caret: srgb8(0xf2d5_cfff),            // Rosewater
            error: srgb8(0xe782_84ff),            // Red
            warning: srgb8(0xe5c8_90ff),          // Yellow
            success: srgb8(0xa6d1_89ff),          // Green
            info: srgb8(0x81c8_beff),             // Teal
        }
    }

    /// The Catppuccin Macchiato palette, for the dark scheme, named `catppuccin-macchiato`.
    ///
    /// Upstream sources:
    /// - catppuccin/catppuccin@d09787dd98ca6fba08af5ef2ae94a7e09f17daca docs/style-guide.md
    ///   §General Usage, §Terminals > Window Colors, §Code Editors > General; values
    ///   catppuccin/palette@v1.8.0 (07d02aa110ef9eb7e7427afca5c73ba9cf7f8ebd) palette.json.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: Base; U: Background Pane | Base
    /// - `view_bg`: Base; G1: = `window_bg`
    /// - `surface_bg`: Surface 0; C2: "Surface Elements | Surface 0, Surface 1, Surface 2", the
    ///   first step from Base
    /// - `header_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `sidebar_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `tooltip_bg`: Surface 0; G2: = `surface_bg`
    /// - `foreground`: Text; U: Body Copy | Text
    /// - `foreground_muted`: Subtext 0; C3: "Sub-Headlines, Labels | Subtext 0, Subtext 1", the
    ///   fainter one
    /// - `border`: Overlay 0; C4: "Inactive Border | Overlay 0"
    /// - `divider`: Overlay 0; G3: = `border`
    /// - `accent`: Blue; C5: "Tags, Pills | Blue" and "Links, URLs | Blue"; the guide names no
    ///   accent
    /// - `accent_fg`: Base; U: On Accent | Base
    /// - `selection_bg`: Overlay 2 at 30% opacity (alpha 0.30); C6: "Selection Background | Overlay
    ///   2 | 20% - 30% Opacity", the upper end
    /// - `selection_fg`: Text; G4: = `foreground`
    /// - `link`: Blue; U: Links, URLs | Blue
    /// - `caret`: Rosewater; U: Cursor | Rosewater
    /// - `error`: Red; U: Errors | Red
    /// - `warning`: Yellow; U: Warnings | Yellow
    /// - `success`: Green; U: Success | Green
    /// - `info`: Teal; U: Information | Teal
    #[must_use]
    pub fn catppuccin_macchiato() -> Self {
        Self {
            name: Arc::from("catppuccin-macchiato"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x2427_3aff),        // Base
            view_bg: srgb8(0x2427_3aff),          // Base
            surface_bg: srgb8(0x363a_4fff),       // Surface 0
            header_bg: srgb8(0x1e20_30ff),        // Mantle
            sidebar_bg: srgb8(0x1e20_30ff),       // Mantle
            tooltip_bg: srgb8(0x363a_4fff),       // Surface 0
            foreground: srgb8(0xcad3_f5ff),       // Text
            foreground_muted: srgb8(0xa5ad_cbff), // Subtext 0
            border: srgb8(0x6e73_8dff),           // Overlay 0
            divider: srgb8(0x6e73_8dff),          // Overlay 0
            accent: srgb8(0x8aad_f4ff),           // Blue
            accent_fg: srgb8(0x2427_3aff),        // Base
            selection_bg: srgb8(0x939a_b74d),     // Overlay 2 at 30% opacity (alpha 0.30)
            selection_fg: srgb8(0xcad3_f5ff),     // Text
            link: srgb8(0x8aad_f4ff),             // Blue
            caret: srgb8(0xf4db_d6ff),            // Rosewater
            error: srgb8(0xed87_96ff),            // Red
            warning: srgb8(0xeed4_9fff),          // Yellow
            success: srgb8(0xa6da_95ff),          // Green
            info: srgb8(0x8bd5_caff),             // Teal
        }
    }

    /// The Catppuccin Mocha palette, for the dark scheme, named `catppuccin-mocha`.
    ///
    /// Upstream sources:
    /// - catppuccin/catppuccin@d09787dd98ca6fba08af5ef2ae94a7e09f17daca docs/style-guide.md
    ///   §General Usage, §Terminals > Window Colors, §Code Editors > General; values
    ///   catppuccin/palette@v1.8.0 (07d02aa110ef9eb7e7427afca5c73ba9cf7f8ebd) palette.json.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: Base; U: Background Pane | Base
    /// - `view_bg`: Base; G1: = `window_bg`
    /// - `surface_bg`: Surface 0; C2: "Surface Elements | Surface 0, Surface 1, Surface 2", the
    ///   first step from Base
    /// - `header_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `sidebar_bg`: Mantle; C1: "Secondary Panes | Crust, Mantle", the one next to Base
    /// - `tooltip_bg`: Surface 0; G2: = `surface_bg`
    /// - `foreground`: Text; U: Body Copy | Text
    /// - `foreground_muted`: Subtext 0; C3: "Sub-Headlines, Labels | Subtext 0, Subtext 1", the
    ///   fainter one
    /// - `border`: Overlay 0; C4: "Inactive Border | Overlay 0"
    /// - `divider`: Overlay 0; G3: = `border`
    /// - `accent`: Blue; C5: "Tags, Pills | Blue" and "Links, URLs | Blue"; the guide names no
    ///   accent
    /// - `accent_fg`: Base; U: On Accent | Base
    /// - `selection_bg`: Overlay 2 at 30% opacity (alpha 0.30); C6: "Selection Background | Overlay
    ///   2 | 20% - 30% Opacity", the upper end
    /// - `selection_fg`: Text; G4: = `foreground`
    /// - `link`: Blue; U: Links, URLs | Blue
    /// - `caret`: Rosewater; U: Cursor | Rosewater
    /// - `error`: Red; U: Errors | Red
    /// - `warning`: Yellow; U: Warnings | Yellow
    /// - `success`: Green; U: Success | Green
    /// - `info`: Teal; U: Information | Teal
    #[must_use]
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: Arc::from("catppuccin-mocha"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x1e1e_2eff),        // Base
            view_bg: srgb8(0x1e1e_2eff),          // Base
            surface_bg: srgb8(0x3132_44ff),       // Surface 0
            header_bg: srgb8(0x1818_25ff),        // Mantle
            sidebar_bg: srgb8(0x1818_25ff),       // Mantle
            tooltip_bg: srgb8(0x3132_44ff),       // Surface 0
            foreground: srgb8(0xcdd6_f4ff),       // Text
            foreground_muted: srgb8(0xa6ad_c8ff), // Subtext 0
            border: srgb8(0x6c70_86ff),           // Overlay 0
            divider: srgb8(0x6c70_86ff),          // Overlay 0
            accent: srgb8(0x89b4_faff),           // Blue
            accent_fg: srgb8(0x1e1e_2eff),        // Base
            selection_bg: srgb8(0x9399_b24d),     // Overlay 2 at 30% opacity (alpha 0.30)
            selection_fg: srgb8(0xcdd6_f4ff),     // Text
            link: srgb8(0x89b4_faff),             // Blue
            caret: srgb8(0xf5e0_dcff),            // Rosewater
            error: srgb8(0xf38b_a8ff),            // Red
            warning: srgb8(0xf9e2_afff),          // Yellow
            success: srgb8(0xa6e3_a1ff),          // Green
            info: srgb8(0x94e2_d5ff),             // Teal
        }
    }

    /// The Dracula palette, for the dark scheme, named `dracula`.
    ///
    /// Upstream sources:
    /// - dracula/draculatheme.com@fea89dd95cf9ba7d594ba8322f2f083906636e0b content/spec.mdx (=
    ///   draculatheme.com/spec, read 2026-11-22) §Color Palette (Dracula Classic), §UI Color
    ///   Palette (Dracula Classic), §Functional Colors and §UI Design Guidelines.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: Background; U: Main background
    /// - `view_bg`: Background; G1: = `window_bg`
    /// - `surface_bg`: Floating interactive elements; U: Floating interactive elements
    /// - `header_bg`: Background Dark; D1: the UI palette's "Background Dark"; the spec lists it
    ///   without a use
    /// - `sidebar_bg`: Background Dark; D1: the UI palette's "Background Dark"; the spec lists it
    ///   without a use
    /// - `tooltip_bg`: Floating interactive elements; G2: = `surface_bg`
    /// - `foreground`: Foreground; U: Default text
    /// - `foreground_muted`: Comment; D2: "Comments, disabled code"
    /// - `border`: Current Line; U: Borders and Separators: Subtle borders: Use `Current Line`
    ///   color
    /// - `divider`: Current Line; U: Borders and Separators: Subtle borders: Use `Current Line`
    ///   color
    /// - `accent`: Functional Purple; D3: "Focus indicators"; "Focus rings: `Functional Purple` or
    ///   appropriate accent"
    /// - `accent_fg`: Foreground; G6: = `foreground` (4.44:1 on the accent; `window_bg` 3.01:1)
    /// - `selection_bg`: Selection; U: Text selection
    /// - `selection_fg`: Foreground; G4: = `foreground`
    /// - `link`: Functional Cyan; U: Information, links
    /// - `caret`: Foreground; G5: = `foreground`
    /// - `error`: Functional Red; U: Critical actions, alerts; State Indicators: Error: `Functional
    ///   Red`
    /// - `warning`: Functional Orange; U: Warnings, notifications; State Indicators: Warning:
    ///   `Functional Orange`
    /// - `success`: Functional Green; U: Success states; State Indicators: Success: `Functional
    ///   Green`
    /// - `info`: Functional Cyan; U: Information, links; State Indicators: Info: `Functional Cyan`
    #[must_use]
    pub fn dracula() -> Self {
        Self {
            name: Arc::from("dracula"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x282a_36ff),        // Background
            view_bg: srgb8(0x282a_36ff),          // Background
            surface_bg: srgb8(0x3437_46ff),       // Floating interactive elements
            header_bg: srgb8(0x2122_2cff),        // Background Dark
            sidebar_bg: srgb8(0x2122_2cff),       // Background Dark
            tooltip_bg: srgb8(0x3437_46ff),       // Floating interactive elements
            foreground: srgb8(0xf8f8_f2ff),       // Foreground
            foreground_muted: srgb8(0x6272_a4ff), // Comment
            border: srgb8(0x6272_a4ff),           // Current Line
            divider: srgb8(0x6272_a4ff),          // Current Line
            accent: srgb8(0x815c_d6ff),           // Functional Purple
            accent_fg: srgb8(0xf8f8_f2ff),        // Foreground
            selection_bg: srgb8(0x4447_5aff),     // Selection
            selection_fg: srgb8(0xf8f8_f2ff),     // Foreground
            link: srgb8(0x0081_d6ff),             // Functional Cyan
            caret: srgb8(0xf8f8_f2ff),            // Foreground
            error: srgb8(0xde57_35ff),            // Functional Red
            warning: srgb8(0xa395_14ff),          // Functional Orange
            success: srgb8(0x0891_08ff),          // Functional Green
            info: srgb8(0x0081_d6ff),             // Functional Cyan
        }
    }

    /// The Alucard (Dracula's light variant) palette, for the light scheme, named `alucard`.
    ///
    /// Upstream sources:
    /// - dracula/draculatheme.com@fea89dd95cf9ba7d594ba8322f2f083906636e0b content/spec.mdx (=
    ///   draculatheme.com/spec, read 2026-11-22) §Color Palette (Alucard Classic), §UI Color
    ///   Palette (Alucard Classic), §Functional Colors and §UI Design Guidelines.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: Background; U: Main background
    /// - `view_bg`: Background; G1: = `window_bg`
    /// - `surface_bg`: Floating interactive elements; U: Floating interactive elements
    /// - `header_bg`: Background Dark; D1: the UI palette's "Background Dark"; the spec lists it
    ///   without a use
    /// - `sidebar_bg`: Background Dark; D1: the UI palette's "Background Dark"; the spec lists it
    ///   without a use
    /// - `tooltip_bg`: Floating interactive elements; G2: = `surface_bg`
    /// - `foreground`: Foreground; U: Default text
    /// - `foreground_muted`: Comment; D2: "Comments, disabled code"
    /// - `border`: Current Line; U: Borders and Separators: Subtle borders: Use `Current Line`
    ///   color
    /// - `divider`: Current Line; U: Borders and Separators: Subtle borders: Use `Current Line`
    ///   color
    /// - `accent`: Functional Purple; D3: "Focus indicators"; "Focus rings: `Functional Purple` or
    ///   appropriate accent"
    /// - `accent_fg`: Background; G6: = `window_bg` (4.57:1 on the accent; `foreground` 3.48:1)
    /// - `selection_bg`: Selection; U: Text selection
    /// - `selection_fg`: Foreground; G4: = `foreground`
    /// - `link`: Functional Cyan; U: Information, links
    /// - `caret`: Foreground; G5: = `foreground`
    /// - `error`: Functional Red; U: Critical actions, alerts; State Indicators: Error: `Functional
    ///   Red`
    /// - `warning`: Functional Orange; U: Warnings, notifications; State Indicators: Warning:
    ///   `Functional Orange`
    /// - `success`: Functional Green; U: Success states; State Indicators: Success: `Functional
    ///   Green`
    /// - `info`: Functional Cyan; U: Information, links; State Indicators: Info: `Functional Cyan`
    #[must_use]
    pub fn alucard() -> Self {
        Self {
            name: Arc::from("alucard"),
            scheme: Scheme::Light,
            window_bg: srgb8(0xfffb_ebff),        // Background
            view_bg: srgb8(0xfffb_ebff),          // Background
            surface_bg: srgb8(0xefed_dcff),       // Floating interactive elements
            header_bg: srgb8(0xcecc_c0ff),        // Background Dark
            sidebar_bg: srgb8(0xcecc_c0ff),       // Background Dark
            tooltip_bg: srgb8(0xefed_dcff),       // Floating interactive elements
            foreground: srgb8(0x1f1f_1fff),       // Foreground
            foreground_muted: srgb8(0x6c66_4bff), // Comment
            border: srgb8(0x6c66_4bff),           // Current Line
            divider: srgb8(0x6c66_4bff),          // Current Line
            accent: srgb8(0x815c_d6ff),           // Functional Purple
            accent_fg: srgb8(0xfffb_ebff),        // Background
            selection_bg: srgb8(0xcfcf_deff),     // Selection
            selection_fg: srgb8(0x1f1f_1fff),     // Foreground
            link: srgb8(0x0081_d6ff),             // Functional Cyan
            caret: srgb8(0x1f1f_1fff),            // Foreground
            error: srgb8(0xde57_35ff),            // Functional Red
            warning: srgb8(0xa395_14ff),          // Functional Orange
            success: srgb8(0x0891_08ff),          // Functional Green
            info: srgb8(0x0081_d6ff),             // Functional Cyan
        }
    }

    /// The Nord palette, for the dark scheme, named `nord`.
    ///
    /// Upstream sources:
    /// - nordtheme.com/docs/colors-and-palettes (read 2026-11-22, site v0.23.0) §Polar Night nord0,
    ///   §Polar Night nord1, §Snow Storm nord6, §Snow Storm nord5, §Frost nord8, §Polar Night
    ///   nord2, §Snow Storm nord4, §Aurora nord11, §Aurora nord13, §Aurora nord14; values
    ///   nordtheme/nord@v0.2.0 (7eae58e58b2d076a6013ceae303446d172d08a90) src/lesscss/nord.less.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: nord0; U: For dark ambiance designs, it is used for background and area
    ///   coloring
    /// - `view_bg`: nord0; G1: = `window_bg`
    /// - `surface_bg`: nord1; U: For dark ambiance designs it is used for elevated, more prominent
    ///   or focused UI elements like … panels, modals and floating popups like notifications or
    ///   auto completion
    /// - `header_bg`: nord1; N2: "elevated, more prominent or focused UI elements like status bars
    ///   and text editor gutters"
    /// - `sidebar_bg`: nord1; N2: "elevated, more prominent or focused UI elements like … panels"
    /// - `tooltip_bg`: nord1; U: panels, modals and floating popups like notifications or auto
    ///   completion
    /// - `foreground`: nord6; N1: "elevated UI text elements that require more visual attention";
    ///   syntax: "text color for plain text"
    /// - `foreground_muted`: nord5; U: For dark ambiance designs, it is used for more
    ///   subtle/inconspicuous UI text elements that do not need so much visual attention
    /// - `border`: nord1; U: It also works fine for more inconspicuous and passive elements like
    ///   borders or as dropshadow between different components
    /// - `divider`: nord1; G3: = `border`
    /// - `accent`: nord8; U: The bright and shiny primary accent color reminiscent of pure and
    ///   clear ice. Used for primary UI elements with main usage purposes that require the most
    ///   visual attention
    /// - `accent_fg`: nord0; G6: = `window_bg` (6.24:1 on the accent; `foreground` 1.74:1)
    /// - `selection_bg`: nord2; U: For dark ambiance designs, it is used to colorize the currently
    ///   active text editor line as well as selection- and text highlighting color
    /// - `selection_fg`: nord6; G4: = `foreground`
    /// - `link`: nord8; G7: = `accent`
    /// - `caret`: nord4; U: For dark ambiance designs, it is used for UI elements like the text
    ///   editor caret
    /// - `error`: nord11; U: Used for UI elements that are rendering error states like linter
    ///   markers and the highlighting of Git diff deletions
    /// - `warning`: nord13; U: Used for UI elements that are rendering warning states like linter
    ///   markers and the highlighting of Git diff modifications
    /// - `success`: nord14; U: Used for UI elements that are rendering success states and
    ///   visualizations and the highlighting of Git diff additions
    /// - `info`: nord8; G7: = `accent`
    #[must_use]
    pub fn nord() -> Self {
        Self {
            name: Arc::from("nord"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x2e34_40ff),        // nord0
            view_bg: srgb8(0x2e34_40ff),          // nord0
            surface_bg: srgb8(0x3b42_52ff),       // nord1
            header_bg: srgb8(0x3b42_52ff),        // nord1
            sidebar_bg: srgb8(0x3b42_52ff),       // nord1
            tooltip_bg: srgb8(0x3b42_52ff),       // nord1
            foreground: srgb8(0xecef_f4ff),       // nord6
            foreground_muted: srgb8(0xe5e9_f0ff), // nord5
            border: srgb8(0x3b42_52ff),           // nord1
            divider: srgb8(0x3b42_52ff),          // nord1
            accent: srgb8(0x88c0_d0ff),           // nord8
            accent_fg: srgb8(0x2e34_40ff),        // nord0
            selection_bg: srgb8(0x434c_5eff),     // nord2
            selection_fg: srgb8(0xecef_f4ff),     // nord6
            link: srgb8(0x88c0_d0ff),             // nord8
            caret: srgb8(0xd8de_e9ff),            // nord4
            error: srgb8(0xbf61_6aff),            // nord11
            warning: srgb8(0xebcb_8bff),          // nord13
            success: srgb8(0xa3be_8cff),          // nord14
            info: srgb8(0x88c0_d0ff),             // nord8
        }
    }

    /// The Solarized light palette, for the light scheme, named `solarized-light`.
    ///
    /// Upstream sources:
    /// - altercation/solarized@62f656a02f93c5190a8753159e34b385588d5ff3
    ///   img/solarized-values-light.png (README §Usage & Development).
    /// - altercation/solarized@62f656a02f93c5190a8753159e34b385588d5ff3
    ///   vim-colors-solarized/colors/solarized.vim (names as for the dark background; the file
    ///   swaps base03..base3 for the light one).
    /// - altercation/solarized@62f656a02f93c5190a8753159e34b385588d5ff3 README.md §The Values,
    ///   §Features, item 5 (scss snippet).
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: base3; U: base 3 – background
    /// - `view_bg`: base3; G1: = `window_bg`
    /// - `surface_bg`: base2; S1: "background highlights"; the port's popup menu is a reversed
    ///   block, which a palette with one text colour cannot hold
    /// - `header_bg`: base2; S1: exe "hi! TabLineFill" .s:fmt_undr .s:fg_base0 .s:bg_base02
    ///   .s:sp_base0
    /// - `sidebar_bg`: base2; S1: exe "hi! LineNr" .s:fmt_none .s:fg_base01 .s:bg_base02; exe "hi!
    ///   FoldColumn" .s:fmt_none .s:fg_base0 .s:bg_base02
    /// - `tooltip_bg`: base2; G2: = `surface_bg`
    /// - `foreground`: base00; U: base 00 – body text / defaul code / primary content
    /// - `foreground_muted`: base1; U: base 1 – comments / secondary content
    /// - `border`: base0; S3: exe "hi! VertSplit" .s:fmt_none .s:fg_base00 .s:bg_base00
    /// - `divider`: base0; G3: = `border`
    /// - `accent`: blue; S4: this crate's choice among the eight accents (0.7.2's); the README
    ///   leaves the accent to the designer
    /// - `accent_fg`: base3; G6: = `window_bg` (3.41:1 on the accent; `foreground` 1.21:1)
    /// - `selection_bg`: base1; S2: exe "hi! PmenuSel" .s:fmt_none .s:fg_base01 .s:bg_base2
    ///   .s:fmt_revbb (drawn reversed: the block is base01); Visual has the same block
    /// - `selection_fg`: base02; S2: PmenuSel and TabLineSel drawn reversed: their base2 is the
    ///   text (Visual's text, base03, is 2.8:1 on the block in the dark mode, 2.5:1 in the light)
    /// - `link`: blue; S4: scss `accentize` mixin: "a, a:active, a:visited, code.url { color:
    ///   $accent; }"
    /// - `caret`: base00; S6 (= G5): exe "hi! Cursor" .s:fmt_none .s:fg_base03 .s:bg_base0
    /// - `error`: red; S5: exe "hi! Error" .s:fmt_bold .s:fg_red .s:bg_none
    /// - `warning`: yellow; S5: by the accent's name (the port's WarningMsg is red, as its errors)
    /// - `success`: green; S5: no success group; the green of additions: exe "hi! DiffAdd" …
    ///   .s:fg_green
    /// - `info`: blue; S5 (= G7): exe "hi! MoreMsg" .s:fmt_none .s:fg_blue .s:bg_none
    #[must_use]
    pub fn solarized_light() -> Self {
        Self {
            name: Arc::from("solarized-light"),
            scheme: Scheme::Light,
            window_bg: srgb8(0xfdf6_e3ff),        // base3
            view_bg: srgb8(0xfdf6_e3ff),          // base3
            surface_bg: srgb8(0xeee8_d5ff),       // base2
            header_bg: srgb8(0xeee8_d5ff),        // base2
            sidebar_bg: srgb8(0xeee8_d5ff),       // base2
            tooltip_bg: srgb8(0xeee8_d5ff),       // base2
            foreground: srgb8(0x657b_83ff),       // base00
            foreground_muted: srgb8(0x93a1_a1ff), // base1
            border: srgb8(0x8394_96ff),           // base0
            divider: srgb8(0x8394_96ff),          // base0
            accent: srgb8(0x268b_d2ff),           // blue
            accent_fg: srgb8(0xfdf6_e3ff),        // base3
            selection_bg: srgb8(0x93a1_a1ff),     // base1
            selection_fg: srgb8(0x0736_42ff),     // base02
            link: srgb8(0x268b_d2ff),             // blue
            caret: srgb8(0x657b_83ff),            // base00
            error: srgb8(0xdc32_2fff),            // red
            warning: srgb8(0xb589_00ff),          // yellow
            success: srgb8(0x8599_00ff),          // green
            info: srgb8(0x268b_d2ff),             // blue
        }
    }

    /// The Solarized dark palette, for the dark scheme, named `solarized-dark`.
    ///
    /// Upstream sources:
    /// - altercation/solarized@62f656a02f93c5190a8753159e34b385588d5ff3
    ///   img/solarized-values-dark.png (README §Usage & Development).
    /// - altercation/solarized@62f656a02f93c5190a8753159e34b385588d5ff3
    ///   vim-colors-solarized/colors/solarized.vim (names as for the dark background; the file
    ///   swaps base03..base3 for the light one).
    /// - altercation/solarized@62f656a02f93c5190a8753159e34b385588d5ff3 README.md §The Values,
    ///   §Features, item 5 (scss snippet).
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: base03; U: base 03 – background
    /// - `view_bg`: base03; G1: = `window_bg`
    /// - `surface_bg`: base02; S1: "background highlights"; the port's popup menu is a reversed
    ///   block, which a palette with one text colour cannot hold
    /// - `header_bg`: base02; S1: exe "hi! TabLineFill" .s:fmt_undr .s:fg_base0 .s:bg_base02
    ///   .s:sp_base0
    /// - `sidebar_bg`: base02; S1: exe "hi! LineNr" .s:fmt_none .s:fg_base01 .s:bg_base02; exe "hi!
    ///   FoldColumn" .s:fmt_none .s:fg_base0 .s:bg_base02
    /// - `tooltip_bg`: base02; G2: = `surface_bg`
    /// - `foreground`: base0; U: base 0 – body text / defaul code / primary content
    /// - `foreground_muted`: base01; U: base 01 – comments / secondary content
    /// - `border`: base00; S3: exe "hi! VertSplit" .s:fmt_none .s:fg_base00 .s:bg_base00
    /// - `divider`: base00; G3: = `border`
    /// - `accent`: blue; S4: this crate's choice among the eight accents (0.7.2's); the README
    ///   leaves the accent to the designer
    /// - `accent_fg`: base03; G6: = `window_bg` (4.08:1 on the accent; `foreground` 1.16:1)
    /// - `selection_bg`: base01; S2: exe "hi! PmenuSel" .s:fmt_none .s:fg_base01 .s:bg_base2
    ///   .s:fmt_revbb (drawn reversed: the block is base01); Visual has the same block
    /// - `selection_fg`: base2; S2: PmenuSel and TabLineSel drawn reversed: their base2 is the text
    ///   (Visual's text, base03, is 2.8:1 on the block in the dark mode, 2.5:1 in the light)
    /// - `link`: blue; S4: scss `accentize` mixin: "a, a:active, a:visited, code.url { color:
    ///   $accent; }"
    /// - `caret`: base0; S6 (= G5): exe "hi! Cursor" .s:fmt_none .s:fg_base03 .s:bg_base0
    /// - `error`: red; S5: exe "hi! Error" .s:fmt_bold .s:fg_red .s:bg_none
    /// - `warning`: yellow; S5: by the accent's name (the port's WarningMsg is red, as its errors)
    /// - `success`: green; S5: no success group; the green of additions: exe "hi! DiffAdd" …
    ///   .s:fg_green
    /// - `info`: blue; S5 (= G7): exe "hi! MoreMsg" .s:fmt_none .s:fg_blue .s:bg_none
    #[must_use]
    pub fn solarized_dark() -> Self {
        Self {
            name: Arc::from("solarized-dark"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x002b_36ff),        // base03
            view_bg: srgb8(0x002b_36ff),          // base03
            surface_bg: srgb8(0x0736_42ff),       // base02
            header_bg: srgb8(0x0736_42ff),        // base02
            sidebar_bg: srgb8(0x0736_42ff),       // base02
            tooltip_bg: srgb8(0x0736_42ff),       // base02
            foreground: srgb8(0x8394_96ff),       // base0
            foreground_muted: srgb8(0x586e_75ff), // base01
            border: srgb8(0x657b_83ff),           // base00
            divider: srgb8(0x657b_83ff),          // base00
            accent: srgb8(0x268b_d2ff),           // blue
            accent_fg: srgb8(0x002b_36ff),        // base03
            selection_bg: srgb8(0x586e_75ff),     // base01
            selection_fg: srgb8(0xeee8_d5ff),     // base2
            link: srgb8(0x268b_d2ff),             // blue
            caret: srgb8(0x8394_96ff),            // base0
            error: srgb8(0xdc32_2fff),            // red
            warning: srgb8(0xb589_00ff),          // yellow
            success: srgb8(0x8599_00ff),          // green
            info: srgb8(0x268b_d2ff),             // blue
        }
    }

    /// The Tokyo Night palette, for the dark scheme, named `tokyo-night`.
    ///
    /// Upstream sources:
    /// - folke/tokyonight.nvim@v4.14.1 (545d72cde6400835d895160ecb5853874fd5156d)
    ///   lua/tokyonight/groups/base.lua; values extras/lua/tokyonight_night.lua.
    /// - folke/tokyonight.nvim@v4.14.1 (545d72cde6400835d895160ecb5853874fd5156d)
    ///   lua/tokyonight/colors/init.lua; values extras/lua/tokyonight_night.lua.
    /// - folke/tokyonight.nvim@v4.14.1 (545d72cde6400835d895160ecb5853874fd5156d)
    ///   lua/tokyonight/groups/treesitter.lua; values extras/lua/tokyonight_night.lua.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: bg; U: Normal = { fg = c.fg, bg = … c.bg }, -- normal text
    /// - `view_bg`: bg; G1: = `window_bg`
    /// - `surface_bg`: bg_float; U: NormalFloat = { fg = c.fg_float, bg = c.bg_float }, -- Normal
    ///   text in floating windows.
    /// - `header_bg`: bg_statusline; T1: "StatusLine = { fg = c.fg_sidebar, bg = c.bg_statusline
    ///   }"; "WinBar = \"StatusLine\" , -- window bar"
    /// - `sidebar_bg`: bg_sidebar; U: NormalSB = { fg = c.fg_sidebar, bg = c.bg_sidebar }, --
    ///   normal text in sidebar
    /// - `tooltip_bg`: bg_float; G2 (and "NormalFloat = { fg = c.fg_float, bg = c.bg_float }, --
    ///   Normal text in floating windows.")
    /// - `foreground`: fg; U: Normal = { fg = c.fg, … }, -- normal text
    /// - `foreground_muted`: fg_dark; T2: "MsgArea = { fg = c.fg_dark }, -- Area for messages and
    ///   cmdline"; colors.fg_sidebar = colors.fg_dark
    /// - `border`: border; U: colors.border = colors.black
    /// - `divider`: border; U: WinSeparator = { fg = c.border, bold = true }, -- the column
    ///   separating vertically split windows
    /// - `accent`: blue; T3: "TabLineSel = { fg = c.black, bg = c.blue }, -- tab pages line, active
    ///   tab page label"
    /// - `accent_fg`: bg; G6 (TabLineSel's own text colour, `black`, is 1.96:1 on blue in the day
    ///   style): = `window_bg` (6.79:1 on the accent; `foreground` 1.56:1)
    /// - `selection_bg`: bg_visual; U: Visual = { bg = c.bg_visual }, -- Visual mode selection
    /// - `selection_fg`: fg; G4: = `foreground`
    /// - `link`: teal; U: \["@markup.link"\] = { fg = c.teal },
    /// - `caret`: fg; U: Cursor = { fg = c.bg, bg = c.fg }, -- character under the cursor
    /// - `error`: error (red1); U: ErrorMsg = { fg = c.error }, -- error messages on the command
    ///   line
    /// - `warning`: warning (yellow); U: WarningMsg = { fg = c.warning }, -- warning messages
    /// - `success`: green1; U: healthSuccess = { fg = c.green1 },
    /// - `info`: info (blue2); U: DiagnosticInfo = { fg = c.info }
    #[must_use]
    pub fn tokyo_night() -> Self {
        Self {
            name: Arc::from("tokyo-night"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x1a1b_26ff),        // bg
            view_bg: srgb8(0x1a1b_26ff),          // bg
            surface_bg: srgb8(0x1616_1eff),       // bg_float
            header_bg: srgb8(0x1616_1eff),        // bg_statusline
            sidebar_bg: srgb8(0x1616_1eff),       // bg_sidebar
            tooltip_bg: srgb8(0x1616_1eff),       // bg_float
            foreground: srgb8(0xc0ca_f5ff),       // fg
            foreground_muted: srgb8(0xa9b1_d6ff), // fg_dark
            border: srgb8(0x1516_1eff),           // border
            divider: srgb8(0x1516_1eff),          // border
            accent: srgb8(0x7aa2_f7ff),           // blue
            accent_fg: srgb8(0x1a1b_26ff),        // bg
            selection_bg: srgb8(0x2834_57ff),     // bg_visual
            selection_fg: srgb8(0xc0ca_f5ff),     // fg
            link: srgb8(0x1abc_9cff),             // teal
            caret: srgb8(0xc0ca_f5ff),            // fg
            error: srgb8(0xdb4b_4bff),            // error (red1)
            warning: srgb8(0xe0af_68ff),          // warning (yellow)
            success: srgb8(0x73da_caff),          // green1
            info: srgb8(0x0db9_d7ff),             // info (blue2)
        }
    }

    /// The Tokyo Night Day palette, for the light scheme, named `tokyo-night-day`.
    ///
    /// Upstream sources:
    /// - folke/tokyonight.nvim@v4.14.1 (545d72cde6400835d895160ecb5853874fd5156d)
    ///   lua/tokyonight/groups/base.lua; values extras/lua/tokyonight_day.lua.
    /// - folke/tokyonight.nvim@v4.14.1 (545d72cde6400835d895160ecb5853874fd5156d)
    ///   lua/tokyonight/colors/init.lua; values extras/lua/tokyonight_day.lua.
    /// - folke/tokyonight.nvim@v4.14.1 (545d72cde6400835d895160ecb5853874fd5156d)
    ///   lua/tokyonight/groups/treesitter.lua; values extras/lua/tokyonight_day.lua.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: bg; U: Normal = { fg = c.fg, bg = … c.bg }, -- normal text
    /// - `view_bg`: bg; G1: = `window_bg`
    /// - `surface_bg`: bg_float; U: NormalFloat = { fg = c.fg_float, bg = c.bg_float }, -- Normal
    ///   text in floating windows.
    /// - `header_bg`: bg_statusline; T1: "StatusLine = { fg = c.fg_sidebar, bg = c.bg_statusline
    ///   }"; "WinBar = \"StatusLine\" , -- window bar"
    /// - `sidebar_bg`: bg_sidebar; U: NormalSB = { fg = c.fg_sidebar, bg = c.bg_sidebar }, --
    ///   normal text in sidebar
    /// - `tooltip_bg`: bg_float; G2 (and "NormalFloat = { fg = c.fg_float, bg = c.bg_float }, --
    ///   Normal text in floating windows.")
    /// - `foreground`: fg; U: Normal = { fg = c.fg, … }, -- normal text
    /// - `foreground_muted`: fg_dark; T2: "MsgArea = { fg = c.fg_dark }, -- Area for messages and
    ///   cmdline"; colors.fg_sidebar = colors.fg_dark
    /// - `border`: border; U: colors.border = colors.black
    /// - `divider`: border; U: WinSeparator = { fg = c.border, bold = true }, -- the column
    ///   separating vertically split windows
    /// - `accent`: blue; T3: "TabLineSel = { fg = c.black, bg = c.blue }, -- tab pages line, active
    ///   tab page label"
    /// - `accent_fg`: bg; G6 (TabLineSel's own text colour, `black`, is 1.96:1 on blue in the day
    ///   style): = `window_bg` (3.11:1 on the accent; `foreground` 1.46:1)
    /// - `selection_bg`: bg_visual; U: Visual = { bg = c.bg_visual }, -- Visual mode selection
    /// - `selection_fg`: fg; G4: = `foreground`
    /// - `link`: teal; U: \["@markup.link"\] = { fg = c.teal },
    /// - `caret`: fg; U: Cursor = { fg = c.bg, bg = c.fg }, -- character under the cursor
    /// - `error`: error (red1); U: ErrorMsg = { fg = c.error }, -- error messages on the command
    ///   line
    /// - `warning`: warning (yellow); U: WarningMsg = { fg = c.warning }, -- warning messages
    /// - `success`: green1; U: healthSuccess = { fg = c.green1 },
    /// - `info`: info (blue2); U: DiagnosticInfo = { fg = c.info }
    #[must_use]
    pub fn tokyo_night_day() -> Self {
        Self {
            name: Arc::from("tokyo-night-day"),
            scheme: Scheme::Light,
            window_bg: srgb8(0xe1e2_e7ff),        // bg
            view_bg: srgb8(0xe1e2_e7ff),          // bg
            surface_bg: srgb8(0xd0d5_e3ff),       // bg_float
            header_bg: srgb8(0xd0d5_e3ff),        // bg_statusline
            sidebar_bg: srgb8(0xd0d5_e3ff),       // bg_sidebar
            tooltip_bg: srgb8(0xd0d5_e3ff),       // bg_float
            foreground: srgb8(0x3760_bfff),       // fg
            foreground_muted: srgb8(0x6172_b0ff), // fg_dark
            border: srgb8(0xb4b5_b9ff),           // border
            divider: srgb8(0xb4b5_b9ff),          // border
            accent: srgb8(0x2e7d_e9ff),           // blue
            accent_fg: srgb8(0xe1e2_e7ff),        // bg
            selection_bg: srgb8(0xb7c1_e3ff),     // bg_visual
            selection_fg: srgb8(0x3760_bfff),     // fg
            link: srgb8(0x118c_74ff),             // teal
            caret: srgb8(0x3760_bfff),            // fg
            error: srgb8(0xc643_43ff),            // error (red1)
            warning: srgb8(0x8c6c_3eff),          // warning (yellow)
            success: srgb8(0x3870_68ff),          // green1
            info: srgb8(0x0787_9dff),             // info (blue2)
        }
    }

    /// The Gruvbox light palette, for the light scheme, named `gruvbox-light`.
    ///
    /// Upstream sources:
    /// - morhetz/gruvbox@ef8864bb42bf244f0295d1c5a403b27e3d139695 colors/gruvbox.vim §General UI,
    ///   §Completion Menu, §Gutter, §Asynchronous Lint Engine, §Syntastic, §GitGutter.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: bg0 (light0); U: comment: Normal text; call s:HL('Normal', s:fg1, s:bg0)
    /// - `view_bg`: bg0 (light0); G1: = `window_bg`
    /// - `surface_bg`: bg2 (light2); B2: "Popup menu: normal item": call s:HL('Pmenu', s:fg1,
    ///   s:bg2)
    /// - `header_bg`: bg1 (light1); B3: "Tab pages line filler": call s:HL('TabLineFill', s:bg4,
    ///   s:bg1, s:invert_tabline)
    /// - `sidebar_bg`: bg1 (light1); B3: "let s:sign_column = s:bg1"; call s:HL('SignColumn',
    ///   s:none, s:sign_column)
    /// - `tooltip_bg`: bg2 (light2); G2: = `surface_bg`
    /// - `foreground`: fg1 (dark1); U: comment: Normal text; call s:HL('Normal', s:fg1, s:bg0)
    /// - `foreground_muted`: fg4 (dark4); B4: call s:HL('StatusLineNC', s:bg1, s:fg4, s:inverse)
    /// - `border`: bg3 (light3); B5 (= G3): "The column separating vertically split windows": call
    ///   s:HL('VertSplit', s:bg3, s:vert_split)
    /// - `divider`: bg3 (light3); U: comment: The column separating vertically split windows; call
    ///   s:HL('VertSplit', s:bg3, s:vert_split)
    /// - `accent`: blue (faded_blue); B6: "Popup menu: selected item": call s:HL('PmenuSel', s:bg2,
    ///   s:blue, s:bold)
    /// - `accent_fg`: bg0 (light0); G6 (PmenuSel's own text colour, bg2, is 3.3:1 on blue in the
    ///   dark mode): = `window_bg` (5.82:1 on the accent; `foreground` 1.76:1)
    /// - `selection_bg`: fg1 (dark1); B7: call s:HL('Visual', s:none, s:bg3, s:invert_selection),
    ///   s:invert_selection = s:inverse by default: drawn reversed, the text's colour is the block
    /// - `selection_fg`: bg3 (light3); B7: Visual drawn reversed: its bg3 is the text
    /// - `link`: blue (faded_blue); U: call s:HL('Underlined', s:blue, s:none, s:underline)
    /// - `caret`: fg1 (dark1); G5 (call s:HL('Cursor', s:none, s:none, s:inverse): the text drawn
    ///   reversed): = `foreground`
    /// - `error`: red (faded_red); U: comment: Error messages on the command line; call
    ///   s:HL('ErrorMsg', s:bg0, s:red, s:bold)
    /// - `warning`: yellow (faded_yellow); B1: hi! link ALEWarningSign GruvboxYellowSign; hi! link
    ///   SyntasticWarningSign GruvboxYellowSign
    /// - `success`: green (faded_green); B1: no success group; the green of additions: hi! link
    ///   GitGutterAdd GruvboxGreenSign
    /// - `info`: blue (faded_blue); B1: hi! link ALEInfoSign GruvboxBlueSign
    #[must_use]
    pub fn gruvbox_light() -> Self {
        Self {
            name: Arc::from("gruvbox-light"),
            scheme: Scheme::Light,
            window_bg: srgb8(0xfbf1_c7ff),        // bg0 (light0)
            view_bg: srgb8(0xfbf1_c7ff),          // bg0 (light0)
            surface_bg: srgb8(0xd5c4_a1ff),       // bg2 (light2)
            header_bg: srgb8(0xebdb_b2ff),        // bg1 (light1)
            sidebar_bg: srgb8(0xebdb_b2ff),       // bg1 (light1)
            tooltip_bg: srgb8(0xd5c4_a1ff),       // bg2 (light2)
            foreground: srgb8(0x3c38_36ff),       // fg1 (dark1)
            foreground_muted: srgb8(0x7c6f_64ff), // fg4 (dark4)
            border: srgb8(0xbdae_93ff),           // bg3 (light3)
            divider: srgb8(0xbdae_93ff),          // bg3 (light3)
            accent: srgb8(0x0766_78ff),           // blue (faded_blue)
            accent_fg: srgb8(0xfbf1_c7ff),        // bg0 (light0)
            selection_bg: srgb8(0x3c38_36ff),     // fg1 (dark1)
            selection_fg: srgb8(0xbdae_93ff),     // bg3 (light3)
            link: srgb8(0x0766_78ff),             // blue (faded_blue)
            caret: srgb8(0x3c38_36ff),            // fg1 (dark1)
            error: srgb8(0x9d00_06ff),            // red (faded_red)
            warning: srgb8(0xb576_14ff),          // yellow (faded_yellow)
            success: srgb8(0x7974_0eff),          // green (faded_green)
            info: srgb8(0x0766_78ff),             // blue (faded_blue)
        }
    }

    /// The Gruvbox dark palette, for the dark scheme, named `gruvbox-dark`.
    ///
    /// Upstream sources:
    /// - morhetz/gruvbox@ef8864bb42bf244f0295d1c5a403b27e3d139695 colors/gruvbox.vim §General UI,
    ///   §Completion Menu, §Gutter, §Asynchronous Lint Engine, §Syntastic, §GitGutter.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: bg0 (dark0); U: comment: Normal text; call s:HL('Normal', s:fg1, s:bg0)
    /// - `view_bg`: bg0 (dark0); G1: = `window_bg`
    /// - `surface_bg`: bg2 (dark2); B2: "Popup menu: normal item": call s:HL('Pmenu', s:fg1, s:bg2)
    /// - `header_bg`: bg1 (dark1); B3: "Tab pages line filler": call s:HL('TabLineFill', s:bg4,
    ///   s:bg1, s:invert_tabline)
    /// - `sidebar_bg`: bg1 (dark1); B3: "let s:sign_column = s:bg1"; call s:HL('SignColumn',
    ///   s:none, s:sign_column)
    /// - `tooltip_bg`: bg2 (dark2); G2: = `surface_bg`
    /// - `foreground`: fg1 (light1); U: comment: Normal text; call s:HL('Normal', s:fg1, s:bg0)
    /// - `foreground_muted`: fg4 (light4); B4: call s:HL('StatusLineNC', s:bg1, s:fg4, s:inverse)
    /// - `border`: bg3 (dark3); B5 (= G3): "The column separating vertically split windows": call
    ///   s:HL('VertSplit', s:bg3, s:vert_split)
    /// - `divider`: bg3 (dark3); U: comment: The column separating vertically split windows; call
    ///   s:HL('VertSplit', s:bg3, s:vert_split)
    /// - `accent`: blue (bright_blue); B6: "Popup menu: selected item": call s:HL('PmenuSel',
    ///   s:bg2, s:blue, s:bold)
    /// - `accent_fg`: bg0 (dark0); G6 (PmenuSel's own text colour, bg2, is 3.3:1 on blue in the
    ///   dark mode): = `window_bg` (5.48:1 on the accent; `foreground` 1.96:1)
    /// - `selection_bg`: fg1 (light1); B7: call s:HL('Visual', s:none, s:bg3, s:invert_selection),
    ///   s:invert_selection = s:inverse by default: drawn reversed, the text's colour is the block
    /// - `selection_fg`: bg3 (dark3); B7: Visual drawn reversed: its bg3 is the text
    /// - `link`: blue (bright_blue); U: call s:HL('Underlined', s:blue, s:none, s:underline)
    /// - `caret`: fg1 (light1); G5 (call s:HL('Cursor', s:none, s:none, s:inverse): the text drawn
    ///   reversed): = `foreground`
    /// - `error`: red (bright_red); U: comment: Error messages on the command line; call
    ///   s:HL('ErrorMsg', s:bg0, s:red, s:bold)
    /// - `warning`: yellow (bright_yellow); B1: hi! link ALEWarningSign GruvboxYellowSign; hi! link
    ///   SyntasticWarningSign GruvboxYellowSign
    /// - `success`: green (bright_green); B1: no success group; the green of additions: hi! link
    ///   GitGutterAdd GruvboxGreenSign
    /// - `info`: blue (bright_blue); B1: hi! link ALEInfoSign GruvboxBlueSign
    #[must_use]
    pub fn gruvbox_dark() -> Self {
        Self {
            name: Arc::from("gruvbox-dark"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x2828_28ff),        // bg0 (dark0)
            view_bg: srgb8(0x2828_28ff),          // bg0 (dark0)
            surface_bg: srgb8(0x5049_45ff),       // bg2 (dark2)
            header_bg: srgb8(0x3c38_36ff),        // bg1 (dark1)
            sidebar_bg: srgb8(0x3c38_36ff),       // bg1 (dark1)
            tooltip_bg: srgb8(0x5049_45ff),       // bg2 (dark2)
            foreground: srgb8(0xebdb_b2ff),       // fg1 (light1)
            foreground_muted: srgb8(0xa899_84ff), // fg4 (light4)
            border: srgb8(0x665c_54ff),           // bg3 (dark3)
            divider: srgb8(0x665c_54ff),          // bg3 (dark3)
            accent: srgb8(0x83a5_98ff),           // blue (bright_blue)
            accent_fg: srgb8(0x2828_28ff),        // bg0 (dark0)
            selection_bg: srgb8(0xebdb_b2ff),     // fg1 (light1)
            selection_fg: srgb8(0x665c_54ff),     // bg3 (dark3)
            link: srgb8(0x83a5_98ff),             // blue (bright_blue)
            caret: srgb8(0xebdb_b2ff),            // fg1 (light1)
            error: srgb8(0xfb49_34ff),            // red (bright_red)
            warning: srgb8(0xfabd_2fff),          // yellow (bright_yellow)
            success: srgb8(0xb8bb_26ff),          // green (bright_green)
            info: srgb8(0x83a5_98ff),             // blue (bright_blue)
        }
    }

    /// The One Dark palette, for the dark scheme, named `one-dark`.
    ///
    /// Upstream sources:
    /// - atom/one-dark-ui@18c2143d41e17b337fff6dc7024562c4d999be17 styles/ui-variables.less,
    ///   ui-variables-custom.less and atom/one-dark-syntax@9c96f4454362267ac45322063e193ccf9d2debb1
    ///   styles/colors.less, styles/syntax-variables.less; hex computed from the Less expressions.
    /// - atom/one-dark-ui@18c2143d41e17b337fff6dc7024562c4d999be17 styles/ui-variables-custom.less
    ///   and atom/one-dark-syntax@9c96f4454362267ac45322063e193ccf9d2debb1 styles/colors.less,
    ///   styles/syntax-variables.less; hex computed from the Less expressions;
    ///   atom/one-dark-syntax@9c96f4454362267ac45322063e193ccf9d2debb1 styles/syntax/gfm.less.
    /// - atom/one-dark-syntax@9c96f4454362267ac45322063e193ccf9d2debb1
    ///   styles/syntax-variables.less, styles/colors.less, styles/editor.less; hex computed from
    ///   the Less expressions.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: @app-background-color; U: @app-background-color: @level-3-color; (atom.less:
    ///   atom-workspace { background-color: @app-background-color; })
    /// - `view_bg`: @pane-item-background-color; U: @pane-item-background-color:
    ///   @base-background-color;
    /// - `surface_bg`: @overlay-background-color; U: @overlay-background-color: mix(@level-2-color,
    ///   @level-3-color); (modal.less, lists.less .select-list.popover-list)
    /// - `header_bg`: @tab-bar-background-color; O1: "@tab-bar-background-color: @level-3-color;";
    ///   title-bar.less sets no background, so the title bar shows @app-background-color, the same
    ///   colour
    /// - `sidebar_bg`: @tool-panel-background-color; U: @tool-panel-background-color:
    ///   @level-3-color;
    /// - `tooltip_bg`: @overlay-background-color; O2 (= G2; not @tooltip-background-color #4d78cc:
    ///   Atom draws white text on it, and a palette has one text colour)
    /// - `foreground`: @text-color; U: @text-color: @ui-fg;
    /// - `foreground_muted`: @text-color-subtle; U: @text-color-subtle: fadeout(@text-color, 40%);
    /// - `border`: @base-border-color; U: @base-border-color: @ui-border;
    /// - `divider`: @base-border-color; G3: = `border`
    /// - `accent`: @accent-color; U: // used for marker, inputs (smaller things) @accent-color:
    ///   mix( hsv(…), hsl(…), @accent-luma )
    /// - `accent_fg`: @accent-text-color; U: @accent-text-color: contrast(@accent-color, …)
    /// - `selection_bg`: @background-color-selected; U: @background-color-selected:
    ///   lighten(@base-background-color, 8%); (lists.less li.selected)
    /// - `selection_fg`: @text-color-selected; U: @text-color-selected: white;
    /// - `link`: @accent-only-text-color; O3: "// used for text only"
    /// - `caret`: @syntax-cursor-color; U: @syntax-cursor-color: @syntax-accent; (editor.less
    ///   .cursor border-left)
    /// - `error`: @text-color-error; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-error is a fill under white text
    /// - `warning`: @text-color-warning; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-warning is a fill under white text
    /// - `success`: @text-color-success; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-success is a fill under white text
    /// - `info`: @text-color-info; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-info is a fill under white text
    #[must_use]
    pub fn one_dark() -> Self {
        Self {
            name: Arc::from("one-dark"),
            scheme: Scheme::Dark,
            window_bg: srgb8(0x2125_2bff),  // @app-background-color
            view_bg: srgb8(0x282c_34ff),    // @pane-item-background-color
            surface_bg: srgb8(0x2528_30ff), // @overlay-background-color
            header_bg: srgb8(0x2125_2bff),  // @tab-bar-background-color
            sidebar_bg: srgb8(0x2125_2bff), // @tool-panel-background-color
            tooltip_bg: srgb8(0x2528_30ff), // @overlay-background-color
            foreground: srgb8(0x9da5_b4ff), // @text-color
            foreground_muted: srgb8(0x9da5_b499), // @text-color-subtle
            border: srgb8(0x181a_1fff),     // @base-border-color
            divider: srgb8(0x181a_1fff),    // @base-border-color
            accent: srgb8(0x568a_f2ff),     // @accent-color
            accent_fg: srgb8(0x0011_33ff),  // @accent-text-color
            selection_bg: srgb8(0x3a3f_4bff), // @background-color-selected
            selection_fg: srgb8(0xffff_ffff), // @text-color-selected
            link: srgb8(0x759e_f2ff),       // @accent-only-text-color
            caret: srgb8(0x528b_ffff),      // @syntax-cursor-color
            error: srgb8(0xff63_47ff),      // @text-color-error
            warning: srgb8(0xe2c0_8dff),    // @text-color-warning
            success: srgb8(0x73c9_90ff),    // @text-color-success
            info: srgb8(0x6494_edff),       // @text-color-info
        }
    }

    /// The One Light palette, for the light scheme, named `one-light`.
    ///
    /// Upstream sources:
    /// - atom/one-light-ui@a27bce35088ecd323e44f5c2510bcf2d4e9efcf8 styles/ui-variables.less,
    ///   ui-variables-custom.less and
    ///   atom/one-light-syntax@d84579027410c576086dfca14d934c4bd74b0438 styles/colors.less,
    ///   styles/syntax-variables.less; hex computed from the Less expressions.
    /// - atom/one-light-ui@a27bce35088ecd323e44f5c2510bcf2d4e9efcf8 styles/ui-variables-custom.less
    ///   and atom/one-light-syntax@d84579027410c576086dfca14d934c4bd74b0438 styles/colors.less,
    ///   styles/syntax-variables.less; hex computed from the Less expressions;
    ///   atom/one-light-syntax@d84579027410c576086dfca14d934c4bd74b0438 styles/syntax/gfm.less.
    /// - atom/one-light-syntax@d84579027410c576086dfca14d934c4bd74b0438
    ///   styles/syntax-variables.less, styles/colors.less, styles/editor.less; hex computed from
    ///   the Less expressions.
    ///
    /// Slots, each with the upstream's colour name and the rule that filled it
    /// (see [`Palette`]'s documentation):
    /// - `window_bg`: @app-background-color; U: @app-background-color: @level-3-color; (atom.less:
    ///   atom-workspace { background-color: @app-background-color; })
    /// - `view_bg`: @pane-item-background-color; U: @pane-item-background-color:
    ///   @base-background-color;
    /// - `surface_bg`: @overlay-background-color; U: @overlay-background-color: mix(@level-2-color,
    ///   @level-3-color); (modal.less, lists.less .select-list.popover-list)
    /// - `header_bg`: @tab-bar-background-color; O1: "@tab-bar-background-color: @level-3-color;";
    ///   title-bar.less sets no background, so the title bar shows @app-background-color, the same
    ///   colour
    /// - `sidebar_bg`: @tool-panel-background-color; U: @tool-panel-background-color:
    ///   @level-3-color;
    /// - `tooltip_bg`: @overlay-background-color; O2 (= G2; not @tooltip-background-color #5871ef:
    ///   Atom draws white text on it, and a palette has one text colour)
    /// - `foreground`: @text-color; U: @text-color: @ui-fg;
    /// - `foreground_muted`: @text-color-subtle; U: @text-color-subtle: lighten(@text-color, 30%);
    /// - `border`: @base-border-color; U: @base-border-color: @ui-border;
    /// - `divider`: @base-border-color; G3: = `border`
    /// - `accent`: @accent-color; U: // used for marker, inputs (smaller things) @accent-color:
    ///   mix( hsv(…), hsl(…), @accent-luma )
    /// - `accent_fg`: @accent-text-color; U: @accent-text-color: contrast(@accent-color, …)
    /// - `selection_bg`: @background-color-selected; U: @background-color-selected:
    ///   darken(@level-3-color, 6%); (lists.less li.selected)
    /// - `selection_fg`: @text-color-selected; U: @text-color-selected:
    ///   darken(@text-color-highlight, 12%);
    /// - `link`: @accent-only-text-color; O3: "// used for text only"
    /// - `caret`: @syntax-cursor-color; U: @syntax-cursor-color: @syntax-accent; (editor.less
    ///   .cursor border-left)
    /// - `error`: @text-color-error; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-error is a fill under white text
    /// - `warning`: @text-color-warning; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-warning is a fill under white text
    /// - `success`: @text-color-success; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-success is a fill under white text
    /// - `info`: @text-color-info; O4: the status colour drawn on the theme's backgrounds;
    ///   @background-color-info is a fill under white text
    #[must_use]
    pub fn one_light() -> Self {
        Self {
            name: Arc::from("one-light"),
            scheme: Scheme::Light,
            window_bg: srgb8(0xeaea_ebff),  // @app-background-color
            view_bg: srgb8(0xfafa_faff),    // @pane-item-background-color
            surface_bg: srgb8(0xf2f2_f2ff), // @overlay-background-color
            header_bg: srgb8(0xeaea_ebff),  // @tab-bar-background-color
            sidebar_bg: srgb8(0xeaea_ebff), // @tool-panel-background-color
            tooltip_bg: srgb8(0xf2f2_f2ff), // @overlay-background-color
            foreground: srgb8(0x4242_43ff), // @text-color
            foreground_muted: srgb8(0x8e8e_90ff), // @text-color-subtle
            border: srgb8(0xdbdb_dcff),     // @base-border-color
            divider: srgb8(0xdbdb_dcff),    // @base-border-color
            accent: srgb8(0x556d_e8ff),     // @accent-color
            accent_fg: srgb8(0xffff_ffff),  // @accent-text-color
            selection_bg: srgb8(0xdbdb_dcff), // @background-color-selected
            selection_fg: srgb8(0x0505_05ff), // @text-color-selected
            link: srgb8(0x304e_e2ff),       // @accent-only-text-color
            caret: srgb8(0x526f_ffff),      // @syntax-cursor-color
            error: srgb8(0xf42a_2aff),      // @text-color-error
            warning: srgb8(0xd588_0bff),    // @text-color-warning
            success: srgb8(0x2db4_48ff),    // @text-color-success
            info: srgb8(0x1492_ffff),       // @text-color-info
        }
    }
}
