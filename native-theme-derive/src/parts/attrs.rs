// The `#[parts(...)]` attributes of `ThemeParts`.

use syn::{Attribute, Error, LitStr, Result};

/// How a block is patched (S2.2, S6.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BlockKind {
    /// A struct block: set whole, or field by field (`Set | Fields`).
    Fields,
    /// A paint: set whole, or its one list appended to (`Set | Append`).
    Append,
    /// An enum block: set whole only.
    Set,
}

/// The attributes on the derive's input type.
#[derive(Debug, Default)]
pub(crate) struct TypeAttrs {
    /// `#[parts(block)]`: block mode.
    pub block: bool,
    /// `#[parts(append)]`: patched by `Set` or by appending to its one list.
    pub append: bool,
    /// `#[parts(own_resolved)]`: `Resolved<T>` is written by hand.
    pub own_resolved: bool,
}

/// The attributes on one field.
#[derive(Debug, Default)]
pub(crate) struct FieldAttrs {
    /// `#[parts(may_be_absent)]` (S6.3).
    pub may_be_absent: bool,
    /// `#[parts(auto)]` (S6.3): a `Value<Px>` that may be `Auto`.
    pub auto: bool,
    /// `#[parts(block = "fields" | "append" | "set")]`: the field holds a
    /// block of that kind.
    pub block: Option<BlockKind>,
    /// `#[parts(keyed)]`: a list whose items merge by their own key (S6.1:
    /// the bindings by role or surface, the transitions by property group).
    pub keyed: bool,
}

fn flag(slot: &mut bool, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<()> {
    if *slot {
        return Err(meta.error("duplicate parts attribute"));
    }
    *slot = true;
    Ok(())
}

pub(crate) fn parse_type_attrs(attrs: &[Attribute]) -> Result<TypeAttrs> {
    let mut out = TypeAttrs::default();
    for attr in attrs.iter().filter(|a| a.path().is_ident("parts")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("block") {
                flag(&mut out.block, &meta)
            } else if meta.path.is_ident("append") {
                flag(&mut out.append, &meta)
            } else if meta.path.is_ident("own_resolved") {
                flag(&mut out.own_resolved, &meta)
            } else {
                Err(meta.error("unknown parts attribute: expected block, append or own_resolved"))
            }
        })?;
    }
    Ok(out)
}

pub(crate) fn parse_field_attrs(attrs: &[Attribute]) -> Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs.iter().filter(|a| a.path().is_ident("parts")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("may_be_absent") {
                flag(&mut out.may_be_absent, &meta)
            } else if meta.path.is_ident("auto") {
                flag(&mut out.auto, &meta)
            } else if meta.path.is_ident("keyed") {
                flag(&mut out.keyed, &meta)
            } else if meta.path.is_ident("block") {
                if out.block.is_some() {
                    return Err(meta.error("duplicate parts attribute"));
                }
                let lit: LitStr = meta.value()?.parse()?;
                out.block = Some(match lit.value().as_str() {
                    "fields" => BlockKind::Fields,
                    "append" => BlockKind::Append,
                    "set" => BlockKind::Set,
                    other => {
                        return Err(Error::new(
                            lit.span(),
                            format!(
                                "unknown block kind \"{other}\": expected \"fields\", \"append\" or \"set\""
                            ),
                        ));
                    }
                });
                Ok(())
            } else {
                Err(meta.error(
                    "unknown parts field attribute: expected may_be_absent, auto, keyed or block",
                ))
            }
        })?;
    }
    Ok(out)
}
