extern crate proc_macro;

use quote::{format_ident, quote, quote_spanned};
use syn::{
    DataStruct, DeriveInput, Fields, Ident, Path, Type,
    spanned::Spanned,
};

use crate::symbols::{COMPAT_PREFIX, COMPAT_NAME, COMPAT_TYPE, FROM, TYPE};

const NAMED_FIELDS_ONLY: &'static str = "GenerateCompat only supports structs with named fields.";

/// Generates a semver-6.0.0 compatible struct for serialization purposes.
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> proc_macro2::TokenStream
{
    // Stores the compat struct's field names where the index into the Vec corresponds to the
    // original (legacy) struct's field's name.
    //
    // These will only differ when a field is flagged with #[compat_name(...)].
    let mut compat_field = Vec::with_capacity(data.fields.len());

    for f in data.fields.iter() {
        let id = f.ident.as_ref().expect(NAMED_FIELDS_ONLY);
        // Push the compat struct's field name (which in most cases will be the same as the
        // original). This will only differ for fields tagged with #[compat_name(...)].
        compat_field.push({
            match f.attrs.iter().find(|a| a.path() == COMPAT_NAME) {
                Some(attr) => match attr.parse_args::<Ident>() {
                    Ok(compat_id) => compat_id,
                    Err(err) => panic!("Invalid \"{}\" name: {}", COMPAT_NAME, err),
                },
                None => id.clone(),
            }
        });
    }

    // Determine the target type for each of the legacy struct's fields.
    let mut conv: Vec<Option<Path>> = Vec::with_capacity(data.fields.len());
    let field_type: Vec<_> = data.fields
        .iter()
        .map(|f| {
            match f.attrs
                .iter()
                .find(|a| a.path() == COMPAT_TYPE)
            {
                Some(attr) => {
                    let mut compat_type: Option<Type> = None;

                    if let Err(err) = attr.parse_nested_meta(|meta| {
                        // #[compat_type(from = Option::map)]
                        if meta.path == FROM {
                            conv.push(meta.value()?.parse().ok());

                        // #[compat_type(type = Option<i32>)]
                        } else if meta.path == TYPE {
                            compat_type = Some(meta.value()?.parse::<Type>()?);
                        }
                        Ok(())
                    }) {
                        panic!("Failed to parse \"{COMPAT_TYPE}\" args: {err}");
                    }

                    match compat_type {
                        Some(ty) => ty,
                        None => panic!("Invalid \"{COMPAT_TYPE}\": no type specified"),
                    }
                },
                None => f.ty.clone(),
            }
        })
        .collect();

    // Generate compat struct field definitions.
    let field_defn = match &data.fields {
        Fields::Named(_) => {
            quote! {
                { #(#compat_field: #field_type),* }
            }
        },

        _ => panic!("{NAMED_FIELDS_ONLY}"),
    };

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
                let compat = compat_field.get(i);
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

    let orig_ident = &ast.ident;
    let compat_ident = format_ident!("{COMPAT_PREFIX}{}", orig_ident);
    let generics = &ast.generics;
    let semi_token = data.semi_token;
    quote! {
        /// Semver-6.0.0 compatible serialization helper.
        #[allow(non_camel_case_types)]
        #[serde_with::skip_serializing_none] // I think this has appear before derive(Serialize).
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct #compat_ident #generics #field_defn #semi_token

        impl #orig_ident {
            /// Converts the legacy struct into its semver-6.0.0 compatible serialization helper
            /// type.
            pub fn into_compat(self) -> #compat_ident {
                #compat_ident #field_conv
            }
        }

        // Helper `From` implementation for the legacy struct -> semver-6.0.0 compatible struct.
        impl From<#orig_ident> for #compat_ident {
            fn from(value: #orig_ident) -> Self {
                value.into_compat()
            }
        }
    }
}
