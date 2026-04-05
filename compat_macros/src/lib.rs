//! Defines helper `#[derive(...)]` macros for pre- and post- Transmission 4.1.0
//! (`rpc-version-semver` 6.0.0, `rpc-version`: 18) request serialization compatibility.

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

use generate::{generate_compat_enum, generate_compat_struct};

mod compat;
mod generate;
mod placeholder;
mod symbols;

/// Generates a helper struct or enum for request serialization compatibility with Transmission
/// 4.1.0 (`rpc-version-semver` 6.0.0, `rpc-version`: 18) and later.
///
/// The original type can be converted into its compat type with the generated `into_compat`
/// method, eg. `x.into_compat()`.
///
/// This derive macro supports:
///
/// * `struct`s
/// * `enum`s with unit variants, eg. `enum Foo { A, B, C }`
/// * `enum`s with a single, unnamed field, eg. `enum Bar { A(i16), B(i32), C(i64) }`
///
/// # Container attributes
///
/// ### `#[compat(placeholder = NAME)]`
///
/// Defines an arbitrary placeholder name to replace with the corresponding `GenerateCompat`
/// derived type when `NAME` is encountered in a [`type = ...`] field/variant attribute.
///
/// This exists so that callers do not need to reference compat types' names directly (since these
/// type names are mangled with an ugly prefix: `__semver_600_compat_`).
///
/// eg.
/// ```rust
/// #[derive(GenerateCompat)]
/// #[compat(placeholder = MyPlaceholderName)]
/// struct Foo {
///     #[compat(type = Option<MyPlaceholderName>, map = Option::map)]
///     bar: Option<Bar>,
/// }
///
/// #[derive(GenerateCompat)]
/// struct Bar {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// Would generate a compat-version of `Foo` like:
/// ```rust
/// struct __semver_600_compat_Foo {
///     bar: Option<__semver_600_compat_Bar>,
/// }
/// ```
///
/// # Field/Variant attributes
///
/// ### `#[compat(name = NAME)]`
///
/// Overrides the field or variant's name with `NAME` in the generated compat type. This can be
/// useful to serialize a given field or variant into a completely different name.
///
/// ### `#[compat(type = TYPE)]`
///
/// Overrides the field or variant's type with `TYPE` in the generated compat type. This is most
/// useful when coupled with the container-level `#[compat(placeholder = ...)]` attribute to
/// replace a field or variant's type into its corresponding compat type.
///
/// ### `#[compat(map = FUNC)]`
///
/// Passes a type conversion mapping function, eg. [`Option::map`], to convert the field or variant
/// into its compat type's corresponding field or variant. This only has any effect with
/// `#[compat(type = ...)]`.
///
/// If `type` is given without `map`, conversion is performed field-by-field (or
/// variant-by-variant) with a simple [`Into`] call (ie. `x.into()`).
///
/// The specified `FUNC` is called like: `FUNC(x, Into::into)`, ie. its signature should be:
/// ```rust
/// // Generics defined here for correctness.
/// // Actual `FUNC`s do not need to be generic.
/// fn FUNC<F, T, U>(x: T, func: F) -> U
/// where
///     F: Fn(T) -> U,
///     T: Into<U>;
/// ```
///
/// # Examples
///
/// ### Struct
/// ```rust
/// use compat_macros::GenerateCompat;
/// use serde::Serialize;
///
/// #[derive(GenerateCompat, Serialize, Debug)]
/// #[serde(rename_all = "camelCase")]
/// #[compat(placeholder = P)]
/// struct Foo {
///     #[compat(name = xyz)]
///     foo_bar: Option<i32>, // Becomes `xyz: Option<i32>` in the compat struct.
///     #[compat(type = i64)]
///     my_var: i32, // Becomes `my_var: i64` in the compat struct.
///
///     #[compat(type = P)]
///     placeholder: Point, // Becomes `placeholder: __semver_600_compat_Point`.
///
///     #[compat(name = abc_def, type = Option<u16>, map = Option::map)]
///     peer_limit: Option<u8>, // Becomes: `abc_def: Option<u16>` in the compat struct.
///
///     lorem_ipsum: String, // Remains: `lorem_ipsum: Option<String>`.
/// }
///
/// #[derive(GenerateCompat, Serialize)]
/// #[serde(rename_all = "kebab-case")]
/// struct Point {
///     x_float: f32,
///     y_float: f32,
/// }
/// ```
///
/// Will result in a generated compat struct looking something like:
/// ```rust
/// #[serde_with::skip_serializing_none]
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_600_compat_Foo {
///     xyz: Option<i32>, // Was `foo_bar: Option<i32>` in the original struct.
///     my_var: i64, // Was `my_var: i32` in the original struct.
///
///     placeholder: __semver_600_compat_Point, // Was: `placeholder: Point`
///
///     abc_def: Option<u16>, // Was: `peer_limit: Option<u8>` in the original struct.
///
///     lorem_ipsum: String, // Unchanged.
/// }
///
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_600_compat_Point {
///     x_float: f32,
///     y_float: f32,
/// }
/// ```
///
/// ### Unit Enum
/// ```rust
/// use compat_macros::GenerateCompat;
/// use serde::Serialize;
///
/// #[derive(GenerateCompat, Serialize, Debug)]
/// #[serde(rename_all = "kebab-case")]
/// enum Bar {
///     MyVariant, // Unchanged.
///     AnotherVariant, // Unchanged.
///     #[compat(name = LoremIpsum)]
///     AThirdVariant, // Renamed to: `LoremIpsum`.
/// }
/// ```
///
/// Will generate into something like:
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) enum __semver_600_compat_Bar {
///     MyVariant, // Unchanged.
///     AnotherVariant, // Unchanged.
///     LoremIpsum, // Was: `AThirdVariant`.
/// }
/// ```
///
/// ### Single-Field Tuple Variant Enum
///
/// ```rust
/// #[derive(GenerateCompat, Serialize)]
/// enum Var {
///     #[compat(name = A, type = Option<u64>, map = Option::map)]
///     X(Option<u8>),
///     #[compat(name = B, type = Option<u64>, map = Option::map)]
///     Y(Option<u8>),
///     #[compat(name = C, type = Option<u64>, map = Option::map)]
///     Z(Option<u8>),
/// }
/// ```
///
/// Will generate something like:
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) enum __semver_600_compat_Var {
///     A(Option<u64>), // Was: `X(Option<u8>)`
///     B(Option<u64>), // Was: `Y(Option<u8>)`
///     C(Option<u64>), // Was: `Z(Option<u8>)`
/// }
/// ```
///
/// [`type = ...`]: derive.GenerateCompat.html#compattype--type
#[proc_macro_derive(GenerateCompat, attributes(compat))] // TODO: rename: include `request`
pub fn generate_semver_600_compat(input: TokenStream) -> TokenStream {
    // REF: https://compilenrun.com/docs/language/rust/rust-advanced-features/rust-derive-macros/
    // REF: https://docs.rs/quote/latest/quote/macro.quote.html#indexing-into-a-tuple-struct
    // REF: https://stackoverflow.com/a/42526546

    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(data) => generate_compat_struct(&input, data),
        Data::Enum(data) => generate_compat_enum(&input, data),
        _ => panic!("GenerateCompat supports only structs and enums."),
    }
    .into()
}
