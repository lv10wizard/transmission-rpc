use quote::format_ident;
use semver::Version;
use syn::{
    Attribute, Error, Field, Fields, Ident, LitStr, Result, Type, Variant,
    meta::ParseNestedMeta,
    spanned::Spanned as _
};

use crate::symbols::{ATTR_ADDED, ATTR_CHANGED, ATTR_REMOVED, NAME, SEMVER};

/// Holds compat data for a struct field or enum variant parsed from its attributes.
#[derive(Default, Debug, Clone)]
pub(crate) struct CompatData<'a> {
    pub(crate) added: Option<(Version, FieldOrVar<'a>)>,
    pub(crate) changed: Option<(Version, ChangeData<'a>)>,
    pub(crate) removed: Option<(Version, FieldOrVar<'a>)>,
}

/// Stores how the struct field or enum variant changed in a particular version.
#[derive(Debug, Clone)]
pub(crate) struct ChangeData<'a> {
    pub(crate) field: FieldOrVar<'a>,
    pub(crate) new_name: Ident,
}

/// Either a struct field or enum variant.
#[derive(Debug, Clone)]
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
pub(crate) fn parse_version_attr<'a>(fv: FieldOrVar<'a>) -> Result<CompatData<'a>> {
    let mut data = CompatData::default();

    for attr in fv.attributes().iter() {
        if attr.path() == ATTR_ADDED { // #[added(semver = ...)]
            attr.parse_nested_meta(|meta| {
                if let Some(semver) = parse_semver(&meta)? {
                    data.added = Some((semver, fv.clone()));
                }
                Ok(())
            })?;

        } else if attr.path() == ATTR_REMOVED { // #[removed(semver = ...)]
            attr.parse_nested_meta(|meta| {
                if let Some(semver) = parse_semver(&meta)? {
                    data.removed = Some((semver, fv.clone()));
                }
                Ok(())
            })?;

        } else if attr.path() == ATTR_CHANGED { // #[changed(semver = ..., name = ...)]
            let mut new_name: Option<Ident> = None;
            let mut semver: Option<Version> = None;

            attr.parse_nested_meta(|meta| {
                if let Some(ver) = parse_semver(&meta)? {
                    semver = Some(ver);
                }

                if meta.path == NAME {
                    let meta_val = meta.value()?;
                    new_name = match meta_val.parse::<LitStr>() {
                        Ok(s) => Some(format_ident!("{}", s.value())),
                        Err(_) => meta_val.parse::<Ident>()
                            .map(Some)
                            .map_err(|_| {
                                let msg = format!("#[{ATTR_CHANGED}({NAME} = ...)] \
                                    value must be either a string literal or valid Ident");
                                meta.error(msg)
                            })?,
                    }
                }

                Ok(())
            })?;

            match (semver, new_name) {
                (Some(semver), Some(new_name)) => {
                    data.changed = Some((semver, ChangeData {
                        field: fv.clone(),
                        new_name,
                    }));
                },

                (..) => {
                    return Err({
                        let msg = format!("#[{ATTR_CHANGED}] \
                            requires both \"{SEMVER}\" and \"{NAME}\" arguments");
                        Error::new(attr.span(), msg)
                    });
                },
            }

        // TODO? } else if attr.path() == ATTR_DEPRECATED {
        // TODO- deprecated handling requires hand-rolled Serialize impl

        }
    }

    Ok(data)
}
