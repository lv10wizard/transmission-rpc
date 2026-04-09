use std::collections::HashMap;

use proc_macro2::Span;
use semver::Version;
use syn::{
    Attribute, Error, Field, Fields, Ident, LitStr, Result, Type, Variant,
    meta::ParseNestedMeta,
    spanned::Spanned as _,
};

use crate::{
    parse::parse_ident,
    symbols::{ATTR_ADDED, ATTR_COMPAT, ATTR_REMOVED, ATTR_RENAMED, NAME, SEMVER, TYPE},
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

/// Holds compat data for a struct field or enum variant parsed from its attributes.
#[derive(Default, Debug, Clone)]
pub(crate) struct CompatData {
    /// Helper map to handle [`Kind`] collision for the same [`Version`] key.
    inner: HashMap<Version, ParsedAttr>,

    /// What changed about the struct field or enum variant in a particular [`Version`].
    pub(crate) changes: HashMap<Version, Kind>,
    /// The struct field's type replacement.
    pub(crate) replace_type: Option<Type>,
}

impl CompatData {
    /// Inserts the semver compat change into the inner [`HashMap`], returning an `Err` if
    /// `version` already exists.
    ///
    /// Any given semver should only change a struct field or enum variant once. (eg. It doesn't
    /// make sense for a field or variant to be both added and renamed in the same semver.)
    fn insert<'a>(&mut self, attr: &'a Attribute, version: Version, kind: Kind) -> Result<()> {
        let parsed = ParsedAttr {
            kind,
            span: attr.span(),
        };
        if let Some(existing) = self.inner.insert(version.clone(), parsed) {
            let span = attr.span().located_at(existing.span);
            return Err(Error::new(span, format!("multiple changes defined for {version}")));
        }
        Ok(())
    }

    fn into_kind_map(mut self) -> Self {
        self.changes = self.inner
            .drain()
            .map(|(version, parsed)| (version, parsed.into()))
            .collect();
        self
    }
}

/// Either a struct field or enum variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FieldOrVar<'a> {
    Field(&'a Field),
    Variant(&'a Variant),
}

impl FieldOrVar<'_> {
    pub(crate) fn require_ident(&self) -> Result<&Ident> {
        match self {
            Self::Field(f) => f.ident
                .as_ref()
                .ok_or(Error::new(f.span(), "unnamed struct field has no ident")),
            Self::Variant(v) => Ok(&v.ident),
        }
    }

    pub(crate) fn ty(&self) -> Result<Option<&Type>> {
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

    pub(crate) fn attributes(&self) -> &Vec<Attribute> {
        match self {
            Self::Field(f) => &f.attrs,
            Self::Variant(v) => &v.attrs,
        }
    }

    pub(crate) fn span(&self) -> proc_macro2::Span {
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
///
/// Among various parsing errors, this will also fail if multiple change attributes (#\[added\],
/// #\[removed\], #\[renamed\]) are specified for the same `semver`.
pub(crate) fn parse_attr<'a>(fv: FieldOrVar<'a>) -> Result<CompatData> {
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

        } else if attr.path() == ATTR_COMPAT { // #[compat(type = ...)]
            attr.parse_nested_meta(|meta| {
                if meta.path == TYPE {
                    match fv {
                        FieldOrVar::Field(_) => {
                            data.replace_type = meta.value()?
                                .parse()
                                .map(Some)?;
                        },

                        FieldOrVar::Variant(_) => {
                            return Err({
                                let msg = format!("unsupported \"{TYPE}\" argument \
                                    on enum variant");
                                meta.error(msg)
                            });
                        },
                    }
                }

                Ok(())
            })?;
        }
    }

    Ok(data)
}
