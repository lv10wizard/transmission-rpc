//! This crate defines a trait shared between the main transmission-rpc library and the helper
//! proc-macro crate.
//!
//! This exists because I couldn't figure out how to generate a trait and determine its module path
//! in the proc-macro helper crate.

/// Defines a method to indicate whether an enum variant is missing in the implementation's compat
/// version enum.
pub trait VariantMissing {
    /// Should return `true` if the variant does not exist in this transmission rpc semver.
    fn __variant_missing(&self) -> bool {
        false
    }

    /// Returns whether the variant exists in this transmission rpc semver. This is implemented as
    /// `!self.variant_missing()`.
    fn __variant_exists(&self) -> bool {
        !self.__variant_missing()
    }
}

/// Provide a blanket implementation on all types so that generated compat enums can override the
/// necessary methods to properly filter out any missing variant when serializing.
///
/// REF: <https://stackoverflow.com/a/71721454>
impl<T> VariantMissing for T {}
