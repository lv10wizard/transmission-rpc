extern crate proc_macro;

use std::collections::HashMap;

use quote::{format_ident, quote, quote_spanned};
use semver::Version;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Result, Type,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

use crate::{
    compat::{FieldOrVar, Kind, parse_attr},
    parse::{parse_container_compat_attr, parse_serde_container_attr, parse_serde_field_attr},
    placeholder::{
        determine_which_into_func, gen_into_wrapper_func, gen_opt_vec_into_func, gen_vec_into_func,
        ident_into_wrapper, replace_compat_placeholder
    },
    symbols::{compat_id, version_id}
};

const NAMED_FIELDS_ONLY: &'static str = "GenerateCompat only supports structs with named fields.";

fn supported_versions() -> Vec<Version> {
    let mut versions = vec![
        Version::new(1, 3, 0), // Transmission 1.50

        Version::new(2, 0, 0), // Transmission 1.60
        Version::new(2, 1, 0), // Transmission 1.70

        Version::new(3, 0, 0), // Transmission 1.80
        Version::new(3, 1, 0), // Transmission 1.90
        Version::new(3, 2, 0), // Transmission 1.92
        Version::new(3, 3, 0), // Transmission 2.00
        Version::new(3, 4, 0), // Transmission 2.10
        Version::new(3, 5, 0), // Transmission 2.12
        Version::new(3, 6, 0), // Transmission 2.20

        Version::new(4, 0, 0), // Transmission 2.30

        Version::new(5, 0, 0), // Transmission 2.40
        Version::new(5, 1, 0), // Transmission 2.80
        Version::new(5, 2, 0), // Transmission 3.00
        Version::new(5, 3, 0), // Transmission 4.0.0

        Version::new(6, 0, 0), // Transmission 4.1.0
        Version::new(6, 0, 1), // Transmission 4.1.1
        // TODO: Version::new(6, 1, 0), // Transmission 4.2.0
    ];

    // Ensure the versions are sorted and unique.
    versions.sort();
    versions.dedup();

    versions
}

pub(crate) struct StructOrEnum<'a> {
    inner: &'a Data,
}

impl<'a> StructOrEnum<'a> {
    #[allow(unused)]
    pub(crate) fn new(data: &'a Data) -> Self {
        data.into()
    }

    fn keyword(&self) -> Result<proc_macro2::TokenStream> {
        match &self.inner {
            Data::Enum(e) => {
                let token = e.enum_token;
                Ok(quote! { #token })
            },
            Data::Struct(s) => {
                let token = s.struct_token;
                Ok(quote! { #token })
            },
            Data::Union(u) => Err(Error::new(u.union_token.span(), "unsupported type")),
        }
    }

    fn fields(&self) -> Result<Vec<FieldOrVar<'_>>> {
        match &self.inner {
            Data::Enum(e) => Ok(e.variants.iter().map(Into::into).collect()),
            Data::Struct(s) => Ok(s.fields.iter().map(Into::into).collect()),
            Data::Union(u) => Err(Error::new(u.union_token.span(), "unsupported type")),
        }
    }
}

impl<'a> From<&'a Data> for StructOrEnum<'a> {
    fn from(value: &'a Data) -> Self {
        let inner = match value {
            Data::Enum(_) => value,
            Data::Struct(_) => value,
            Data::Union(_) => panic!("unsupported type: union"),
        };
        Self { inner }
    }
}

/*
/// Generates compatible structs for each supported (hardcoded) transmission rpc semver.
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> Result<proc_macro2::TokenStream>
{
    let parsed_container = parse_container_compat_attr(&ast.attrs)?;

    let mut semver_compat = HashMap::new();
    let versions = supported_versions();
    let mut struct_fields = Vec::with_capacity(data.fields.len());
    let mut struct_replace_types = Vec::with_capacity(data.fields.len());

    for field in data.fields.iter() {
        struct_fields.push(field);

        let parsed = parse_attr(field.into())?;
        if !parsed.changes.is_empty() {
            for (version, kind) in parsed.changes.into_iter() {
                // No need to worry about duplicate versions; they should be handled by
                // `parse_attr`.
                semver_compat.entry(field)
                    .or_insert(HashMap::<_, _>::default())
                    .insert(version.clone(), kind);
            }
            struct_replace_types.push(parsed.replace_type);
        }
    }

    let orig_struct_id = &ast.ident;
    let keyword = &data.struct_token;
    let generics = &ast.generics;
    let mut compat_structs = HashMap::new();
    for version in versions.iter() {
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

            field_ident.push({
                include.then_some(
                    ident.unwrap_or_else(|| f.ident.clone().expect(NAMED_FIELDS_ONLY))
                )
            });

            // Try to replace the field's type if it has a placeholder type attribute.
            let mut ty = None;
            if let Some(placeholder) = parsed_container.placeholder.as_ref()
                && let Some(mut attr_type) = struct_replace_types
                    .get(i)
                    .map(Clone::clone)
                    .flatten()
            {
                replace_compat_placeholder(version, Some(&f.ty), &mut attr_type, placeholder)?;
                ty = Some(attr_type);
            }
            let into_func = include.then(|| {
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
            });
            field_into.push(into_func);
            field_type.push(include.then(|| {
                ty.unwrap_or(f.ty.clone())
            }));
        }

        let mut field_serde = Vec::with_capacity(struct_fields.len());
        for &f in struct_fields.iter() {
            let attrs = parse_serde_field_attr(version, f.into())?;
            field_serde.push(attrs);
        }
        let container_serde = parse_serde_container_attr(version, ast)?;
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
            pub(crate) #keyword #struct_id #generics {
                #( #fields ),*
            }
        };

        let into_func_defn = [
            gen_into_wrapper_func(),
            gen_opt_vec_into_func(),
            gen_vec_into_func(),
        ];
        // Build out each field's into-conversion to avoid emitting stray ':' if a field is not
        // included.
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

        compat_structs.insert(version, quote! {
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
        });
    }

    let compat_enum_doc = format!("Holds every semver-compat struct generated for \
        {orig_struct_id}");
    let compat_enum_ident = format_ident!("__{orig_struct_id}_compat__");
    let compat_struct_defn = compat_structs.values();

    let variant_ident: Vec<_> = versions.iter().map(version_id).collect();
    let struct_ident: Vec<_> = versions.iter().map(|v| compat_id(v, orig_struct_id)).collect();

    let into_compat_doc = format!("Converts the source-defined `{orig_struct_id}` into its \
        generated `target` semver compatible struct.\n\
        \n\
        Returns `None` if the `target` semver is unsupported.");
    let into_compat_arm: Vec<_> = versions.iter()
        .enumerate()
        .map(|(i, version)| {
            let variant = variant_ident.get(i).expect("variant ident should exist");
            let (major, minor, patch) = (version.major, version.minor, version.patch);
            quote! {
                (#major, #minor, #patch) => Some(#compat_enum_ident::#variant(self.into()))
            }
        })
        .collect();

    Ok(quote! {
        // Emit all of the generated struct definitions.
        #( #compat_struct_defn )*

        // Define an enum to consolidate all of the generated structs into a single type so that we
        // can convert the source-defined original struct into its corresponding compat struct.
        #[doc = #compat_enum_doc]
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(untagged)]
        pub(crate) enum #compat_enum_ident {
            #( #variant_ident(#struct_ident) ),*
        }

        impl #orig_struct_id {
            #[doc = #into_compat_doc]
            pub(crate) fn into_compat(self, target: &semver::Version) -> Option<#compat_enum_ident>
            {
                match (target.major, target.minor, target.patch) {
                    #( #into_compat_arm ),*

                    (..) => None,
                }
            }
        }
    })
}
*/

// ------------------------------------------------------------------------------------------------

/// Generates compatible structs or enums for each supported (hardcoded) transmission rpc semver.
pub(crate) fn generate_compat_types(ast: &DeriveInput, data: StructOrEnum)
    -> Result<proc_macro2::TokenStream>
{
    let parsed_container_attr = parse_container_compat_attr(&ast.attrs)?;
    let versions = supported_versions();
    let container_fields = data.fields()?;
    let orig_container_id = &ast.ident;
    let keyword = &data.keyword()?;
    let generics = &ast.generics;
    let mut generated_compat_types = HashMap::new();
    for version in versions.iter() {
        let container_id = compat_id(version, orig_container_id);
        let from_arg_ident = format_ident!("orig");

        let mut fields = Vec::with_capacity(container_fields.len());
        let mut field_into = Vec::with_capacity(container_fields.len());
        let mut semver_compat = HashMap::new();
        for field in container_fields.iter() {
            let parsed = parse_attr(*field)?;
            if !parsed.changes.is_empty() {
                for (version, kind) in parsed.changes.into_iter() {
                    semver_compat.entry(field)
                        .or_insert(HashMap::<_, _>::default())
                        .insert(version.clone(), kind);
                }
            }

            let mut ident = field.require_ident()?; // The compat type's field/variant ident.
            // Process transmission semver changes.
            if let Some(changes) = semver_compat.get(field) {
                for (change_version, kind) in changes.iter() {
                    match kind {
                        Kind::Added => if version < change_version { continue; },
                        Kind::Removed => if version >= change_version { continue; },
                        Kind::Renamed(id) => ident = id,
                    }
                }
            }

            // Replace the placeholder with its corresponding semver type, if needed.
            let mut ty = None;
            if let Some(placeholder) = parsed_container_attr.placeholder.as_ref()
                && let Some(mut attr_type) = parsed.replace_type
            {
                replace_compat_placeholder(version, field.ty()?, &mut attr_type, placeholder)?;
                ty = Some(attr_type);
            }

            // Determine the conversion function to use to map the source-defined original type
            // into its corresponding compat type.
            let map_func = match ty.as_ref() {
                Some(ty) => {
                    let func = determine_which_into_func(ty)?;
                    quote! { #func }
                },
                None => {
                    let into_wrapper = ident_into_wrapper();
                    quote! { #into_wrapper }
                },
            };

            // The field/variant may have been renamed so we need to explicitly use the
            // source-defined original ident.
            let orig_ident = field.require_ident()?;
            let attrs = parse_serde_field_attr(version, *field)?;
            let ty = ty.as_ref().or(field.ty()?);
            match &data.inner {
                Data::Struct(_) => {
                    let Some(ty) = ty else {
                        return Err(Error::new(field.span(), "struct field must have a type"));
                    };
                    fields.push(quote_spanned! {field.span()=>
                        #( #attrs )*
                        #ident: #ty
                    });
                    field_into.push(quote_spanned! {field.span()=>
                        #ident: #map_func(#from_arg_ident.#orig_ident, Into::into)
                    });
                },

                Data::Enum(_) => {
                    let mut field_tokens = quote_spanned! {field.span()=>
                        #( #attrs )*
                        #ident
                    };
                    let mut src_variant = quote_spanned! {field.span()=>
                        #orig_container_id::#orig_ident
                    };
                    let mut dst_variant = quote_spanned! {field.span()=>
                        Self::#ident
                    };

                    if let Some(ty) = ty {
                        field_tokens = quote_spanned! {field.span()=>
                            #field_tokens(#ty)
                        };
                        src_variant = quote_spanned! {field.span()=>
                            #src_variant(x)
                        };
                        dst_variant = quote_spanned! {field.span()=>
                            #dst_variant(#map_func(x, Into::into))
                        };
                    }
                    fields.push(field_tokens);
                    field_into.push(quote_spanned! {field.span()=>
                        #src_variant => #dst_variant
                    });
                },

                // This shouldn't happen here (should be caught earlier).
                Data::Union(u) => {
                    return Err(Error::new(u.union_token.span(), "unexpected container type"));
                },
            }
        }

        let container_serde = parse_serde_container_attr(version, ast)?;
        let container_doc = format!("Transmission semver-{version} compatible \
            request serialization helper type");
        let compat_type = quote! {
            #[doc = #container_doc]
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[serde_with::skip_serializing_none]
            #[derive(serde::Serialize, Debug, Clone)]
            #(#container_serde)*
            pub(crate) #keyword #container_id #generics {
                #( #fields ),*
            }
        };

        let into_func_defn = [
            gen_into_wrapper_func(),
            gen_opt_vec_into_func(),
            gen_vec_into_func(),
        ];
        // Format the `From` implementation based on whether we're processing a struct or enum.
        let from_impl = match &data.inner {
            Data::Enum(_) => quote! {
                match #from_arg_ident {
                    #( #field_into ),*
                }
            },
            Data::Struct(_) => quote! {
                Self {
                    #( #field_into ),*
                }
            },

            // This shouldn't happen.
            kind => panic!("unexpected container type: {kind:?}"),
        };

        generated_compat_types.insert(version, quote! {
            #compat_type
            impl From<#orig_container_id> for #container_id {
                fn from(#from_arg_ident: #orig_container_id) -> Self {
                    // Define the helper field conversion functions which may or may not be
                    // used.
                    #( #[automatically_derived] #into_func_defn )*
                    #from_impl
                }
            }
        });
    }

    let compat_container_doc = format!("Holds every semver-compat type generated for \
        {orig_container_id}");
    let compat_container_ident = format_ident!("__{orig_container_id}_compat__");
    let compat_type_defn = generated_compat_types.values();

    let compat_container_variant_ident: Vec<_> = versions.iter().map(version_id).collect();
    let container_ident: Vec<_> = versions.iter()
        .map(|v| compat_id(v, orig_container_id))
        .collect();

    let into_compat_doc = format!("Converts the source-defined `{orig_container_id}` into its \
        generated `target` semver compatible type.\n\
        \n\
        Returns `None` if the `target` semver is unsupported.");
    let mut into_compat_arm: Vec<_> = versions.iter()
        .enumerate()
        .map(|(i, version)| {
            let variant = compat_container_variant_ident.get(i)
                .expect("variant ident should exist");
            let (major, minor, patch) = (version.major, version.minor, version.patch);
            quote! {
                (#major, #minor, #patch) => Some(#compat_container_ident::#variant(self.into()))
            }
        })
        .collect();
    into_compat_arm.push(quote! {
        (..) => None
    });

    Ok(quote! {
        // Emit all of the generated type definitions.
        #( #compat_type_defn )*

        // Define an enum to consolidate all of the generated structs or enums into a single type
        // so that we can convert the source-defined original struct/enum into its corresponding
        // compat struct/enum.
        #[doc = #compat_container_doc]
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(untagged)]
        pub(crate) enum #compat_container_ident {
            #( #compat_container_variant_ident(#container_ident) ),*
        }

        impl #orig_container_id {
            #[doc = #into_compat_doc]
            pub(crate) fn into_compat(self, target: &semver::Version)
                -> Option<#compat_container_ident>
            {
                match (target.major, target.minor, target.patch) {
                    #( #into_compat_arm ),*
                }
            }
        }
    })
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
