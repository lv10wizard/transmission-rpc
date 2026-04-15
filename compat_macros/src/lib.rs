//! Defines helper `#[derive(...)]` macros for request serialization compatibility with
//! Transmission.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use generate::generate_compat_types;

mod compat;
mod generate;
mod placeholder;
mod serde;
mod symbols;

/// Generates structs/enums for request serialization compatibility with every Transmission version
/// down to `1.50` (`rpc-version-semver` 1.3.0, `rpc-version`: 4). Ideally, `SemverCompat` would
/// accommodate every Transmission version, but the rpc api did not add a way to get the
/// Transmission `rpc-version` until this version (`1.50`, `rpc-version-semver` 1.3.0).
///
/// The source-defined original struct or enum can be converted into its corresponding compat type
/// with the generated `into_compat` method, eg. `x.into_compat(version)`. This `into_compat`
/// method returns an enum implementing `Serialize` where each of its variants corresponds to a
/// specific version's generated compat type.
///
/// This derive macro supports:
///
/// * `struct`s
/// * `enum`s with unit variants, eg. `enum Foo { A, B, C }`
/// * `enum`s with a single, unnamed field, eg. `enum Bar { A(i16), B(i32), C(i64) }`
///
/// # `#[serde]` attribute handling
///
/// Because transmission semver-`6.0.0` changed all rpc strings to `snake_case`,
///
/// * Container-level `#[serde(rename_all = "...")]` arguments, if defined,  will be forced into
/// `rename_all = "snake_case"` in all generated types for sem-versions >= `6.0.0`.
///
/// * Field/Variant-level `#[serde(rename = "...")]` arguments, if defined, will be omitted in all
/// generated types for sem-versions >= `6.0.0`.
///
/// Any other `#[serde(...)]` attribute arguments will remain in all generated compat types
/// unchanged (eg. `#[serde(skip_serializing_if = "Option::is_none")]`).
///
/// # Field/Variant attributes
///
/// ### `#[added(semver = "...")]`
///
/// TODO
///
/// ### `#[removed(semver = "...")]`
///
/// TODO
///
/// ### `#[renamed(semver = "...", name = ...)]`
///
/// TODO
///
/// ### `#[compat(type = _)]`
///
/// Flags that the tagged field- or variant's type should be converted into its corresponding
/// version's compat type.
///
/// `SemverCompat` parses the `type = ...` for `_` (ie, a single underscore) somewhere in the
/// defined value which can be either a valid Type or a string literal, eg.
///
/// * `#[compat(type = "_")]`
/// * `#[compat(type = Option<Vec<_>>)]`
/// * `#[compat(type = "Vec<_>")]`
///
/// Parsing the `type` will result in an error about "mismatched placeholder types" if the value
/// does not match the actual tagged field- or variant's type arguments. For example, the following
/// will not compile:
///
/// ```
/// use compat_macros::SemverCompat;
/// use serde::Serialize;
///
/// #[derive(SemverCompat, Serialize, Debug)]
/// struct Fail {
///     #[compat(type = _)] // Should be `type = Option<_>`.
///                         // (Would generate semver compatible types where `compat_field` would
///                         // be defined with a type like `__semver_xyz_compat_CompatType` for
///                         // each semver-x.y.z >= `1.3.0` defined in [rpc-spec.md].
///     compat_field: Option<CompatType>,
/// }
///
/// #[derive(SemverCompat, Serialize, Debug)]
/// struct CompatType {
///     my_compat_var: i32,
/// }
/// ```
///
/// # Examples
///
/// ### Struct
///
/// The following `#[derive(SemverCompat)]` struct definitions will generate structs for every rpc
/// semver >= `1.3.0` defined in [rpc-spec.md].
///
/// ```rust
/// use compat_macros::SemverCompat;
/// use serde::Serialize;
///
/// #[serde_with::skip_serializing_none]
/// #[derive(SemverCompat, Serialize, Debug)]
/// #[serde(rename_all = "camelCase")] // #[serde(rename_all = ...)] forced to "snake_case" in
///                                    // sem-versions >= 6.0.0
/// struct Foo {
///     #[added(semver = "2.1.0")]
///     #[renamed(semver = "5.2.0", name = xyz)]
///     #[removed(semver = "6.0.0")]
///     foo_bar: Option<i32>, // Omitted in sem-versions < 2.1.0
///                           // Exists as `foo_bar: Option<i32>` in sem-versions >= 2.1.0
///                           // Becomes `xyz: Option<i32>` in sem-versions >= 5.2.0
///                           // Omitted (again) in sem-versions >= 6.0.0
///
///     #[compat(type = _)]
///     placeholder: Point, // Becomes `placeholder: __semver_*_compat_Point`.
///
///     #[serde(rename = "lorem-ipsum")] // #[serde(rename = ...)] omitted sem-versions >= 6.0.0
///     lorem_ipsum: String, // Remains: `lorem_ipsum: Option<String>`.
/// }
///
/// #[derive(SemverCompat, Serialize)]
/// #[serde(rename_all = "kebab-case")]
/// struct Point {
///     x_float: f32,
///     y_float: f32,
/// }
/// ```
///
/// Some notable generated structs:
///
/// ##### semver 2.0.0
///
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "camelCase")]
/// pub(crate) struct __semver_200_compat_Foo {
///     // `foo_bar` field not defined (not yet added in semver-2.0.0).
///
///     placeholder: __semver_200_compat_Point, // Was: `placeholder: Point`
///
///     #[serde(rename = "lorem-ipsum")] // Unchanged.
///     lorem_ipsum: String, // Unchanged.
/// }
///
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_200_compat_Point {
///     x_float: f32,
///     y_float: f32,
/// }
/// ```
///
/// ##### semver 2.1.0
///
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "camelCase")]
/// pub(crate) struct __semver_210_compat_Foo {
///     #[serde(skip_serializing_if = "Option::is_none")]
///     foo_bar: Option<i32>, // Defined because it was added in semver-2.1.0.
///
///     placeholder: __semver_210_compat_Point, // Was: `placeholder: Point`
///
///     #[serde(rename = "lorem-ipsum")] // Unchanged.
///     lorem_ipsum: String, // Unchanged.
/// }
///
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_210_compat_Point {
///     x_float: f32,
///     y_float: f32,
/// }
/// ```
///
/// ##### semver 5.2.0
///
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "camelCase")]
/// pub(crate) struct __semver_520_compat_Foo {
///     #[serde(skip_serializing_if = "Option::is_none")]
///     xyz: Option<i32>, // Was `foo_bar: Option<i32>` (renamed in semver-5.2.0).
///
///     placeholder: __semver_520_compat_Point, // Was: `placeholder: Point`
///
///     #[serde(rename = "lorem-ipsum")] // Unchanged.
///     lorem_ipsum: String, // Unchanged.
/// }
///
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_520_compat_Point {
///     x_float: f32,
///     y_float: f32,
/// }
/// ```
///
/// ##### semver 6.0.0
///
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_600_compat_Foo {
///     // Neither `foo_bar` nor `xyz` field is defined (removed in semver-6.0.0).
///
///     placeholder: __semver_600_compat_Point, // Was: `placeholder: Point`
///
///     // #[semver(rename = ...)] not defined (semver-6.0.0 changed all string to `snake_case`.
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
/// ### Unit Enum TODO
///
/// ```rust
/// use compat_macros::SemverCompat;
/// use serde::Serialize;
///
/// #[derive(SemverCompat, Serialize, Debug)]
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
/// ### Single-Field Tuple Variant Enum TODO
///
/// ```rust
/// #[derive(SemverCompat, Serialize)]
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
/// [rpc-spec.md]:
/// <https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md#5-protocol-versions>
#[proc_macro_derive(SemverCompat, attributes(added, compat, removed, renamed))]
pub fn generate_semver_compat(input: TokenStream) -> TokenStream {
    // REF: https://compilenrun.com/docs/language/rust/rust-advanced-features/rust-derive-macros/
    // REF: https://docs.rs/quote/latest/quote/macro.quote.html#indexing-into-a-tuple-struct
    // REF: https://stackoverflow.com/a/42526546

    let input = parse_macro_input!(input as DeriveInput);
    let data = &input.data;
    match generate_compat_types(&input, data.into()) {
        Ok(generated) => generated,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
