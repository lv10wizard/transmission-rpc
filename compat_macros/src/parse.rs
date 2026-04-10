use std::fmt::Debug;

use proc_macro2::Span;
use quote::{quote, quote_spanned};
use semver::Version;
use syn::{
    Attribute, DeriveInput, Error, Expr, Ident, Lit, LitStr, Meta, MetaList, Path, Result, Token,
    Type, parse_quote,
    parse::ParseBuffer,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

use crate::{
    compat::FieldOrVar,
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
pub(crate) fn parse_field_compat_attr<'a, I>( // TODO: DELETE
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
                        Lit::Str(s) => placeholder = Some(s.parse()?),
                        lit => return Err(placeholder_err(expr.span(), lit)),
                    },

                    expr => return Err(placeholder_err(expr.span(), expr)),
                }

                break;
            }
        }
    }

    Ok(ParsedOuterAttr { placeholder })
}

fn placeholder_err<T: Debug>(span: Span, got: T) -> Error {
    let msg = format!("\"{PLACEHOLDER}\" expected string literal or Ident, got: {got:?}");
    Error::new(span, msg)
}

/// Linearly searches `attributes` for #\[serde(...)\] attributes, returning a [`Vec`] of matching
/// [`Attribute`]s.
fn parse_serde_attr<'a, I>(attributes: I) -> Vec<&'a Attribute>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut serde_attrs = vec![];
    for attr in attributes.into_iter() {
        if attr.path().is_ident("serde") {
            serde_attrs.push(attr);
        }
    }
    serde_attrs
}

/// Linearly searches the attributes of `fv` for any #\[serde(...)\] attributes.
///
/// Returns a [`Vec`] containing only `serde` attributes (`Vec` in case multiple #\[serde(...)\]
/// attributes are defined).
///
/// This always returns an empty [`Vec`] if `version` >= `Version::new(6, 0, 0)` because we
/// specifically want to ignore any `rename` serde attributes since transmission unified all rpc
/// strings to snake_case in semver-6.0.0.
pub(crate) fn parse_serde_field_attr<'a>(version: &Version, fv: FieldOrVar<'a>)
    -> Vec<&'a Attribute>
{
    if version >= &Version::new(6, 0, 0) {
        return Vec::new();
    }

    parse_serde_attr(fv.attributes())
    // TODO: parse nested arguments => strip out `rename`
}

/// Linearly searches the container-level attributes for any #\[serde(...)\] attributes.
///
/// Returns a [`Vec`] containing only `serde` attributes. If `version` >= Version::new(6, 0, 0),
/// the `rename_all = ...` serde argument will be stripped out (if it was defined).
pub(crate) fn parse_serde_container_attr(version: &Version, ast: &DeriveInput)
    -> Result<Vec<Attribute>>
{
    let mut container_serde: Vec<_> = parse_serde_attr(ast.attrs.iter())
        .into_iter()
        .map(ToOwned::to_owned)
        .collect();

    if version >= &Version::new(6, 0, 0) {
        // Search for any `#[serde(rename_all = ...)]` attribute; if found, drop it to avoid
        // generating a duplicate `rename_all` serde attribute for post- semver-6.0.0.
        let mut serde_attrs = Vec::with_capacity(container_serde.len());
        for attr in container_serde.into_iter() {
            // Parse each #[serde] argument.
            // eg. #[serde(rename_all = "...", untagged, ...)]
            //             ^^^^^^^^^^^^^^^^^^  ^^^^^^^^  ^^^
            let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
            let nested = match attr.parse_args_with(parser) {
                Ok(parsed) => parsed,
                Err(err) => return Err({
                    let msg = format!("unexpected #[serde] attribute: {err}");
                    Error::new(attr.span(), msg)
                }),
            };

            let mut needs_new_attr = false;
            let mut args = Vec::with_capacity(nested.len());
            for meta in nested.iter() {
                match meta.path().is_ident("rename_all") {
                    // Flag that we need to construct a new attribute.
                    true => needs_new_attr = true,
                    // Only keep non-`rename_all` serde args.
                    false => args.push(quote! { meta }),
                }
            }
            
            serde_attrs.push(match needs_new_attr {
                true => {
                    // Reconstruct the attribute but without the `rename_all` argument.
                    let meta = match &attr.meta {
                        // eg. #[serde(untagged, rename_all = "...")]
                        Meta::List(ml) => { // I think this is the only possible case.
                            MetaList {
                                tokens: quote_spanned! {attr.span()=>
                                    #(#args),*
                                },
                                ..ml.clone()
                            }.into()
                        },

                        // eg. #[serde = ...] or #[serde]
                        // I don't think this can happen.
                        meta => return Err({
                            let msg = format!("unexpected serde attribute: {meta:?}");
                            Error::new(attr.span(), msg)
                        }),
                    };
                    Attribute { meta, ..attr }
                },
                false => attr,
            });
        }
        container_serde = serde_attrs;
        container_serde.push(parse_quote!{ #[serde(rename_all = "snake_case")] });
    }
    Ok(container_serde)
}
