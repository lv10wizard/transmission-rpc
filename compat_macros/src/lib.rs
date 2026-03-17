//! Defines helper `#[derive(...)`] macros for pre- and post- Transmission 4.1.0
//! (`rpc-version-semver` 6.0.0, `rpc-version`: 18) request serialization compatibility.

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

use generate::{generate_compat_enum, generate_compat_struct};

mod generate;
mod symbols;

/// Generates a helper struct for request serialization compatibility with Transmission 4.1.0
/// (`rpc-version-semver` 6.0.0, `rpc-version`: 18) and later.
///
/// Struct field names and types can be individually (and separately) overridden with the
/// derive-macro helper attribute, `#[compat]`, which recognizes the following arguments:
///
/// * `name` (eg. `#[compat(name = foo)]`): Replaces the field's name.
///
/// * `type` (eg. `#[compat(type = Option<u64>)]`: Replaces the field's type.
///
///     > **NOTE:** The replacement type **MUST** be [`Into`]-compatible with the original field's
///     type (eg. by implementing [`From`] on the original type into the replacement type).
///
/// * `map` (eg. `#[compat(map = Option::map)]`: Calls the given function with the original
/// struct's field to convert it into the generated compat struct's field, eg. `Option::map(self.x,
/// Into::into)`.
///
///     > **NOTE:** At time of writing, this probably only works for [`Option::map`].
///
/// ### Example
///
/// Defining a `GenerateCompat`-derived struct like:
///
/// ```rust
/// use compat_macros::GenerateCompat;
/// use serde::Serialize;
///
/// #[derive(GenerateCompat, Serialize, Debug)]
/// #[serde(rename_all = "camelCase")]
/// struct Foo {
///     #[compat(name = xyz)]
///     foo_bar: Option<i32>, // Becomes `xyz: Option<i32>` in the compat struct.
///     #[compat(type = i64)]
///     my_var: i32, // Becomes `my_var: i64` in the compat struct.
///
///     #[compat(name = abc_def, type = Option<u16>, map = Option::map)]
///     peer_limit: Option<u8>, // Becomes: `abc_def: Option<u16>` in the compat struct.
///
///     lorem_ipsum: String, // Remains: `lorem_ipsum: Option<String>`.
/// }
/// ```
///
/// Will result in a generated compat struct looking something like:
///
/// ```rust
/// #[serde_with::skip_serializing_none]
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct FooCompat {
///     xyz: Option<i32>, // Was `foo_bar: Option<i32>` in the original struct.
///     my_var: i64, // Was `my_var: i32` in the original struct.
///
///     abc_def: Option<u16>, // Was: `peer_limit: Option<u8>` in the original struct.
///
///     lorem_ipsum: String, // Unchanged.
/// }
/// ```
#[proc_macro_derive(GenerateCompat, attributes(compat))]
pub fn generate_semver_600_compat(input: TokenStream) -> TokenStream {
    // REF: https://compilenrun.com/docs/language/rust/rust-advanced-features/rust-derive-macros/
    // REF: https://docs.rs/quote/latest/quote/macro.quote.html#indexing-into-a-tuple-struct
    // REF: https://stackoverflow.com/a/42526546

    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(data) => generate_compat_struct(&input, data),
        Data::Enum(data) => generate_compat_enum(&input, data),
        _ => panic!("GenerateCompat supports only structs."),
    }
    .into()
}

#[proc_macro_derive(UseCompat)]
pub fn use_semver_600_compat(input: TokenStream) -> TokenStream {
    input
}
