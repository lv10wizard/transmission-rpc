use std::fmt::{self, Display};

use quote::format_ident;
use syn::{Ident, Path};
use semver::Version;

/// Formats a (hopefully) unique prefix suitable for generated struct and enum names from the given
/// transmission semver, `v`.
pub(crate) fn compat_prefix(v: &Version) -> Ident {
    format_ident!("__semver_{}{}{}_compat_", v.major, v.minor, v.patch)
}

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub(crate) const ATTR_ADDED: Symbol = Symbol("added");
pub(crate) const ATTR_COMPAT: Symbol = Symbol("compat");
pub(crate) const ATTR_DEPRECATED: Symbol = Symbol("deprecated"); // TODO? requires custom ser
pub(crate) const ATTR_REMOVED: Symbol = Symbol("removed");
pub(crate) const ATTR_RENAMED: Symbol = Symbol("renamed");

pub(crate) const PLACEHOLDER: Symbol = Symbol("placeholder");
pub(crate) const MAP: Symbol = Symbol("map");
pub(crate) const NAME: Symbol = Symbol("name");
pub(crate) const SEMVER: Symbol = Symbol("semver");
pub(crate) const TYPE: Symbol = Symbol("type");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, other: &Symbol) -> bool {
        self == other.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl PartialEq<Symbol> for &Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
