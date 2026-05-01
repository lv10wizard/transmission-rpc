//! Defines helper `#[derive(...)]` macros for request serialization compatibility with
//! Transmission.

use std::sync::LazyLock;

use proc_macro::TokenStream;
use semver::Version;
use syn::{DeriveInput, parse_macro_input};

use generate::{StructOrEnum, generate_compat_types};

mod compat;
mod generate;
mod serde;
mod serialize;
mod symbols;
mod r#type;

static SUPPORTED_VERSIONS: LazyLock<Vec<Version>> = LazyLock::new(|| {
    let mut versions = vec![
        Version::new(1, 3, 0), // Transmission 1.50

        Version::new(2, 0, 0), // Transmission 1.60
        Version::new(2, 1, 0), // Transmission 1.70

        Version::new(3, 0, 0), // Transmission 1.80
        Version::new(3, 1, 0), // Transmission 1.90
        Version::new(3, 2, 0), // Transmission 1.92
        Version::new(3, 3, 0), // Transmission 2.00
        Version::new(3, 4, 0), // Transmission 2.10
        Version::new(3, 5, 0), // Transmission 2.12
        Version::new(3, 6, 0), // Transmission 2.20

        Version::new(4, 0, 0), // Transmission 2.30

        Version::new(5, 0, 0), // Transmission 2.40
        Version::new(5, 1, 0), // Transmission 2.80
        Version::new(5, 2, 0), // Transmission 3.00
        Version::new(5, 3, 0), // Transmission 4.0.0

        Version::new(6, 0, 0), // Transmission 4.1.0
        Version::new(6, 0, 1), // Transmission 4.1.1
        // TODO: Version::new(6, 1, 0), // Transmission 4.2.0
    ];

    // Ensure the versions are sorted and unique.
    versions.sort();
    versions.dedup();

    versions
});

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
/// ### `#[added = "x.y.z"]`
///
/// Flags that the struct field or enum variant was added in this rpc-semver.
///
/// Specifically, the field/variant will only be serialized for requests to rpc servers with semver
/// `>= "x.y.z" semver`.
///
/// ### `#[removed = "x.y.z")]`
///
/// Flags that the struct field or enum variant was removed in this rpc-semver.
///
/// Specifically, the field/variant will only be serialized for requests to rpc servers with semver
/// `< "x.y.z" semver`.
///
/// ### `#[renamed = r#"("x.y.z", "name")"#]`
///
/// Flags that the struct field or enum variant was renamed in this rpc-semver to the given
/// `"name"`.
///
/// That is,
///
/// * if `rpc server semver < "x.y.z" semver`, the field/variant will be serialized with the
/// source-defined ident.
/// * if `rpc server semver >= "x.y.z" semver`, the field/variant will be serialized with the given
/// `"name"` ident.
///
/// ### `#[compat]`
///
/// Parses the field- or variant's type and replaces its inner-most generic argument with its
/// corresponding version's compat type, eg.
///
/// * `Foo` => `__semver_xyz_compat_Foo`
/// * `Option<Foo>` => `Option<__semver_xyz_compat_Foo>`
/// * `Option<Vec<Foo>>` => `Option<Vec<__semver_xyz_compat_Foo>>`
///
/// # Examples
///
/// ### Struct
///
/// The following `#[derive(SemverCompat)]` struct definitions will generate structs for every rpc
/// semver \>= `1.3.0` defined in [rpc-spec.md].
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
///     #[added = "2.1.0"]
///     #[renamed = r#"("5.2.0", "xyz")"#]
///     #[removed = "6.0.0"]
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
/// ### Unit Enum
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
///     #[renamed = r#"("6.0.0", "LoremIpsum")"#]
///     AThirdVariant, // Renamed to: `LoremIpsum` in sem-versions >= 6.0.0.
/// }
/// ```
///
/// The generated enum for semver 6.0.0 will look something like:
///
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
/// #[derive(SemverCompat, Serialize)]
/// enum Var {
///     #[compat]
///     X(A),
/// }
///
/// #[derive(SemverCompat, Serialize)]
/// #[serde(rename_all = "camelCase")]
/// struct A {
///     a_one: Option<i32>,
///     a_two: Option<i32>,
/// }
/// ```
///
/// Will generate a compat type for all [rpc-spec.md]-defined semver >= `1.3.0`, eg. semver 6.0.0:
///
/// ```rust
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) enum __semver_600_compat_Var {
///     X(__semver_600_compat_A),
/// }
///
/// #[derive(Serialize, Debug, Clone)]
/// #[serde(rename_all = "snake_case")]
/// pub(crate) struct __semver_600_compat_A {
///     a_one: Option<i32>,
///     a_two: Option<i32>,
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
    let ident = &input.ident;
    let data = &input.data;
    match generate_compat_types(&input, StructOrEnum::new(ident, data)) {
        Ok(generated) => generated,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

// TODO: #[semver_doc] (or something) to update field/variant docs with change version info.
