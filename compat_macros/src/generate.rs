extern crate proc_macro;

use quote::{format_ident, quote, quote_spanned};
use syn::{
    DataStruct, DeriveInput, Fields, Ident, Path, Type,
    spanned::Spanned,
};

use crate::symbols::{COMPAT_ATTR, COMPAT_PREFIX, NAME, MAP, TYPE};

const NAMED_FIELDS_ONLY: &'static str = "GenerateCompat only supports structs with named fields.";

/// Generates a semver-6.0.0 compatible struct for serialization purposes.
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> proc_macro2::TokenStream
{
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
        let mut attr_name: Option<Ident> = None;
        let mut attr_type: Option<Type> = None;

        match field.attrs
            .iter()
            .find(|a| a.path() == COMPAT_ATTR)
        {
            Some(attr) => {
                if let Err(err) = attr.parse_nested_meta(|meta| {
                    // #[compat(name = foo)]
                    if meta.path == NAME {
                        attr_name = Some(meta.value()?.parse()?);

                    // #[compat(type = Option<i32>)]
                    } else if meta.path == TYPE {
                        attr_type = Some(meta.value()?.parse()?);

                    // #[compat(map = Option::map)]
                    } else if meta.path == MAP {
                        conv.push(meta.value()?.parse().ok());
                    }
                    Ok(())
                }) {
                    panic!("Failed to parse #[{COMPAT_ATTR}] args: {err}");
                }
            },
            None => {},
        }

        compat_ident.push(attr_name
            .or_else(|| field.ident.clone())
            .expect(NAMED_FIELDS_ONLY)); // Only handle structs with named fields.
        type_defn.push(attr_type
            .unwrap_or_else(|| field.ty.clone()));
    }

    // Generate `into_compat` conversions.
    let field_conv = {
        let converted_field: Vec<_> = data.fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let ident = field.ident.as_ref().expect(NAMED_FIELDS_ONLY);
                let converted = match conv.get(i) {
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
    let compat_struct_id = format_ident!("{COMPAT_PREFIX}{}", orig_struct_id);
    let generics = &ast.generics;
    let semi_token = data.semi_token;
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
