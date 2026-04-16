use std::fmt::{self, Display};

use quote::format_ident;
use syn::{Ident, Path};
use semver::Version;

/// Formats a (hopefully) unique name prefixed with the given transmission semver, `v`, for the
/// specified `id`.
pub(crate) fn compat_id(v: &Version, id: &Ident) -> Ident {
    format_ident!("__semver_{}_compat_{id}", version_id(v))
}

pub(crate) fn version_id(v: &Version) -> Ident {
    format_ident!("v{}{}{}", v.major, v.minor, v.patch)
}

#[derive(Copy, Clone)]
pub(crate) struct Symbol(&'static str);

pub(crate) const ATTR_ADDED: Symbol = Symbol("added");
pub(crate) const ATTR_COMPAT: Symbol = Symbol("compat");
pub(crate) const ATTR_DEPRECATED: Symbol = Symbol("deprecated"); // TODO? requires custom ser
pub(crate) const ATTR_REMOVED: Symbol = Symbol("removed");
pub(crate) const ATTR_RENAMED: Symbol = Symbol("renamed");

pub(crate) const NAME: Symbol = Symbol("name");
pub(crate) const SEMVER: Symbol = Symbol("semver");

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
