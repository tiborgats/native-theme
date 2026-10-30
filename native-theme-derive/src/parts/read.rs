// Reading from TOML (S11): a struct block's patch read key by key and its
// block built whole from it, and an enum read by its variants, for blocks and
// for the model's other enums (`#[parts(toml)]`).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, Ident, Result, Type};

use super::attrs::BlockKind;
use super::block::{BlockField, Shape, Wrap, generic_arg, map_pair, parse_field, snake_case};

/// The module the expansions name the reader by.
fn reader() -> TokenStream {
    quote!(crate::next::toml::read)
}

/// How a field's value is read from its key's item, and what its key is
/// spelled by.
struct Reading {
    /// Reads `item` under `spelling` at `at` into an `Option` of the value.
    read: TokenStream,
    /// The spellings of the field's key.
    spellings: TokenStream,
    /// How a second key for the field combines.
    combine: TokenStream,
    /// The value is a `FromToml` type, finished after its table is read.
    finished: Option<TokenStream>,
}

/// The reading of a value of type `ty` (a patch's field, a variant's field),
/// by the field's shape: maps by their keys, every other value by its type.
fn reading(shape: &Shape, ty: &TokenStream, value_ty: &Type) -> Reading {
    let t = reader();
    let bare = quote!(&[#t::Spelling::Bare]);
    match shape {
        Shape::NestedMap => {
            let (key, inner) = map_pair(value_ty).map_or((None, None), |(k, v)| (Some(k), Some(v)));
            let (inner_key, value) = inner
                .and_then(map_pair)
                .map_or((None, None), |(k, v)| (Some(k), Some(v)));
            Reading {
                read: quote! {
                    #t::read_nested_map::<#key, #inner_key, #value>(item, &at, cx)
                },
                spellings: bare,
                combine: quote!(#t::no_combine),
                finished: None,
            }
        }
        Shape::Map(map) => {
            let key = map_pair(value_ty).map(|(k, _)| k);
            let value = match map.as_ref().and_then(|m| m.entry_patch()) {
                Some(entry) => entry,
                None => {
                    let value = map_pair(value_ty).map(|(_, v)| v);
                    quote!(#value)
                }
            };
            Reading {
                read: quote!(#t::read_map::<#key, #value>(item, &at, cx)),
                spellings: bare,
                combine: quote!(#t::no_combine),
                finished: None,
            }
        }
        Shape::Surfaces(block) => Reading {
            read: {
                let block = &block.ty;
                quote! {
                    #t::read_surfaces::<crate::next::blocks::BlockPatch<#block>>(item, &at, cx)
                }
            },
            spellings: bare,
            combine: quote!(#t::no_combine),
            finished: None,
        },
        Shape::Leaf | Shape::Block { .. } | Shape::Compound(_) => Reading {
            read: quote!(<#ty as #t::FromToml>::read(item, spelling, &at, cx)),
            spellings: quote!(<#ty as #t::FromToml>::SPELLINGS),
            combine: quote!(<#ty as #t::FromToml>::combine),
            finished: Some(ty.clone()),
        },
    }
}

/// Whether `ty` is an `Option`, which a block built whole leaves `None` where
/// its key is left out of a list's item or a variant's payload (S11).
fn is_option(ty: &Type) -> bool {
    generic_arg(ty, "Option").is_some()
}

/// Reads one key of a table into the slot `slot` of the field `name`: an
/// expression `true` where the key spells the field.
fn read_key(name: &str, slot: &TokenStream, reading: &Reading) -> TokenStream {
    let t = reader();
    let Reading {
        read,
        spellings,
        combine,
        ..
    } = reading;
    quote! {
        if let ::core::option::Option::Some(spelling) = #t::spelled(#name, key, #spellings) {
            let _ = spelling;
            let table = at;
            let at = #t::join(at, key);
            if let ::core::option::Option::Some(read_value) = #read {
                #t::put(#slot, read_value, #combine, table, key, #name, cx);
            }
            true
        } else {
            false
        }
    }
}

/// A struct block's readers: its patch read key by key (`Fields`), unless
/// written by hand its patch and its whole block read from a table
/// (`FromToml`), its block built whole from its patch (`Build`, S1's
/// `try_from`).
pub(super) fn struct_impls(name: &Ident, fields: &[BlockField], own_toml: bool) -> TokenStream {
    let t = reader();
    let m = quote!(crate::next);
    let o = quote!(::core::option::Option);
    let patch = format_ident!("{name}Patch");
    let idents: Vec<&Ident> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let names: Vec<String> = idents.iter().map(ToString::to_string).collect();
    let locals: Vec<Ident> = idents.iter().map(|i| format_ident!("field_{i}")).collect();

    let readings: Vec<Reading> = fields
        .iter()
        .map(|f| {
            let inner = f.patch_inner();
            reading(&f.shape, &inner, &f.ty)
        })
        .collect();
    let entries = idents
        .iter()
        .zip(&names)
        .zip(&readings)
        .map(|((ident, name), reading)| {
            let read = read_key(name, &quote!(&mut self.#ident), reading);
            quote!(if #read { return true; })
        });
    let field_of = names.iter().zip(&readings).map(|(name, reading)| {
        let spellings = &reading.spellings;
        quote! {
            if #t::spelled(#name, key, #spellings).is_some() {
                return #o::Some(#name);
            }
        }
    });
    let finishes =
        idents
            .iter()
            .zip(&names)
            .zip(&readings)
            .filter_map(|((ident, name), reading)| {
                reading.finished.as_ref().map(|ty| {
                    quote! {
                        if let #o::Some(value) = self.#ident.take() {
                            self.#ident = #o::Some(
                                <#ty as #t::FromToml>::finish(value, &#t::join(at, #name), cx),
                            );
                        }
                    }
                })
            });
    let disjoint = idents
        .iter()
        .map(|ident| quote!((self.#ident.is_some() && later.#ident.is_some())));
    let builds = fields
        .iter()
        .zip(&idents)
        .zip(&names)
        .zip(&locals)
        .zip(&readings)
        .map(|((((f, _), name), local), reading)| build_field(f, local, name, &reading.spellings));

    let from_toml = if own_toml {
        TokenStream::new()
    } else {
        quote! {
            impl #t::FromToml for #patch {
                fn read(
                    item: &::toml::Value,
                    _: #t::Spelling,
                    at: &str,
                    cx: &mut #t::Reader,
                ) -> #o<Self> {
                    let table = #t::table(item, at, cx)?;
                    #o::Some(#t::read_fields::<Self>(table, at, cx))
                }

                fn combine(&mut self, later: Self) -> bool {
                    #t::Fields::combine_fields(self, later)
                }
            }

            impl #t::FromToml for #name {
                fn read(
                    item: &::toml::Value,
                    spelling: #t::Spelling,
                    at: &str,
                    cx: &mut #t::Reader,
                ) -> #o<Self> {
                    let patch = <#patch as #t::FromToml>::read(item, spelling, at, cx)?;
                    <Self as #t::Build>::build(patch, at, true, cx)
                }
            }
        }
    };

    quote! {
        impl #t::Fields for #patch {
            fn read_entry(
                &mut self,
                key: &str,
                item: &::toml::Value,
                at: &str,
                cx: &mut #t::Reader,
            ) -> bool {
                let _ = (item, &cx);
                #(#entries)*
                false
            }

            fn field_of(key: &str) -> #o<&'static str> {
                #(#field_of)*
                #o::None
            }

            fn finish_fields(&mut self, at: &str, cx: &mut #t::Reader) {
                let _ = (at, &cx);
                #(#finishes)*
            }

            fn combine_fields(&mut self, later: Self) -> bool {
                if false #(|| #disjoint)* {
                    return false;
                }
                let Self { #(#idents),* } = later;
                #(
                    if #idents.is_some() {
                        self.#idents = #idents;
                    }
                )*
                true
            }
        }

        #from_toml

        impl #t::Build for #name {
            type Patch = #patch;

            fn build(
                patch: #patch,
                at: &str,
                options_none: bool,
                cx: &mut #t::Reader,
            ) -> #o<Self> {
                let _ = (at, options_none, &cx);
                let #patch { #(#idents: #locals),* } = patch;
                #(#builds)*
                #o::Some(Self { #(#idents: #locals?),* })
            }
        }

        impl ::core::convert::TryFrom<#patch> for #name {
            type Error = #m::LayerError;

            /// The block `patch` states whole, or every leaf it leaves
            /// missing (S1).
            fn try_from(patch: #patch) -> ::core::result::Result<Self, #m::LayerError> {
                let mut cx = #t::Reader::default();
                match <Self as #t::Build>::build(patch, "", false, &mut cx) {
                    #o::Some(block) if !cx.failed() => ::core::result::Result::Ok(block),
                    _ => ::core::result::Result::Err(#m::LayerError::Invalid(cx.into_problems())),
                }
            }
        }


    }
}

/// One field of a block built whole from its patch, the patch's field bound
/// to the field's name: bound again to an `Option` of the field's value,
/// `None` where it is missing (each missing leaf recorded).
fn build_field(f: &BlockField, ident: &Ident, name: &str, spellings: &TokenStream) -> TokenStream {
    let t = reader();
    let b = quote!(crate::next::blocks);
    let o = quote!(::core::option::Option);
    let at = quote!(&#t::join(at, &#t::missing_key(#name, #spellings)));
    let value = match &f.shape {
        _ if f.attrs.keyed => quote! {
            #t::required(#ident, #at, cx).map(|items| {
                let mut held = ::std::vec::Vec::new();
                #b::KeyedItem::merge_keyed(&mut held, items);
                held
            })
        },
        Shape::Compound(compound) => {
            let name_ty = format_ident!("{}", compound.name);
            let inner = &compound.inner;
            let members: Vec<Ident> = compound
                .members
                .iter()
                .map(|member| format_ident!("{member}"))
                .collect();
            let names = compound.members.iter();
            quote! {{
                let held = #ident.unwrap_or(crate::next::#name_ty {
                    #(#members: #o::None),*
                });
                #(let #members = #t::required(
                    held.#members,
                    &#t::join(
                        &#t::join(at, #name),
                        &#t::missing_key(#names, <#inner as #t::FromToml>::SPELLINGS),
                    ),
                    cx,
                );)*
                match (#(#members),*) {
                    (#(#o::Some(#members)),*) => #o::Some(crate::next::#name_ty { #(#members),* }),
                    _ => #o::None,
                }
            }}
        }
        Shape::Block {
            kind: BlockKind::Fields,
            wrap: Wrap::Bare,
            block,
        } => {
            let ty = &block.ty;
            quote! {
                <#ty as #t::Build>::build(#ident.unwrap_or_default(), #at, options_none, cx)
            }
        }
        Shape::Block {
            kind: BlockKind::Fields,
            wrap: Wrap::Value,
            block,
        } => {
            let ty = &block.ty;
            quote! {
                match #ident {
                    #o::Some(patch) => #t::build_value::<#ty>(patch, #at, options_none, cx),
                    #o::None => #t::required(#o::None, #at, cx),
                }
            }
        }
        Shape::Block {
            kind: BlockKind::Append,
            wrap: Wrap::Value,
            ..
        } => quote! {
            match #ident {
                #o::Some(patch) => #o::Some(#t::build_paint(patch)),
                #o::None => #t::required(#o::None, #at, cx),
            }
        },
        Shape::Surfaces(block) => {
            let ty = &block.ty;
            quote! {
                match #ident {
                    #o::Some(sets) => #t::build_surfaces(sets, #at, cx, |patch, at, cx| {
                        #t::build_bare::<#ty>(patch, at, options_none, cx)
                    }),
                    #o::None => #t::required(#o::None, #at, cx),
                }
            }
        }
        Shape::Map(Some(map)) if map.entry_patch().is_some() => {
            let ty = &map.block.ty;
            let entry = match (map.kind, map.wrap) {
                (BlockKind::Fields, Wrap::Bare) => {
                    quote!(#t::build_bare::<#ty>(patch, at, options_none, cx))
                }
                (BlockKind::Fields, _) => {
                    quote!(#t::build_value::<#ty>(patch, at, options_none, cx))
                }
                _ => quote! {{
                    let _ = (at, &cx);
                    #o::Some(#t::build_paint(patch))
                }},
            };
            quote! {
                match #ident {
                    #o::Some(entries) => #t::build_entries(entries, #at, cx, |patch, at, cx| #entry),
                    #o::None => #t::required(#o::None, #at, cx),
                }
            }
        }
        Shape::Leaf | Shape::Map(_) | Shape::NestedMap | Shape::Block { .. } => {
            if is_option(&f.ty) {
                quote!(#t::optional(#ident, options_none, #at, cx))
            } else {
                quote!(#t::required(#ident, #at, cx))
            }
        }
    };
    quote!(let #ident = #value;)
}

/// A one-field tuple variant `Keyed(Key)`: spelled `"@<key>"`, as a keyed
/// name is (S2.2, S10).
fn is_keyed(ident: &Ident, fields: &[BlockField]) -> bool {
    ident == "Keyed"
        && matches!(fields, [only] if only.ident.is_none()
            && matches!(&only.ty, Type::Path(path) if path.path.is_ident("Key")))
}

/// An enum's reader (S11): a unit variant by its snake_case name, a keyed
/// one as `"@<key>"`, a variant with data as a one-key table of its payload
/// (a struct variant's fields as a table, a tuple variant's one value as it
/// is, several as a positional array); and for an enum of unit variants
/// only, its names (`Named`).
pub(super) fn enum_impl(name: &Ident, data: &syn::DataEnum) -> Result<TokenStream> {
    let t = reader();
    let o = quote!(::core::option::Option);
    let mut units = Vec::new();
    let mut unit_names = Vec::new();
    let mut keyed = TokenStream::new();
    let mut payloads = Vec::new();
    for variant in &data.variants {
        let ident = &variant.ident;
        let snake = snake_case(&ident.to_string());
        match &variant.fields {
            Fields::Unit => {
                units.push(ident.clone());
                unit_names.push(snake);
            }
            Fields::Named(named) => {
                let fields = named
                    .named
                    .iter()
                    .map(parse_field)
                    .collect::<Result<Vec<_>>>()?;
                payloads.push(struct_variant(name, ident, &snake, &fields));
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed
                    .unnamed
                    .iter()
                    .map(parse_field)
                    .collect::<Result<Vec<_>>>()?;
                if is_keyed(ident, &fields) {
                    keyed = quote! {
                        if let #o::Some(key) = name.strip_prefix('@') {
                            return #t::key_named(key, at, cx).map(#name::#ident);
                        }
                    };
                    continue;
                }
                payloads.push(tuple_variant(name, ident, &snake, &fields)?);
            }
        }
    }
    let named = if payloads.is_empty() && keyed.is_empty() {
        let from_name = if units.is_empty() {
            quote! {
                let _ = name;
                #o::None
            }
        } else {
            quote! {
                match name {
                    #(#unit_names => #o::Some(#name::#units),)*
                    _ => #o::None,
                }
            }
        };
        quote! {
            impl #t::Named for #name {
                fn name(&self) -> &'static str {
                    match *self {
                        #(#name::#units => #unit_names,)*
                    }
                }

                fn from_name(name: &str) -> #o<Self> {
                    #from_name
                }
            }
        }
    } else {
        TokenStream::new()
    };
    let by_name = if units.is_empty() {
        quote!(cx.unknown(at, name))
    } else {
        quote! {
            match name {
                #(#unit_names => #o::Some(#name::#units),)*
                _ => cx.unknown(at, name),
            }
        }
    };
    let by_payload = if payloads.is_empty() {
        quote! {
            let _ = payload;
            cx.problem(&#t::join(at, name), crate::next::ProblemKind::UnknownKey);
            #o::None
        }
    } else {
        quote! {
            let at = #t::join(at, name);
            match name {
                #(#payloads)*
                _ => {
                    cx.problem(&at, crate::next::ProblemKind::UnknownKey);
                    #o::None
                }
            }
        }
    };
    Ok(quote! {
        #named

        impl #t::FromToml for #name {
            fn read(
                item: &::toml::Value,
                _: #t::Spelling,
                at: &str,
                cx: &mut #t::Reader,
            ) -> #o<Self> {
                let (name, payload) = #t::variant(item, at, cx)?;
                let #o::Some(payload) = payload else {
                    #keyed
                    return #by_name;
                };
                #by_payload
            }
        }
    })
}

/// A struct variant's arm: its payload a table of its fields, each keyed as
/// a block's field is; an `Option` left out is `None` (S11).
fn struct_variant(owner: &Ident, ident: &Ident, snake: &str, fields: &[BlockField]) -> TokenStream {
    let t = reader();
    let o = quote!(::core::option::Option);
    let idents: Vec<&Ident> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let names: Vec<String> = idents.iter().map(ToString::to_string).collect();
    let locals: Vec<Ident> = idents.iter().map(|i| format_ident!("field_{i}")).collect();
    let keys = fields
        .iter()
        .zip(&locals)
        .zip(&names)
        .map(|((f, local), name)| {
            let ty = &f.ty;
            let reading = reading(&f.shape, &quote!(#ty), ty);
            let read = read_key(name, &quote!(&mut #local), &reading);
            quote!(if #read { continue; })
        });
    let builds = fields
        .iter()
        .zip(&locals)
        .zip(&names)
        .map(|((f, local), name)| {
            let ty = &f.ty;
            let spellings = reading(&f.shape, &quote!(#ty), ty).spellings;
            let at = quote!(&#t::join(&at, &#t::missing_key(#name, #spellings)));
            if is_option(&f.ty) {
                quote!(let #local = #t::optional(#local, true, #at, cx);)
            } else {
                quote!(let #local = #t::required(#local, #at, cx);)
            }
        });
    quote! {
        #snake => {
            let table = #t::table(payload, &at, cx)?;
            #(let mut #locals = #o::None;)*
            for (key, item) in table {
                let at = at.as_str();
                #(#keys)*
                cx.problem(&#t::join(at, key), crate::next::ProblemKind::UnknownKey);
            }
            #(#builds)*
            #o::Some(#owner::#ident { #(#idents: #locals?),* })
        }
    }
}

/// A tuple variant's arm: one value as it is, several as a positional array
/// (S11).
fn tuple_variant(
    owner: &Ident,
    ident: &Ident,
    snake: &str,
    fields: &[BlockField],
) -> Result<TokenStream> {
    let t = reader();
    let o = quote!(::core::option::Option);
    match fields {
        [only] => {
            let ty = &only.ty;
            let read = match &only.shape {
                Shape::Map(_) | Shape::NestedMap => {
                    let reading = reading(&only.shape, &quote!(#ty), ty);
                    let read = reading.read;
                    quote! {{
                        let item = payload;
                        #read
                    }}
                }
                _ => quote!(<#ty as #t::FromToml>::read(payload, #t::Spelling::Item, &at, cx)),
            };
            Ok(quote!(#snake => #read.map(#owner::#ident),))
        }
        several => {
            let count = several.len();
            let expected = format!("an array of {count} values");
            let members: Vec<Ident> = (0..count).map(|i| format_ident!("member{i}")).collect();
            let reads = several
                .iter()
                .zip(&members)
                .enumerate()
                .map(|(i, (f, member))| {
                    let ty = &f.ty;
                    if matches!(f.shape, Shape::Map(_) | Shape::NestedMap) {
                        return Err(Error::new_spanned(
                            ty,
                            "a tuple variant of several fields holds no map",
                        ));
                    }
                    Ok(quote! {
                        let #member = <#ty as #t::FromToml>::read(
                            #member, #t::Spelling::Item, &#t::item(&at, #i), cx,
                        );
                    })
                });
            let reads = reads.collect::<Result<Vec<_>>>()?;
            Ok(quote! {
                #snake => {
                    let [#(#members),*] = #t::members::<#count>(payload, #expected, &at, cx)?;
                    #(#reads)*
                    #o::Some(#owner::#ident(#(#members?),*))
                }
            })
        }
    }
}

/// `#[parts(toml)]`: an enum read from TOML that is no block.
pub(super) fn expand_toml(input: &DeriveInput) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "an enum read from TOML has no generic parameters",
        ));
    }
    match &input.data {
        Data::Enum(data) => {
            let read = enum_impl(&input.ident, data)?;
            let walk =
                super::walk::enum_walk(&input.ident, data, &super::walk::Expr::NoCalls, false)?;
            Ok(quote!(#read #walk))
        }
        Data::Struct(_) | Data::Union(_) => Err(Error::new_spanned(
            &input.ident,
            "`#[parts(toml)]` marks an enum",
        )),
    }
}
