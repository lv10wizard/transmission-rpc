use std::{cmp::Ordering, collections::HashMap, fmt::{self, Display}};

use proc_macro2::Span;
use semver::Version;
use syn::{
    Attribute, Error, ExprAssign, Field, Fields, Ident, LitStr, Path, Result, Type, Variant,
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

impl Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Renamed(_) => "renamed",
        })
    }
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

#[derive(Default, Debug, Clone)]
struct InternalCompatData {
    /// Helper map to handle [`Kind`] collision for the same [`Version`] key.
    map: HashMap<Version, ParsedAttr>,

    replace_type: Option<Type>,
    replace_map: Option<Path>,
}

impl InternalCompatData {
    /// Inserts the semver compat change into the inner [`HashMap`], returning an `Err` if
    /// `version` already exists.
    ///
    /// Any given semver should only change a struct field or enum variant once. (eg. It doesn't
    /// make sense for a field or variant to be both added and renamed in the same semver.)
    fn insert<'a>(&mut self, span: Span, version: Version, kind: Kind) -> Result<()> {
        let parsed = ParsedAttr { kind, span };
        if let Some(existing) = self.map.insert(version.clone(), parsed) {
            let span = span.located_at(existing.span);
            return Err(Error::new(span, format!("multiple changes defined for {version}")));
        }
        Ok(())
    }

    /// Validates change versions for logical consistency (eg. a `removed` version cannot be less
    /// than an `added` version; that is, a field cannot be added after it was removed).
    ///
    /// This method assumes all of the struct field's or enum variant's attributes are parsed and
    /// any duplicate [`Kind`]s have been handled.
    fn validate(&self) -> Result<()> {
        let mut added = None;
        let mut removed = None;
        let mut renamed = None;

        for (version, parsed) in self.map.iter() {
            match &parsed.kind {
                Kind::Added => added = Some((version, parsed.span)),
                Kind::Removed => removed = Some((version, parsed.span)),
                Kind::Renamed(_) => renamed = Some((version, parsed.span)),
            }
        }

        if let (Some(added), Some(removed)) = (&added, &removed)
            && added.0 >= removed.0
        {
            let span = added.1.located_at(removed.1);
            let phrase = match added.0.cmp(&removed.0) {
                Ordering::Less => panic!("how did this happen?"),

                Ordering::Equal => "in the same",
                Ordering::Greater => "after the",
            };
            let msg = format!("field or variant cannot be added {phrase} version it was removed");
            return Err(Error::new(span, msg));
        }

        if let (Some(added), Some(renamed)) = (&added, &renamed)
            && added.0 >= renamed.0
        {
            let span = added.1.located_at(renamed.1);
            let phrase = match added.0.cmp(&renamed.0) {
                Ordering::Less => panic!("how did this happen?"),

                Ordering::Equal => "in the same",
                Ordering::Greater => "before the",
            };
            let msg = format!("field or variant cannot be renamed {phrase} version it was added");
            return Err(Error::new(span, msg));
        }

        if let (Some(removed), Some(renamed)) = (&removed, &renamed)
            && renamed.0 >= removed.0
        {
            let span = removed.1.located_at(renamed.1);
            let phrase = match renamed.0.cmp(&removed.0) {
                Ordering::Less => panic!("how did this happen?"),

                Ordering::Equal => "in the same",
                Ordering::Greater => "after the",
            };
            let msg = format!("field or variant cannot be renamed {phrase} version \
                it was removed");
            return Err(Error::new(span, msg));
        }

        Ok(())
    }
}

/// Holds compat data for a struct field or enum variant parsed from its attributes.
#[derive(Debug, Clone)]
pub(crate) struct CompatData {
    /// What changed about the struct field or enum variant in a particular [`Version`].
    pub(crate) changes: HashMap<Version, Kind>,
    /// The struct field's type replacement.
    pub(crate) replace_type: Option<Type>,
    /// The struct field's type replacement conversion function.
    pub(crate) replace_map: Option<Path>,
}

impl From<InternalCompatData> for CompatData {
    fn from(mut value: InternalCompatData) -> Self {
        Self {
            changes: value.map
                .drain()
                .map(|(version, parsed)| (version, parsed.into()))
                .collect(),
            replace_type: value.replace_type,
            replace_map: value.replace_map,
        }
    }
}

/// Either a struct field or enum variant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FieldOrVar<'a> {
    Field(&'a Field),
    Variant(&'a Variant),
}

impl<'a> FieldOrVar<'a> {
    pub(crate) fn require_ident(&self) -> Result<&'a Ident> {
        match self {
            Self::Field(f) => f.ident
                .as_ref()
                .ok_or(Error::new(f.span(), "struct field has no ident")),
            Self::Variant(v) => Ok(&v.ident),
        }
    }

    pub(crate) fn ty(&self) -> Result<Option<&'a Type>> {
        match self {
            Self::Field(f) => Ok(Some(&f.ty)),
            Self::Variant(v) => match &v.fields {
                Fields::Unit => Ok(None),
                Fields::Unnamed(fields) => match fields.unnamed.len() {
                    1 => Ok(fields.unnamed.get(0).map(|f| &f.ty)),
                    n => {
                        let msg = format!("unsupported: enum variant with #{n} unnamed fields");
                        Err(Error::new(v.span(), msg))
                    },
                },
                Fields::Named(_) => {
                    let msg = "enum with struct variant unsupported";
                    Err(Error::new(v.span(), msg))
                },
            },
        }
    }

    pub(crate) fn attributes(&self) -> &'a Vec<Attribute> {
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
    let mut data = InternalCompatData::default();

    for attr in fv.attributes().iter() {
        if attr.path() == ATTR_ADDED { // #[added(semver = "...")]
            attr.parse_nested_meta(|meta| {
                if let Some(semver) = parse_semver(&meta)? {
                    let span = meta.input
                        .parse::<ExprAssign>()?
                        .span();
                    data.insert(span, semver, Kind::Added)?;
                }
                Ok(())
            })?;

        } else if attr.path() == ATTR_REMOVED { // #[removed(semver = "...")]
            attr.parse_nested_meta(|meta| {
                if let Some(semver) = parse_semver(&meta)? {
                    let span = meta.input
                        .parse::<ExprAssign>()?
                        .span();
                    data.insert(span, semver, Kind::Removed)?;
                }
                Ok(())
            })?;

        } else if attr.path() == ATTR_RENAMED { // #[renamed(semver = "...", name = ...)]
            let mut new_name: Option<Ident> = None;
            let mut semver: Option<Version> = None;
            let mut span = None;

            // Parse each argument first to ensure both `semver` and `name` are specified.
            attr.parse_nested_meta(|meta| {
                if let Some(ver) = parse_semver(&meta)? {
                    span = meta.input
                        .parse::<ExprAssign>()?
                        .span()
                        .into();
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
                    let span = span.expect("semver span should exist");
                    data.insert(span, semver, Kind::Renamed(new_name))?;
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

                        FieldOrVar::Variant(var) => {
                            let enum_kind = match &var.fields {
                                Fields::Unnamed(fields) => {
                                    match fields.unnamed.len() {
                                        0 => Some("zero-field tuple"), // Can this happen?
                                        1 => {
                                            data.replace_type = meta.value()?
                                                .parse()
                                                .map(Some)?;
                                            None
                                        },

                                        _ => Some("multi-field tuple"),
                                    }
                                },

                                Fields::Named(_) => Some("struct"),
                                Fields::Unit => Some("unit"),
                            };

                            if let Some(enum_kind) = enum_kind {
                                return Err({
                                    let msg = format!("unsupported \"{TYPE}\" argument \
                                        on {enum_kind} enum variant");
                                    meta.error(msg)
                                });
                            }
                        },
                    }

                } else {
                    let ident = meta.path.require_ident()?;
                    let msg = format!("unexpected #[{ATTR_COMPAT}] argument: \"{ident}\"");
                    return Err(meta.error(msg));
                }

                Ok(())
            })?;
        }
    }

    data.validate()?;

    Ok(data.into())
}
