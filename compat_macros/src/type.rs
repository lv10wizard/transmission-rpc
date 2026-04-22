use proc_macro2::Span;
use semver::Version;
use syn::{
    Error, GenericArgument, Ident, PathArguments, PathSegment, Result, Type,
    spanned::Spanned
};
use quote::{format_ident, quote};

use crate::symbols::compat_id;

/// Replaces `ty`'s inner-most generic argument with its corresponding version compat type, eg.
/// `Option<Vec<Foo>>` becomes `Option<Vec<__semver_xyz_compat_Foo>>`.
///
/// `ty` should be a [`Clone`]d version the field- or variant's [`Type`].
///
/// # Example
///
/// ```rust
/// // Defining a `SemverCompat`-derived struct with #[compat] flagged fields:
/// #[derive(SemverCompat, Serialize)]
/// struct Foo {
///     #[compat]
///     my_var: Bar,
///
///     #[compat]
///     my_other_var: Vec<Bar>,
/// }
///
/// // Defining a `SemverCompat`-derived struct used as an inner type for `Foo`'s fields:
/// #[derive(SemverCompat, Serialize)]
/// struct Bar {
///     bar: i32,
/// }
///
/// // Should generate compat versions like:
/// pub(crate) struct __semver_xyz_compat_Foo {
///     my_var: __semver_xyz_compat_Bar,
///     my_other_var: Vec<__semver_xyz_compat_Bar>,
/// }
/// pub(crate) struct __semver_xyz_compat_Bar {
///     bar: i32,
/// }
/// ```
pub(crate) fn replace_with_compat_type(
    version: &Version,
    ty: Option<&mut Type>,
) -> Result<()> {
    let Some(ty) = ty else {
        return Ok(())
    };

    match ty {
        Type::Path(ty_path) => {
            // We only care about the last item in the path, eg.
            // foo::bar::Vec<Option<MyType>>
            //           ^^^^^^^^^^^^^^^^^^^ last segment
            let Some(ty_last) = ty_path.path.segments.last_mut() else {
                // I don't think this can happen (implies a trailing `::`, eg. `foo::bar::`).
                return Err(Error::new(ty.span(), "type should have a last segment"))
            };

            match &mut ty_last.arguments {
                // This segment's identifier is the type we want to replace with its generated
                // compat version's type.
                PathArguments::None => {
                    ty_last.ident = compat_id(version, &ty_last.ident);
                    Ok(())
                },

                PathArguments::AngleBracketed(ty_brackets) => {
                    match ty_brackets.args.len() {
                        1 => {
                            let ty = ty_brackets.args
                                .get_mut(0)
                                .map(|arg| match arg {
                                    GenericArgument::Type(ty) => Ok(ty),
                                    _ => {
                                        let msg = "unsupported generic argument";
                                        Err(Error::new(arg.span(), msg))
                                    },
                                })
                                .transpose()?;
                            replace_with_compat_type(version, ty)
                        },
                        _ => {
                            let msg = "unsupported number of generic arguments";
                            Err(Error::new(ty_brackets.args.span(), msg))
                        },
                    }
                },

                segment => Err(Error::new(segment.span(), "unsupported type")),
            }
        },

        _ => return Err(Error::new(ty.span(), "unsupported type")),
    }
}

/// Parses `ty` to determine which mapping function to employ to convert the source-defined
/// original struct field type into its semver-compat type.
///
/// Specifically, this maps the following types:
///
/// * `Option<T>` -> [`Option::map`]
/// * `Option<Vec<T>>` -> See: [`gen_opt_vec_into_func`]
/// * `Vec<T>` -> See: [`gen_vec_into_func`]
/// * `T` -> [`Into::into`] via a wrapper function conforming to `Option::map`'s signature (see:
/// [`gen_into_wrapper_func`])
///
/// Returns:
///
/// * `Ok(..)` - The conversion function [`Ident`] and tokens defining it.
/// * `Err(_)` - If there was an error parsing the type.
pub(crate) fn determine_which_into_func(ty: Option<&Type>)
    -> Result<(Ident, proc_macro2::TokenStream)>
{
    let Some(ty) = ty else {
        return Ok((ident_into_wrapper(), gen_into_wrapper_func()));
    };

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
                // Determine if we're using Option::map or `opt_vec_into`.
                let seg = extract_option_arg_type_path(ty.span(), seg)?;
                return Ok({
                    match seg.ident == "Vec" {
                        true => (ident_opt_vec_into(), gen_opt_vec_into_func()?),
                        false => (ident_opt_into(), gen_opt_into_func()?),
                    }
                });

            } else if seg.ident == "Vec" {
                // Just assume whatever Vec<T> maps cleanly with `vec_into` (eg. doesn't handle
                // something like `Vec<Option<T>>` -> `Vec<Option<U>>`).
                return Ok((ident_vec_into(), gen_vec_into_func()?));
            }

           Ok((ident_into_wrapper(), gen_into_wrapper_func()))
        },

        _ => Err(Error::new(ty.span(), "unsupported type")),
    }
}

fn extract_option_arg_type_path(span: Span, seg: &PathSegment) -> Result<&PathSegment> {
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
        return Err(Error::new(span, "expected Option<T>"))
    };

    opt_ty_path.path.segments.last()
        .ok_or(Error::new(span, "Option<T> should have a last segment"))
}

fn ident_into_wrapper() -> Ident {
    format_ident!("into_wrapper")
}

fn ident_opt_into() -> Ident {
    format_ident!("opt_into")
}

fn ident_vec_into() -> Ident {
    format_ident!("vec_into")
}

fn ident_opt_vec_into() -> Ident {
    format_ident!("opt_vec_into")
}

/// Generates tokens defining a function intended as an [`Into::into`] wrapper conforming to
/// [`Option::map`]'s signature.
fn gen_into_wrapper_func() -> proc_macro2::TokenStream {
    let into_wrapper = ident_into_wrapper();
    quote! {
        #[automatically_derived]
        fn #into_wrapper<T, U>(t: T) -> U
        where
            T: Into<U>,
        {
            t.into()
        }
    }
}

/// Generates tokens defining a [`Option::map`] wrapper.
fn gen_opt_into_func() -> Result<proc_macro2::TokenStream> {
    let opt_into = ident_opt_into();
    Ok(quote! {
        #[automatically_derived]
        fn #opt_into<T, U>(opt: Option<T>) -> Option<U>
        where
            T: Into<U>,
        {
            opt.map(Into::into)
        }
    })
}

/// Generates tokens defining a `vec_into` function to convert a `Vec<T>` into a `Vec<U>` by
/// iterating over the input vec and applying [`Into::into`] on each item.
fn gen_vec_into_func() -> Result<proc_macro2::TokenStream> {
    let vec_into = ident_vec_into();
    Ok(quote! {
        #[automatically_derived]
        fn #vec_into<T, U>(vec: Vec<T>) -> Vec<U>
        where
            T: Into<U>,
        {
            vec.into_iter()
                .map(Into::into)
                .collect()
        }
    })
}

/// Generates tokens defining a `opt_vec_into` function to convert a `Option<Vec<T>>` into a
/// `Option<Vec<U>>`.
fn gen_opt_vec_into_func() -> Result<proc_macro2::TokenStream> {
    let vec_into = ident_vec_into();
    let opt_vec_into = ident_opt_vec_into();
    let vec_into_defn = gen_vec_into_func()?;
    Ok(quote! {
        #[automatically_derived]
        fn #opt_vec_into<T, U>(opt: Option<Vec<T>>) -> Option<Vec<U>>
        where
            T: Into<U>,
        {
            opt.map(|vec| {
                #vec_into_defn

                #vec_into(vec)
            })
        }
    })
}
