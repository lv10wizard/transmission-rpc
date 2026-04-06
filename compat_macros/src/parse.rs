use proc_macro2::Span;
use syn::{
    Attribute, Error, Expr, Ident, Lit, LitStr, Meta, Path, Result, Token, Type,
    parse::ParseBuffer,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

use crate::{
    placeholder::replace_compat_placeholder,
    symbols::{ATTR_COMPAT, NAME, PLACEHOLDER, TYPE, MAP},
};

pub(crate) struct ParsedFieldAttr {
    pub(crate) span: Option<Span>,
    pub(crate) name: Option<Ident>,
    pub(crate) ty: Option<Type>,
    pub(crate) map_fn: Option<Path>,
}

pub(crate) struct ParsedOuterAttr {
    pub(crate) placeholder: Option<Ident>,
}

/// Parses an [`Ident`] from a [`ParseBuffer`] like an [`Attribute`] meta value in
/// [`parse_nested_meta`].
///
/// [`Attribute`]: syn::Attribute
/// [`parse_nested_meta`]: syn::Attribute::parse_nested_meta
pub(crate) fn parse_ident<'a>(buffer: &'a ParseBuffer<'_>) -> Result<Ident> {
    match buffer.parse::<LitStr>() {
        Ok(s) => Ok(s.parse()?),
        Err(_) => buffer.parse::<Ident>()
            .map_err(|_| buffer.error("value must be either a string literal or valid Ident")),
    }
}

/// Parses the #\[compat(...)\] helper attribute on struct fields or enum variants.
///
/// eg.
/// ```
/// #[derive(GenerateCompat)]
/// struct Foo {
///     #[compat(name = xyzzy)] // <<< Parses this
///     bar: i32,
/// }
/// ```
pub(crate) fn parse_field_compat_attr<'a, I>(
    attributes: I,
    orig_type: Option<&Type>,
    outer: &ParsedOuterAttr,
) -> Result<ParsedFieldAttr>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut span = None;
    let mut attr_name = None;
    let mut attr_type = None;
    let mut mapping = None;

    // TODO: ----- Generate a compat container (struct/enum) for each semver
    // TODO: #[added(semver = "VERSION")] => include only for compat versions >=
    // TODO: (?) #[deprecated(semver = "VERSION", reason = "...")]
    // TODO-    => one-time warn + a way to disable -- needs custom Serialize impl tho
    // TODO: #[removed(semver = "VERSION")] => do not include for compat versions >=
    // TODO: #[renamed(semver = "VERSION", name = "...")] => 
    // TODO-    ver >= "VERSION" => transform generated container field/variant
    // TODO: ----- 

    for attr in attributes.into_iter() {
        if attr.path() != ATTR_COMPAT {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            // #[compat(name = foo)]
            if meta.path == NAME {
                let value = meta.value()?;
                span = Some(value.span());
                attr_name = Some(value.parse()?);

            // #[compat(type = Option<i32>)]
            } else if meta.path == TYPE {
                match orig_type {
                    None => {
                        let msg = format!("\"{TYPE}\" missing original field or enum variant \
                            type");
                        return Err(meta.error(msg));
                    },

                    Some(orig_type) => {
                        let value = meta.value()?;
                        span = Some(value.span());
                        let mut meta_type: Type = value.parse()?;
                        if let Some(placeholder) = outer.placeholder.as_ref() {
                            replace_compat_placeholder(
                                orig_type,
                                &mut meta_type,
                                placeholder)?;
                        }
                        attr_type = Some(meta_type);
                    },
                }

            // #[compat(map = Option::map)]
            } else if meta.path == MAP {
                let value = meta.value()?;
                span = Some(value.span());
                mapping = Some(value.parse()?);
            }
            Ok(())
        })?;
    }

    Ok(ParsedFieldAttr {
        span,
        name: attr_name,
        ty: attr_type,
        map_fn: mapping,
    })
}

/// Parses the #\[compat(...)\] helper attribute on the outer struct or enum definition.
///
/// eg.
/// ```
/// #[derive(GenerateCompat)]
/// #[compat(placeholder = P)] // <<< Parses this
/// struct Foo {
///     bar: i32,
/// }
/// ```
pub(crate) fn parse_outer_compat_attr<'a, I>(attributes: I) -> Result<ParsedOuterAttr>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut placeholder = None;

    for ast_attr in attributes.into_iter() {
        if ast_attr.path() != ATTR_COMPAT {
            continue;
        }

        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        let nested = ast_attr.parse_args_with(parser)?;
        for meta in nested.iter() {
            if meta.path() == PLACEHOLDER {
                match &meta.require_name_value()?.value {
                    Expr::Path(expr) => {
                        placeholder = expr.path.require_ident()?
                            .clone()
                            .into();
                    },

                    Expr::Lit(expr) => match &expr.lit {
                        Lit::Str(s) => {
                            placeholder = Some(s.parse()?);
                        },

                        lit => {
                            let msg = format!("unexpected \"{PLACEHOLDER}\": {lit:?}");
                            return Err(Error::new(meta.span(), msg));
                        },
                    },

                    expr => {
                        let msg = format!("unexpected \"{PLACEHOLDER}\": {expr:?}");
                        return Err(Error::new(meta.span(), msg));
                    },
                }
            }
        }
    }

    Ok(ParsedOuterAttr { placeholder })
}
