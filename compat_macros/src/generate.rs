extern crate proc_macro;

use std::collections::HashMap;

use quote::{format_ident, quote, quote_spanned};
use semver::Version;
use syn::{
    Attribute, DataEnum, DataStruct, DeriveInput, Error, Fields, Meta, Path, Result, Token, Type,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

use crate::{
    compat::{Kind, parse_attr},
    parse::{parse_container_compat_attr, parse_serde_container_attr, parse_serde_field_attr},
    placeholder::{
        determine_which_into_func, gen_into_wrapper_func, gen_opt_vec_into_func, gen_vec_into_func,
        ident_into_wrapper, replace_compat_placeholder
    },
    symbols::{CompatVersion, MAP, compat_id, version_id}
};

const NAMED_FIELDS_ONLY: &'static str = "GenerateCompat only supports structs with named fields.";

/// Generates compatible structs for each attribute-defined semver (eg. `#\[added(semver =
/// "6.0.0")\]`).
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> proc_macro2::TokenStream
{
    let parsed_container = match parse_container_compat_attr(&ast.attrs) {
        Ok(parsed) => parsed,
        Err(err) => return err.into_compile_error(),
    };

    let mut has_added_field = false;
    let mut semver_compat = HashMap::new();
    let mut versions = vec![];
    let mut struct_fields = Vec::with_capacity(data.fields.len());
    let mut struct_replace_types = Vec::with_capacity(data.fields.len());

    for field in data.fields.iter() {
        struct_fields.push(field);

        match parse_attr(field.into()) {
            Ok(parsed) => if !parsed.changes.is_empty() {
                for (version, kind) in parsed.changes.into_iter() {
                    // Determine if we need to include a faked "0.0.0" version to handle
                    // compatibility with versions missing newly added fields.
                    has_added_field = has_added_field || kind == Kind::Added;

                    // No need to worry about duplicate versions; they should be handled by
                    // `parse_attr`.
                    semver_compat.entry(field)
                        .or_insert(HashMap::<_, _>::default())
                        .insert(version.clone(), kind);
                    versions.push(version);
                }
                struct_replace_types.push(parsed.replace_type);
            },

            Err(err) => return err.into_compile_error(),
        }
    }

    // Always include semver-6.0.0 since we need to force snake_case serialization.
    versions.push(Version::new(6, 0, 0));
    if has_added_field {
        // Force a "0.0.0" version to handle transmission versions older than any added fields.
        versions.push(Version::new(0, 0, 0));
    }
    // Sort the versions so we can find the nearest-lower version to convert from the
    // source-defined struct to its corresponding version-compat version.
    versions.sort();
    versions.dedup(); // Remove duplicate semver-6.0.0 if needed.

    let orig_struct_id = &ast.ident;
    let generics = &ast.generics;
    let semi_token = data.semi_token;
    let compat_structs: HashMap<_, _> = versions
        .iter()
        .map(|version| {
            let struct_id = compat_id(version, orig_struct_id);
            let from_arg_ident = format_ident!("orig");

            let mut field_ident = Vec::with_capacity(struct_fields.len());
            let mut field_type = Vec::with_capacity(struct_fields.len());
            let mut field_into = Vec::with_capacity(struct_fields.len());
            for (i, &f) in struct_fields.iter().enumerate() {
                let mut include = true;
                let mut ident = None;
                if let Some(changes) = semver_compat.get(f) {
                    for (change_version, kind) in changes.iter() {
                        match kind {
                            // NOTE: We don't blindly set `include`, eg.
                            // NOTE- `include = version >= change_version`,
                            // NOTE- to prevent incorrectly defining a field for a version
                            // NOTE- where it should not exist (before it was added or after it
                            // NOTE- was removed).
                            Kind::Added => if version < change_version {
                                include = false;
                            },
                            Kind::Removed => if version >= change_version {
                                include = false;
                            },
                            Kind::Renamed(id) => ident = Some(id.clone()),
                        }
                    }
                }

                ident = include.then_some(
                    ident.unwrap_or_else(|| f.ident.clone().expect(NAMED_FIELDS_ONLY))
                );
                field_ident.push(ident);

                // Try to replace the field's type if it has a placeholder type attribute.
                let mut ty = None;
                if let Some(placeholder) = parsed_container.placeholder.as_ref()
                    && let Some(mut attr_type) = struct_replace_types
                        .get(i)
                        .map(Clone::clone)
                        .flatten()
                {
                    // TODO: FIXME: only works if field has a semver attr (eg. #[added(semver = ...)])
                    // TODO- FIXME- ... doesn't work for inner type conversions (if the field
                    // TODO- FIXME- itself isn't tagged with a semver attr but the field's type
                    // TODO- FIXME- does have compat versions - eg. Encryption).
                    match replace_compat_placeholder(version, &f.ty, &mut attr_type, placeholder) {
                        // TODO: need to detect Option< or Vec< and generate conversion func/tokens
                        Ok(()) => ty = Some(attr_type),
                        Err(err) => return (version, err.into_compile_error()),
                    }
                }
                field_into.push({
                    include.then(|| {
                        let map_func = match ty.as_ref()
                            .map(determine_which_into_func)
                            .map(|result| match result {
                                Ok(path) => quote! { #path },
                                Err(err) => err.into_compile_error(),
                            })
                        {
                            // Convert the source-defined field -> placeholder-replaced field with
                            // the parsed conversion function.
                            Some(func) => func,

                            // Fallback to `Into::into` if the field has no placeholder replacement
                            // type (effectively: move the original field into the compat struct's
                            // field).
                            None => {
                                let into_wrapper = ident_into_wrapper();
                                quote! { #into_wrapper }
                            },
                        };

                        let orig_field = f.ident.as_ref().expect(NAMED_FIELDS_ONLY);
                        quote! {
                            #map_func(#from_arg_ident.#orig_field, Into::into)
                        }
                    })
                });
                field_type.push(include.then(|| {
                    ty.unwrap_or(f.ty.clone())
                }));
            }

            let mut field_serde = Vec::with_capacity(struct_fields.len());
            for &f in struct_fields.iter() {
                match parse_serde_field_attr(version, f.into()) {
                    Ok(attrs) => field_serde.push(attrs),
                    Err(err) => return (version, err.into_compile_error()),
                }
            }
            let container_serde = match parse_serde_container_attr(version, ast) {
                Ok(parsed) => parsed,
                Err(err) => return (version, err.into_compile_error()),
            };
            let struct_doc = format!("Transmission semver-{version} compatible \
                request serialization helper type");
            // Build a vec of field definitions to avoid emitting stray ':'.
            let fields = {
                let mut fields = Vec::with_capacity(field_ident.len());
                for ((id, ty), attr) in field_ident.iter()
                    .zip(field_type.iter())
                    .zip(field_serde.iter())
                    {
                        let (Some(id), Some(ty)) = (id, ty) else {
                            continue
                        };
                        fields.push(quote! {
                            #( #attr )*
                            #id: #ty
                        })
                    }
                fields
            };
            let compat_struct = quote! {
                #[doc = #struct_doc]
                #[automatically_derived]
                #[allow(non_camel_case_types)]
                #[serde_with::skip_serializing_none]
                #[derive(serde::Serialize, Debug, Clone)]
                #(#container_serde)*
                pub(crate) struct #struct_id #generics {
                    #( #fields ),*
                } #semi_token
            };

            let into_func_defn = [
                gen_into_wrapper_func(),
                gen_opt_vec_into_func(),
                gen_vec_into_func(),
            ];
            // Build a vec of field-into conversions to avoid emitting stray ':'.
            let into_fields = {
                let mut into = Vec::with_capacity(field_into.len());
                for (id, field_into) in field_ident.iter().zip(field_into.iter()) {
                    let (Some(id), Some(field_into)) = (id, field_into) else {
                        continue
                    };
                    into.push(quote! {
                        #id: #field_into
                    });
                }
                into
            };

            (version, quote! {
                #compat_struct

                impl From<#orig_struct_id> for #struct_id {
                    fn from(#from_arg_ident: #orig_struct_id) -> Self {
                        // Define the helper field conversion functions which may or may not be
                        // used.
                        #( #[automatically_derived] #into_func_defn )*

                        Self {
                            #( #into_fields ),*
                        }
                    }
                }
            })
        })
        .collect();

    let compat_enum_doc = format!("Holds every compat struct generated for {orig_struct_id}");
    let compat_enum_ident = format_ident!("__{orig_struct_id}_compat__");
    let compat_struct_defn = compat_structs.values();

    let variant_ident: Vec<_> = versions.iter().map(version_id).collect();
    let struct_ident: Vec<_> = versions.iter().map(|v| compat_id(v, orig_struct_id)).collect();
    let versions: Vec<_> = versions.iter()
        .map(|version| CompatVersion(version.clone()))
        .collect();

    quote! {
        #( #compat_struct_defn )*

        #[doc = #compat_enum_doc]
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(untagged)]
        pub(crate) enum #compat_enum_ident {
            #( #variant_ident(#struct_ident) ),*

            Base(#orig_struct_id),
        }

        impl #orig_struct_id {
            pub(crate) fn into_compat(self, target: &semver::Version) -> #compat_enum_ident {
                let compat_map: std::collections::HashMap<_, _> = [
                    #( (#versions, #compat_enum_ident::#variant_ident) ),*
                ]
                .into_iter()
                .collect();

                match compat_map.get(target) {
                    // We generated a compat type for this version.
                    Some(variant) => variant(self.into()),

                    // We need to search for the compat type that corresponds to this version (the
                    // nearest generated compat version that is less than the target version; this
                    // compat version should encapsulate all of the transmission semver changes
                    // that apply to the target version).
                    //
                    // eg. If we generated compat types for
                    //          * 2.1.0
                    //          * 5.0.0
                    //          * 6.0.0
                    //     and are serializing a request for semver `5.1.0`, we want the compat
                    //     type generated for `5.0.0`.
                    None => {
                        let mut compat_ver = None;
                        // This should iterate in sorted (lowest -> highest) order.
                        // (The loop MUST iterate in sorted order.)
                        for v in [ #( #versions ),* ] {
                            if v >= target { // The `==` case should never happen.
                                let Some(compat_ver) = compat_ver else {
                                    // We did not generate a compat version lower than the target
                                    // version so we can just use the base (source-defined) type.
                                    break
                                };
                                // The previous iteration's version should be the correct compat
                                // type for the target version.
                                let variant = compat_map.get(&compat_ver)
                                    .expect("request compat type should exist");
                                return variant(self.into());
                            }
                            // Keep track of the previous iteration's version which could be the
                            // version corresponding to the compat type we want (assuming we're
                            // iterating in sorted order).
                            compat_ver = Some(v);
                        }

                        #compat_enum_ident::Base(self)
                    },
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

/// Generates compatible enums for each attribute-defined semver (eg. `#\[renamed(semver = "6.0.0",
/// name = "...")\]`).
pub(crate) fn generate_compat_enum(ast: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream
{
    let parsed_container = match parse_container_compat_attr(&ast.attrs) {
        Ok(parsed) => parsed,
        Err(err) => return err.into_compile_error(),
    };

    let mut has_added_field = false;
    let mut semver_compat = HashMap::new();
    let mut versions = vec![];
    let mut enum_variants = Vec::with_capacity(data.variants.len());

    quote! {
    }
}

/* TODO: DELETE (OLD)
/// Generates a semver-6.0.0 compatible enum for serialization purposes.
pub(crate) fn __OLD_generate_compat_enum(ast: &DeriveInput, data: &DataEnum)
    -> proc_macro2::TokenStream
{
    let orig_enum_id = &ast.ident;
    let generics = &ast.generics;
    let compat_enum_id = {
        let tmp = semver::Version::new(6, 0, 0);
        compat_id(&tmp, orig_enum_id)
    };

    let parsed_container = match parse_container_compat_attr(&ast.attrs) {
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
        let parsed_attr = match parse_field_compat_attr(&var.attrs, orig_type, &parsed_container) {
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
*/
