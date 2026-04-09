extern crate proc_macro;

use std::collections::HashMap;

use quote::{format_ident, quote, quote_spanned};
use semver::Version;
use syn::{
    Attribute, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, Meta, MetaList, Path,
    Result, Token, Type,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

use crate::{
    compat::{Kind, parse_attr},
    parse::{
        parse_field_compat_attr, parse_outer_compat_attr, parse_serde_container_attr,
        parse_serde_field_attr,
    },
    symbols::{MAP, compat_id},
};

const NAMED_FIELDS_ONLY: &'static str = "GenerateCompat only supports structs with named fields.";

/// Generates a semver-6.0.0 compatible struct for serialization purposes.
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> proc_macro2::TokenStream
{
    let parsed_outer = match parse_outer_compat_attr(&ast.attrs) {
        Ok(parsed) => parsed,
        Err(err) => return err.into_compile_error(),
    };

    let mut need_base_version = false;
    let mut semver_compat = HashMap::new();
    let mut struct_fields = Vec::with_capacity(data.fields.len());

    for field in data.fields.iter() {
        struct_fields.push(field);

        match parse_attr(field.into()) {
            Ok(parsed) => if !parsed.changes.is_empty() {
                for (version, kind) in parsed.changes.into_iter() {
                    // Determine if we need to include a faked "0.0.0" version to handle
                    // compatibility with versions missing newly added fields.
                    need_base_version = need_base_version || kind == Kind::Added;

                    // Duplicates should be handled by `parse_attr`.
                    semver_compat.entry(version)
                        .or_insert(HashMap::<&Field, Kind>::default())
                        .insert(field, kind);
                }
            },
            Err(err) => return err.into_compile_error(),
        }
    }

    // Build out which versions we need to generate.
    let mut versions: Vec<_> = semver_compat.keys()
        .map(|version| version.clone())
        .collect();
    let semver_600 = Version::new(6, 0, 0);
    if !semver_compat.contains_key(&semver_600) {
        versions.push(semver_600);
    }
    if need_base_version {
        // Force a "0.0.0" version to handle transmission versions older than any added fields.
        versions.push(Version::new(0, 0, 0));
    }
    versions.sort(); // Sort the versions so we can find the nearest-lower version.

    // TODO: generate compat structs, orig ->into-> compat-version
    // TODO: generate `.into_compat<T>(v: &Version) -> T` on original struct

    let orig_struct_id = &ast.ident;
    let generics = &ast.generics;
    let semi_token = data.semi_token;
    let compat_structs: HashMap<_, _> = versions
        .iter()
        .map(|version| {
            let struct_id = compat_id(version, orig_struct_id);

            let field_ident = struct_fields.iter().map(|f| {
                f.ident.as_ref().expect(NAMED_FIELDS_ONLY)
            });
            let field_type = struct_fields.iter()
                .map(|f| {
                    match parse_field_compat_attr(&f.attrs, Some(&f.ty), &parsed_outer) {
                        Ok(parsed) => {
                            let ty = parsed.ty.as_ref()
                                .unwrap_or(&f.ty);
                            quote_spanned! {f.ty.span()=> ty }
                        },
                        Err(err) => return err.into_compile_error(),
                    }
                });

            let field_serde: Vec<_> = struct_fields.iter()
                .map(|&f| parse_serde_field_attr(version, &f.into()))
                .collect();
            let container_serde = match parse_serde_container_attr(version, ast) {
                Ok(parsed) => parsed,
                Err(err) => return (version, err.into_compile_error()),
            };

            (version, quote! {
                // TODO
            })
        })
        .collect();



    // ----- OLD

    // Stores the compat struct's field names where the index into the Vec corresponds to the
    // original (legacy) struct's field's name.
    //
    // These will only differ when a field is flagged with #[compat(...)].
    let mut compat_ident = Vec::with_capacity(data.fields.len());
    // Stores the target type for each of the legacy struct's fields.
    let mut type_defn: Vec<Type> = Vec::with_capacity(data.fields.len());
    // Stores the conversion function, if any, for each of the legacy struct's fields.
    let mut conv: Vec<Option<Path>> = Vec::with_capacity(data.fields.len());

    for field in data.fields.iter() {
        let ty = Some(&field.ty);
        let parsed_attr = match parse_field_compat_attr(&field.attrs, ty, &parsed_outer) {
            Err(err) => return err.into_compile_error(),
            Ok(parsed) => parsed,
        };

        compat_ident.push(parsed_attr.name
            .or_else(|| field.ident.clone())
            .expect(NAMED_FIELDS_ONLY)); // Only handle structs with named fields.
        type_defn.push(parsed_attr.ty
            .unwrap_or_else(|| field.ty.clone()));
        conv.push(parsed_attr.map_fn);
    }

    // Generate the compat-struct `into_compat` field conversions.
    let field_conv = {
        let converted_field: Vec<_> = data.fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let ident = field.ident.as_ref().expect(NAMED_FIELDS_ONLY);
                const MSG: &'static str = "every field should have a `conv` item";
                let converted = match conv.get(i).expect(MSG) {
                    // eg. `Option::map(self.x, Into::into)`
                    // NOTE: Probably won't work for non- `Option::map` methods.
                    Some(conv) => quote_spanned! {field.span()=>
                        #conv(self.#ident, Into::into)
                    },
                    // eg. `self.x.into()`
                    None => quote_spanned! {field.span()=>
                        self.#ident.into()
                    },
                };

                // Move each field in `self` to its serialization helper type's corresponding
                // field, converting it if required.
                //
                // eg.
                // Orig   { x: i32, y: i32 }
                // Compat { a: i32, b: i32 }
                //
                // // return Compat { a: self.x, b: self.y }
                let compat = compat_ident.get(i);
                quote_spanned! {field.span()=>
                    #compat: #converted
                }
            })
            .collect();

        match &data.fields {
            Fields::Named(f) => quote_spanned! {f.span()=>
                { #( #converted_field, )* }
            },
            Fields::Unnamed(f) => quote_spanned! {f.span()=>
                ( #( #converted_field, )* )
            },
            Fields::Unit => proc_macro2::TokenStream::new(),
        }
    };

    let orig_struct_id = &ast.ident;
    let compat_struct_id = {
        let tmp = semver::Version::new(6, 0, 0);
        compat_id(&tmp, orig_struct_id)
    };
    let generics = &ast.generics;
    let semi_token = data.semi_token;
    // TODO: let compat_doc = format!("TODO");
    // TODO: Generate a compat struct for each transmission semver
    quote! {
        /// Semver-6.0.0 compatible serialization helper.
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[serde_with::skip_serializing_none] // I think this has appear before derive(Serialize).
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct #compat_struct_id #generics {
            #(#compat_ident: #type_defn),*
        } #semi_token

        #[automatically_derived]
        impl #orig_struct_id {
            //TODO: pub fn into_compat(self, semver: Version) -> #compat_struct_id { ... }
            /// Converts the legacy struct into its semver-6.0.0 compatible serialization helper
            /// type.
            pub fn into_compat(self) -> #compat_struct_id {
                #compat_struct_id #field_conv
            }
        }

        // Helper `From` implementation for the legacy struct -> semver-6.0.0 compatible struct.
        impl From<#orig_struct_id> for #compat_struct_id {
            fn from(value: #orig_struct_id) -> Self {
                value.into_compat()
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Generates a semver-6.0.0 compatible enum for serialization purposes.
pub(crate) fn generate_compat_enum(ast: &DeriveInput, data: &DataEnum)
    -> proc_macro2::TokenStream
{
    let orig_enum_id = &ast.ident;
    let generics = &ast.generics;
    let compat_enum_id = {
        let tmp = semver::Version::new(6, 0, 0);
        compat_id(&tmp, orig_enum_id)
    };

    let parsed_outer = match parse_outer_compat_attr(&ast.attrs) {
        Ok(parsed) => parsed,
        Err(err) => return err.into_compile_error(),
    };

    // Check if the enum is #[serde(untagged)].
    let untagged = match parse_serde_untagged(&ast.attrs) {
        Ok(untagged) => untagged,
        Err(err) => return err.into_compile_error(),
    };

    // (either `Serialize` if no explicit discriminant is encountered or `Serialize_repr` if an
    // explicit discriminant assignment _is_ encountered).
    let (mut ser_crate, mut ser_derive)  = (format_ident!("serde"), format_ident!("Serialize"));

    // Stores the generated compat enum's variant definitions.
    let mut compat_defn = Vec::with_capacity(data.variants.len());
    // Stores the original -> compat variant `into` conversions.
    let mut convert_defn = Vec::with_capacity(data.variants.len());

    for var in data.variants.iter() {
        if var.discriminant.is_some() {
            // Serialize the enum into its discriminant number representation if any variant has an
            // explicitly assigned discriminant.
            //
            // eg. `enum Repr { A, B, C = 123 }`
            (ser_crate, ser_derive) = (
                format_ident!("serde_repr"),
                format_ident!("Serialize_repr"),
            );
        }

        let orig_type: Option<&Type> = match &var.fields {
            Fields::Unit => None,
            Fields::Unnamed(fields) => {
                match fields.unnamed.len() {
                    1 => fields.unnamed.get(0)
                        .map(|f| &f.ty),
                    _ => {
                        let msg = "GenerateCompat does not support enum variants with multiple \
                                  fields";
                        return Error::new(var.span(), msg)
                            .into_compile_error();
                    },
                }
            }
            Fields::Named(_) => {
                let msg = "GenerateCompat does not support enums with struct-like variants.";
                return Error::new(var.span(), msg)
                    .into_compile_error();
            },
        };
        // TODO: store HashMap<Version, ParsedFieldAttr> for each attr: added,changed,depr,removed
        let parsed_attr = match parse_field_compat_attr(&var.attrs, orig_type, &parsed_outer) {
            Err(err) => return err.into_compile_error(),
            Ok(parsed) => parsed,
        };

        let var_id = &var.ident;
        let compat_ident = parsed_attr.name.unwrap_or_else(|| var_id.clone());
        // Construct the compat-enum variant definition and `into_compat` conversion.
        match (&parsed_attr.ty, &parsed_attr.map_fn) {
            (Some(ty), map_fn) => {
                compat_defn.push(quote_spanned! {ty.span()=>
                    #compat_ident(#ty)
                });

                let into = match map_fn {
                    // Map the original -> compat type.
                    Some(map_fn) => quote! {
                        #map_fn(x, Into::into)
                    },
                    // Call the `Into` implementation if no `map` was specified. Usually this will
                    // be a no-op (eg. `let x: i32 = 2.into();`).
                    None => quote! {
                        x.into()
                    },
                };
                let map_span = parsed_attr.span
                    .unwrap_or_else(|| ty.span());
                convert_defn.push(quote_spanned! {map_span=>
                    Self::#var_id(x) => #compat_enum_id::#compat_ident(#into)
                });
            },

            (None, Some(map)) => {
                // #[compat(map = ...)] defined but no type change specified.
                // This isn't really an error but may be indicative of one; may as well force the
                // caller to confront it.
                let msg = format!("\"{MAP}\"");
                return Error::new(map.span(), msg)
                    .into_compile_error();
            },

            (None, None) => {
                compat_defn.push(quote_spanned! {var_id.span()=>
                    #compat_ident
                });
                convert_defn.push(quote_spanned! {var.span()=>
                    Self::#var_id => #compat_enum_id::#compat_ident
                });
            },
        };
    }

    quote! {
        /// Semver-6.0.0 compatible serialization helper.
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[derive(#ser_crate::#ser_derive, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        #untagged
        pub(crate) enum #compat_enum_id #generics {
            #(#compat_defn),*
        }

        #[automatically_derived]
        impl #orig_enum_id {
            /// Converts the legacy enum into its semver-6.0.0 compatible serialization helper
            /// type.
            pub fn into_compat(self) -> #compat_enum_id {
                match self {
                    #(#convert_defn),*
                }
            }
        }

        // This Display impl for enums is specifically to facilitate testing Method serialization.
        #[automatically_derived]
        #[cfg(test)]
        impl std::fmt::Display for #compat_enum_id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let as_str = serde_json::to_string(self)
                    .map_err(|_| std::fmt::Error)?;
                write!(f, "{as_str}")
            }
        }

        // Helper `From` implementation for the legacy enum -> semver-6.0.0 compatible enum.
        impl From<#orig_enum_id> for #compat_enum_id {
            fn from(value: #orig_enum_id) -> Self {
                value.into_compat()
            }
        }
    }
}

/// Parses the original enum-level attributes for the `#[serde(untagged)]` attribute.
///
/// Returns a [`Result`] containing `Some("#[serde(untagged)]")` if found; `None` if no attribute
/// matches.
fn parse_serde_untagged<'a, I>(attributes: I) -> Result<Option<proc_macro2::TokenStream>>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut untagged = None;
    for ast_attr in attributes.into_iter() {
        if !ast_attr.path().is_ident("serde") {
            continue;
        }

        // `attr.parse_nested_meta` seems to fail if there is only a single expression (eg.
        // `#[serde(rename = "foo")]`). So we need to "manually" parse with `parse_args_with`.
        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        let nested = ast_attr.parse_args_with(parser)?;
        for meta in nested.iter() {
            if meta.path().is_ident("untagged") {
                untagged = Some(quote_spanned! {ast_attr.span()=>
                    #[serde(untagged)]
                });
                break;
            }
        }

        if untagged.is_some() {
            break;
        }
    }
    Ok(untagged)
}
