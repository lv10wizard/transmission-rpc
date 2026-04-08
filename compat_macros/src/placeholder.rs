use syn::{Error, GenericArgument, Ident, PathArguments, Result, Type};
use quote::{ToTokens, quote};

use crate::symbols::compat_id;

/// Recursively replaces [`Ident`]s found in `meta` matching `placeholder` with the compat-type's
/// [`Ident`] with [`compat_id`].
///
/// The `orig` and `meta` [`Type`]s must match forms:
///
/// eg.
///
/// > orig: `Foo`
///
/// > meta: `P`
///
/// eg.
///
/// > orig: `Option<Vec<Foo>>`
///
/// > meta: `Option<Vec<P>>`
///
/// eg.
/// Defining [`GenerateCompat`]-derived structs like:
/// ```rust
/// #[derive(GenerateCompat)]
/// #[compat(placeholder = compat_placeholder)]
/// struct Foo {
///     #[compat(type = Vec<compat_placeholder>)]
///     //              ^^^^^^^^^^^^^^^^^^^^^^^ `meta`
///     bar: Vec<Bar>,
///     //   ^^^^^^^^ `orig`
/// }
///
/// #[derive(GenerateCompat)]
/// struct Bar {
///     some_bar_field: i32, // unchanged
/// }
/// ```
///
/// Would generate corresponding compat structs like:
/// ```rust
/// struct __semver_600_compat_Foo {
///     bar: Vec<__semver_600_compat_Bar>,
/// }
///
/// struct __semver_600_compat_Bar {
///     some_bar_field: i32, // unchanged
/// }
/// ```
///
/// [`Ident`]: struct@syn::Ident
/// [`GenerateCompat`]: crate::GenerateCompat
pub(crate) fn replace_compat_placeholder(orig: &Type, meta: &mut Type, placeholder: &Ident)
    -> Result<()>
{
    match (orig, meta) {
        (Type::Path(orig_path), Type::Path(meta_path)) => {
            // We only care about the last item in the path, eg.
            // foo::bar::Vec<Option<MyType>>
            //           ^^^^^^^^^^^^^^^^^^^ last segment
            match (orig_path.path.segments.last(), meta_path.path.segments.last_mut()) {
                (Some(orig), Some(meta)) => {
                    match &meta.ident == placeholder {
                        // The segment's type identifier is the compat placeholder.
                        true => {
                            (*meta).ident = {
                                let tmp = semver::Version::new(6, 0, 0);
                                compat_id(&tmp, &orig.ident)
                            };
                            Ok(())
                        },

                        false => match (&orig.arguments, &mut meta.arguments) {
                            (PathArguments::None, PathArguments::None) => Ok(()),

                            // HashMap<K, V>
                            //        ^^^^^^
                            (PathArguments::AngleBracketed(orig),
                             PathArguments::AngleBracketed(meta)) =>
                            {
                                // Replace each <T> that matches the compat placeholder.
                                for (orig, meta) in orig.args.iter()
                                    .zip(meta.args.iter_mut())
                                    {
                                        match (orig, meta) {
                                            (GenericArgument::Type(orig),
                                             GenericArgument::Type(meta)) =>
                                            {
                                                replace_compat_placeholder(
                                                    orig,
                                                    meta,
                                                    placeholder)?;
                                            }

                                            (orig, meta) => return check_mismatch(orig, meta),
                                        }
                                    }
                                Ok(())
                            },

                            // Fn(A, B) -> C
                            //   ^^^^^^^^^^^
                            (PathArguments::Parenthesized(orig),
                             PathArguments::Parenthesized(meta)) =>
                            {
                                // Replace each (T) that matches the compat placeholder.
                                for (orig, meta) in orig.inputs.iter()
                                    .zip(meta.inputs.iter_mut())
                                    {
                                        replace_compat_placeholder(
                                            orig,
                                            meta,
                                            placeholder)?;
                                    }
                                Ok(())
                            },

                            (orig, meta) => check_mismatch(orig, meta),
                        },
                    }
                },

                (orig, meta) => check_mismatch(orig, meta.as_deref()),
            }
        },

        (Type::Array(orig_array), Type::Array(meta_array)) => {
            replace_compat_placeholder(&orig_array.elem, &mut meta_array.elem, placeholder)
        },

        (Type::Tuple(orig_tuple), Type::Tuple(meta_tuple)) => {
            for (orig, meta) in orig_tuple.elems.iter()
                .zip(meta_tuple.elems.iter_mut())
                {
                    replace_compat_placeholder(orig, meta, placeholder)?;
                }
            Ok(())
        },

        (orig, meta) => check_mismatch(orig, meta),
    }
}

fn check_mismatch<T, U>(orig: T, meta: U) -> Result<()>
where
    T: PartialEq<U> + ToTokens,
    U: PartialEq<T> + ToTokens,
{
    match orig == meta {
        true => Ok(()),
        false => {
            let msg = {
                let orig = quote! { #orig };
                let meta = quote! { #meta };
                format!("mismatched placeholder types (\"{orig}\" != \"{meta}\")")
            };
            Err(Error::new_spanned(meta, msg))
        },
    }
}
