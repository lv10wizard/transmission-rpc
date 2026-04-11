use semver::Version;
use syn::{
    Error, GenericArgument, Ident, Path, PathArguments, Result, Type, parse_quote,
    spanned::Spanned as _,
};
use quote::{ToTokens, format_ident, quote};

use crate::symbols::compat_id;

/// Recursively replaces [`Ident`]s found in `meta` matching `placeholder` with the `orig` type's
/// corresponding compat type (via [`compat_id`]).
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
pub(crate) fn replace_compat_placeholder(
    version: &Version,
    orig: &Type,
    meta: &mut Type,
    placeholder: &Ident,
) -> Result<()> {
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
                            (*meta).ident = compat_id(version, &orig.ident);
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
                                                    version,
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
                                            version,
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
            replace_compat_placeholder(
                version,
                &orig_array.elem,
                &mut meta_array.elem,
                placeholder)
        },

        (Type::Tuple(orig_tuple), Type::Tuple(meta_tuple)) => {
            for (orig, meta) in orig_tuple.elems.iter()
                .zip(meta_tuple.elems.iter_mut())
                {
                    replace_compat_placeholder(version, orig, meta, placeholder)?;
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

/// Parses `ty` to determine which mapping function to employ to convert the source-defined
/// original struct field type into its placeholder-replaced type.
///
/// Specifically, this maps the following types:
///
/// * `Option<T>` -> [`Option::map`]
/// * `Option<Vec<T>>` -> See: [`gen_opt_vec_into_func`]
/// * `Vec<T>` -> See: [`gen_vec_into_func`]
/// * `_` -> [`Into::into`] via a wrapper function conforming to `Option::map`'s signature (see:
/// [`gen_into_wrapper_func`])
pub(crate) fn determine_which_into_func(ty: &Type) -> Result<Path> {
    let into_wrapper = ident_into_wrapper();
    match ty {
        Type::Path(ty_path) => {
            // We only care about the last segment of the type path, eg:
            //     std::vec::Vec<Option<i32>>
            //               ^^^^^^^^^^^^^^^^
            // This could be made more robust by parsing each path segment to verify that the type
            // is actually the one we expect rather than just assuming that it is.
            let seg = ty_path.path.segments.last()
                .expect("type should have a last segment");
            
            if seg.ident == "Option" {
                let span = ty.span(); // TODO: fix span (should be #[compat(type = ...)] span)
                let PathArguments::AngleBracketed(bracketed) = &seg.arguments else {
                    // Either a bare `Option` or `Option(T)`...
                    return Err(Error::new(span, "unexpected Option arguments"))
                };
                if bracketed.args.len() != 1 {
                    // This is some other kind of `Option`...
                    return Err(Error::new(span, "unexpected custom Option type"));
                }
                let opt_arg = bracketed.args.last()
                    // This can't happen. The previous checks should have filtered this case out.
                    .expect("Option should have an argument type (how did this happen?)");
                let GenericArgument::Type(opt_ty) = opt_arg else {
                    // I don't think this can happen. Implies something like `Option<'a>`.
                    return Err(Error::new(span, "unexpected Option generic argument"))
                };
                let Type::Path(opt_ty_path) = opt_ty else {
                    // Only handle Option<T> Path types
                    let msg = format!("unexpected Option type: {opt_ty:?}");
                    return Err(Error::new(span, msg))
                };

                // Determine if we're using Option::map or `opt_vec_into`.
                let seg = opt_ty_path.path.segments.last()
                    .expect("Option<T> should have a last segment");
                return Ok(match seg.ident == "Vec" {
                    true => {
                        let opt_vec_into = ident_opt_vec_into();
                        parse_quote! { #opt_vec_into }
                    },
                    false => parse_quote! { Option::map },
                });

            } else if seg.ident == "Vec" {
                // Just assume whatever Vec<T> maps cleanly with `vec_into` (eg. doesn't handle
                // something like `Vec<Option<T>>` -> `Vec<Option<U>>`).
                let vec_into = ident_vec_into();
                return Ok(parse_quote!{ #vec_into });
            }

            Ok(parse_quote! { #into_wrapper })
        },

        _ => Ok(parse_quote! { #into_wrapper }), // TODO: This might be an error instead?
    }
}

pub(crate) fn ident_into_wrapper() -> Ident {
    format_ident!("into_wrapper")
}

fn ident_vec_into() -> Ident {
    format_ident!("vec_into")
}

fn ident_opt_vec_into() -> Ident {
    format_ident!("opt_vec_into")
}

/// Generates tokens defining a function intended as an [`Into::into`] wrapper conforming to
/// [`Option::map`]'s signature.
pub(crate) fn gen_into_wrapper_func() -> proc_macro2::TokenStream {
    let into_wrapper = ident_into_wrapper();
    quote! {
        fn #into_wrapper<F, T, U>(t: T, func: F) -> U
        where
            F: Fn(T) -> U,
        {
            func(t)
        }
    }
}


/// Generates tokens defining a `vec_into` function to convert a `Vec<T>` into a `Vec<U>` by
/// iterating over the input vec and applying `func` on each item.
pub(crate) fn gen_vec_into_func() -> proc_macro2::TokenStream {
    let vec_into = ident_vec_into();
    quote! {
        fn #vec_into<F, T, U>(vec: Vec<T>, func: F) -> Vec<U>
        where
            F: Fn(T) -> U,
        {
            vec.into_iter()
                .map(func)
                .collect()
        }
    }
}

/// Generates tokens defining a `opt_vec_into` function to convert a `Option<Vec<T>>` into a
/// `Option<Vec<U>>`.
pub(crate) fn gen_opt_vec_into_func() -> proc_macro2::TokenStream {
    let vec_into = ident_vec_into();
    let opt_vec_into = ident_opt_vec_into();
    let define_vec_into = gen_vec_into_func();
    quote! {
        fn #opt_vec_into<F, T, U>(opt: Option<Vec<T>>, func: F) -> Option<Vec<U>>
        where
            F: Fn(T) -> U,
        {
            opt.map(|vec| {
                #define_vec_into

                #vec_into(vec, func)
            })
        }
    }
}
