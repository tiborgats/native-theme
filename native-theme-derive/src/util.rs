// Helpers shared by the generators.

use syn::Ident;

/// The name a widget struct registers under: its name without a trailing
/// "Theme", in snake_case (`ButtonTheme` -> `"button"`,
/// `SegmentedControlTheme` -> `"segmented_control"`).
pub(crate) fn widget_name(opt_name: &Ident) -> String {
    let s = opt_name.to_string();
    let base = s.strip_suffix("Theme").unwrap_or(&s);
    let mut out = String::with_capacity(base.len().saturating_add(4));
    for (i, ch) in base.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}
