extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, Item, Type};

// TODO: cleanup!
/// TODO: doc
/// TODO: mention that this #[attr] MUST be the first/top decorator (MUST be defined before/above
/// TODO- all other eg. #[derive], #[cfg_attr], etc).
#[proc_macro_attribute]
pub fn generate_semver_600_compat(_attr: TokenStream, item: TokenStream) -> TokenStream {
    /*
    println!("----- attr: \"{_attr}\""); // TODO: DELETE
    println!("----- item: \"{item}\""); // TODO: DELETE
    */

    // REF: https://compilenrun.com/docs/language/rust/rust-advanced-features/rust-derive-macros/
    // REF: https://docs.rs/quote/latest/quote/macro.quote.html#indexing-into-a-tuple-struct

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
}
