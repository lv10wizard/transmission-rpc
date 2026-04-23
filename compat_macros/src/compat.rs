use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{self, Display},
};

use proc_macro2::Span;
use semver::Version;
use syn::{
    Attribute, Error, Expr, ExprTuple, Field, Fields, Ident, Lit, LitStr, Path, Result, Type,
    Variant,
    spanned::Spanned as _,
};

use crate::symbols::{ATTR_ADDED, ATTR_COMPAT, ATTR_REMOVED, ATTR_RENAMED};

/// Represents the change to a struct field or enum variant for a specific transmission semver.
#[derive(Debug, Clone, Eq, Hash)]
pub(crate) enum Kind {
    /// The struct field or enum variant was added.
    Added,
    // TODO? Deprecated,
    /// The struct field or enum variant was removed.
    Removed,
    /// The struct field or enum variant was renamed. The [`Ident`] field is the new name of the
    /// field or varaint.
    ///
    /// [`Ident`]: struct@syn::Ident
    Renamed(Ident),
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Added, Self::Added) => true,
            (Self::Removed, Self::Removed) => true,
            (Self::Renamed(_), Self::Renamed(_)) => true,
            (..) => false,
        }
    }
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

    replace_type: bool,
}

impl InternalCompatData {
    /// Inserts the semver compat change into the inner [`HashMap`], returning an `Err` if
    /// `version` already exists.
    ///
    /// Any given semver should only change a struct field or enum variant once. (eg. It doesn't
    /// make sense for a field or variant to be both added and renamed in the same semver.)
    fn insert<'a>(&mut self, key: (Version, Span), kind: Kind) -> Result<()> {
        let (version, span) = key;
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
    /// Whether the source-defined original field/variant's type should be replaced with its
    /// corresponding compat version in the generated struct/enum.
    pub(crate) replace_type: bool,
}

impl From<InternalCompatData> for CompatData {
    fn from(mut value: InternalCompatData) -> Self {
        Self {
            changes: value.map
                .drain()
                .map(|(version, parsed)| (version, parsed.into()))
                .collect(),
            replace_type: value.replace_type,
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

fn parse_str_lit(expr: &Expr) -> Result<&LitStr> {
    match expr {
        Expr::Lit(exprlit) if let Lit::Str(s) = &exprlit.lit => Ok(s),
        _ => Err(Error::new(expr.span(), "expected string literal")),
    }
}

fn parse_semver(value: &Expr) -> Result<(Version, Span)> {
    match Version::parse(&parse_str_lit(value)?.value()) {
        Ok(version) => {
            // TODO? Emit a warning if version not in SUPPORTED_VERSIONS?
            // TODO- (Span::warning is currently unstable)
            /*
            SUPPORTED_VERSIONS.iter()
                .find(|&v| &version == v)
                .map(|_| (version, value.span()))
                .ok_or(Error::new(value.span(), "unrecognized transmission rpc-version-semver"))
            */
            Ok((version, value.span()))
        },
        Err(err) => return Err(Error::new(value.span(), err)),
    }
}

fn parse_new_name(value: &Expr) -> Result<Ident> {
    let s = parse_str_lit(value)?;
    s.parse()
}

fn check_duplicate<'a>(seen: &mut HashSet<&'a Path>, attr: &'a Attribute) -> Result<()> {
    if !seen.insert(attr.path()) {
        let Some(attr_id) = attr.path().get_ident() else {
            return Err(Error::new(attr.path().span(), "attribute should have a valid ident"));
        };

        // This error should never be emitted for unknown attributes because that case is
        // handled by the `else` clause below.
        let msg = format!("multiple \"{attr_id}\" changes defined");
        return Err(Error::new(attr_id.span(), msg));
    }
    Ok(())
}

/// Parses a struct field or enum variant's attributes for semver compat data.
///
/// Among various parsing errors, this will also fail if:
///
/// * multiple change attributes (#\[added\], #\[removed\], #\[renamed\]) are specified for the
/// same version, or
/// * duplicate change attributes are defined.
pub(crate) fn parse_attr<'a>(fv: FieldOrVar<'a>) -> Result<CompatData> {
    let mut data = InternalCompatData::default();
    let mut seen = HashSet::new();

    for attr in fv.attributes().iter() {
        if attr.path() == ATTR_ADDED { // #[added = "semver"]
            check_duplicate(&mut seen, attr)?;
            let meta = attr.meta.require_name_value()?;
            data.insert(parse_semver(&meta.value)?, Kind::Added)?;

        } else if attr.path() == ATTR_REMOVED { // #[removed = "semver"]
            check_duplicate(&mut seen, attr)?;
            let meta = attr.meta.require_name_value()?;
            data.insert(parse_semver(&meta.value)?, Kind::Removed)?;

        } else if attr.path() == ATTR_RENAMED { // #[renamed = ("semver", "name")]
            check_duplicate(&mut seen, attr)?;
            let meta = attr.meta.require_name_value()?;
            let s = parse_str_lit(&meta.value)?;
            let tuple: ExprTuple = s.parse()?;
            let (semver, name) = match tuple.elems.len() {
                2 => (
                    parse_semver(&tuple.elems[0]).map(Some)?,
                    parse_new_name(&tuple.elems[1]).map(Some)?,
                ),
                _ => (None, None),
            };

            match (semver, name) {
                (Some(semver), Some(name)) => data.insert(semver, Kind::Renamed(name))?,
                (..) => {
                    let msg = "expected tuple: (\"semver\", \"name\")";
                    return Err(Error::new(meta.span(), msg));
                },
            }

        // TODO? } else if attr.path() == ATTR_DEPRECATED {
        // TODO- deprecated handling requires hand-rolled Serialize impl

        } else if attr.path() == ATTR_COMPAT { // #[compat]
            if fv.ty()?.is_none() {
                let msg = "expected field or variant to have a type";
                return Err(Error::new(attr.path().span(), msg));
            }
            check_duplicate(&mut seen, attr)?;
            attr.meta.require_path_only()?;
            data.replace_type = true;
        }
    }

    data.validate()?;

    Ok(data.into())
}
