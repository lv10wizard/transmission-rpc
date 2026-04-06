use std::collections::HashMap;

use proc_macro2::Span;
use semver::Version;
use syn::{
    Attribute, Error, Field, Fields, Ident, LitStr, Result, Type, Variant,
    meta::ParseNestedMeta,
    spanned::Spanned as _
};

use crate::{
    parse::parse_ident,
    symbols::{ATTR_ADDED, ATTR_REMOVED, ATTR_RENAMED, NAME, SEMVER},
};

/// Represents the change to a struct field or enum variant for a specific transmission semver.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Kind {
    /// The struct field or enum variant was added.
    Added,
    // TODO? Deprecated,
    /// The struct field or enum variant was removed.
    Removed,
    /// The struct field or enum variant was renamed. The [`Ident`] field is the new name of the
    /// field or varaint.
    Renamed(Ident),
}

/// Holds compat data for a struct field or enum variant parsed from its attributes.
#[derive(Debug, Clone)]
struct ParsedAttr {
    /// What changed in this version.
    kind: Kind,
    /// The location of source code of the parsed attribute that this `ParsedAttr` corresponds to.
    span: Span,
}

impl From<ParsedAttr> for Kind {
    fn from(value: ParsedAttr) -> Self {
        value.kind
    }
}

/// Helper wrapper over [`HashMap`] to handle duplicate changes for the same transmission semver
/// for a given struct field or enum variant.
#[derive(Default, Debug, Clone)]
struct CompatData {
    map: HashMap<Version, ParsedAttr>,
}

impl CompatData {
    /// Inserts the semver compat change into the inner [`HashMap`], returning an `Err` if
    /// `version` already exists.
    fn insert<'a>(&mut self, attr: &'a Attribute, version: Version, kind: Kind) -> Result<()> {
        let parsed = ParsedAttr {
            kind,
            span: attr.span(),
        };
        if let Some(existing) = self.map.insert(version.clone(), parsed) {
            let span = attr.span().located_at(existing.span);
            return Err(Error::new(span, format!("multiple changes defined for {version}")));
        }
        Ok(())
    }
}

impl From<CompatData> for HashMap<Version, Kind> {
    fn from(value: CompatData) -> Self {
        value.map
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect()
    }
}

/// Either a struct field or enum variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FieldOrVar<'a> {
    Field(&'a Field),
    Variant(&'a Variant),
}

impl FieldOrVar<'_> {
    fn require_ident(&self) -> Result<&Ident> {
        match self {
            Self::Field(f) => f.ident
                .as_ref()
                .ok_or(Error::new(f.span(), "unnamed struct field has no ident")),
            Self::Variant(v) => Ok(&v.ident),
        }
    }

    fn ty(&self) -> Result<Option<&Type>> {
        match self {
            Self::Field(f) => Ok(Some(&f.ty)),
            Self::Variant(v) => match &v.fields {
                Fields::Unit => Ok(None),
                Fields::Unnamed(fields) => match fields.unnamed.len() {
                    1 => Ok(fields.unnamed.get(0).map(|f| &f.ty)),
                    _ => {
                        let msg = "";
                        Err(Error::new(v.span(), msg))
                    },
                },
                Fields::Named(_) => {
                    let msg = "";
                    Err(Error::new(v.span(), msg))
                },
            },
        }
    }

    fn attributes(&self) -> &Vec<Attribute> {
        match self {
            Self::Field(f) => &f.attrs,
            Self::Variant(v) => &v.attrs,
        }
    }

    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Field(f) => f.span(),
            Self::Variant(v) => v.span(),
        }
    }
}

impl<'a> From<&'a Field> for FieldOrVar<'a> {
    fn from(value: &'a Field) -> Self {
        FieldOrVar::Field(value)
    }
}

impl<'a> From<&'a Variant> for FieldOrVar<'a> {
    fn from(value: &'a Variant) -> Self {
        FieldOrVar::Variant(value)
    }
}

fn parse_semver<'a>(meta: &'a ParseNestedMeta<'_>) -> Result<Option<Version>> {
    if meta.path == SEMVER {
        let parsed: LitStr = meta.value()?.parse()?;
        Version::parse(&parsed.value())
            .map(Some)
            .map_err(|err| meta.error(format!("{err}")))
    } else {
        Ok(None)
    }
}

/// Parses a struct field or enum variant's attributes for semver compat data.
pub(crate) fn parse_version_attr<'a>(fv: FieldOrVar<'a>) -> Result<HashMap<Version, Kind>> {
    let mut data = CompatData::default();

    for attr in fv.attributes().iter() {
        if attr.path() == ATTR_ADDED { // #[added(semver = "...")]
            attr.parse_nested_meta(|meta| {
                if let Some(semver) = parse_semver(&meta)? {
                    data.insert(attr, semver, Kind::Added)?;
                }
                Ok(())
            })?;

        } else if attr.path() == ATTR_REMOVED { // #[removed(semver = "...")]
            attr.parse_nested_meta(|meta| {
                if let Some(semver) = parse_semver(&meta)? {
                    data.insert(attr, semver, Kind::Removed)?;
                }
                Ok(())
            })?;

        } else if attr.path() == ATTR_RENAMED { // #[renamed(semver = "...", name = ...)]
            let mut new_name: Option<Ident> = None;
            let mut semver: Option<Version> = None;

            // Parse each argument first to ensure both `semver` and `name` are specified.
            attr.parse_nested_meta(|meta| {
                if let Some(ver) = parse_semver(&meta)? {
                    semver = Some(ver);
                }

                if meta.path == NAME {
                    new_name = parse_ident(meta.value()?)
                        .map(Some)?;
                }

                Ok(())
            })?;

            match (semver, new_name) {
                (Some(semver), Some(new_name)) => {
                    data.insert(attr, semver, Kind::Renamed(new_name))?;
                },

                (..) => {
                    return Err({
                        let msg = format!("expected both \"{SEMVER}\" and \"{NAME}\" arguments");
                        Error::new(attr.span(), msg)
                    });
                },
            }

        // TODO? } else if attr.path() == ATTR_DEPRECATED {
        // TODO- deprecated handling requires hand-rolled Serialize impl

        }
    }

    Ok(data.into())
}
