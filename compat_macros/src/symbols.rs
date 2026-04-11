use std::fmt::{self, Display};

use proc_macro2::{Punct, Spacing};
use quote::{ToTokens, TokenStreamExt as _, format_ident};
use syn::{Ident, Path, parse_quote};
use semver::Version;

/// Formats a (hopefully) unique name prefixed with the given transmission semver, `v`, for the
/// specified `id`.
pub(crate) fn compat_id(v: &Version, id: &Ident) -> Ident {
    format_ident!("__semver_{}_compat_{id}", version_id(v))
}

pub(crate) fn version_id(v: &Version) -> Ident {
    format_ident!("v{}{}{}", v.major, v.minor, v.patch)
}

/// [`semver::Version`] wrapper for use with [`quote::quote`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CompatVersion(pub Version);

impl ToTokens for CompatVersion {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ctor: Path = parse_quote! { semver::Version::new };
        ctor.to_tokens(tokens);

        tokens.append(Punct::new('(', Spacing::Alone));
        self.0.major.to_tokens(tokens);
        tokens.append(Punct::new(',', Spacing::Alone));
        self.0.minor.to_tokens(tokens);
        tokens.append(Punct::new(',', Spacing::Alone));
        self.0.patch.to_tokens(tokens);
        tokens.append(Punct::new(')', Spacing::Alone));
    }
}

impl From<CompatVersion> for Version {
    fn from(value: CompatVersion) -> Self {
        value.0
    }
}

impl PartialEq<Version> for CompatVersion {
    fn eq(&self, other: &Version) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<&Version> for CompatVersion {
    fn eq(&self, other: &&Version) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<Version> for CompatVersion {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<&Version> for CompatVersion {
    fn partial_cmp(&self, other: &&Version) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
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
