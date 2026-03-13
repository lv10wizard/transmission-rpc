extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Fields, Ident, Item, Type,
    parse_macro_input,
};

// TODO: cleanup!
/// TODO: doc
/// TODO: mention that this #[attr] MUST be the first/top decorator (MUST be defined before/above
/// TODO- all other eg. #[derive], #[cfg_attr], etc).
#[proc_macro_derive(GenerateCompat, attributes(compat_name, compat_type))]
pub fn generate_semver_600_compat(input: TokenStream) -> TokenStream {
    /*
    println!("----- input: \"{input}\""); // TODO: DELETE
    */

    // REF: https://compilenrun.com/docs/language/rust/rust-advanced-features/rust-derive-macros/
    // REF: https://docs.rs/quote/latest/quote/macro.quote.html#indexing-into-a-tuple-struct
    // REF: https://stackoverflow.com/a/42526546

    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(data) => generate_compat_struct(&input, data),
        Data::Enum(data) => generate_compat_enum(&input, data),
        Data::Union(data) => generate_compat_union(&input, data),
    }
    .into()

    /* ============ OLD
    let input = {
        let item = item.clone();
        parse_macro_input!(item as Item)
    };
    let item = proc_macro2::TokenStream::from(item);
    match &input {
        Item::Struct(input_struct) => {
            let token = &input_struct.struct_token; // "struct"
            let name = &input_struct.ident;
            let generics = &input_struct.generics;
            let fields = &input_struct.fields;

            let compat_name = format_ident!("__semver_600_compat_{}", name);
            // Extract the field names so that we can generate the `into_compat` method.
            let field_name = fields
                .iter()
                .map(|f| &f.ident)
                .collect::<Vec<&Option<Ident>>>();
            // Extract each fields' type because we specifically _DO NOT_ want attributes like
            // #[serde(rename = "...")].
            let field_type = fields
                .iter()
                .map(|f| &f.ty)
                .collect::<Vec<&Type>>();

            let out = quote! {
                // Define the original (legacy) struct as-is.
                #item

                // Define the semver-6.0.0 compatible type.
                // We don't bother with any meta macros (eg. derive, cfg_attr, etc) that exist on
                // the legacy (original) struct because this generated compat type should be used
                // only for serialization.
                #[allow(non_camel_case_types)]
                #[serde_with::skip_serializing_none] // Ordering might matter here.
                #[derive(serde::Serialize, Debug, Clone)]
                #[serde(rename_all = "snake_case")]
                pub(crate) #token #compat_name #generics {
                    // Expand out each `field: type`.
                    #(#field_name: #field_type,)*
                }

                // Generate a helper method on the legacy (original) struct to convert into the
                // semver-6.0.0 compatible struct.
                impl #name {
                    pub fn into_compat(self) -> #compat_name {
                        #compat_name {
                            // Move each `self.field` into the newly constructed compat instance's
                            // corresponding field.
                            //
                            // ie:
                            // struct Foo { a: i32, b: i32 }
                            // struct compat_Foo { /* same as Foo */ }
                            //
                            // // self = Foo { ... }
                            // compat_foo { a: self.a, b: self.b }
                            #(#field_name: self.#field_name,)*
                        }
                    }
                }

                // Generate helper `from` implementations for the legacy struct -> compat struct.
                impl From<#name> for #compat_name {
                    fn from(value: #name) -> Self {
                        value.into_compat()
                    }
                }
            };
            /*
            println!("===== out:\n{out}\n\n"); // TODO: DELETE
            */
            out.into()
        },

        /* TODO
        Item::Enum(input_enum) => {
        },
        */

        _ => panic!("generate_semver_600_compat only supports structs and enums."),
    }
    // =========== OLD */
}

const COMPAT_PREFIX: &'static str = "__semver_600_compat_";
const COMPAT_NAME: &'static str = "compat_name";
const COMPAT_TYPE: &'static str = "compat_type";

/// Generates a semver-6.0.0 compatible struct for serialization purposes.
fn generate_compat_struct(ast: &DeriveInput, data: &DataStruct) -> proc_macro2::TokenStream {
    // Legacy struct field names mapped to its corresponding compatible struct's field name.
    // In most cases, these will be the same but when a field is flagged with #[compat_name(...)],
    // the generated (semver-6.0.0 compatible) struct's field name will differ. We need this
    // mapping to generate the `into_compat` conversion helper method.
    let mut orig_field = Vec::with_capacity(data.fields.len());
    let mut compat_field = Vec::with_capacity(data.fields.len());

    for f in data.fields.iter() {
        if let Some(id) = f.ident.as_ref() {
            let fname = match f.attrs.iter().find(|a| a.path().is_ident(COMPAT_NAME)) {
                Some(attr) => match attr.parse_args::<Ident>() {
                    Ok(compat_id) => compat_id,
                    Err(err) => panic!("Invalid {} name: {}", COMPAT_NAME, err),
                },
                None => id.clone(),
            };
            orig_field.push(id);
            compat_field.push(fname);
        }
    }

    let field_type: Vec<_> = data.fields
        .iter()
        .map(|f| {
            match f.attrs
                .iter()
                .find(|a| a.path().is_ident(COMPAT_TYPE))
            {
                Some(attr) => match attr.parse_args::<Type>() {
                    Ok(ty) => ty,
                    Err(err) => panic!("Invalid {} type: {}", COMPAT_TYPE, err),
                },
                None => f.ty.clone(),
            }
        })
        .collect();
    let orig_ident = &ast.ident;
    let compat_ident = format_ident!("{COMPAT_PREFIX}{}", orig_ident);
    let generics = &ast.generics;

    // Generate field definitions and `into_compat` conversions based on what kind of struct we're
    // processing.
    let (field_defn, field_conv) = match &data.fields {
        Fields::Named(_) => {(
            quote! {
                { #(#compat_field: #field_type),* }
            },
            quote! {
                { #(#compat_field: self.#orig_field),* }
            },
        )},
        Fields::Unnamed(_) => {(
            quote! {
                ( #(#field_type),* )
            },
            quote! {
                ( #(self.#orig_field),* )
            },
        )},
        Fields::Unit => (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()),
    };

    quote! {
        /// Semver-6.0.0 compatible serialization helper for `#orig_ident`.
        #[allow(non_camel_case_types)]
        #[serde_with::skip_serializing_none] // I think this has appear before derive(Serialize).
        #[derive(serde::Serialize, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct #compat_ident #generics #field_defn

        impl #orig_ident {
            /// Converts `#orig_ident` into its semver-6.0.0 compatible serialization helper type.
            pub fn into_compat(self) -> #compat_ident {
                // Move each field in `self` to its corresponding serialization helper type's
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

/// Generates a semver-6.0.0 compatible enum for serialization purposes.
fn generate_compat_enum(ast: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    quote! { }
}

/// Generates a semver-6.0.0 compatible union for serialization purposes.
fn generate_compat_union(ast: &DeriveInput, data: &DataUnion) -> proc_macro2::TokenStream {
    quote! { }
}
