// Helpers shared by the generators.

use syn::{Ident, Type};

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

/// Extract `T` from `Option<T>`. Returns the type as-is if not `Option`.
pub(crate) fn option_inner(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty
        && let Some(seg) = type_path.path.segments.last()
        && seg.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return inner.clone();
    }
    ty.clone()
}

/// Check if a type path's last segment is `name`, e.g. "ResolvedWidgetBorder".
pub(crate) fn is_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(seg) = type_path.path.segments.last()
    {
        return seg.ident == name;
    }
    false
}
