use quote::{quote, quote_spanned};
use semver::Version;
use syn::{
    Attribute, DeriveInput, Error, Meta, MetaList, Result, Token, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

use crate::compat::{CompatData, FieldOrVar, Kind};

/// Linearly searches `attributes` for #\[serde(...)\] attributes, returning a [`Vec`] of matching
/// [`Attribute`]s.
pub(crate) fn parse_serde_attr<'a, I>(attributes: I) -> Vec<Attribute>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    let mut serde_attrs = vec![];
    for attr in attributes.into_iter() {
        if attr.path().is_ident("serde") {
            serde_attrs.push(attr.to_owned());
        }
    }
    serde_attrs
}

/// Removes any #\[serde(...)\] arguments whose [`Ident`] matching `to_strip`.
///
/// eg. With `serde_attrs`: `#\[serde(rename_all = "kebab-case", deny_unknown_fields)],
/// and `to_strip = "rename_all"`, the returned attributes will be:
///
/// `#\[serde(deny_unknown_fields)\]`
///
/// [`Ident`]: struct@syn::Ident
fn strip_serde_attr(serde_attrs: Vec<Attribute>, to_strip: &str) -> Result<Vec<Attribute>> {
    let mut stripped_attrs = Vec::with_capacity(serde_attrs.len());
    for attr in serde_attrs.into_iter() {
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
            match meta.path().is_ident(to_strip) {
                // Flag that we need to construct a new attribute.
                true => needs_new_attr = true,
                // Only keep serde args that do not match `to_strip`.
                false => args.push(quote! { #meta }),
            }
        }

        let attr = match needs_new_attr {
            false => Some(attr), // Nothing changed, no need to reconstruct the serde args.
            true => match args.is_empty() {
                true => None, // We stripped all of the serde args (don't emit `#[serde()]`).
                false => {
                    // Reconstruct the attribute but without the stripped arguments.
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
                    Some(Attribute { meta, ..attr })
                },
            },
        };
        if let Some(attr) = attr {
            stripped_attrs.push(attr);
        }
    }
    Ok(stripped_attrs)
}

/// Linearly searches the attributes of `fv` for any #\[serde(...)\] attributes.
///
/// Returns a [`Vec`] containing only `serde` attributes (`Vec` in case multiple #\[serde(...)\]
/// attributes are defined).
///
/// Any `rename = "..."` serde argument will be stripped out if:
/// 
/// * `version >= 6.0.0`, or
/// * `#[renamed = "RENAMED_VER"]` is defined and `version >= RENAMED_VER`
///
/// This behavior assumes that any `rename = "..."` serde argument is used to override
/// container-level `rename_all = "..."` behavior, eg. `rename = "file-count"` to override
/// container-level `#[serde(rename_all = "camelCase")]`.
pub(crate) fn parse_serde_field_attr<'a>(
    version: &Version,
    fv: FieldOrVar<'a>,
    parsed: &CompatData,
) -> Result<Vec<Attribute>> {
    let mut field_serde = parse_serde_attr(fv.attributes());

    let renamed = parsed.changes.iter()
        .find_map(|(change_version, kind)| {
            match kind {
                Kind::Renamed(_) => Some(version >= change_version),
                _ => None,
            }
        })
        .unwrap_or(false);

    if version >= &Version::new(6, 0, 0) || renamed {
        field_serde = strip_serde_attr(field_serde, "rename")?;
    }
    Ok(field_serde)
}

/// Linearly searches the container-level attributes for any #\[serde(...)\] attributes.
///
/// Returns a [`Vec`] containing only `serde` attributes. (This is a `Vec` in case multiple
/// #\[serde(...)\] attributes are defined.)
///
/// If `version >= Version::new(6, 0, 0)`, the `rename_all = "..."` serde argument will be stripped
/// out if it was defined since we want to override with `rename_all = "snake_case"` (starting with
/// semver-6.0.0, all serialized strings were converted to `snake_case`).
pub(crate) fn parse_serde_container_attr(version: &Version, ast: &DeriveInput)
    -> Result<Vec<Attribute>>
{
    let mut container_serde: Vec<_> = parse_serde_attr(ast.attrs.iter());

    if version >= &Version::new(6, 0, 0) {
        // Search for any `#[serde(rename_all = ...)]` attribute; if found, drop it to avoid
        // generating a duplicate `rename_all` serde attribute for post- semver-6.0.0.
        container_serde = strip_serde_attr(container_serde, "rename_all")?;
        container_serde.push(parse_quote!{ #[serde(rename_all = "snake_case")] });
    }
    Ok(container_serde)
}
