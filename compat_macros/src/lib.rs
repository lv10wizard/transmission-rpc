//! Defines helper `#[derive(...)`] macros for pre- and post- Transmission 4.1.0
//! (`rpc-version-semver` 6.0.0, `rpc-version`: 18) request serialization compatibility.

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

use generate::generate_compat_struct;

mod generate;
mod symbols;

/// Generates a helper struct for request serialization compatibility with Transmission 4.1.0
/// (`rpc-version-semver` 6.0.0, `rpc-version`: 18) and later.
///
/// Struct field names and types can be individually (and separately) overridden with the
/// derive-macro helper attributes:
///
/// * `#[compat_name(FIELD_NAME_OVERRIDE)]`: Overrides the decorated field name with
/// `FIELD_NAME_OVERRIDE`.
///
/// * `#[compat_type(FIELD_TYPE_OVERRIDE)]`: Overrides the decorated field type with
/// `FIELD_TYPE_OVERRIDE`. The target override type (`FIELD_TYPE_OVERRIDE`) must implement [`From`]
/// (or [`Into`]) for conversion from the original type into the target type.
///
/// ### Example
///
/// ```rust
/// use compat_macros::GenerateCompat;
/// use serde::Serialize;
///
/// #[derive(GenerateCompat, Serialize, Debug)]
/// #[serde(rename_all = "camelCase")]
/// struct Foo {
///     #[compat_name(xyz)]
///     foo_bar: Option<i32>, // Becomes `xyz: Option<i32>` in the compat struct.
///     #[compat_name(Option<i64>)]
///     my_var: Option<i32>, // Becomes `my_var: Option<i64>` in the compat struct.
///
///     #[compat_name(abc_def)]
///     #[compat_type(Option<u16>)]
///     peer_limit: Option<u8>, // Becomes: `abc_def: Option<u16>` in the compat struct.
///
///     lorem_ipsum: Option<String>, // Remains: `lorem_ipsum: Option<String>`.
/// }
/// ```
#[proc_macro_derive(GenerateCompat, attributes(compat_name, compat_type))]
pub fn generate_semver_600_compat(input: TokenStream) -> TokenStream {
    // REF: https://compilenrun.com/docs/language/rust/rust-advanced-features/rust-derive-macros/
    // REF: https://docs.rs/quote/latest/quote/macro.quote.html#indexing-into-a-tuple-struct
    // REF: https://stackoverflow.com/a/42526546

    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(data) => generate_compat_struct(&input, data),
        _ => panic!("GenerateCompat supports only structs."),
    }
    .into()
}

#[proc_macro_derive(UseCompat)]
pub fn use_semver_600_compat(input: TokenStream) -> TokenStream {
    input
}
