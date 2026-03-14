extern crate proc_macro;

use quote::{format_ident, quote, quote_spanned};
use syn::{
    Attribute, DataStruct, DeriveInput, Expr, ExprAssign, ExprPath, Fields, Ident, Path, Meta,
    Token, Type,
    meta::ParseNestedMeta,
    parse::{Parse, ParseStream},
    punctuated::Punctuated
};

use crate::symbols::{COMPAT_PREFIX, COMPAT_NAME, COMPAT_TYPE, FROM, TYPE};

/// Generates a semver-6.0.0 compatible struct for serialization purposes.
pub(crate) fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct)
    -> proc_macro2::TokenStream
{
    // Legacy struct field names mapped to its corresponding compatible struct's field name.
    // In most cases, these will be the same but when a field is flagged with #[compat_name(...)],
    // the generated (semver-6.0.0 compatible) struct's field name will differ. We need this
    // mapping to generate the `into_compat` conversion helper method.
    let mut orig_field = Vec::with_capacity(data.fields.len());
    let mut compat_field = Vec::with_capacity(data.fields.len());

    for f in data.fields.iter() {
        if let Some(id) = f.ident.as_ref() {
            let fname = match f.attrs.iter().find(|a| a.path() == COMPAT_NAME) {
                Some(attr) => match attr.parse_args::<Ident>() {
                    Ok(compat_id) => compat_id,
                    Err(err) => panic!("Invalid \"{}\" name: {}", COMPAT_NAME, err),
                },
                None => id.clone(),
            };
            // Push the original (legacy) struct's field name so that we can look it up when
            // generating the `into_compat` conversion helper method.
            orig_field.push(id.clone());
            // Push the compat struct's field name (which in most cases will be the same as the
            // original). This will only differ for fields tagged with #[compat_name(...)].
            compat_field.push(fname);
        }
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
    let orig_ident = &ast.ident;
    let compat_ident = format_ident!("{COMPAT_PREFIX}{}", orig_ident);
    let generics = &ast.generics;
    let semi_token = data.semi_token;

    // Generate field definitions and `into_compat` conversions based on what kind of struct we're
    // processing.
    let (field_defn, field_conv) = {
        let members = data.fields.members();
        match &data.fields {
            Fields::Named(_) => {(
                // Struct definition.
                quote! {
                    { #(#compat_field: #field_type),* }
                },

                // Legacy -> compat conversion.
                {
                    let converted_field: Vec<_> = compat_field
                        .into_iter()
                        .zip(orig_field.into_iter())
                        .enumerate()
                        .map(|(i, (compat, orig))| {
                            let conv = match conv.get(i) {
                                // eg. `Option::map(self.x, Into::into)`
                                // NOTE: Probably won't work for non- `Option::map` methods.
                                Some(conv) => quote! {
                                    #conv(self.#orig, Into::into)
                                },
                                // eg. `self.x.into()`
                                None => quote! {
                                    self.#orig.into()
                                },
                            };

                            quote_spanned! {compat.span()=>
                                #compat: #conv
                            }
                        })
                        .collect();
                    quote! {
                        { #( #converted_field, )* }
                    }
                },
            )},
            Fields::Unnamed(_) => {(
                // Struct definition.
                quote! {
                    ( #(#field_type),* )
                },
                // Legacy -> compat conversion.
                // TODO: reuse `converted_field`
                quote! {
                    (
                        #(self.#members.into()),*
                    )
                },
            )},
            Fields::Unit => (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
        }
    };

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
                // Move each field in `self` to its serialization helper type's corresponding
                // field.
                //
                // eg.
                // Orig   { x: i32, y: i32 }
                // Compat { a: i32, b: i32 }
                //
                // // return Compat { a: self.x, b: self.y }
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
