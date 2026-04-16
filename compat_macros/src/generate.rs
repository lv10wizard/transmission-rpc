extern crate proc_macro;

use std::collections::HashMap;

use quote::{format_ident, quote, quote_spanned};
use syn::{
    Data, DeriveInput, Error, Result,
    spanned::Spanned as _,
};

use crate::{
    SUPPORTED_VERSIONS,
    compat::{FieldOrVar, Kind, parse_attr},
    serde::{parse_serde_container_attr, parse_serde_field_attr},
    symbols::{compat_id, version_id},
    r#type::{
         determine_which_into_func, gen_into_wrapper_func, gen_opt_vec_into_func,
         gen_vec_into_func, ident_into_wrapper, replace_with_compat_type,
    },
};

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

// ------------------------------------------------------------------------------------------------

/// Generates compatible structs or enums for each supported (hardcoded) transmission rpc semver.
pub(crate) fn generate_compat_types(ast: &DeriveInput, data: StructOrEnum)
    -> Result<proc_macro2::TokenStream>
{
    let container_fields = data.fields()?;
    let orig_container_id = &ast.ident;
    let keyword = &data.keyword()?;
    let generics = &ast.generics;
    let mut generated_compat_types = HashMap::new();
    for version in SUPPORTED_VERSIONS.iter() {
        let container_id = compat_id(version, orig_container_id);
        let from_arg_ident = format_ident!("orig");

        let mut fields = Vec::with_capacity(container_fields.len());
        let mut field_into = Vec::with_capacity(container_fields.len());
        'fields: for field in container_fields.iter() {
            let mut semver_compat = HashMap::new();
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
                        Kind::Added => if version < change_version { continue 'fields; },
                        Kind::Removed => if version >= change_version { continue 'fields; },
                        Kind::Renamed(id) => if version >= change_version {
                            ident = id;
                        },
                    }
                }
            }

            // Replace the placeholder with its corresponding semver type, if needed.
            let mut ty = field.ty()?.map(Clone::clone);
            if parsed.replace_type {
                replace_with_compat_type(version, ty.as_mut())?;
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

    let compat_container_variant_ident: Vec<_> = SUPPORTED_VERSIONS.iter()
        .map(version_id)
        .collect();
    let container_ident: Vec<_> = SUPPORTED_VERSIONS.iter()
        .map(|v| compat_id(v, orig_container_id))
        .collect();

    let into_compat_doc = format!("Converts the source-defined `{orig_container_id}` into its \
        generated `target` semver compatible type.\n\
        \n\
        Returns `None` if the `target` semver is unsupported.");
    let mut into_compat_arm: Vec<_> = SUPPORTED_VERSIONS.iter()
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
