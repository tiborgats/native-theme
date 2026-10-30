// Widget mode (`#[parts(widget)]`): a typed widget's part enum, its patch,
// and its tables for the field-path registry (S5, S6.1).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Error, Fields, Ident, LitStr, Result, Type};

/// One part, as the widget's `#[part(...)]` attribute states it.
struct Part {
    ident: Ident,
    ty: Type,
    docs: Vec<Attribute>,
    root: bool,
    element: bool,
    may_be_absent: bool,
    parent: Option<LitStr>,
    fields: Vec<Ident>,
    absent: Vec<Ident>,
}

/// `menu_indicator` -> `MenuIndicator`.
fn camel_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_ascii_uppercase().to_string() + chars.as_str()
            })
        })
        .collect()
}

fn docs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .cloned()
        .collect()
}

fn idents(meta: &syn::meta::ParseNestedMeta<'_>, into: &mut Vec<Ident>) -> Result<()> {
    if !into.is_empty() {
        return Err(meta.error("duplicate part attribute"));
    }
    meta.parse_nested_meta(|inner| {
        let ident = inner
            .path
            .get_ident()
            .cloned()
            .ok_or_else(|| inner.error("a field's name"))?;
        into.push(ident);
        Ok(())
    })
}

fn flag(slot: &mut bool, meta: &syn::meta::ParseNestedMeta<'_>) -> Result<()> {
    if *slot {
        return Err(meta.error("duplicate part attribute"));
    }
    *slot = true;
    Ok(())
}

/// The field's `#[part(...)]`, if it has one.
fn parse_part(field: &syn::Field) -> Result<Option<Part>> {
    let Some(ident) = field.ident.clone() else {
        return Err(Error::new_spanned(field, "a widget has named fields"));
    };
    let mut attrs = field.attrs.iter().filter(|a| a.path().is_ident("part"));
    let Some(attr) = attrs.next() else {
        return Ok(None);
    };
    if let Some(again) = attrs.next() {
        return Err(Error::new_spanned(again, "one `#[part(...)]` per part"));
    }
    let mut part = Part {
        ident,
        ty: field.ty.clone(),
        docs: docs(&field.attrs),
        root: false,
        element: false,
        may_be_absent: false,
        parent: None,
        fields: Vec::new(),
        absent: Vec::new(),
    };
    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("root") {
            flag(&mut part.root, &meta)
        } else if meta.path.is_ident("element") {
            flag(&mut part.element, &meta)
        } else if meta.path.is_ident("may_be_absent") {
            flag(&mut part.may_be_absent, &meta)
        } else if meta.path.is_ident("parent") {
            if part.parent.is_some() {
                return Err(meta.error("duplicate part attribute"));
            }
            part.parent = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("fields") {
            idents(&meta, &mut part.fields)
        } else if meta.path.is_ident("absent") {
            idents(&meta, &mut part.absent)
        } else {
            Err(meta.error(
                "unknown part attribute: expected root, element, may_be_absent, parent, fields \
                 or absent",
            ))
        }
    })?;
    if part.fields.is_empty() {
        return Err(Error::new_spanned(
            attr,
            "a part lists its fields, `fields(...)`",
        ));
    }
    if let Some(stray) = part.absent.iter().find(|a| !part.fields.contains(a)) {
        return Err(Error::new_spanned(
            stray,
            "absent names a field the part lists in `fields(...)`",
        ));
    }
    if part.root && (part.may_be_absent || part.element || part.parent.is_some()) {
        return Err(Error::new_spanned(
            attr,
            "the root part is never absent as a whole, never an element and nests in nothing",
        ));
    }
    Ok(Some(part))
}

pub(crate) fn expand(input: &DeriveInput) -> Result<TokenStream> {
    let Data::Struct(data) = &input.data else {
        return Err(Error::new_spanned(&input.ident, "a widget is a struct"));
    };
    let Fields::Named(named) = &data.fields else {
        return Err(Error::new_spanned(
            &input.ident,
            "a widget has named fields",
        ));
    };
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "a widget has no generic parameters",
        ));
    }
    let name = &input.ident;
    let widget = name
        .to_string()
        .strip_suffix("Theme")
        .map(str::to_owned)
        .ok_or_else(|| Error::new_spanned(name, "a widget's input is named `<Widget>Theme`"))?;
    let mut parts = Vec::new();
    let mut values = None;
    for field in &named.named {
        match parse_part(field)? {
            Some(part) => parts.push(part),
            None if field.ident.as_ref().is_some_and(|i| i == "values") && values.is_none() => {
                values = Some(field.ty.clone());
            }
            None => {
                return Err(Error::new_spanned(
                    field,
                    "a widget's field is a part, marked `#[part(...)]`, or its one `values`",
                ));
            }
        }
    }
    let values = values.ok_or_else(|| {
        Error::new_spanned(name, "a widget has a `values` field, its values' struct")
    })?;
    match parts.split_first() {
        Some((root, rest)) if root.root && rest.iter().all(|p| !p.root) => {}
        _ => {
            return Err(Error::new_spanned(
                name,
                "a widget's first part, and only it, is its root, `#[part(root, ...)]`",
            ));
        }
    }
    for part in &parts {
        if let Some(parent) = &part.parent
            && !parts
                .iter()
                .any(|p| p.ident == parent.value() && p.ident != part.ident)
        {
            return Err(Error::new_spanned(parent, "parent names another part"));
        }
    }

    let vis = &input.vis;
    let snake = crate::util::widget_name(name);
    let part_enum = format_ident!("{widget}Part");
    let patch = format_ident!("{widget}Patch");
    let values_patch = match &values {
        Type::Path(path) => path.path.segments.last().map(|seg| {
            let mut patch = path.clone();
            if let Some(last) = patch.path.segments.last_mut() {
                last.ident = format_ident!("{}Patch", seg.ident);
            }
            patch
        }),
        _ => None,
    }
    .ok_or_else(|| Error::new_spanned(&values, "a widget's values are a struct block"))?;

    let variants: Vec<Ident> = parts
        .iter()
        .map(|p| format_ident!("{}", camel_case(&p.ident.to_string())))
        .collect();
    let part_docs: Vec<&Vec<Attribute>> = parts.iter().map(|p| &p.docs).collect();
    let fields: Vec<&Ident> = parts.iter().map(|p| &p.ident).collect();
    let enum_doc = format!("`{snake}`'s parts (S5), each styled by a part style.");
    let patch_doc = format!(
        "`{snake}`'s patch (S6.1): a part patch per part and the values' patch, each `None` \
         where the patch leaves it as it is."
    );

    let r = quote!(crate::next::registry);
    let b = quote!(crate::next::blocks);
    let w = quote!(crate::next::widgets);
    let infos = parts.iter().map(|part| {
        let part_name = part.ident.to_string();
        let ty = &part.ty;
        let root = part.root;
        let element = part.element;
        let may_be_absent = part.may_be_absent;
        let parent = match &part.parent {
            Some(parent) => quote!(::core::option::Option::Some(#parent)),
            None => quote!(::core::option::Option::None),
        };
        let names: Vec<String> = part.fields.iter().map(Ident::to_string).collect();
        let absent: Vec<String> = part.absent.iter().map(Ident::to_string).collect();
        let len = names.len();
        let message = format!("`{snake}.{part_name}` lists a field its style does not have");
        quote! {
            #r::PartInfo {
                name: #part_name,
                root: #root,
                element: #element,
                parent: #parent,
                may_be_absent: #may_be_absent,
                fields: {
                    const _: () = ::core::assert!(
                        #r::names_fields(<#ty as #b::Table>::FIELDS, &[#(#names),*]),
                        #message
                    );
                    const FIELDS: [#r::FieldInfo; #len] = #r::select(
                        <#ty as #b::Table>::FIELDS,
                        &[#(#names),*],
                        &[#(#absent),*],
                    );
                    &FIELDS
                },
            }
        }
    });

    Ok(quote! {
        #[doc = #enum_doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum #part_enum {
            #(#(#part_docs)* #variants,)*
        }

        #[doc = #patch_doc]
        #[derive(Clone, Debug, Default, PartialEq)]
        #[non_exhaustive]
        pub struct #patch {
            #(#(#part_docs)* pub #fields: ::core::option::Option<#w::PartPatch>,)*
            /// The widget values' patch.
            pub values: ::core::option::Option<#values_patch>,
        }

        impl #patch {
            /// The parts' patches, in the catalogue's order, and the values'
            /// patch.
            #vis fn into_parts(
                self,
            ) -> (
                ::std::vec::Vec<(#part_enum, #w::PartPatch)>,
                ::core::option::Option<#values_patch>,
            ) {
                let Self { #(#fields,)* values } = self;
                let parts = [#((#part_enum::#variants, #fields)),*]
                    .into_iter()
                    .filter_map(|(part, patch)| patch.map(|patch| (part, patch)))
                    .collect();
                (parts, values)
            }
        }

        impl #w::WidgetTable for #name {
            const PARTS: &'static [#r::PartInfo] = &[#(#infos),*];
            const VALUES: &'static [#r::FieldInfo] = <#values as #b::Table>::FIELDS;
        }
    })
}
