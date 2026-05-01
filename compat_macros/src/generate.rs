extern crate proc_macro;

use std::collections::HashMap;

use quote::{format_ident, quote, quote_spanned};
use syn::{
    Data, DeriveInput, Error, Ident, Result,
    spanned::Spanned as _,
};

use crate::{
    SUPPORTED_VERSIONS,
    compat::{FieldOrVar, Kind, parse_attr},
    serde::{parse_serde_attr, parse_serde_container_attr, parse_serde_field_attr},
    serialize::SerializeImpl as _,
    symbols::{compat_id, version_id},
    r#type::{determine_which_into_func, replace_with_compat_type},
};

pub(crate) struct StructOrEnum<'a> {
    ident: &'a Ident,
    inner: &'a Data,
}

impl<'a> StructOrEnum<'a> {
    pub(crate) fn new(ident: &'a Ident, data: &'a Data) -> Self {
        let inner = match data {
            Data::Enum(_) => data,
            Data::Struct(_) => data,
            Data::Union(_) => panic!("unsupported type: union"),
        };
        Self { ident, inner }
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

    fn serialize_impl(&self) -> Result<proc_macro2::TokenStream> {
        match &self.inner {
            Data::Enum(e) => e.to_serialize_impl(&self.ident),
            Data::Struct(s) => s.to_serialize_impl(&self.ident),
            Data::Union(u) => Err(Error::new(u.union_token.span(), "unsupported type")),
        }
    }
}

/// Generates tokens to serialize missing enum variants for the compat enum, `container_id`.
fn gen_serialize_missing(container_id: &Ident, none_variant: &Ident) -> proc_macro2::TokenStream {
    quote! {
        // Override the blanket [`internal_trait::VariantMissing`] implementation.
        // REF: https://stackoverflow.com/a/71721454
        #[automatically_derived]
        #[allow(dead_code, non_camel_case_types)]
        impl #container_id {
            fn __variant_missing(&self) -> bool {
                match self {
                    Self::#none_variant => true,
                    _ => false,
                }
            }

            fn __variant_exists(&self) -> bool {
                !self.__variant_missing()
            }
        }
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

    let mut has_replaced_type = false;
    let mut compat_data = HashMap::with_capacity(container_fields.len()); 
    for fv in container_fields.iter() {
        let serde = parse_serde_attr(fv.attributes());
        let compat = parse_attr(*fv)?;
        has_replaced_type = has_replaced_type || compat.replace_type;

        compat_data.insert(fv, (compat, serde));
    }

    let compat_id = format_ident!("__{orig_container_id}_Compat");
    let orig_id = orig_container_id; // TODO: use generated type if `has_replaced_type`

    Ok(quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        pub(crate) struct #compat_id {
            version: semver::Version,
            base: #orig_id,
        }

        // TODO: orig_container_id -> compat_id conversion
        // TODO: [if needed] define helper container if `has_replaced_type`
        // TODO: [if needed] orig_container_id -> helper `has_replaced_type` container conversion

        impl serde::Serialize for #compat_id {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                // TODO
            }
        }
    })
}












// TODO: OLD

// ------------------------------------------------------------------------------------------------

/// Generates compatible structs or enums for each supported (hardcoded) transmission rpc semver.
pub(crate) fn generate_compat_types__OLD(ast: &DeriveInput, data: StructOrEnum)
    -> Result<proc_macro2::TokenStream>
{
    let container_fields = data.fields()?;
    let orig_container_id = &ast.ident;
    let keyword = &data.keyword()?;
    let generics = &ast.generics;
    let mut generated_compat_types = HashMap::new();

    let mut parsed = Vec::with_capacity(container_fields.len());
    for field in container_fields.iter() {
        parsed.push(parse_attr(*field)?);
    }

    for version in SUPPORTED_VERSIONS.iter() {
        let container_id = compat_id(version, orig_container_id);
        let from_arg_ident = format_ident!("orig");
        let none_variant = format_ident!("__None_{container_id}");

        let mut fields = Vec::with_capacity(container_fields.len());
        let mut field_into = Vec::with_capacity(container_fields.len());
        'fields: for (i, field) in container_fields.iter().enumerate() {
            let mut include = true;
            let mut ident = field.require_ident()?; // The compat type's field/variant ident.
            // Process transmission semver changes.
            let parsed = &parsed[i];
            for (change_version, kind) in parsed.changes.iter() {
                match kind {
                    Kind::Added => if version < change_version {
                        match &data.inner {
                            // We can just skip generating the field if it doesn't exist in
                            // this version.
                            Data::Struct(_) => continue 'fields,
                            // We need to still process the variant to convert the
                            // source-defined variant into a non-serialized `None` variant.
                            Data::Enum(_) => include = false,
                            _ => return Err(
                                Error::new(keyword.span(), "unexpected container type")
                            ),
                        }
                    },
                    Kind::Removed => if version >= change_version {
                        match &data.inner {
                            Data::Struct(_) => continue 'fields,
                            Data::Enum(_) => include = false,
                            _ => return Err(
                                Error::new(keyword.span(), "unexpected container type")
                            ),
                        }
                    },
                    Kind::Renamed(id) => if version >= change_version {
                        ident = id;
                    },
                }
            }

            // Replace the source-defined type with its corresponding semver type, if needed.
            let mut ty = field.ty()?.map(Clone::clone);
            if parsed.replace_type {
                replace_with_compat_type(version, ty.as_mut())?;
            }

            // Determine the conversion function to use to map the source-defined original type
            // into its corresponding compat type.
            let (map_func_ident, map_func_defn) = determine_which_into_func(
                field.ty()?,
                ty.as_ref(),
            )?;

            // The field/variant may have been renamed so we need to explicitly use the
            // source-defined original ident.
            let orig_ident = field.require_ident()?;
            let attrs = parse_serde_field_attr(version, *field, &parsed)?;
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
                        #ident: {
                            #map_func_defn

                            #map_func_ident(#from_arg_ident.#orig_ident)
                        }
                    });
                },

                Data::Enum(_) => {
                    let ident = match include {
                        true => ident.clone(),

                        // The variant does not exist in this version (it was either removed or has
                        // yet to be added).
                        false => none_variant.clone(),
                    };
                    if include {
                        let mut field_tokens = quote_spanned! {field.span()=>
                            #( #attrs )*
                            #ident
                        };
                        if let Some(ty) = ty {
                            field_tokens = quote_spanned! {field.span()=>
                                #field_tokens(#ty)
                            };
                        }
                        fields.push(field_tokens);
                    }

                    let mut src_variant = quote_spanned! {field.span()=>
                        #orig_container_id::#orig_ident
                    };
                    let mut dst_variant = quote_spanned! {field.span()=>
                        Self::#ident
                    };

                    if ty.is_some() {
                        src_variant = quote_spanned! {field.span()=>
                            #src_variant(x)
                        };
                        dst_variant = quote_spanned! {field.span()=>
                            {
                                #map_func_defn

                                #dst_variant(#map_func_ident(x))
                            }
                        };
                    }
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

        // Include the `None` variant in case any source-defined variants do not exist in this
        // version.
        //
        // We assume all enums are wrapped in either an `Option` or a `Vec`, meaning that this
        // `None` variant should not ever be directly serialized. So we can tag it with
        // #[serde(skip_serializing)] to prevent ever sending it to a transmission rpc server.
        if let Data::Enum(_) = &data.inner {
            let tokens = quote_spanned! {keyword.span()=>
                #[allow(dead_code)]
                #[serde(skip_serializing)]
                #none_variant
            };
            fields.push(tokens);
        }

        let container_serde = parse_serde_container_attr(version, ast)?;
        let container_doc = format!("Transmission semver-{version} compatible \
            request serialization helper type");
        let compat_type = quote! {
            #[doc = #container_doc]
            #[automatically_derived]
            #[allow(non_camel_case_types, unused)]
            #[derive(serde::Serialize, Debug, Clone)]
            #(#container_serde)*
            pub(crate) #keyword #container_id #generics {
                #( #fields ),*
            }
        };

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
            _ => return Err(Error::new(keyword.span(), "unexpected container type")),
        };

        let serialize_missing = match &data.inner {
            Data::Enum(_) => {
                let tokens = gen_serialize_missing(&container_id, &none_variant);
                Some(quote_spanned! {keyword.span()=> #tokens })
            },

            _ => None,
        };

        generated_compat_types.insert(version, quote! {
            #compat_type
            #serialize_missing
            impl From<#orig_container_id> for #container_id {
                fn from(#from_arg_ident: #orig_container_id) -> Self {
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
        // Bring the blanket implementation into scope so that the corresponding filtering methods
        // are implemented.
        #[allow(unused_imports)]
        use internal_trait::VariantMissing as _;

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
